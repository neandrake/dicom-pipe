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

use std::io::{Cursor, Read, Write};

use crate::{
    core::{
        charset::CSRef,
        dcmelement::DicomElement,
        dcmobject::DicomRoot,
        defn::{
            constants::tags::FILE_META_GROUP_END, dcmdict::DicomDictionary, tag::Tag, ts::TSRef,
        },
        read::{ParseError, Parser, ParserBuilder, ParserState},
        RawValue,
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::{QueryRetrieveLevel, SOPClassUID, SOPInstanceUID, SpecificCharacterSet},
        transfer_syntaxes::ImplicitVRLittleEndian,
        uids::{
            PatientRootQueryRetrieveInformationModelFIND,
            PatientRootQueryRetrieveInformationModelGET,
            PatientRootQueryRetrieveInformationModelMOVE,
            StudyRootQueryRetrieveInformationModelFIND, StudyRootQueryRetrieveInformationModelGET,
            StudyRootQueryRetrieveInformationModelMOVE, VerificationSOPClass,
        },
    },
    dimse::{
        assoc::{CommonAssoc, DimseMsg, QueryLevel},
        commands::{messages::CommandMessage, CommandPriority},
        error::{AssocError, DimseError},
        pdus::{mainpdus::PresentationDataItem, pduiter::PresDataIter},
    },
};

pub enum AssocUserOp {
    Echo(EchoUserOp),
    Find(FindUserOp),
    Get(GetUserOp),
    Move(MoveUserOp),
    Store(StoreUserOp),
}

pub struct EchoUserOp {
    msg_id: u16,
    is_complete: bool,
}

impl EchoUserOp {
    #[must_use]
    pub fn new(msg_id: u16) -> Self {
        Self {
            msg_id,
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn create_req(&self, assoc: &CommonAssoc) -> Result<CommandMessage, AssocError> {
        let (pres_ctx, _ts) = assoc.get_rq_pres_ctx_and_ts_by_ab(&VerificationSOPClass)?;
        let ctx_id = pres_ctx.ctx_id();
        Ok(CommandMessage::c_echo_req(
            ctx_id,
            self.msg_id,
            VerificationSOPClass.uid(),
        ))
    }

    pub fn process_rsp<R: Read, W: Write>(
        &mut self,
        _reader: R,
        _writer: W,
        msg: &CommandMessage,
    ) -> Result<(), AssocError> {
        self.is_complete = true;
        if !msg.status().is_success() {
            return Err(AssocError::ab_failure(DimseError::UnexpectedCommandStatus(
                msg.status().clone(),
            )));
        }
        Ok(())
    }
}

pub struct FindUserOp {
    msg_id: u16,
    max_pdu_rcv_size: usize,
    ts: TSRef,
    is_complete: bool,
}

impl FindUserOp {
    #[must_use]
    pub fn new(msg_id: u16, max_pdu_rcv_size: usize) -> Self {
        Self {
            msg_id,
            max_pdu_rcv_size,
            ts: &ImplicitVRLittleEndian,
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn create_req(
        &mut self,
        assoc: &CommonAssoc,
        ql: QueryLevel,
        query: Vec<(&Tag, RawValue)>,
    ) -> Result<(CommandMessage, DicomRoot), AssocError> {
        let sop_class_uid = match ql {
            QueryLevel::Patient => &PatientRootQueryRetrieveInformationModelFIND,
            _ => &StudyRootQueryRetrieveInformationModelFIND,
        };
        let (pres_ctx, ts) = assoc.get_rq_pres_ctx_and_ts_by_ab(sop_class_uid)?;
        self.ts = ts;

        let ctx_id = pres_ctx.ctx_id();
        let cmd = CommandMessage::c_find_req(ctx_id, self.msg_id, sop_class_uid.uid());

        let mut dcm_query = DicomRoot::new_empty(ts, CSRef::default());
        dcm_query.add_child_with_val(&QueryRetrieveLevel, RawValue::of_string(ql.as_str()));
        for (tag, val) in query {
            dcm_query.add_child_with_val(tag, val);
        }
        Ok((cmd, dcm_query))
    }

    pub fn process_rsp<R: Read, W: Write>(
        &mut self,
        mut reader: R,
        mut writer: W,
        msg: &CommandMessage,
    ) -> Result<Option<DicomRoot>, AssocError> {
        if !msg.has_dataset() || !msg.status().is_pending() {
            self.is_complete = true;
            return Ok(None);
        }

        let mut buf = Vec::<u8>::with_capacity(self.max_pdu_rcv_size);
        loop {
            let next_msg = CommonAssoc::next_msg(&mut reader, &mut writer, self.max_pdu_rcv_size);
            let dimse_msg = match next_msg {
                Ok(dimse_msg) => dimse_msg,
                Err(e) => return Err(e),
            };
            match dimse_msg {
                DimseMsg::Dataset(pdv) => {
                    let is_last = pdv.is_last_fragment();
                    buf.append(pdv.into_data().as_mut());
                    if is_last {
                        break;
                    }
                }
                DimseMsg::Cmd(cmd) => {
                    return Err(AssocError::ab_failure(DimseError::DimseDicomMissing(
                        DimseMsg::Cmd(cmd),
                    )));
                }
                DimseMsg::CloseMsg(close_msg) => return Err(AssocError::handled_close(close_msg)),
            }
        }

        if !buf.is_empty() {
            let mut buf = Cursor::new(buf);
            let mut parser = ParserBuilder::default()
                .state(ParserState::ReadElement)
                .dataset_ts(self.ts)
                .build(&mut buf, &STANDARD_DICOM_DICTIONARY);
            match DicomRoot::parse(&mut parser) {
                Ok(Some(dcm_root)) => Ok(Some(dcm_root)),
                Ok(None) => Err(AssocError::ab_failure(DimseError::ParseError(
                    ParseError::GeneralDecodeError("Failed parsing dicom dataset".to_owned()),
                ))),
                Err(err) => Err(AssocError::ab_failure(DimseError::from(err))),
            }
        } else {
            Ok(None)
        }
    }
}

pub struct StoreUserOp {
    msg_id: u16,
    max_pdu_snd_size: usize,
    is_complete: bool,
}

impl StoreUserOp {
    #[must_use]
    pub fn new(msg_id: u16, max_pdu_snd_size: usize) -> Self {
        Self {
            msg_id,
            max_pdu_snd_size,
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn create_req<'p, PR: Read + 'p>(
        &self,
        assoc: &CommonAssoc,
        parser: Parser<'p, PR>,
        store_msg_id: u16,
        origin_ae: &str,
        orig_msg_id: u16,
    ) -> Result<
        (
            CommandMessage,
            Box<dyn Iterator<Item = Result<PresentationDataItem, DimseError>> + 'p>,
        ),
        AssocError,
    > {
        let mut parser = parser
            .filter_map(Result::ok)
            // Do not transfer any beginning FileMeta elements.
            .skip_while(|e| e.tag() <= FILE_META_GROUP_END);

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
            .and_then(|s| CSRef::lookup_charset(&s))
            .unwrap_or_default();
        let sop_class_uid = sop_class_uid
            .and_then(|s| STANDARD_DICOM_DICTIONARY.get_uid_by_uid(&s))
            .ok_or_else(|| {
                AssocError::ab_failure(DimseError::DimseElementMissing(
                    SOPClassUID.ident().to_owned(),
                ))
            })?;
        let sop_inst_uid = sop_inst_uid.ok_or_else(|| {
            AssocError::ab_failure(DimseError::DimseElementMissing(
                SOPInstanceUID.ident().to_owned(),
            ))
        })?;

        let (pres_ctx, ts) = assoc.get_rq_pres_ctx_and_ts_by_ab(sop_class_uid)?;
        let ctx_id = pres_ctx.ctx_id();
        let priority = CommandPriority::Medium;
        let cmd = CommandMessage::c_store_req(
            ctx_id,
            store_msg_id,
            &priority,
            sop_class_uid.uid(),
            &sop_inst_uid,
            origin_ae,
            orig_msg_id,
        );

        let pdi_iter = Box::new(PresDataIter::new(
            ctx_id,
            self.max_pdu_snd_size,
            false,
            stitched_elems,
            ts,
            spec_char_set,
        ));

        Ok((cmd, pdi_iter))
    }

    pub fn process_rsp(&mut self, msg: &CommandMessage) {
        self.is_complete |= !msg.status().is_pending();
    }
}

pub struct GetUserOp {
    msg_id: u16,
    is_complete: bool,
}

impl GetUserOp {
    #[must_use]
    pub fn new(msg_id: u16) -> Self {
        Self {
            msg_id,
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn create_req(
        &mut self,
        assoc: &CommonAssoc,
        ql: QueryLevel,
        query: Vec<(&Tag, RawValue)>,
    ) -> Result<(CommandMessage, DicomRoot), AssocError> {
        let sop_class_uid = match ql {
            QueryLevel::Patient => &PatientRootQueryRetrieveInformationModelGET,
            _ => &StudyRootQueryRetrieveInformationModelGET,
        };
        let (pres_ctx, ts) = assoc.get_rq_pres_ctx_and_ts_by_ab(sop_class_uid)?;

        let ctx_id = pres_ctx.ctx_id();
        let cmd = CommandMessage::c_get_req(ctx_id, self.msg_id, sop_class_uid.uid());

        let mut dcm_query = DicomRoot::new_empty(ts, CSRef::default());
        dcm_query.add_child_with_val(&QueryRetrieveLevel, RawValue::of_string(ql.as_str()));
        for (tag, val) in query {
            dcm_query.add_child_with_val(tag, val);
        }

        Ok((cmd, dcm_query))
    }

    pub fn process_rsp<R: Read, W: Write>(
        &mut self,
        mut _reader: R,
        mut _writer: W,
        msg: &CommandMessage,
    ) -> Result<(), AssocError> {
        self.is_complete |= !msg.status().is_pending();
        Ok(())
    }
}

pub struct MoveUserOp {
    msg_id: u16,
    is_complete: bool,
}

impl MoveUserOp {
    #[must_use]
    pub fn new(msg_id: u16) -> Self {
        Self {
            msg_id,
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn create_req(
        &mut self,
        assoc: &CommonAssoc,
        dest_ae: &str,
        ql: QueryLevel,
        query: Vec<(&Tag, RawValue)>,
    ) -> Result<(CommandMessage, DicomRoot), AssocError> {
        let sop_class_uid = if ql == QueryLevel::Patient {
            &PatientRootQueryRetrieveInformationModelMOVE
        } else {
            &StudyRootQueryRetrieveInformationModelMOVE
        };

        let (pres_ctx, ts) = assoc.get_rq_pres_ctx_and_ts_by_ab(sop_class_uid)?;
        let ctx_id = pres_ctx.ctx_id();
        let cmd = CommandMessage::c_move_req(ctx_id, self.msg_id, sop_class_uid.uid(), dest_ae);

        let mut dcm_root = DicomRoot::new_empty(ts, CSRef::default());
        dcm_root.add_child_with_val(&QueryRetrieveLevel, RawValue::of_string(ql.as_str()));
        for (tag, val) in query {
            dcm_root.add_child_with_val(tag, val);
        }

        Ok((cmd, dcm_root))
    }

    pub fn process_rsp<R: Read, W: Write>(
        &mut self,
        mut _reader: R,
        mut _writer: W,
        msg: &CommandMessage,
    ) -> Result<(), AssocError> {
        self.is_complete |= !msg.status().is_pending();
        Ok(())
    }
}
