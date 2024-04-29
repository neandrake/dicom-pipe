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
        defn::constants::ts::ImplicitVRLittleEndian,
        read::{ParserBuilder, ParserState},
    },
    dict::stdlookup::STANDARD_DICOM_DICTIONARY,
    dimse::{
        commands::messages::CommandMessage,
        pdus::{mainpdus::PresentationDataValue, Pdu},
        DimseError,
    },
};

#[derive(Debug)]
pub enum PduIterItem {
    Pdu(Pdu),
    CmdMessage(CommandMessage),
    Dataset(PresentationDataValue),
}

#[derive(Debug)]
pub enum PduIterState {
    ReadPdu,
    ReadPdiVal,
    ReadCmdMessage,
}

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
