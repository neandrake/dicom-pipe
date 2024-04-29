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
        charset::{lookup_charset, DEFAULT_CHARACTER_SET},
        dcmelement::DicomElement,
        dcmobject::DicomRoot,
        defn::{dcmdict::DicomDictionary, tag::Tag, ts::TSRef, uid::UIDRef, vr::UI},
        read::Parser,
        RawValue,
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::{QueryRetrieveLevel, SOPClassUID, SOPInstanceUID, SpecificCharacterSet},
        uids::{
            DICOMApplicationContextName, PatientRootQueryRetrieveInformationModelFIND,
            StudyRootQueryRetrieveInformationModelFIND, VerificationSOPClass,
        },
    },
    dimse::{
        assoc::{CommonAssoc, DimseMsg, QueryLevel},
        commands::{messages::CommandMessage, CommandPriority},
        error::{AssocError, DimseError},
        pdus::{
            mainpdus::{
                AbstractSyntaxItem, ApplicationContextItem, AssocACPresentationContext, AssocRQ,
                AssocRQPresentationContext, ReleaseRQ, TransferSyntaxItem, UserInformationItem,
            },
            pduiter::{CommandIter, PresDataIter},
            userpdus::{AsyncOperationsWindowItem, MaxLengthItem, RoleSelectionItem},
            Pdu, PduType, UserPdu,
        },
        AeTitle, Syntax,
    },
};

pub struct UserAssoc {
    common: CommonAssoc,
    service_ae: String,
    ctx_id_counter: u8,
    msg_id_counter: u16,
    requested_pres_ctx: HashMap<UIDRef, AssocRQPresentationContext>,
}

impl UserAssoc {
    /// Retrieve the accepted presentation context and its negotiated transfer syntax, by the given
    /// abstract syntax.
    ///
    /// # Errors
    /// `AssocError` may occur if the requested or negotiated presentation context cannot be
    /// resolved, or if a known transfer syntax for it cannot be resolved.
    pub fn get_rq_pres_ctx_and_ts_by_ab(
        &self,
        ab: UIDRef,
    ) -> Result<(&AssocACPresentationContext, TSRef), AssocError> {
        let Some(pres_ctx) = self.requested_pres_ctx.get(ab) else {
            return Err(AssocError::ab_failure(DimseError::GeneralError(format!(
                "Requested Presentation Context not found by abstract syntax: {}",
                ab.uid()
            ))));
        };

        let Some(pres_ctx) = self.common.negotiated_pres_ctx.get(&pres_ctx.ctx_id()) else {
            return Err(AssocError::ab_failure(DimseError::GeneralError(format!(
                "Negotiated Presentation Context not found by ctx_id: {}",
                pres_ctx.ctx_id()
            ))));
        };

        let ts = String::try_from(&Syntax(pres_ctx.transfer_syntax().transfer_syntaxes()))
            .ok()
            .and_then(|v| STANDARD_DICOM_DICTIONARY.get_ts_by_uid(&v))
            .ok_or_else(|| {
                AssocError::ab_failure(DimseError::GeneralError(
                    "Failed to resolve transfer syntax".to_string(),
                ))
            })?;

        Ok((pres_ctx, ts))
    }

    #[must_use]
    fn next_msg_id(&mut self) -> u16 {
        let msg_id = self.msg_id_counter;
        self.msg_id_counter += 1;
        msg_id
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

        let mut pres_ctxs: Vec<AssocRQPresentationContext> =
            Vec::with_capacity(self.common.supported_abs.len());
        for ab in &self.common.supported_abs {
            let ctx_id = self.ctx_id_counter;
            self.ctx_id_counter += 2;

            let abstract_syntax = AbstractSyntaxItem::new(ab.uid().as_bytes().to_vec());
            let pc =
                AssocRQPresentationContext::new(ctx_id, abstract_syntax, transfer_syntaxes.clone());
            self.requested_pres_ctx.insert(ab, pc.clone());
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

        self.common.write_pdu(&Pdu::AssocRQ(rq), &mut writer)?;

        let response = Pdu::read(reader).map_err(AssocError::ab_failure)?;
        let Pdu::AssocAC(ac) = response else {
            return self
                .common
                .handle_disconnect(response, &mut writer)
                .map(Some);
        };

        self.common.their_user_data.clear();
        self.common
            .their_user_data
            .append(ac.user_info().user_data().clone().as_mut());

        // TODO: Do things with SOPClassCommonExtendedNegotiationItem, UserIdentityItem, etc.

        for pres_ctx in ac.pres_ctxs() {
            if pres_ctx.is_accepted() {
                self.common
                    .negotiated_pres_ctx
                    .insert(pres_ctx.ctx_id(), pres_ctx.to_owned());
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
    /// I/O errors may occur with the reader/writer.
    /// `DimseError` may occur for protocol errors.
    /// Other `DimseError::GeneralError` may occur if a RELEASE-RP was not received in response.
    pub fn release_association<R: Read, W: Write>(
        &mut self,
        reader: &mut R,
        mut writer: &mut W,
    ) -> Result<Option<DimseMsg>, AssocError> {
        self.common
            .write_pdu(&Pdu::ReleaseRQ(ReleaseRQ::new()), &mut writer)?;
        match self.common.next_msg(reader, &mut writer)? {
            DimseMsg::ReleaseRP => Ok(Some(DimseMsg::ReleaseRP)),
            other => Err(AssocError::error(DimseError::GeneralError(format!(
                "Did not get response for {:?}: {other:?}",
                PduType::ReleaseRQ
            )))),
        }
    }

    /// Issue a C-ECHO request.
    ///
    /// # Errors
    /// I/O errors may occur with the reader/writer.
    /// Errors will also be returned if there are protocol errors.
    /// An error will be returned if the response is not successful.
    pub fn c_echo_rq<R: Read, W: Write>(
        &mut self,
        mut reader: R,
        mut writer: W,
    ) -> Result<Option<DimseMsg>, AssocError> {
        let (pres_ctx, _ts) = self.get_rq_pres_ctx_and_ts_by_ab(&VerificationSOPClass)?;

        let ctx_id = pres_ctx.ctx_id();
        let msg_id = self.next_msg_id();
        let cmd = CommandMessage::c_echo_req(ctx_id, msg_id, VerificationSOPClass.uid());
        self.common.write_command(&cmd, &mut writer)?;

        let rsp_cmd = self.common.next_msg(&mut reader, &mut writer)?;
        if let DimseMsg::Dataset(_ds) = rsp_cmd {
            return Err(AssocError::ab_failure(DimseError::GeneralError(
                "Got dataset instead of command".to_owned(),
            )));
        }
        let DimseMsg::Cmd(rsp_msg) = rsp_cmd else {
            return Ok(Some(rsp_cmd));
        };

        if !rsp_msg.status().is_success() {
            return Err(AssocError::ab_failure(DimseError::GeneralError(format!(
                "Response status is not success: {:?}",
                rsp_msg.status()
            ))));
        }

        Ok(None)
    }

    /// Issues a C-FIND query.
    ///
    /// # Return
    /// An iterator over `(CommandMessage, Option<DicomRoot>)` for reading the results. If the
    /// status of the command message is `CommandStatus::Pending` then the `DicomRoot` should be
    /// present and contain the search result. All other statuses indicate the end of the stream.
    ///
    /// # Errors
    /// I/O errors may occur while using the reader/writer.
    /// `DimseError` may occur if no associated negotatiated presentation context can be found.
    pub fn c_find_req<R: Read, W: Write>(
        &mut self,
        reader: R,
        mut writer: W,
        ql: QueryLevel,
        dcm_query: Vec<(&Tag, RawValue)>,
    ) -> Result<CommandIter<R>, AssocError> {
        let sop_class_uid = match ql {
            QueryLevel::Patient => &PatientRootQueryRetrieveInformationModelFIND,
            _ => &StudyRootQueryRetrieveInformationModelFIND,
        };
        let (pres_ctx, ts) = self.get_rq_pres_ctx_and_ts_by_ab(sop_class_uid)?;

        let ctx_id = pres_ctx.ctx_id();
        let msg_id = self.next_msg_id();
        let cmd = CommandMessage::c_find_req(ctx_id, msg_id, sop_class_uid.uid());
        self.common.write_command(&cmd, &mut writer)?;

        let mut dcm_root = DicomRoot::new_empty(ts, DEFAULT_CHARACTER_SET);
        dcm_root.add_child_with_val(&QueryRetrieveLevel, RawValue::of_string(ql.as_str()));
        for (tag, val) in dcm_query {
            dcm_root.add_child_with_val(tag, val);
        }

        self.common.write_dataset(ctx_id, &dcm_root, &mut writer)?;

        Ok(CommandIter::new(
            reader,
            ts,
            self.common.get_pdu_max_rcv_size(),
        ))
    }

    /// Issue a C-STORE request.
    ///
    /// # Errors
    /// I/O errors may occur while using the reader/writer.
    /// `DimseError` may occur if no associated negotatiated presentation context can be found.
    pub fn c_store_req<PR: Read, R: Read, W: Write>(
        &mut self,
        mut reader: R,
        mut writer: W,
        parser: Parser<'_, PR>,
        origin_ae: &str,
        orig_msg_id: u16,
    ) -> Result<Option<DimseMsg>, AssocError> {
        let mut parser = parser.filter_map(Result::ok);

        let mut spec_char_set: Option<String> = None;
        let mut sop_class_uid: Option<String> = None;
        let mut sop_inst_uid: Option<String> = None;
        let mut header_elems: Vec<DicomElement> = Vec::new();
        for elem in parser.by_ref() {
            let tag = elem.tag();
            if tag == SpecificCharacterSet.tag() {
                spec_char_set = elem
                    .parse_value()
                    .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?
                    .string()
                    .cloned();
            } else if tag == SOPClassUID.tag() {
                sop_class_uid = elem
                    .parse_value()
                    .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?
                    .string()
                    .cloned();
            } else if tag == SOPInstanceUID.tag() {
                sop_inst_uid = elem
                    .parse_value()
                    .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?
                    .string()
                    .cloned();
            }
            header_elems.push(elem);
            if tag >= SOPInstanceUID.tag() {
                break;
            }
        }
        let stitched_elems = header_elems.into_iter().chain(parser);

        let spec_char_set = spec_char_set
            .and_then(|s| lookup_charset(&s))
            .unwrap_or(DEFAULT_CHARACTER_SET);
        let sop_class_uid = sop_class_uid
            .and_then(|s| STANDARD_DICOM_DICTIONARY.get_uid_by_uid(&s))
            .ok_or_else(|| {
                AssocError::ab_failure(DimseError::GeneralError(
                    "SOP Instance to send is missing SOPClassUID".to_owned(),
                ))
            })?;
        let sop_inst_uid = sop_inst_uid.ok_or_else(|| {
            AssocError::ab_failure(DimseError::GeneralError(
                "SOP Instance to send is missing SOPInstanceUID".to_owned(),
            ))
        })?;

        let (pres_ctx, ts) = self.get_rq_pres_ctx_and_ts_by_ab(sop_class_uid)?;
        let ctx_id = pres_ctx.ctx_id();
        let msg_id = self.next_msg_id();
        let priority = CommandPriority::Medium;
        let cmd = CommandMessage::c_store_req(
            ctx_id,
            msg_id,
            &priority,
            sop_class_uid.uid(),
            &sop_inst_uid,
            origin_ae,
            orig_msg_id,
        );
        self.common.write_command(&cmd, &mut writer)?;

        let pdi_iter = PresDataIter::new(
            ctx_id,
            self.common.get_pdu_max_snd_size(),
            false,
            stitched_elems,
            ts,
            spec_char_set,
        );
        for pdi in pdi_iter {
            match pdi {
                Ok(pdi) => self
                    .common
                    .write_pdu(&Pdu::PresentationDataItem(pdi), &mut writer)?,
                Err(e) => return Err(AssocError::ab_failure(e)),
            }
        }

        let rsp = self.common.next_msg(&mut reader, &mut writer)?;
        Ok(Some(rsp))
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
            requested_pres_ctx: HashMap::with_capacity(num_abs),
        }
    }
}
