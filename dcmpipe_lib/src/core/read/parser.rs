use std::{convert::TryFrom, io::Read, iter::once};

use crate::core::{
    charset::{lookup_charset, CSRef, DEFAULT_CHARACTER_SET},
    dcmelement::DicomElement,
    dcmsqelem::SequenceElement,
    defn::{
        constants::tags::{DOUBLE_PIXEL_DATA, FLOAT_PIXEL_DATA, ITEM, PIXEL_DATA},
        dcmdict::DicomDictionary,
        tag::{TagNode, TagPath},
        ts::TSRef,
        vl::ValueLength,
        vr::VRRef,
    },
    read::{behavior::ParseBehavior, ds::dataset::Dataset, error::ParseError, stop::ParseStop},
    DICOM_PREFIX, DICOM_PREFIX_LENGTH, FILE_PREAMBLE_LENGTH,
};

mod detect;
mod dsread;
mod elem;
mod fme;
mod iter;
mod util;

/// The `Result` type of the parser
pub type ParseResult<T> = core::result::Result<T, ParseError>;

/// The different parsing behaviors of the dataset.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ParserState {
    /// An initial state in which we're trying to detect the transfer syntax
    DetectTransferSyntax,

    /// The File Preamble. This is not required for all dicom datasets but is commonly present in
    /// file media.
    Preamble,

    /// The DICOM prefix. This is only present if Preamble is present.
    Prefix,

    /// The first element of most valid dicom datasets will be the Group Length element which is
    /// always encoded as `ExplicitVRLittleEndian`. The value of this element is the number of
    /// remaining bytes in the File Meta group.
    GroupLength,

    /// The first set of elements of a valid dicom dataset which provide details on how the dicom is
    /// encoded. These elements are always encoded using `ExplicitVRLittleEndian`.
    FileMeta,

    /// The primary content of a dicom dataset. They are parsed using the transfer syntax specified
    /// in the File Meta group.
    Element,
}

/// Provides an iterator that parses through a dicom dataset returning dicom elements.
#[derive(Debug)]
pub struct Parser<'d, R: Read> {
    /// The dataset to parse dicom from.
    pub(super) dataset: Dataset<R>,

    /// The current state of reading elements from the dataset.
    pub(super) state: ParserState,

    /// Configurations that modify how the parser behaves.
    pub(super) behavior: ParseBehavior,

    /// The DICOM dictionary. Parsing uses `get_ts_by_uid` to identify transfer syntax for parsing
    /// through the stream, and `get_tag_by_number` for resolving VR of parsed elements. The VR is
    /// not strictly necessary for parsing elements however there is potential for sequences to not
    /// have their sub-elements parsed properly without this.
    pub(super) dictionary: &'d dyn DicomDictionary,

    /// Tracks the number of bytes read from the dataset. It's not required that the dataset
    /// implement `Seek` (network streams won't implement `Seek` without a buffer). Bytes read from
    /// the dataset are counted in order to track relative positioning for allocating elements with
    /// defined value lengths. Used to determine if File Meta elements are being parsed vs. switch
    /// to regular elements. Also used to track when sequences of explicit length begin/end.
    pub(super) bytes_read: u64,

    /// The file preamble read from the dataset. Not all datasets may have a preamble.
    pub(super) file_preamble: Option<[u8; FILE_PREAMBLE_LENGTH]>,

    /// The prefix read from the dataset. This should be a value of `"DICM"` but not all datasets
    /// have a prefix. If the dataset has a file preamble then it should also have a prefix.
    pub(super) dicom_prefix: Option<[u8; DICOM_PREFIX_LENGTH]>,

    /// The number of bytes read just after having read the `FileMetaInformationGroupLength`. This
    /// is used to determine how many bytes to continue parsing until we switch to reading regular
    /// DICOM elements, by checking `bytes_read` against `fmi_start + fmi_grouplength`.
    pub(super) fmi_start: u64,

    /// The value of the `FileMetaInformationGroupLength` tag if one is present. If present this is
    /// is the number of bytes remaining in the File Meta Information section until the non-meta
    /// section of the DICOM dataset starts. Only after the File Meta Information section does the
    /// transfer syntax and character encoding take effect. If the dataset does not contain the
    /// `FileMetaInformationGroupLength` element then this will have a value of zero and unused.
    pub(super) fmi_grouplength: u32,

    /// This is the last element tag successfully read from the dataset, regardless of whether
    /// the element it's for was successfully finished parsing.
    pub(super) tag_last_read: u32,

    /// This is the VR detected for the last tag successfully read from the dataset,
    /// regardless of whether the element it's for successfully finished parsing.
    pub(super) vr_last_used: Option<VRRef>,

    /// This is the value length detected for the last tag successfully read from the dataset,
    /// regardless of whether the element it's for successfully finished parsing.
    pub(super) vl_last_used: Option<ValueLength>,

    /// This is the transfer syntax used for the last tag successfully read from the dataset,
    /// regardless of whether the element it's for sucessfully finished parsing.
    pub(super) ts_last_used: Option<TSRef>,

    /// This is the element tag currently being read from the dataset. It will be `Some` once the
    /// element starts parsing and will be `None` after the element has completed parsing. Elements
    /// may be partially parsed either due to parsing errors or `ParseStop`. This is used regularly
    /// through all element parsing as a means of "peeking" what tag is next, particularly for
    /// checking `self.stop` for when to stop parsing. This is done in the case of wanting to parse
    /// all tags up to a commonly large tag such as `PixelData`.
    pub(super) partial_tag: Option<u32>,

    /// This is the element's VR read from the dataset when in `ParseState::DetectState`. This will
    /// only ever be used once in this regard. Since bytes need to be parsed from the dataset in
    /// order to detect the transfer syntax, if the file preamble is missing then this will be
    /// set as the vr parsed from the dataset of the first valid dicom element (or it will be the
    /// look-up if implicit vr is determined).
    pub(super) partial_vr: Option<VRRef>,

    /// This is the element's value length read from the dataset when in `ParseState::DetectState`.
    /// This wll only ever be used once in this regard. Since bytes need to be parsed from the
    /// dataset in order to detect the transfer syntax, if the file preamble is missing then this
    /// will be set as the value length parsed from the dataset of the first valid dicom element.
    pub(super) partial_vl: Option<ValueLength>,

    /// This should start as `ExplicitVRLittleEndian` which is the standard transfer syntax for
    /// file meta elements (not all dicom will follow the standard and may have file meta encoded
    /// using a different transfer syntax). During detection of transfer syntax this may change to
    /// another transfer syntax.
    ///
    /// **Note**: This will be the detected transfer syntax of the first elements read in from the
    ///           dataset. It *may* represent the main dataset's transfer syntax if there is no
    ///           `TransferSyntax` element parsed. If a `TransferSyntax` element exists in the
    ///           dataset then `self.dataset_ts` will be that value, and should be used to parse
    ///           the primary dataset.
    pub(super) detected_ts: TSRef,

    /// The transfer syntax used for this dataset (not for the file-meta). This is only populated if
    /// the dataset specifies a transfer syntax via tag `TransferSyntax`. If this is not populated
    /// then the dataset should be read using `self.detected_ts`.
    pub(super) dataset_ts: Option<TSRef>,

    /// The specific character set used for this dataset. This defaults to the dicom default which
    /// is `WINDOWS_1252` but is changed after having successully parsed the specific character set
    /// element.
    pub(super) cs: CSRef,

    /// The current sequence stack. Whenever an SQ element is parsed a new `SequenceElement` is
    /// appened to this stack. The last element is popped of when the sequence ends (via byte
    /// position or `SequenceDelimitationItem`). This also tracks the current `Item` within a
    /// sequence. Whenever an `Item` element is read the last element in this list has its item
    /// count initialized/incremented. Every element parsed from the dataset clones this stack.
    pub(super) current_path: Vec<SequenceElement>,

    /// When the `next()` returns an `Error` or `None` future calls to `next()` should not attempt
    /// to read from the dataset. This is used to track when the iterator should be considered fully
    /// consumed in those cases and prevent further attempts at reading from the dataset.
    pub(super) iterator_ended: bool,
}

impl<'d, R: Read> Parser<'d, R> {
    pub(crate) fn behavior(&self) -> &ParseBehavior {
        &self.behavior
    }

    /// Get the number of bytes read from the dataset.
    pub fn bytes_read(&self) -> u64 {
        self.bytes_read
    }

    /// Get the last tag read from the dataset. Note that the element for this tag may not have
    /// successfully parsed.
    pub fn tag_last_read(&self) -> u32 {
        self.tag_last_read
    }

    /// Get the current state of the parser.
    pub fn parser_state(&self) -> ParserState {
        self.state
    }

    /// Get the transfer syntax the dataset is encoded in.
    pub fn ts(&self) -> TSRef {
        self.dataset_ts.unwrap_or(self.detected_ts)
    }

    /// Get the character set string values are encoded in.
    pub fn cs(&self) -> CSRef {
        self.cs
    }

    /// Get the dictionary used during parsing.
    pub fn dictionary(&self) -> &'d dyn DicomDictionary {
        self.dictionary
    }

    /// Get the file preamble (128-bytes) read from the dataset. If the dataset did not have a file
    /// preamble or if it has not yet been read from the dataset then this will be `None`.
    pub fn file_preamble(&self) -> &Option<[u8; FILE_PREAMBLE_LENGTH]> {
        &self.file_preamble
    }

    /// The standard DICOM 4-byte prefix parsed from the dataset.  If this has not yet been parsed
    /// from the dataset this will be `None`. According to the DICOM standard this should always be
    /// the value `DICM`.
    pub fn dicom_prefix(&self) -> &Option<[u8; DICOM_PREFIX_LENGTH]> {
        &self.dicom_prefix
    }

    /// Checks if the stream should stop being parsed based on `self.stop`. This should be checked
    /// after parsing a tag number from the dataset.
    fn is_at_parse_stop(&self) -> bool {
        match &self.behavior.stop() {
            // If the entire dataset is intended to be read then never indicate to stop.
            ParseStop::EndOfDataset => false,

            // Check whether the parsing has surpassed the desired stopping byte position.
            ParseStop::AfterBytePos(byte_pos) => self.bytes_read > *byte_pos,

            ParseStop::BeforeTagValue(_) | ParseStop::AfterTagValue(_) => {
                let current: TagPath = TagPath::from(
                    self.current_path
                        .iter()
                        .map(|sq_el| sq_el.node().clone())
                        .chain(once(TagNode::from(self.tag_last_read)))
                        .collect::<Vec<TagNode>>(),
                );
                self.behavior.stop().evaluate(current)
            }
        }
    }

    /// Checks if the current path is within a pixeldata tag.
    fn is_in_pixeldata(&self) -> bool {
        for seq_elem in self.current_path.iter().rev() {
            if seq_elem.seq_tag() == FLOAT_PIXEL_DATA
                || seq_elem.seq_tag() == DOUBLE_PIXEL_DATA
                || seq_elem.seq_tag() == PIXEL_DATA
            {
                return true;
            }
            // If the parent element is an ITEM then keep walking up the chain to check against the
            // actual sequence element -- if it's not ITEM and not a PixelData then it's something
            // else and we can assume to not be within PixelData.
            if seq_elem.seq_tag() != ITEM {
                break;
            }
        }
        false
    }

    /// Datasets don't always end their sequences/items with delimiters. This will pop items off
    /// `self.current_path` which have indicated their length and the `self.bytes_read` indicate
    /// the stream has already passed this position.
    fn pop_sequence_items_base_on_byte_pos(&mut self) {
        while let Some(seq_elem) = self.current_path.last() {
            if let Some(seq_end_pos) = seq_elem.seq_end_pos() {
                if self.bytes_read >= seq_end_pos {
                    self.current_path.pop();
                } else {
                    break;
                }
            } else {
                // undefined length, stop checking the sequence path
                break;
            }
        }
    }

    /// Parses the value of the given element as the transfer syntax return. If the transfer syntax
    /// cannot be resolved then this sets it to the default DICOM transfer syntax which is IVRLE.
    fn parse_transfer_syntax(&mut self, element: &DicomElement) -> ParseResult<Option<TSRef>> {
        let ts_uid: String = String::try_from(element)?;
        Ok(self.dictionary.get_ts_by_uid(ts_uid.as_ref()))
    }

    /// Parses the value of the given element as the specific character set and sets the `cs` value
    /// on this iterator to affect the parsing of further text-type element values.
    fn parse_specific_character_set(&mut self, element: &DicomElement) -> ParseResult<CSRef> {
        let new_cs: Option<String> = Vec::<String>::try_from(element)?
            .into_iter()
            .find(|cs_entry: &String| !cs_entry.is_empty());

        // TODO: There are options for what to do if we can't support the character repertoire
        //       See note on Ch 5 Part 6.1.2.3 under "Considerations on the Handling of
        //       Unsupported Character Sets"

        Ok(new_cs
            .and_then(|cs: String| lookup_charset(&cs))
            .unwrap_or(DEFAULT_CHARACTER_SET))
    }

    /// Builds a string containing debug state of parsing, for errors and spurious output while
    /// debugging. Format is multiple lines, first line no indent, each other single-tab indent.
    /// ```text
    /// state: ParseState @ byte pos 0xDEAD_BEEF
    ///     vr: OB, vl: 128, ts: ImplicitVRLittleEndian
    ///     tagpath: ReferenceSequence[1].(00A1,0000)
    /// ```
    pub(super) fn current_debug_str(&self) -> String {
        // Render the full tag path
        let tag = self.tag_last_read;
        let mut full_path: TagPath = TagPath::from(self.current_path.as_slice());
        full_path.nodes_mut().push(TagNode::from(tag));
        let tagpath_display: String =
            TagPath::format_tagpath_to_display(&full_path, Some(self.dictionary));

        let vr_display = if let Some(vr) = self.vr_last_used {
            vr.ident
        } else {
            "N/A"
        };
        let vl_display = if let Some(vl) = self.vl_last_used {
            format!("{vl:?}")
        } else {
            "N/A".to_string()
        };

        // Format the bytes_read as 64bit hex value in "0x0000_0000" format.
        let msb = self.bytes_read >> 16;
        let lsb = self.bytes_read & 0x0000_FFFF;
        let byte_str = format!("{:#06X}_{:04X}", msb, lsb);

        // Display "dataset_ts" if it's the same as the dataset's, otherwise show the ident name.
        let ts_str = if self.dataset_ts == self.ts_last_used {
            "dataset_ts"
        } else if let Some(ts) = self.ts_last_used {
            ts.uid.ident
        } else {
            "N/A"
        };

        let state_str = format!("{:?}", self.state);

        format!(
            "state: {state_str} @ byte pos {byte_str}\n\ttagpath: {tagpath_display}\n\tvr: {vr_display}, vl: {vl_display}, ts: {ts_str}"
        )
    }

    /// Performs the primary iteration for the parser but the return type is consistent for error
    /// handling and not iteration. This should be called once for each invocation of `next()`.
    pub(super) fn iterate(&mut self) -> ParseResult<Option<DicomElement>> {
        // The earlier parse states will read non-elements from the dataset and move to another
        // state. A loop is used so once those succeed they continue the loop and move to next
        // states which will eventually return a dicom element.
        loop {
            match self.state {
                ParserState::DetectTransferSyntax => {
                    self.iterate_detect_state()?;
                }
                ParserState::Preamble => {
                    self.iterate_preamble()?;
                }
                ParserState::Prefix => {
                    self.iterate_prefix()?;
                }
                ParserState::GroupLength => {
                    return match self.iterate_group_length()? {
                        None => {
                            // if none is returned and the state changed then let the iterator go to the
                            // next state -- it's likely group length wasn't read but another tag was
                            if self.state != ParserState::GroupLength {
                                continue;
                            }
                            Ok(None)
                        }
                        Some(element) => Ok(Some(element)),
                    };
                }
                ParserState::FileMeta => {
                    return match self.iterate_file_meta()? {
                        None => {
                            // if none is returned and the state changed then let the iterator go to the
                            // next state -- it's likely file meta wasn't read but another tag was
                            if self.state != ParserState::FileMeta {
                                continue;
                            }
                            Ok(None)
                        }
                        Some(element) => Ok(Some(element)),
                    };
                }
                ParserState::Element => {
                    return self.iterate_element();
                }
            }
        }
    }
}
