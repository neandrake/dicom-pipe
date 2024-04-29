use std::io::Read;

use crate::{
    core::{
        dcmobject::DicomRoot,
        defn::constants::{lookup::MINIMAL_DICOM_DICTIONARY, ts::ImplicitVRLittleEndian},
        read::{stop::ParseStop, ParserBuilder, ParserState},
    },
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
                    // Command P-DATA always uses IVRLE.
                    let mut parser = ParserBuilder::default()
                        .state(ParserState::ReadElement)
                        .dataset_ts(&ImplicitVRLittleEndian)
                        .stop(ParseStop::AfterBytesRead(u64::from(pdvh.length_of_data())))
                        .build(&mut self.stream, &MINIMAL_DICOM_DICTIONARY);
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
