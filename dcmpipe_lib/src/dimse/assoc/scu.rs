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
    core::defn::{ts::TSRef, uid::UIDRef, vr::UI},
    dict::uids::DICOMApplicationContextName,
    dimse::{
        assoc::{CommonAssoc, DimseMsg},
        error::{AssocError, DimseError},
        pdus::{
            mainpdus::{
                AbstractSyntaxItem, ApplicationContextItem, AssocRQ, AssocRQPresentationContext,
                ReleaseRQ, TransferSyntaxItem, UserInformationItem,
            },
            userpdus::{AsyncOperationsWindowItem, MaxLengthItem, RoleSelectionItem},
            Pdu, PduType, UserPdu,
        },
        AeTitle,
    },
};

pub struct UserAssoc {
    common: CommonAssoc,
    service_ae: String,
    ctx_id_counter: u8,
    msg_id_counter: u16,
}

impl UserAssoc {
    /// Access the fields & functions that are common to both SCU and SCP associations.
    #[must_use]
    pub fn common(&self) -> &CommonAssoc {
        &self.common
    }

    /// Produces the next message ID that should be used for the `MessageID` field in requests.
    #[must_use]
    pub fn next_msg_id(&mut self) -> u16 {
        let msg_id = self.msg_id_counter;
        self.msg_id_counter += 1;
        msg_id
    }

    /// Initiate the association.
    ///
    /// # Return
    /// This returns an `Option<DimseMsg>`, which, if the association is accepted/negotiated
    /// successfully, will be `None`. If the association is rejected or aborted then this returns a
    /// `Some(DimseMsg)` to indicate which response was received.
    ///
    /// # Errors
    /// - I/O errors may occur with the reader/writer.
    /// - `DimseError` may be returned if: an unexpected PDU was received during negotiation, or if
    /// no presentation contexts could be negotiated.
    pub fn request_association<R: Read, W: Write>(
        &mut self,
        reader: R,
        mut writer: W,
    ) -> Result<Option<DimseMsg>, AssocError> {
        let called_ae = AeTitle::try_from(self.service_ae.trim())
            .map_err(|e| AssocError::error(DimseError::OtherError(e.into())))?;
        let calling_ae = AeTitle::try_from(self.common.this_ae.trim())
            .map_err(|e| AssocError::error(DimseError::OtherError(e.into())))?;

        let mut app_ctx = DICOMApplicationContextName.uid().as_bytes().to_vec();
        if app_ctx.len() % 2 != 0 {
            app_ctx.push(UI.padding);
        }
        let app_ctx = ApplicationContextItem::new(app_ctx);

        let transfer_syntaxes = self
            .common
            .supported_ts
            .iter()
            .map(|ts| TransferSyntaxItem::new(ts.uid().uid().as_bytes().to_vec()))
            .collect::<Vec<TransferSyntaxItem>>();

        let mut ab_by_ctxid: HashMap<u8, UIDRef> =
            HashMap::with_capacity(self.common.supported_abs.len());
        let mut pres_ctxs: Vec<AssocRQPresentationContext> =
            Vec::with_capacity(self.common.supported_abs.len());
        for ab in &self.common.supported_abs {
            let ctx_id = self.ctx_id_counter;
            self.ctx_id_counter += 2;

            let abstract_syntax = AbstractSyntaxItem::new(ab.uid().as_bytes().to_vec());
            let pc =
                AssocRQPresentationContext::new(ctx_id, abstract_syntax, transfer_syntaxes.clone());
            ab_by_ctxid.insert(ctx_id, *ab);
            pres_ctxs.push(pc);
        }

        let user_info = UserInformationItem::new(self.common.this_user_data.clone());

        let rq = AssocRQ::new(
            called_ae.into(),
            calling_ae.into(),
            app_ctx,
            pres_ctxs,
            user_info,
        );

        CommonAssoc::write_pdu(&Pdu::AssocRQ(rq), &mut writer)?;

        let response = Pdu::read(reader).map_err(AssocError::ab_failure)?;
        let Pdu::AssocAC(ac) = response else {
            return CommonAssoc::handle_disconnect(response, &mut writer).map(Some);
        };

        self.common.their_user_data.clear();
        self.common
            .their_user_data
            .append(ac.user_info().user_data().clone().as_mut());

        // TODO: Do things with SOPClassCommonExtendedNegotiationItem, UserIdentityItem, etc.

        for pres_ctx in ac.pres_ctxs() {
            if pres_ctx.is_accepted() {
                let Some(ab_uid) = ab_by_ctxid.get(&pres_ctx.ctx_id()) else {
                    continue;
                };
                self.common
                    .negotiated_pres_ctx
                    .insert(pres_ctx.ctx_id(), (pres_ctx.to_owned(), ab_uid));
            }
        }

        if self.common.negotiated_pres_ctx.is_empty() {
            return Err(AssocError::ab_failure(DimseError::GeneralError(
                "No presentation contexts negotiated".to_owned(),
            )));
        }

        Ok(None)
    }

    /// Release the association and confirm the RELEASE-RP
    ///
    /// # Errors
    /// - I/O errors may occur with the reader/writer.
    /// - `DimseError` may occur for protocol errors.
    /// - Other `DimseError::GeneralError` may occur if a RELEASE-RP was not received in response.
    pub fn release_association<R: Read, W: Write>(
        &mut self,
        reader: &mut R,
        mut writer: &mut W,
    ) -> Result<Option<DimseMsg>, AssocError> {
        CommonAssoc::write_pdu(&Pdu::ReleaseRQ(ReleaseRQ::new()), &mut writer)?;
        match self.common.next_msg(reader, &mut writer)? {
            DimseMsg::ReleaseRP => Ok(Some(DimseMsg::ReleaseRP)),
            other => Err(AssocError::error(DimseError::GeneralError(format!(
                "Did not get response for {:?}: {other:?}",
                PduType::ReleaseRQ
            )))),
        }
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
        let mut this_user_data = Vec::<UserPdu>::new();
        this_user_data.push(UserPdu::MaxLengthItem(MaxLengthItem::new(
            self.pdu_rcv_max_len,
        )));

        // Require synchronous transfers, until async is incorporated.
        this_user_data.push(UserPdu::AsyncOperationsWindowItem(
            AsyncOperationsWindowItem::new(1, 1),
        ));

        for ab in &self.supported_abs {
            this_user_data.push(UserPdu::RoleSelectionItem(RoleSelectionItem::new(
                ab.uid().into(),
                1,
                0,
            )));
        }

        let num_abs = self.supported_abs.len();
        let num_user_data = this_user_data.len();

        let common = CommonAssoc {
            _id: self.id,
            this_ae: self.my_ae,
            supported_abs: self.supported_abs,
            supported_ts: self.supported_ts,
            this_user_data,
            their_user_data: Vec::with_capacity(num_user_data),
            negotiated_pres_ctx: HashMap::with_capacity(num_abs),
        };

        UserAssoc {
            common,
            service_ae: self.service_ae,
            ctx_id_counter: 1,
            msg_id_counter: 0,
        }
    }
}
