/*
   Copyright 2024 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::{
    collections::{HashMap, HashSet},
    io::{Read, Write},
};

use crate::{
    core::defn::{ts::TSRef, uid::UIDRef},
    dict::uids::{DICOMApplicationContextName, VerificationSOPClass},
    dimse::{
        assoc::{serialize, DimseMsg},
        commands::{messages::CommandMessage, CommandStatus},
        error::{AssocError, DimseError},
        pdus::{
            mainpdus::{
                Abort, AbstractSyntaxItem, ApplicationContextItem, AssocACPresentationContext,
                AssocRQ, AssocRQPresentationContext, PresentationDataItem, PresentationDataValue,
                ReleaseRP, ReleaseRQ, TransferSyntaxItem, UserInformationItem, P_DATA_CMD_LAST,
            },
            pduiter::{PduIter, PduIterItem},
            userpdus::{AsyncOperationsWindowItem, MaxLengthItem, RoleSelectionItem},
            Pdu, UserPdu,
        },
        AeTitle,
    },
};

pub struct UserAssoc {
    /* Fields configured by this SCU. */
    _id: usize,
    my_ae: String,
    service_ae: String,
    supported_abs: HashSet<UIDRef>,
    supported_ts: HashSet<TSRef>,
    my_user_data: Vec<UserPdu>,
    ctx_id_counter: u8,
    msg_id_counter: u16,
    requested_pres_ctx: HashMap<UIDRef, AssocRQPresentationContext>,

    /* Fields negotiated with other SCU. */
    their_user_data: Vec<UserPdu>,
    negotiated_pres_ctx: HashMap<u8, AssocACPresentationContext>,
}

impl UserAssoc {
    /// Write the given PDU to the given writer.
    ///
    /// # Errors
    /// I/O errors may occur when writing to the writer or flushing the writer.
    pub fn write_pdu<W: Write>(&self, pdu: &Pdu, mut writer: &mut W) -> Result<(), AssocError> {
        pdu.write(&mut writer).map_err(AssocError::error)?;
        writer
            .flush()
            .map_err(|e| AssocError::error(DimseError::IOError(e)))?;
        Ok(())
    }

    /// Retrieve the accepted presentation context by the given context ID.
    #[must_use]
    pub fn get_pres_ctx(&self, ctx_id: u8) -> Option<&AssocACPresentationContext> {
        self.negotiated_pres_ctx.get(&ctx_id)
    }

    #[must_use]
    pub fn get_rq_pres_ctx_by_ab(&self, ab: UIDRef) -> Option<&AssocRQPresentationContext> {
        self.requested_pres_ctx.get(ab)
    }

    #[must_use]
    pub fn get_pdu_max_snd_size(&self) -> u32 {
        for user_pdu in &self.their_user_data {
            if let UserPdu::MaxLengthItem(mli) = user_pdu {
                return mli.max_length();
            }
        }
        // 0 means no limit specified.
        0
    }

    #[must_use]
    pub fn get_pdu_max_rcv_size(&self) -> u32 {
        for user_pdu in &self.my_user_data {
            if let UserPdu::MaxLengthItem(mli) = user_pdu {
                return mli.max_length();
            }
        }
        // 0 means no limit specified.
        0
    }

    #[must_use]
    fn next_msg_id(&mut self) -> u16 {
        let msg_id = self.msg_id_counter;
        self.msg_id_counter += 1;
        msg_id
    }

    /// Handles a PDU that is not a `PresentationDataItem`, after the association is negotiated. In
    /// this scenario the only valid PDUs are `ReleaseRQ` or `Abort`.
    fn handle_disconnect<W: Write>(
        &self,
        pdu: Pdu,
        writer: &mut W,
    ) -> Result<DimseMsg, AssocError> {
        match pdu {
            Pdu::AssocRJ(rj) => Ok(DimseMsg::Reject(rj)),
            Pdu::ReleaseRQ(_) => {
                self.write_pdu(&Pdu::ReleaseRP(ReleaseRP::new()), writer)?;
                Ok(DimseMsg::ReleaseRQ)
            }
            Pdu::Abort(ab) => Ok(DimseMsg::Abort(ab)),
            other => {
                self.write_pdu(&Pdu::Abort(Abort::new(2, 2)), writer)?;
                Err(AssocError::error(DimseError::UnexpectedPduType(
                    other.pdu_type(),
                )))
            }
        }
    }

    /// Initiate the association.
    ///
    /// # Errors
    /// I/O errors may occur with the reader/writer.
    /// An error will be returned if the association cannot be negotiated.
    pub fn request_association<R: Read, W: Write>(
        &mut self,
        reader: R,
        mut writer: W,
    ) -> Result<Option<DimseMsg>, AssocError> {
        let called_ae = AeTitle::try_from(self.service_ae.trim())
            .map_err(|e| AssocError::error(DimseError::OtherError(e.into())))?;
        let calling_ae = AeTitle::try_from(self.my_ae.trim())
            .map_err(|e| AssocError::error(DimseError::OtherError(e.into())))?;

        let app_ctx =
            ApplicationContextItem::new(DICOMApplicationContextName.uid().as_bytes().to_vec());

        let transfer_syntaxes = self
            .supported_ts
            .iter()
            .map(|ts| TransferSyntaxItem::new(ts.uid().uid().as_bytes().to_vec()))
            .collect::<Vec<TransferSyntaxItem>>();

        let mut pres_ctxs: Vec<AssocRQPresentationContext> =
            Vec::with_capacity(self.supported_abs.len());
        for ab in &self.supported_abs {
            let ctx_id = self.ctx_id_counter;
            self.ctx_id_counter += 2;

            let abstract_syntax = AbstractSyntaxItem::new(ab.uid().as_bytes().to_vec());
            let pc =
                AssocRQPresentationContext::new(ctx_id, abstract_syntax, transfer_syntaxes.clone());
            self.requested_pres_ctx.insert(ab, pc.clone());
            pres_ctxs.push(pc);
        }

        let user_info = UserInformationItem::new(self.my_user_data.clone());

        let rq = AssocRQ::new(
            called_ae.into(),
            calling_ae.into(),
            app_ctx,
            pres_ctxs,
            user_info,
        );

        self.write_pdu(&Pdu::AssocRQ(rq), &mut writer)?;

        let response = Pdu::read(reader).map_err(AssocError::ab_failure)?;
        let Pdu::AssocAC(ac) = response else {
            return self.handle_disconnect(response, &mut writer).map(Some);
        };

        self.their_user_data.clear();
        self.their_user_data
            .append(ac.user_info().user_data().clone().as_mut());

        // TODO: Do things with SOPClassCommonExtendedNegotiationItem, UserIdentityItem, etc.

        for pres_ctx in ac.pres_ctxs() {
            if pres_ctx.is_accepted() {
                self.negotiated_pres_ctx
                    .insert(pres_ctx.ctx_id(), pres_ctx.to_owned());
            }
        }

        if self.negotiated_pres_ctx.is_empty() {
            return Err(AssocError::ab_failure(DimseError::GeneralError(
                "No presentation contexts negotiated".to_owned(),
            )));
        }

        Ok(None)
    }

    /// Issue a C-ECHO request.
    ///
    /// # Errors
    /// I/O errors may occur with the reader/write.
    /// Errors will also be returned if there are protocol errors.
    /// An error will be returned if the response is not successful.
    pub fn c_echo_rq<R: Read, W: Write>(
        &mut self,
        reader: R,
        mut writer: W,
    ) -> Result<Option<DimseMsg>, AssocError> {
        let Some(pres_ctx) = self.get_rq_pres_ctx_by_ab(&VerificationSOPClass) else {
            return Err(AssocError::error(DimseError::UnsupportedAbstractSyntax(
                &VerificationSOPClass,
            )));
        };

        let ctx_id = pres_ctx.ctx_id();
        let msg_id = self.next_msg_id();
        let rq = CommandMessage::c_echo_req(ctx_id, msg_id, VerificationSOPClass.uid());
        let data = serialize(rq.message())?;
        let cmd = PresentationDataValue::new(ctx_id, P_DATA_CMD_LAST, data);
        let pres_data_item = PresentationDataItem::new(vec![cmd]);
        self.write_pdu(&Pdu::PresentationDataItem(pres_data_item), &mut writer)?;

        let rp = PduIter::new(reader).next();
        let Some(rp) = rp else {
            return Err(AssocError::ab_failure(DimseError::GeneralError(
                "Did not get response".to_owned(),
            )));
        };

        let rp = rp.map_err(AssocError::ab_failure)?;
        let rp_msg = match rp {
            PduIterItem::Pdu(pdu) => {
                return self.handle_disconnect(pdu, &mut writer).map(Some);
            }
            PduIterItem::CmdMessage(rp_cmd) => rp_cmd,
            PduIterItem::Dataset(_ds) => {
                return Err(AssocError::ab_failure(DimseError::GeneralError(
                    "Got dataset instead of command".to_owned(),
                )))
            }
        };

        if rp_msg.status() != &CommandStatus::success() {
            return Err(AssocError::ab_failure(DimseError::GeneralError(format!(
                "Response status is not success: {:?}",
                rp_msg.status()
            ))));
        }

        self.write_pdu(&Pdu::ReleaseRQ(ReleaseRQ::new()), &mut writer)?;
        Ok(None)
    }
}

#[derive(Default, Clone)]
pub struct UserAssocBuilder {
    id: usize,
    my_ae: String,
    service_ae: String,
    supported_abs: HashSet<UIDRef>,
    supported_ts: HashSet<TSRef>,
    pdu_rcv_max_len: u32,
}

impl UserAssocBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    #[must_use]
    pub fn my_ae(mut self, my_ae: String) -> Self {
        self.my_ae = my_ae;
        self
    }

    #[must_use]
    pub fn service_ae(mut self, service_ae: String) -> Self {
        self.service_ae = service_ae;
        self
    }

    #[must_use]
    pub fn supported_abs(mut self, supported_abs: HashSet<UIDRef>) -> Self {
        self.supported_abs = supported_abs;
        self
    }

    #[must_use]
    pub fn supported_ts(mut self, supported_ts: HashSet<TSRef>) -> Self {
        self.supported_ts = supported_ts;
        self
    }

    #[must_use]
    pub fn pdu_rcv_max_len(mut self, pdu_rcv_max_len: u32) -> Self {
        self.pdu_rcv_max_len = pdu_rcv_max_len;
        self
    }

    #[must_use]
    pub fn build(self) -> UserAssoc {
        let mut my_user_data = Vec::<UserPdu>::new();
        my_user_data.push(UserPdu::MaxLengthItem(MaxLengthItem::new(
            self.pdu_rcv_max_len,
        )));

        // Require synchronous transfers, until async is incorporated.
        my_user_data.push(UserPdu::AsyncOperationsWindowItem(
            AsyncOperationsWindowItem::new(1, 1),
        ));

        for ab in &self.supported_abs {
            my_user_data.push(UserPdu::RoleSelectionItem(RoleSelectionItem::new(
                ab.uid().into(),
                1,
                0,
            )));
        }

        let num_abs = self.supported_abs.len();
        let num_user_data = my_user_data.len();

        UserAssoc {
            _id: self.id,
            my_ae: self.my_ae,
            service_ae: self.service_ae,
            supported_abs: self.supported_abs,
            supported_ts: self.supported_ts,
            my_user_data,
            ctx_id_counter: 1,
            msg_id_counter: 0,
            requested_pres_ctx: HashMap::with_capacity(num_abs),

            their_user_data: Vec::with_capacity(num_user_data),
            negotiated_pres_ctx: HashMap::with_capacity(num_abs),
        }
    }
}
