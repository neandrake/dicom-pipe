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
    core::{
        charset::DEFAULT_CHARACTER_SET as CS,
        dcmobject::DicomRoot,
        defn::{dcmdict::DicomDictionary, ts::TSRef, uid::UIDRef},
        write::{builder::WriterBuilder, writer::WriterState},
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY, transfer_syntaxes::ImplicitVRLittleEndian,
        uids::DICOMApplicationContextName,
    },
    dimse::{
        commands::{messages::CommandMessage, CommandStatus},
        error::{AssocError, DimseError},
        pdus::{
            mainpdus::{
                Abort, AssocAC, AssocACPresentationContext, AssocRQ, PresentationDataItem,
                PresentationDataValue, ReleaseRP, TransferSyntaxItem, UserInformationItem,
                P_DATA_CMD_LAST, P_DATA_DCM_DATASET_LAST,
            },
            pduiter::{PduIter, PduIterItem},
            userpdus::{AsyncOperationsWindowItem, MaxLengthItem, RoleSelectionItem},
            Pdu, UserPdu,
        },
        Syntax,
    },
};

#[derive(Debug)]
pub enum DimseMsg {
    Cmd(CommandMessage),
    Dataset(PresentationDataValue),
    ReleaseRQ,
    Abort(Abort),
}

pub struct Association {
    /* Fields configured by this SCU. */
    _id: usize,
    host_ae: String,
    accept_aets: HashSet<String>,
    accept_abs: HashSet<UIDRef>,
    accept_ts: HashSet<TSRef>,
    my_user_data: Vec<UserPdu>,

    /* Fields negotiated with other SCU. */
    their_user_data: Vec<UserPdu>,
    negotiated_pres_ctx: HashMap<u8, AssocACPresentationContext>,
}

impl Association {
    /// Serialize the given `DicomRoot` into in-memory bytes.
    ///
    /// # Errors
    /// An I/O error may occur when writing to the in-memory `Vec`.
    pub fn serialize(dicom: &DicomRoot) -> Result<Vec<u8>, AssocError> {
        let mut ds_writer = WriterBuilder::default()
            .state(WriterState::Element)
            .ts(&ImplicitVRLittleEndian)
            .build(Vec::<u8>::new());
        ds_writer
            .write_dcmroot(dicom)
            .map_err(|e| AssocError::ab_invalid_pdu(DimseError::WriteError(e)))?;
        Ok(ds_writer.into_dataset())
    }

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
    pub fn get_pdu_max_snd_size(&self) -> u32 {
        for user_pdu in &self.their_user_data {
            if let UserPdu::MaxLengthItem(mli) = user_pdu {
                return mli.max_length();
            }
        }
        0
    }

    #[must_use]
    pub fn get_pdu_max_rcv_size(&self) -> u32 {
        for user_pdu in &self.my_user_data {
            if let UserPdu::MaxLengthItem(mli) = user_pdu {
                return mli.max_length();
            }
        }
        0
    }

    /// Accept the association request, negotiating the association parameters and leaving the
    /// reader/writer in a state to start exchanging PDUs.
    ///
    /// # Errors
    /// - I/O errors may occur when reading/writing from the reader/writer.
    /// - Any misbehaving SCU will be managed within, and this will not propagate these as errors.
    pub fn accept<R: Read, W: Write>(
        &mut self,
        mut reader: R,
        mut writer: W,
    ) -> Result<(), AssocError> {
        let rq = Pdu::read(&mut reader)
            .map_err(AssocError::ab_failure)
            .and_then(|rq| match rq {
                Pdu::AssocRQ(rq) => Ok(rq),
                pdu => Err(AssocError::ab_unexpected_pdu(
                    DimseError::UnexpectedPduType(pdu.pdu_type()),
                )),
            });

        let rq = match rq {
            Ok(rq) => rq,
            Err(e) => {
                if let Err(inner) = e.write(&mut writer) {
                    eprintln!("[ err xx]: Failure writing error response: {inner}");
                }
                return Err(AssocError::error(e.into_err()));
            }
        };

        let assoc_ac = match self.validate_assoc_rq(&rq) {
            Ok(rq) => rq,
            Err(e) => {
                if let Err(inner) = e.write(&mut writer) {
                    eprintln!("[ err xx]: Failure writing error response: {inner}");
                }
                return Err(AssocError::error(e.into_err()));
            }
        };

        for pres_ctx in assoc_ac.pres_ctxs() {
            if pres_ctx.is_accepted() {
                self.negotiated_pres_ctx
                    .insert(pres_ctx.ctx_id(), pres_ctx.to_owned());
            }
        }

        if self.negotiated_pres_ctx.is_empty() {
            return Err(AssocError::rj_failure(DimseError::GeneralError(
                "No presentation contexts negotiated".to_owned(),
            )));
        }

        self.write_pdu(&Pdu::AssocAC(assoc_ac), &mut writer)?;

        Ok(())
    }

    /// Validates the association request, checking that this `Association`'s configuration can
    /// handle and will accept the request.
    ///
    /// # Return
    /// Whether to accept, reject, or abort the association request. If accepted, also includes the
    /// accepted abstract and transfer syntaxes.
    ///
    /// # Errors
    /// If the result of the request is to reject or abort, those are propagated as an `AssocError`.
    fn validate_assoc_rq(&mut self, rq: &AssocRQ) -> Result<AssocAC, AssocError> {
        // There's only a single DICOM Standard defined Application Context Name. Private
        // Application Context Names are allowed by the standard.
        let app_ctx = String::try_from(&Syntax(rq.app_ctx().app_context_name()))
            .map_err(AssocError::rj_unsupported)?;

        let app_ctx_uid = STANDARD_DICOM_DICTIONARY
            .get_uid_by_uid(&app_ctx)
            .ok_or_else(|| {
                AssocError::rj_unsupported(DimseError::GeneralError(format!(
                    "Application Context not found: {app_ctx}"
                )))
            })?;

        if app_ctx_uid != &DICOMApplicationContextName {
            return Err(AssocError::rj_unsupported(DimseError::GeneralError(
                format!(
                    "Unsupported Application Context: {}, {}",
                    app_ctx_uid.name(),
                    app_ctx_uid.uid()
                ),
            )));
        }

        let host_ae = self.host_ae.trim();

        let calling_ae = CS
            .decode(rq.calling_ae())
            .map(|ae| ae.trim().to_owned())
            .map_err(|e| AssocError::ab_invalid_pdu(DimseError::from(e)))?;
        if !self.accept_aets.is_empty() && !self.accept_aets.contains(&calling_ae) {
            return Err(AssocError::rj_calling_aet(DimseError::GeneralError(
                format!("Calling AE Title \"{calling_ae}\" not in accepted list"),
            )));
        }

        let called_ae = CS
            .decode(rq.called_ae())
            .map(|ae| ae.trim().to_owned())
            .map_err(|e| AssocError::ab_invalid_pdu(DimseError::CharsetError(e)))?;
        if called_ae != host_ae {
            return Err(AssocError::rj_called_aet(DimseError::GeneralError(
                format!("Called AE \"{called_ae}\" is not host AE \"{host_ae}\""),
            )));
        }

        self.their_user_data
            .append(rq.user_info().user_data().clone().as_mut());
        /*
        for user_pdu in rq.user_info().user_data() {
            match user_pdu {
                UserPdu::SOPClassExtendedNegotiationItem(_)
                | UserPdu::SOPClassCommonExtendedNegotiationItem(_)
                | UserPdu::UserIdentityItem(_)
                | UserPdu::UserIdentityNegotiationItem(_) => {}
                _ => {}
            }
        }
        */

        // Check the proposed presentation contexts and create responses for each.
        let mut rsp_pres_ctx: Vec<AssocACPresentationContext> =
            Vec::with_capacity(rq.pres_ctxs().len());
        for req_pres_ctx in rq.pres_ctxs() {
            let ts = req_pres_ctx
                .transfer_syntaxes()
                .iter()
                .filter_map(|ts| String::try_from(&Syntax(ts.transfer_syntaxes())).ok())
                .filter_map(|ts| STANDARD_DICOM_DICTIONARY.get_ts_by_uid(&ts))
                .find(|ts| self.accept_ts.contains(ts));

            let Some(ts) = ts else {
                let deny_pres_ctx = AssocACPresentationContext::new(
                    req_pres_ctx.ctx_id(),
                    4,
                    TransferSyntaxItem::new(Vec::with_capacity(0)),
                );
                rsp_pres_ctx.push(deny_pres_ctx);
                continue;
            };

            let ab = String::try_from(&Syntax(req_pres_ctx.abstract_syntax().abstract_syntax()))
                .ok()
                .and_then(|ab| STANDARD_DICOM_DICTIONARY.get_uid_by_uid(&ab))
                .filter(|ab| self.accept_abs.contains(ab));

            if ab.is_none() {
                let deny_pres_ctx = AssocACPresentationContext::new(
                    req_pres_ctx.ctx_id(),
                    3,
                    TransferSyntaxItem::new(Vec::with_capacity(0)),
                );
                rsp_pres_ctx.push(deny_pres_ctx);
                continue;
            }

            let acc_pres_ctx = AssocACPresentationContext::new(
                req_pres_ctx.ctx_id(),
                0,
                TransferSyntaxItem::from(ts),
            );
            rsp_pres_ctx.push(acc_pres_ctx);
        }

        Ok(AssocAC::new(
            rq.called_ae().to_owned(),
            rq.calling_ae().to_owned(),
            rq.reserved_3().to_owned(),
            rq.app_ctx().to_owned(),
            rsp_pres_ctx,
            UserInformationItem::new(self.my_user_data.clone()),
        ))
    }

    /// Parse the next message, either a Command, DICOM Dataset, release/abort, or unexpected PDU.
    ///
    /// # Errors
    /// I/O errors may occcur attempting to read PDU from the reader, or write an appropriate
    /// disconnect response to the writer.
    pub fn next_msg<R: Read, W: Write>(
        &self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<DimseMsg, AssocError> {
        match PduIter::new(reader).next() {
            Some(Ok(PduIterItem::Pdu(pdu))) => self.handle_disconnect(pdu, writer),
            Some(Ok(PduIterItem::CmdMessage(cmd))) => Ok(DimseMsg::Cmd(cmd)),
            Some(Ok(PduIterItem::Dataset(dataset))) => Ok(DimseMsg::Dataset(dataset)),
            Some(Err(err)) => Err(AssocError::ab_failure(err)),
            None => Err(AssocError::ab_failure(DimseError::GeneralError(
                "No DIMSE message received".to_owned(),
            ))),
        }
    }

    /// Handles a PDU that is not a `PresentationDataItem`, after the association is negotiated. In
    /// this scenario the only valid PDUs are `ReleaseRQ` or `Abort`.
    fn handle_disconnect<W: Write>(
        &self,
        pdu: Pdu,
        writer: &mut W,
    ) -> Result<DimseMsg, AssocError> {
        match pdu {
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

    /// Create a C-ECHO ending response, as a `PresentationDataItem`.
    ///
    /// # Errors
    /// I/O errors may occur serializing the response object into `PresentationDataItem`.
    pub fn create_cecho_end(
        ctx_id: u8,
        msg_id: u16,
        aff_sop_class: &str,
    ) -> Result<Pdu, AssocError> {
        let status = CommandStatus::success();
        let rsp_cmd = CommandMessage::c_echo_rsp(ctx_id, msg_id, aff_sop_class, &status);

        let cmd_rsp_data = Association::serialize(rsp_cmd.message())?;
        let cmd_rsp_pdi =
            Pdu::PresentationDataItem(PresentationDataItem::new(vec![PresentationDataValue::new(
                ctx_id,
                P_DATA_CMD_LAST,
                cmd_rsp_data,
            )]));

        Ok(cmd_rsp_pdi)
    }

    /// Create a C-FIND result, as a pair of `PresentationDataItem` to be sent back to the SCU.
    ///
    /// # Errors
    /// I/O errors may occur serializing the response objects into `PresentationDataItem`.
    pub fn create_cfind_result(
        ctx_id: u8,
        msg_id: u16,
        aff_sop_class: &str,
        res: &DicomRoot,
    ) -> Result<(Pdu, Pdu), AssocError> {
        let status = CommandStatus::pending();
        let rsp_cmd = CommandMessage::c_find_rsp(ctx_id, msg_id, aff_sop_class, &status);

        let cmd_rsp_data = Association::serialize(rsp_cmd.message())?;
        let cmd_rsp_pdi =
            Pdu::PresentationDataItem(PresentationDataItem::new(vec![PresentationDataValue::new(
                ctx_id,
                P_DATA_CMD_LAST,
                cmd_rsp_data,
            )]));

        let dcm_rsp_data = Association::serialize(res)?;
        let dcm_rsp_pdi =
            Pdu::PresentationDataItem(PresentationDataItem::new(vec![PresentationDataValue::new(
                ctx_id,
                P_DATA_DCM_DATASET_LAST,
                dcm_rsp_data,
            )]));

        Ok((cmd_rsp_pdi, dcm_rsp_pdi))
    }

    /// Create a C-FIND ending response, as a `PresentationDataItem`.
    ///
    /// # Errors
    /// I/O errors may occur serializing the response object into `PresentationDataItem`.
    pub fn create_cfind_end(
        ctx_id: u8,
        msg_id: u16,
        aff_sop_class: &str,
    ) -> Result<Pdu, AssocError> {
        let status = CommandStatus::success();
        let rsp_cmd = CommandMessage::c_find_rsp(ctx_id, msg_id, aff_sop_class, &status);

        let cmd_rsp_data = Association::serialize(rsp_cmd.message())?;
        let cmd_rsp_pdi =
            Pdu::PresentationDataItem(PresentationDataItem::new(vec![PresentationDataValue::new(
                ctx_id,
                P_DATA_CMD_LAST,
                cmd_rsp_data,
            )]));

        Ok(cmd_rsp_pdi)
    }

    /// Create a C-STORE ending response, as a `PresentationDataItem`.
    ///
    /// # Errors
    /// I/O errors may occur serializing the response object into `PresentationDataItem`.
    pub fn create_cstore_end(
        ctx_id: u8,
        msg_id: u16,
        aff_sop_class: &str,
        status: &CommandStatus,
    ) -> Result<Pdu, AssocError> {
        let rsp_cmd = CommandMessage::c_store_rsp(ctx_id, msg_id, aff_sop_class, status);

        let cmd_rsp_data = Association::serialize(rsp_cmd.message())?;
        let cmd_rsp_pdi =
            Pdu::PresentationDataItem(PresentationDataItem::new(vec![PresentationDataValue::new(
                ctx_id,
                P_DATA_CMD_LAST,
                cmd_rsp_data,
            )]));

        Ok(cmd_rsp_pdi)
    }
}

#[derive(Default, Clone)]
pub struct AssociationBuilder {
    id: usize,
    host_ae: String,
    accept_aets: HashSet<String>,
    accept_abs: HashSet<UIDRef>,
    accept_ts: HashSet<TSRef>,
    pdu_rcv_max_len: u32,
}

impl AssociationBuilder {
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
    pub fn host_ae(mut self, host_ae: String) -> Self {
        self.host_ae = host_ae;
        self
    }

    #[must_use]
    pub fn accept_aets(mut self, accept_aets: HashSet<String>) -> Self {
        self.accept_aets = accept_aets;
        self
    }

    #[must_use]
    pub fn accept_abs(mut self, accept_abs: HashSet<UIDRef>) -> Self {
        self.accept_abs = accept_abs;
        self
    }

    #[must_use]
    pub fn accept_ts(mut self, accept_ts: HashSet<TSRef>) -> Self {
        self.accept_ts = accept_ts;
        self
    }

    #[must_use]
    pub fn pdu_rcv_max_len(mut self, pdu_rcv_max_len: u32) -> Self {
        self.pdu_rcv_max_len = pdu_rcv_max_len;
        self
    }

    #[must_use]
    pub fn build(self) -> Association {
        let mut my_user_data = Vec::<UserPdu>::new();
        my_user_data.push(UserPdu::MaxLengthItem(MaxLengthItem::new(
            self.pdu_rcv_max_len,
        )));

        // Require synchronous transfers, until async is incorporated.
        my_user_data.push(UserPdu::AsyncOperationsWindowItem(
            AsyncOperationsWindowItem::new(1, 1),
        ));

        // TODO: Double-check how roles should be set up. Based on behavior it seems like this is
        // indicating which roles to accept from incoming associations, and not what roles this
        // device enacts?
        for ab in &self.accept_abs {
            my_user_data.push(UserPdu::RoleSelectionItem(RoleSelectionItem::new(
                ab.uid().into(),
                1,
                0,
            )));
        }

        Association {
            _id: self.id,
            host_ae: self.host_ae,
            accept_aets: self.accept_aets,
            accept_abs: self.accept_abs,
            accept_ts: self.accept_ts,
            my_user_data,

            their_user_data: Vec::new(),
            negotiated_pres_ctx: HashMap::new(),
        }
    }
}
