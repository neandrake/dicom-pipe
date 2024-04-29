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

//! This module contains the logic for reading an element from the dataset and determining its
//! position within the dataset structure (sequences, items, etc.).
//!
//! The byte-level logic for reading an element is contained within the `dsread` module.

use std::io::Read;

use crate::core::{
    charset::CSRef,
    dcmelement::DicomElement,
    dcmsqelem::SequenceElement,
    defn::{
        constants::{
            tags::{
                ITEM, ITEM_DELIMITATION_ITEM, SEQUENCE_DELIMITATION_ITEM, SPECIFIC_CHARACTER_SET,
                TRANSFER_SYNTAX_UID,
            },
            ts::ImplicitVRLittleEndian,
        },
        ts::TSRef,
        vl::ValueLength,
    },
    read::parser::{ParseResult, Parser},
};

impl<'d, R: Read> Parser<'d, R> {
    /// Performs the `ParserState::Element` iteration
    pub(crate) fn iterate_element(&mut self) -> ParseResult<Option<DicomElement>> {
        // use the transfer syntax defined by the dataset, or if not specified or not yet seen then
        // use the one initialized/detected.
        let ts: TSRef = self.dataset_ts.unwrap_or(self.detected_ts);

        #[cfg(feature = "compress")]
        {
            self.dataset.set_read_deflated(ts.deflated());
        }

        let tag: u32 = self.read_tag(ts)?;
        if self.is_at_parse_stop() {
            return Ok(None);
        }

        // check after reading a tag - some items seem to have 0-length and are followed by another
        // item. without popping here it will create an item-in-item structure. also need to check
        // if a sequence delimiter is ending an item which didn't have item delimiter - otherwise
        // the sequence delimiter will not be parented properly
        if tag == SEQUENCE_DELIMITATION_ITEM {
            if let Some(seq_elem) = self.current_path.last() {
                if seq_elem.sq_tag() == ITEM {
                    self.current_path.pop();
                }
            }
        }
        self.pop_sequence_items_based_on_byte_pos();

        // reading element clones the current path so update prior to reading element
        if tag == ITEM {
            // get the sequence this item is for and increment its item number
            if let Some(seq_elem) = self.current_path.last_mut() {
                seq_elem.increment_item();
            }
        }

        let element: DicomElement = self.read_dicom_element(tag, ts)?;

        // if the file-meta state was skipped due to the initial detection we may still need to
        // switch transfer syntax -- only do this if the element is at the root of the dataset
        if element.tag() == TRANSFER_SYNTAX_UID && element.sq_path().is_empty() {
            self.dataset_ts = self
                .parse_transfer_syntax(&element)?
                .or(Some(&ImplicitVRLittleEndian));
        } else if element.tag() == SPECIFIC_CHARACTER_SET {
            let cs: CSRef = Parser::<'d, R>::parse_specific_character_set(&element)?;
            if element.sq_path().is_empty() {
                self.cs = cs;
            } else if let Some(sq) = self.current_path.last_mut() {
                sq.set_cs(cs);
            }
        }

        // reset partial_tag to None
        self.partial_tag.take();

        // check for exiting a sequence based on being sequence delimiter - do before checking
        // against byte position
        if tag == SEQUENCE_DELIMITATION_ITEM || tag == ITEM_DELIMITATION_ITEM {
            if let Some(seq_elem) = self.current_path.last() {
                // if the parent is item then pop at least once for end of item
                if seq_elem.sq_tag() == ITEM {
                    self.current_path.pop();
                }
            }
            // if the sequence ended we pop again to get out of the sequence
            if tag == SEQUENCE_DELIMITATION_ITEM {
                self.current_path.pop();
            }
        }

        self.pop_sequence_items_based_on_byte_pos();

        if element.is_sq_like() || tag == ITEM {
            let seq_end_pos: Option<u64> = if let ValueLength::Explicit(len) = element.vl() {
                Some(self.bytes_read + u64::from(len))
            } else {
                None
            };

            let sq_cs: CSRef = if let Some(sq) = self.current_path.last() {
                sq.cs()
            } else {
                self.cs
            };

            self.current_path.push(SequenceElement::new(
                tag,
                seq_end_pos,
                element.vr(),
                element.vl(),
                sq_cs,
            ));
        }

        Ok(Some(element))
    }
}
