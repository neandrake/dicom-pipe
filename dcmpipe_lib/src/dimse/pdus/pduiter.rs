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

use std::io::{Cursor, Read};

use crate::{
    core::{
        dcmobject::DicomRoot,
        defn::{constants::ts::ImplicitVRLittleEndian, ts::TSRef},
        read::{ParserBuilder, ParserState},
    },
    dict::stdlookup::STANDARD_DICOM_DICTIONARY,
    dimse::{
        commands::messages::CommandMessage,
        pdus::{mainpdus::PresentationDataValue, Pdu},
        DimseError,
    },
};

pub fn read_next_pdu<R: Read>(reader: R) -> Option<Result<PduIterItem, DimseError>> {
    PduIter::new(reader).next()
}

#[derive(Debug)]
pub enum PduIterItem {
    Pdu(Pdu),
    CmdMessage(CommandMessage),
    Dataset(PresentationDataValue),
}

/// Parsing PDU's from a stream was originally implemented as an iterator however it turns out that
/// most activity over DIMSE protocol is organized as request/response activity. The result is that
/// the user of this API will rarely need to read a stream of PDUs. To simplify, the
/// `read_next_pdu()` function above was created to encapsulate this iterator implementation.
pub struct PduIter<R: Read> {
    stream: R,
}

impl<R: Read> PduIter<R> {
    pub fn new(stream: R) -> Self {
        Self { stream }
    }
}

impl<R: Read> Iterator for PduIter<R> {
    type Item = Result<PduIterItem, DimseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let pdu = Pdu::read(&mut self.stream);

        let pres_data_item = match pdu {
            Ok(Pdu::PresentationDataItemPartial(pres_data_item)) => pres_data_item,
            Ok(other) => return Some(Ok(PduIterItem::Pdu(other))),
            Err(e) => return Some(Err(e)),
        };

        let pres_data_val = PresentationDataValue::read(&mut self.stream);
        if let Err(e) = pres_data_val {
            return Some(Err(e));
        }
        let Ok(pres_data_val) = pres_data_val else {
            return None;
        };

        if pres_data_val.is_command() {
            let ctx_id = pres_data_val.ctx_id();
            // Continue readind PDV segments until the full command message can be read into
            // memory. Unless the `MaxLengthItem` is set abnormally small then the command should
            // rarely be split into multiple PDVs.
            let mut is_done = pres_data_val.is_last_fragment();

            let mut buffer = Vec::<u8>::with_capacity(
                usize::try_from(pres_data_item.length()).unwrap_or_default(),
            );
            buffer.append(&mut pres_data_val.into_data());

            while !is_done {
                // TODO: Check for PDU first which might be `PresentationDataItem`?
                //       The standard is terribly unclear on whether the max length applies to the
                //       PDI or the PDV! Given that PDV is the structure that indicates it being
                //       the last fragment or not, it seems that max length applies to PDV.
                let pres_data_val = PresentationDataValue::read(&mut self.stream);
                if let Err(e) = pres_data_val {
                    return Some(Err(e));
                }
                let Ok(pres_data_val) = pres_data_val else {
                    return None;
                };
                is_done = pres_data_val.is_last_fragment();
                buffer.append(&mut pres_data_val.into_data());
            }

            // Command P-DATA always uses IVRLE. Use the standard dicom dictinoary for
            // parsing, otherwise anything pulling values from the command dataset will
            // have to explicitly specify the VR to parse values as since they would
            // otherwise all be `UN`.
            let mut buffer = Cursor::new(buffer);
            let mut parser = ParserBuilder::default()
                .state(ParserState::ReadElement)
                .dataset_ts(&ImplicitVRLittleEndian)
                .build(&mut buffer, &STANDARD_DICOM_DICTIONARY);
            match DicomRoot::parse(&mut parser) {
                Ok(Some(cmd_root)) => {
                    return Some(Ok(PduIterItem::CmdMessage(CommandMessage::new(
                        ctx_id, cmd_root,
                    ))));
                }
                Ok(None) => return None,
                Err(source) => return Some(Err(DimseError::ParseError(source))),
            }
        }

        // If it's a DICOM fragment then send it up as-is, allowing the user of this API to
        // manually stitch or forward the data without fully loading it into memory.
        Some(Ok(PduIterItem::Dataset(pres_data_val)))
    }
}

pub struct CommandIter<R: Read> {
    ts: TSRef,
    reader: PduIter<R>,
}

impl<R: Read> CommandIter<R> {
    pub fn new(ts: TSRef, reader: R) -> Self {
        Self {
            ts,
            reader: PduIter::new(reader),
        }
    }
}

impl<R: Read> Iterator for CommandIter<R> {
    type Item = Result<(CommandMessage, Option<DicomRoot>), DimseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let cmd = match self.reader.next() {
            Some(Ok(PduIterItem::Dataset(_pdv))) => {
                return Some(Err(DimseError::GeneralError(
                    "Received Dataset instead of Command".to_owned(),
                )))
            }
            Some(Ok(PduIterItem::Pdu(pdu))) => {
                return Some(Err(DimseError::UnexpectedPduType(pdu.pdu_type())));
            }
            Some(Ok(PduIterItem::CmdMessage(cmd))) => cmd,
            Some(Err(e)) => return Some(Err(e)),
            None => return None,
        };

        if !cmd.has_dataset() {
            return Some(Ok((cmd, None)));
        }

        let mut buf: Vec<u8> = Vec::new();
        loop {
            match self.reader.next() {
                Some(Ok(PduIterItem::Dataset(pdv))) => {
                    let is_last = pdv.is_last_fragment();
                    buf.append(pdv.into_data().as_mut());
                    if is_last {
                        break;
                    }
                }
                Some(Ok(PduIterItem::Pdu(pdu))) => {
                    return Some(Err(DimseError::UnexpectedPduType(pdu.pdu_type())))
                }
                Some(Ok(PduIterItem::CmdMessage(_cmd))) => {
                    return Some(Err(DimseError::GeneralError(
                        "Received Command instead of Dataset".to_owned(),
                    )))
                }
                Some(Err(e)) => return Some(Err(e)),
                None => break,
            }
        }

        let mut buf = Cursor::new(buf);
        let mut parser = ParserBuilder::default()
            .state(ParserState::ReadElement)
            .dataset_ts(self.ts)
            .build(&mut buf, &STANDARD_DICOM_DICTIONARY);
        match DicomRoot::parse(&mut parser) {
            Ok(Some(dcm_root)) => Some(Ok((cmd, Some(dcm_root)))),
            Ok(None) => Some(Ok((cmd, None))),
            Err(source) => Some(Err(DimseError::ParseError(source))),
        }
    }
}
