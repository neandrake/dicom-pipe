//! This module contains the logic for reading the file preamble and File Meta Elements of a
//! dataset encoded as a file on disk (most other DICOM datasets are network transmitted DIMSE
//! objects which don't make use of File Meta structure/elements).
//!
//! The byte-level parsing is managed by the `dsread` module.

use std::io::Read;

use crate::core::{
    dcmelement::DicomElement,
    defn::{constants::tags, ts::TSRef},
    read::parser::{ParseError, ParseResult, Parser, ParserState},
    DICOM_PREFIX, DICOM_PREFIX_LENGTH, FILE_PREAMBLE_LENGTH,
};

impl<'d, R: Read> Parser<'d, R> {
    /// Performs the `ParserState::Preamble` iteration
    pub(super) fn iterate_preamble(&mut self) -> ParseResult<()> {
        let mut file_preamble: [u8; FILE_PREAMBLE_LENGTH] = [0; FILE_PREAMBLE_LENGTH];
        self.dataset.read_exact(&mut file_preamble)?;
        self.bytes_read += u64::try_from(file_preamble.len()).unwrap_or_default();
        self.file_preamble = Some(file_preamble);
        self.state = ParserState::ReadPrefix;
        Ok(())
    }

    /// Performs the `ParserState::Prefix` iteration
    pub(super) fn iterate_prefix(&mut self) -> ParseResult<()> {
        let mut dicom_prefix: [u8; DICOM_PREFIX_LENGTH] = [0; DICOM_PREFIX_LENGTH];
        self.dataset.read_exact(&mut dicom_prefix)?;
        self.bytes_read += u64::try_from(dicom_prefix.len()).unwrap_or_default();
        for (n, prefix_item) in DICOM_PREFIX.iter().enumerate() {
            if dicom_prefix[n] != *prefix_item {
                return Err(ParseError::BadDICOMPrefix(dicom_prefix));
            }
        }
        self.dicom_prefix = Some(dicom_prefix);
        self.state = ParserState::ReadGroupLength;
        Ok(())
    }

    /// Performs the `ParserState::GroupLength` iteration
    pub(super) fn iterate_group_length(&mut self) -> ParseResult<Option<DicomElement>> {
        // See comment on `detected_ts` for further details on why this is being used as to
        // hard-coded to ExplicitVRLittleEndian, as the standard defines File Meta to use.
        let ts: TSRef = self.detected_ts;
        let tag: u32 = self.read_tag(ts)?;
        if self.is_at_parse_stop() {
            return Ok(None);
        }

        if tag != tags::FILE_META_INFORMATION_GROUP_LENGTH {
            if tag > tags::FILE_META_INFORMATION_GROUP_LENGTH && tag < tags::FILE_META_GROUP_END {
                self.state = ParserState::ReadFileMeta;
            } else {
                self.state = ParserState::ReadElement;
            }
            return Ok(None);
        }

        let grouplength: DicomElement = self.read_dicom_element(tag, ts)?;
        self.fmi_grouplength = u32::try_from(&grouplength)?;
        self.fmi_start = self.bytes_read;
        self.state = ParserState::ReadFileMeta;
        // reset partial_tag to None
        self.partial_tag.take();

        Ok(Some(grouplength))
    }

    /// Performs the `ParserState::FileMeta` iteration
    pub(super) fn iterate_file_meta(&mut self) -> ParseResult<Option<DicomElement>> {
        // check if we're about to read an element which is outside the file meta section, if so
        // then change states outside of this one.
        if self.fmi_grouplength > 0
            && (self.bytes_read >= self.fmi_start + u64::from(self.fmi_grouplength))
        {
            // if we never read a transfer syntax in the file-meta then jump back to detecting the
            // transfer syntax of the main dataset.
            self.state = if self.dataset_ts.is_some() {
                ParserState::ReadElement
            } else {
                ParserState::DetectTransferSyntax
            };
            return Ok(None);
        }

        // See comment on `detected_ts` for further details on why this is being used as to
        // hard-coded to ExplicitVRLittleEndian, as the standard defines File Meta to use.
        let ts: TSRef = self.detected_ts;

        let tag: u32 = self.read_tag(ts)?;
        if self.is_at_parse_stop() {
            return Ok(None);
        }

        let element: DicomElement = self.read_dicom_element(tag, ts)?;
        if element.tag() == tags::TRANSFER_SYNTAX_UID {
            match self.parse_transfer_syntax(&element) {
                Ok(Some(ts)) => {
                    self.dataset_ts = Some(ts);
                }
                Ok(None) => {}
                Err(e) => return Err(e),
            }
        }

        // if group length was read use the byte position to determine if we're out of file-meta
        if (self.fmi_grouplength > 0
            && (self.bytes_read >= self.fmi_start + u64::from(self.fmi_grouplength)))
            || tag > tags::FILE_META_GROUP_END
        {
            // if we exit file-meta without having parsed transfer-syntax or if the ts is unknown to
            // the dictionary used for parsing then flip to DetectState so the implicit vs. explicit
            // can be detected since it's likely to change after file-meta.
            self.state = if self.dataset_ts.is_some() {
                ParserState::ReadElement
            } else {
                ParserState::DetectTransferSyntax
            };
        }

        // reset partial_tag to None
        self.partial_tag.take();

        Ok(Some(element))
    }
}
