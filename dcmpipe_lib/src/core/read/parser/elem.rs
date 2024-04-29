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
        constants::{tags, ts},
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
        if tag == tags::SEQUENCE_DELIMITATION_ITEM {
            if let Some(seq_elem) = self.current_path.last() {
                if seq_elem.seq_tag() == tags::ITEM {
                    self.current_path.pop();
                }
            }
        }
        self.pop_sequence_items_base_on_byte_pos();

        // reading element clones the current path so update prior to reading element
        if tag == tags::ITEM {
            // get the sequence this item is for and increment its item number
            if let Some(seq_elem) = self.current_path.last_mut() {
                seq_elem.increment_item();
            }
        }

        let element: DicomElement = self.read_dicom_element(tag, ts)?;

        // if the file-meta state was skipped due to the initial detection we may still need to
        // switch transfer syntax -- only do this if the element is at the root of the dataset
        if element.tag() == tags::TRANSFER_SYNTAX_UID && element.sequence_path().is_empty() {
            self.dataset_ts = self
                .parse_transfer_syntax(&element)?
                .or(Some(&ts::ImplicitVRLittleEndian));
        } else if element.tag() == tags::SPECIFIC_CHARACTER_SET {
            let cs: CSRef = self.parse_specific_character_set(&element)?;
            if element.sequence_path().is_empty() {
                self.cs = cs;
            } else if let Some(sq) = self.current_path.last_mut() {
                sq.set_cs(cs);
            }
        }

        // reset partial_tag to None
        self.partial_tag.take();

        // check for exiting a sequence based on being sequence delimiter - do before checking
        // against byte position
        if tag == tags::SEQUENCE_DELIMITATION_ITEM || tag == tags::ITEM_DELIMITATION_ITEM {
            if let Some(seq_elem) = self.current_path.last() {
                // if the parent is item then pop at least once for end of item
                if seq_elem.seq_tag() == tags::ITEM {
                    self.current_path.pop();
                }
            }
            // if the sequence ended we pop again to get out of the sequence
            if tag == tags::SEQUENCE_DELIMITATION_ITEM {
                self.current_path.pop();
            }
        }

        self.pop_sequence_items_base_on_byte_pos();

        if element.is_seq_like() || tag == tags::ITEM {
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
