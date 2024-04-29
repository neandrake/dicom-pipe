use std::io::Read;

use crate::{
    core::{
        dcmobject::DicomRoot,
        defn::constants::ts::ImplicitVRLittleEndian,
        read::{stop::ParseStop, ParserBuilder, ParserState},
    },
    dict::stdlookup::STANDARD_DICOM_DICTIONARY,
    dimse::{
        commands::messages::CommandMessage,
        pdus::{Pdu, PresentationDataItemPartial, PresentationDataValueHeader},
        DimseError,
    },
};

#[derive(Debug)]
pub enum PduIterItem {
    Pdu(Pdu),
    CmdMessage(PresentationDataValueHeader, CommandMessage),
    Dataset(PresentationDataValueHeader),
}

#[derive(Debug)]
pub enum PduIterState {
    ReadPdu,
    ReadPdiVal,
    ReadCmdMessage,
}

pub struct PduIter<R: Read> {
    stream: R,
    state: PduIterState,

    pdi: Option<PresentationDataItemPartial>,
    pdvh: Option<PresentationDataValueHeader>,
}

impl<R: Read> PduIter<R> {
    pub fn new(stream: R) -> Self {
        Self {
            stream,
            state: PduIterState::ReadPdu,
            pdi: None,
            pdvh: None,
        }
    }
}

impl<R: Read> Iterator for PduIter<R> {
    type Item = Result<PduIterItem, DimseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                PduIterState::ReadPdu => {
                    let pdu = Pdu::read(&mut self.stream);
                    if let Ok(Pdu::PresentationDataItemPartial(pdi)) = pdu {
                        self.pdi = Some(pdi);
                        self.state = PduIterState::ReadPdiVal;
                        continue;
                    }
                    return Some(pdu.map(PduIterItem::Pdu));
                }
                PduIterState::ReadPdiVal => {
                    let pdvh = PresentationDataValueHeader::read(&mut self.stream);
                    match pdvh {
                        Ok(pdvh) => {
                            let is_cmd = pdvh.is_command();
                            let is_last_fragment = pdvh.is_last_fragment();

                            if is_cmd {
                                self.pdvh = Some(pdvh);
                                self.state = PduIterState::ReadCmdMessage;
                                continue;
                            }
                            if is_last_fragment {
                                self.pdi.take();
                                self.state = PduIterState::ReadPdu;
                            } else {
                                self.state = PduIterState::ReadPdiVal;
                            }
                            return Some(Ok(PduIterItem::Dataset(pdvh)));
                        }
                        Err(e) => return Some(Err(e)),
                    }
                }
                PduIterState::ReadCmdMessage => {
                    let Some(pdvh) = self.pdvh.take() else {
                        return Some(Err(DimseError::InvalidPduParseState(format!(
                            "In state {:?} but no PresentationDataValueHeader is set",
                            self.state
                        ))));
                    };
                    // Command P-DATA always uses IVRLE. Use the standard dicom dictinoary for
                    // parsing, otherwise anything pulling values from the command dataset will
                    // have to explicitly specify the VR to parse values as since they would
                    // otherwise all be `UN`.
                    let mut parser = ParserBuilder::default()
                        .state(ParserState::ReadElement)
                        .dataset_ts(&ImplicitVRLittleEndian)
                        .stop(ParseStop::AfterBytesRead(u64::from(pdvh.length_of_data())))
                        .build(&mut self.stream, &STANDARD_DICOM_DICTIONARY);
                    match DicomRoot::parse(&mut parser) {
                        Ok(Some(cmd_root)) => {
                            if pdvh.is_last_fragment() {
                                self.pdi.take();
                                self.state = PduIterState::ReadPdu;
                            } else {
                                self.pdvh = Some(pdvh.clone());
                                self.state = PduIterState::ReadPdiVal;
                            }
                            return Some(Ok(PduIterItem::CmdMessage(
                                pdvh.clone(),
                                CommandMessage::new(cmd_root),
                            )));
                        }
                        Ok(None) => return None,
                        Err(source) => return Some(Err(DimseError::ParseError(source))),
                    }
                }
            }
        }
    }
}

pub struct DimseMsg {
    cmd: CommandMessage,
    ctx_id: u8,
    dcm_len: u32,
}

impl DimseMsg {
    pub fn new(cmd: CommandMessage, ctx_id: u8) -> Self {
        Self {
            cmd,
            ctx_id,
            dcm_len: 0,
        }
    }

    pub fn new_with_dcm(cmd: CommandMessage, ctx_id: u8, dcm_len: u32) -> Self {
        Self {
            cmd,
            ctx_id,
            dcm_len,
        }
    }

    #[must_use]
    pub fn cmd(&self) -> &CommandMessage {
        &self.cmd
    }

    #[must_use]
    pub fn ctx_id(&self) -> u8 {
        self.ctx_id
    }

    #[must_use]
    pub fn dcm_len(&self) -> u32 {
        self.dcm_len
    }
}

pub struct DimseMsgIter<R: Read> {
    reader: R,
}

impl<R: Read> DimseMsgIter<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: Read> Iterator for DimseMsgIter<R> {
    type Item = Result<DimseMsg, DimseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut last_cmd: Option<DimseMsg> = None;
            let pdu_iter = PduIter::new(&mut self.reader);
            for iter_item in pdu_iter {
                match iter_item {
                    Ok(PduIterItem::Pdu(pdu)) => {
                        return Some(Err(DimseError::UnexpectedPduType(pdu.pdu_type())));
                    }
                    Ok(PduIterItem::CmdMessage(pdvh, cmd)) => {
                        if !cmd.has_dataset() {
                            return Some(Ok(DimseMsg::new(cmd, pdvh.ctx_id())));
                        }
                        last_cmd = Some(DimseMsg::new(cmd, pdvh.ctx_id()));
                    }
                    Ok(PduIterItem::Dataset(pdvh)) => {
                        let Some(cmd) = last_cmd else {
                            return Some(Err(DimseError::InvalidPduParseState(
                                "DimseMsgIter expecting Dataset without prior Command.".to_owned(),
                            )));
                        };
                        return Some(Ok(DimseMsg::new_with_dcm(
                            cmd.cmd,
                            cmd.ctx_id,
                            pdvh.length_of_data(),
                        )));
                    }
                    Err(err) => {
                        return Some(Err(err));
                    }
                };
            }
        }
    }
}
