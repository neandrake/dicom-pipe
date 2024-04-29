use std::convert::TryFrom;
use std::io::{Cursor, ErrorKind, Read};

use crate::core::charset::{self, CSRef};
use crate::core::dcmelement::DicomElement;
use crate::core::dcmsqelem::SequenceElement;
use crate::core::read;
use crate::core::read::ds::dataset::Dataset;
use crate::core::read::error::ParseError;
use crate::core::read::stop::ParseStop;
use crate::defn::constants::{tags, ts};
use crate::defn::dcmdict::DicomDictionary;
use crate::defn::tag::Tag;
use crate::defn::ts::TSRef;
use crate::defn::vl::ValueLength;
use crate::defn::vr::{self, VRRef};

pub const FILE_PREAMBLE_LENGTH: usize = 128;
pub const DICOM_PREFIX_LENGTH: usize = 4;
const MAX_VALUE_LENGTH_IN_DETECT: u32 = 100;

pub static DICOM_PREFIX: &[u8; DICOM_PREFIX_LENGTH] = b"DICM";

/// The `Result` type of the parser
pub type Result<T> = core::result::Result<T, ParseError>;

/// The different parsing behaviors of the dataset.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ParseState {
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
pub struct Parser<'dict, DatasetType: Read> {
    /// The dataset to parse dicom from.
    pub(crate) dataset: Dataset<DatasetType>,

    /// The current state of reading elements from the dataset.
    pub(crate) state: ParseState,

    /// The condition under which this iterator should stop parsing elements from the dataset. This
    /// can be used for only partially parsing through a dataset.
    pub(crate) stop: ParseStop,

    /// The DICOM dictionary. Parsing uses `get_ts_by_uid` to identify transfer syntax for parsing
    /// through the stream, and `get_tag_by_number` for resolving VR of parsed elements. The VR is
    /// not strictly necessary for parsing elements however there is potential for sequences to not
    /// have their sub-elements parsed properly without this.
    pub(crate) dictionary: &'dict dyn DicomDictionary,

    /// Tracks the number of bytes read from the dataset. It's not required that the dataset
    /// implement `Seek` (network streams won't implement `Seek` without a buffer). Bytes read from
    /// the dataset are counted in order to track relative positioning for allocating elements with
    /// defined value lengths. Used to determine if File Meta elements are being parsed vs. switch
    /// to regular elements. Also used to track when sequences of explicit length begin/end.
    pub(crate) bytes_read: u64,

    /// The file preamble read from the dataset. Not all datasets may have a preamble.
    pub(crate) file_preamble: Option<[u8; FILE_PREAMBLE_LENGTH]>,

    /// The prefix read from the dataset. This should be a value of `"DICM"` but not all datasets
    /// have a prefix. If the dataset has a file preamble then it should also have a prefix.
    pub(crate) dicom_prefix: Option<[u8; DICOM_PREFIX_LENGTH]>,

    /// The number of bytes read just after having read the `FileMetaInformationGroupLength`. This
    /// is used to determine how many bytes to continue parsing until we switch to reading regular
    /// DICOM elements, by checking `bytes_read` against `fmi_start + fmi_grouplength`.
    pub(crate) fmi_start: u64,

    /// The value of the `FileMetaInformationGroupLength` tag if one is present. If present this is
    /// is the number of bytes remaining in the File Meta Information section until the non-meta
    /// section of the DICOM dataset starts. Only after the File Meta Information section does the
    /// transfer syntax and character encoding take effect. If the dataset does not contain the
    /// `FileMetaInformationGroupLength` element then this will have a value of zero and unused.
    pub(crate) fmi_grouplength: u32,

    /// This is the last element tag successfully read from the dataset, regardless of whether
    /// the element it's for was successfully finished parsing.
    pub(crate) tag_last_read: u32,

    /// This is the element tag currently being read from the dataset. It will be `Some` once the
    /// element starts parsing and will be `None` after the element has completed parsing. Elements
    /// may be partially parsed either due to parsing errors or `ParseStop`. This is used regularly
    /// through all element parsing as a means of "peeking" what tag is next, particularly for
    /// checking `self.stop` for when to stop parsing. This is done in the case of wanting to parse
    /// all tags up to a commonly large tag such as `PixelData`.
    pub(crate) partial_tag: Option<u32>,

    /// This is the element's VR read from the dataset when in `ParseState::DetectState`. This will
    /// only ever be used once in this regard. Since bytes need to be parsed from the dataset in
    /// order to detect the transfer syntax, if the file preamble is missing then this will be
    /// set as the vr parsed from the dataset of the first valid dicom element (or it will be the
    /// look-up if implicit vr is determined).
    pub(crate) partial_vr: Option<VRRef>,

    /// This is the element's value length read from the dataset when in `ParseState::DetectState`.
    /// This wll only ever be used once in this regard. Since bytes need to be parsed from the
    /// dataset in order to detect the transfer syntax, if the file preamble is missing then this
    /// will be set as the value length parsed from the dataset of the first valid dicom element.
    pub(crate) partial_vl: Option<ValueLength>,

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
    pub(crate) detected_ts: TSRef,

    /// The transfer syntax used for this dataset (not for the file-meta). This is only populated if
    /// the dataset specifies a transfer syntax via tag `TransferSyntax`. If this is not populated
    /// then the dataset should be read using `self.detected_ts`.
    pub(crate) dataset_ts: Option<TSRef>,

    /// The specific character set used for this dataset. This defaults to the dicom default which
    /// is `WINDOWS_1252` but is changed after having successully parsed the specific character set
    /// element.
    pub(crate) cs: CSRef,

    /// The current sequence stack. Whenever an SQ element is parsed a new `SequenceElement` is
    /// appened to this stack. The last element is popped of when the sequence ends (via byte
    /// position or `SequenceDelimitationItem`). This also tracks the current `Item` within a
    /// sequence. Whenever an `Item` element is read the last element in this list has its item
    /// count initialized/incremented. Every element parsed from the dataset clones this stack.
    pub(crate) current_path: Vec<SequenceElement>,

    /// When the `next()` returns an `Error` or `None` future calls to `next()` should not attempt
    /// to read from the dataset. This is used to track when the iterator should be considered fully
    /// consumed in those cases and prevent further attempts at reading from the dataset.
    pub(crate) iterator_ended: bool,
}

impl<'dict, DatasetType: Read> Parser<'dict, DatasetType> {
    /// Get the number of bytes read from the dataset.
    pub fn get_bytes_read(&self) -> u64 {
        self.bytes_read
    }

    /// Get the last tag read from the dataset. Note that the element for this tag may not have
    /// successfully parsed.
    pub fn get_tag_last_read(&self) -> u32 {
        self.tag_last_read
    }

    /// Get the current state of the parser.
    pub fn get_parser_state(&self) -> ParseState {
        self.state
    }

    /// Get the transfer syntax the dataset is encoded in.
    pub fn get_ts(&self) -> TSRef {
        self.dataset_ts.unwrap_or(self.detected_ts)
    }

    /// Get the character set string values are encoded in.
    pub fn get_cs(&self) -> CSRef {
        self.cs
    }

    /// Get the dictionary used during parsing.
    pub fn get_dictionary(&self) -> &'dict dyn DicomDictionary {
        self.dictionary
    }

    /// Get the file preamble (128-bytes) read from the dataset. If the dataset did not have a file
    /// preamble or if it has not yet been read from the dataset then this will be `None`.
    pub fn get_file_preamble(&self) -> &Option<[u8; FILE_PREAMBLE_LENGTH]> {
        &self.file_preamble
    }

    /// The standard DICOM 4-byte prefix parsed from the dataset.  If this has not yet been parsed
    /// from the dataset this will be `None`. According to the DICOM standard this should always be
    /// the value `DICM`.
    pub fn get_dicom_prefix(&self) -> &Option<[u8; DICOM_PREFIX_LENGTH]> {
        &self.dicom_prefix
    }

    /// Checks if the stream should stop being parsed based on `self.stop`. This should be checked
    /// after parsing a tag number from the dataset.
    fn is_at_parse_stop(&self) -> bool {
        match &self.stop {
            ParseStop::EndOfDataset => false,
            ParseStop::BeforeTag(tagpath) => ParseStop::eval_tagpath(
                tagpath,
                &self.current_path,
                self.tag_last_read,
                |(to_check, current)| current >= to_check,
            ),
            ParseStop::AfterTag(tagpath) => ParseStop::eval_tagpath(
                tagpath,
                &self.current_path,
                self.tag_last_read,
                |(to_check, current)| current > to_check,
            ),
            ParseStop::AfterBytePos(byte_pos) => self.bytes_read > *byte_pos,
        }
    }

    /// Checks if the current path is within a pixeldata tag.
    fn is_in_pixeldata(&self) -> bool {
        for seq_elem in self.current_path.iter().rev() {
            if seq_elem.get_seq_tag() == tags::FLOAT_PIXEL_DATA
                || seq_elem.get_seq_tag() == tags::DOUBLE_PIXEL_DATA
                || seq_elem.get_seq_tag() == tags::PIXEL_DATA
            {
                return true;
            }
            // If the parent element is an ITEM then keep walking up the chain to check against the
            // actual sequence element -- if it's not ITEM and not a PixelData then it's something
            // else and we can assume to not be within PixelData.
            if seq_elem.get_seq_tag() != tags::ITEM {
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
            if let Some(seq_end_pos) = seq_elem.get_seq_end_pos() {
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

    /// Reads a tag attribute from the dataset, unless `self.partial_tag` is `Some`.
    fn read_tag(&mut self, ts: TSRef) -> Result<u32> {
        let tag: u32 = if let Some(partial_tag) = self.partial_tag {
            partial_tag
        } else {
            let tag: u32 =
                read::util::read_tag_from_dataset(&mut self.dataset, ts.is_big_endian())?;
            self.bytes_read += 4;
            self.partial_tag.replace(tag);
            tag
        };
        self.tag_last_read = tag;
        Ok(tag)
    }

    /// Reads the remainder of the dicom element from the dataset. This assumes `self.read_tag()`
    /// was called just prior and its result passed as the tag parameter here.
    fn read_dicom_element(&mut self, tag: u32, ts: TSRef) -> Result<DicomElement> {
        // Part 5, Section 7.5
        // There are three special SQ related Data Elements that are not ruled by the VR encoding
        // rules conveyed by the Transfer Syntax. They shall be encoded as Implicit VR. These
        // special Data Elements are Item (FFFE,E000), Item Delimitation Item (FFFE,E00D), and
        // Sequence Delimitation Item (FFFE,E0DD). However, the Data Set within the Value Field of
        // the Data Element Item (FFFE,E000) shall be encoded according to the rules conveyed by the
        // Transfer Syntax.
        let ts: TSRef = if tag == tags::SEQUENCE_DELIMITATION_ITEM
            || tag == tags::ITEM_DELIMITATION_ITEM
            || tag == tags::ITEM
        {
            &ts::ImplicitVRLittleEndian
        } else {
            ts
        };

        let vr_ts: (VRRef, TSRef) = if let Some(partial_vr) = self.partial_vr {
            self.partial_vr.take();
            (partial_vr, ts)
        } else if ts.explicit_vr {
            match self.read_vr(tag, ts) {
                Err(ParseError::UnknownExplicitVR(_code)) => (&vr::INVALID, ts),
                Err(e) => return Err(e),
                Ok(vr_ts) => vr_ts,
            }
        } else {
            let vr: VRRef = if let Some(vr) = self.lookup_vr(tag) {
                vr
            } else {
                &vr::UN
            };
            (vr, ts)
        };

        let vr: VRRef = vr_ts.0;
        let vl_read_4bytes: bool = !ts.explicit_vr || vr.has_explicit_2byte_pad;
        // If VR is explicitly UN but we can tell it's SQ then the inner elements are encoded as
        // IVRLE -- but only the contents should be parsed as such, do not switch transfer syntax
        // prior to reading in the value length.
        let vl: ValueLength = if let Some(partial_vl) = self.partial_vl {
            self.partial_vl.take();
            partial_vl
        } else {
            self.read_value_length(vl_read_4bytes, ts)?
        };

        let parse_as_seq: bool = read::util::is_non_standard_seq(tag, vr, vl);
        let ts: TSRef = if parse_as_seq {
            &ts::ImplicitVRLittleEndian
        } else {
            vr_ts.1
        };

        // Sequence and item elements should let the iterator handle parsing its contents and not
        // associate bytes to the element's value. The exception are item elements within pixel data
        // which are used to encapsulate frames; their value is pixel data and not other elements.
        let in_pixeldata: bool = self.is_in_pixeldata();

        // Determine whether the value should be read in as byte values or instead should continue
        // being parsed as more elements.
        let skip_bytes: bool =
            vr == &vr::SQ || (tag == tags::ITEM && !in_pixeldata) || parse_as_seq;

        // eprintln!("{:?}: Tag: {}, VR: {:?}, VL: {:?}, ts: {}, bytesread: {}", self.state, Tag::format_tag_to_display(tag), vr, vl, ts.uid.ident, self.bytes_read);
        let bytes: Vec<u8> = if skip_bytes {
            Vec::new()
        } else {
            self.read_value_field(tag, vl)?
        };

        let ancestors: Vec<SequenceElement> = self.current_path.clone();

        let cs: CSRef = if let Some(sq) = ancestors.last() {
            sq.get_cs()
        } else {
            self.cs
        };

        Ok(DicomElement::new(tag, vr, vl, ts, cs, bytes, ancestors))
    }

    /// Reads an explicit VR attribute from the dataset. This returns a tuple of `(VRRef, TSRef)`
    /// containing the parsed VR and the passed in transfer syntax. If the VR is explicitly written
    /// as `UN` then the dictionary used for parsing is checked for the default/implicit VR. If the
    /// VR found from the dictionary is `SQ` then the returned transfer syntax will be IVRLE as it
    /// is assumed the value field is encoded this way, irrespective of defined transfer syntax.
    /// See Part 5 Section 6.2.2 Note 2.
    fn read_vr(&mut self, tag: u32, ts: TSRef) -> Result<(VRRef, TSRef)> {
        let vr_res: Result<VRRef> = read::util::read_vr_from_dataset(&mut self.dataset);
        match vr_res {
            // if the VR couldn't be matched to a known VR there were still 2 bytes read from stream
            Ok(_) | Err(ParseError::UnknownExplicitVR(_)) => self.bytes_read += 2,
            _ => {}
        }

        let mut vr: VRRef = vr_res?;
        if vr.has_explicit_2byte_pad {
            self.bytes_read += 2;
        }

        // Part 5 Section 6.2.2 Note 2
        // If at some point an application knows the actual VR for an Attribute of VR UN (e.g., has
        // has its own applicable data dictionary), it can assume that the Value Field of the
        // Attribute is encoded in Little Endian byte ordering with implicit VR encoding,
        // irrespective of the current Transfer Syntax.
        // --
        // Only do this for potential sequences and not elements which have values
        let mut ts: TSRef = ts;
        if vr == &vr::UN {
            if let Some(found_vr) = self.lookup_vr(tag) {
                if found_vr == &vr::SQ {
                    ts = &ts::ImplicitVRLittleEndian;
                    vr = &vr::SQ;
                }
            }
        }

        Ok((vr, ts))
    }

    /// Looks up the VR of the given tag in the current dictionary.
    fn lookup_vr(&self, tag: u32) -> Option<VRRef> {
        self.dictionary
            .get_tag_by_number(tag)
            .and_then(|read_tag: &Tag| read_tag.implicit_vr)
    }

    /// Reads a Value Length attribute from the dataset using the given transfer syntax. The number
    /// of bytes representing the value length depends on transfer syntax. If the VR has a 2-byte
    /// padding then those bytes are also read from the dataset.
    fn read_value_length(&mut self, read_4bytes: bool, ts: TSRef) -> Result<ValueLength> {
        let result: Result<ValueLength> = read::util::read_value_length_from_dataset(
            &mut self.dataset,
            read_4bytes,
            ts.big_endian,
        );
        if result.is_ok() {
            if read_4bytes {
                self.bytes_read += 4;
            } else {
                self.bytes_read += 2;
            }
        }
        result
    }

    /// Reads the value field of the dicom element into a byte array. If the `ValueLength` is
    /// undefined then this returns an empty array as elements with undefined length should have
    /// their contents parsed as dicom elements.
    fn read_value_field(&mut self, tag: u32, vl: ValueLength) -> Result<Vec<u8>> {
        match vl {
            // Undefined length means that the contents of the element are other dicom elements to
            // be parsed. Don't read data from the dataset in this case.
            ValueLength::Explicit(0) | ValueLength::UndefinedLength => Ok(Vec::new()),
            ValueLength::Explicit(value_length) => {
                // If length is odd we only read that exact bytes from the dataset but the bytes
                // we should return from this should be padded with a zero in order to always
                // return an even-length value.
                let buffer_size: usize = if value_length % 2 != 0 {
                    value_length as usize + 1
                } else {
                    value_length as usize
                };
                let mut buffer: Vec<u8> = vec![0; buffer_size];
                let buffer_slice: &mut [u8] = &mut buffer.as_mut_slice()[0..value_length as usize];
                let result: Result<()> = self.dataset.read_exact(buffer_slice).map_err(|e| {
                    // Some datasets may end with this DataSetTrailingPadding tag and also have a
                    // value length which does not match the actual value field's size. The standard
                    // indicates that the content of the value field should hold no significance -
                    // so it should be okay to consider this not an error.
                    // See Part 10, Section 7.2
                    if tag == tags::DATASET_TRAILING_PADDING && e.kind() == ErrorKind::UnexpectedEof
                    {
                        // TODO: Take what values were read and return that as a byte array, so the
                        //       original contents of the dataset are retained if needed.
                        ParseError::ExpectedEOF
                    } else {
                        ParseError::DetailedIOError {
                            source: e,
                            detail: format!(
                                "reading tag: {}, vl: {}",
                                Tag::format_tag_to_display(tag),
                                value_length
                            ),
                        }
                    }
                });

                match result {
                    Ok(_) => {
                        self.bytes_read += u64::from(value_length);
                        Ok(buffer)
                    }
                    Err(ParseError::ExpectedEOF) => {
                        self.bytes_read += u64::from(value_length);
                        Err(ParseError::ExpectedEOF)
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }

    /// Parses the value of the given element as the transfer syntax return. If the transfer syntax
    /// cannot be resolved then this sets it to the default DICOM transfer syntax which is IVRLE.
    fn parse_transfer_syntax(&mut self, element: &DicomElement) -> Result<Option<TSRef>> {
        let ts_uid: String = String::try_from(element)?;
        Ok(self.dictionary.get_ts_by_uid(ts_uid.as_ref()))
    }

    /// Parses the value of the given element as the specific character set and sets the `cs` value
    /// on this iterator to affect the parsing of further text-type element values.
    fn parse_specific_character_set(&mut self, element: &DicomElement) -> Result<CSRef> {
        let new_cs: Option<String> = Vec::<String>::try_from(element)?
            .into_iter()
            .find(|cs_entry: &String| !cs_entry.is_empty());

        // TODO: There are options for what to do if we can't support the character repertoire
        //       See note on Ch 5 Part 6.1.2.3 under "Considerations on the Handling of
        //       Unsupported Character Sets"

        Ok(new_cs
            .and_then(|cs: String| charset::lookup_charset(&cs))
            .unwrap_or(charset::DEFAULT_CHARACTER_SET))
    }

    /// Performs the primary iteration for the parser but the return type is consistent for error
    /// handling and not iteration. This should be called once for each invocation of `next()`.
    pub(crate) fn iterate(&mut self) -> Result<Option<DicomElement>> {
        // The earlier parse states will read non-elements from the dataset and move to another
        // state. A loop is used so once those succeed they continue the loop and move to next
        // states which will eventually return a dicom element.
        loop {
            match self.state {
                ParseState::DetectTransferSyntax => {
                    self.iterate_detect_state()?;
                }
                ParseState::Preamble => {
                    self.iterate_preamble()?;
                }
                ParseState::Prefix => {
                    self.iterate_prefix()?;
                }
                ParseState::GroupLength => {
                    return match self.iterate_group_length()? {
                        None => {
                            // if none is returned and the state changed then let the iterator go to the
                            // next state -- it's likely group length wasn't read but another tag was
                            if self.state != ParseState::GroupLength {
                                continue;
                            }
                            Ok(None)
                        }
                        Some(element) => Ok(Some(element)),
                    };
                }
                ParseState::FileMeta => {
                    return match self.iterate_file_meta()? {
                        None => {
                            // if none is returned and the state changed then let the iterator go to the
                            // next state -- it's likely file meta wasn't read but another tag was
                            if self.state != ParseState::FileMeta {
                                continue;
                            }
                            Ok(None)
                        }
                        Some(element) => Ok(Some(element)),
                    };
                }
                ParseState::Element => {
                    return self.iterate_element();
                }
            }
        }
    }

    /// Performs the `ParserState::DetectState` iteration.
    /// Detects little-vs-big endian and implicit-vs-explicit endian. This strategy is not fully
    /// complete however it does cover a wide variety of cases. It does not cover scenarios where
    /// endian-detection could possibly succeed incorrectly however these scenarios would likely
    /// be very odd dicom cases that most other libraries are also unable to parse.
    ///
    /// After this iteration runs once the `self.partial_` fields may be filled in,
    /// `self.file_preamble` may be filled in, and `self.detected_ts` may be changed.
    ///
    /// The strategy this uses is to parse just a few bytes from the dataset to see if it looks like
    /// the start of a dicom element, back-tracking the read bytes if they are not interpretable as
    /// valid parts of a dicom element. The steps for parsing are based on
    /// 1. Parse a tag and check whether it looks like a valid tag value
    ///    (`0 < tag < SOP_INSTANCE_UID`). The tag is parsed as both little and big endian.
    /// 2. Parse a VR assuming it's explicit. If the VR does not match a known VR then it's assumed
    ///    the transfer syntax is implicit.
    /// 3. Parse a value length. The value length parsed is checked to be reasonable for a dicom
    ///    element that would appear early in a dicom dataset (file-meta or early group `0008`,
    ///    which would be `< MAX_VALUE_LENGTH_IN_DETECT`). The elements at the beginning of a
    ///    dataset should have fairly small values (for things like UIDs, etc.). The Pixel-med
    ///    library also uses this same value comparison.
    /// 4. Otherwise it's assumed the start of the file is proprietary file preamble. These bytes
    ///    are then skipped and detection begins again.
    fn iterate_detect_state(&mut self) -> Result<()> {
        // start off assuming EVRLE, the default for File-Meta
        let mut ts: TSRef = &ts::ExplicitVRLittleEndian;

        // if this function has been called previously it will have read data into
        // `self.file_preamble` for invalid element cases - if preamble is already read then those
        // invalid element cases should now turn into errors
        let already_read_preamble: bool = self.file_preamble.is_some();

        // as bytes are read from `self.dataset` they will be copied into this `file_preamble`, then
        // if it's determined that we're likely in a file preamble the rest of the standard
        // preamble will be read from the dataset and stored into `self.file_preamble`
        let mut file_preamble: [u8; FILE_PREAMBLE_LENGTH] = [0; FILE_PREAMBLE_LENGTH];
        let mut bytes_read: usize = 0;

        let mut buf: [u8; 4] = [0; 4];
        self.dataset.read_exact(&mut buf)?;

        // copy the read bytes into preamble in case we determine this is a preamble -- use a
        // cursor to allow re-parsing the same bytes again for checking both endian
        file_preamble[bytes_read..(bytes_read + buf.len())].copy_from_slice(&buf);
        bytes_read += buf.len();
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);

        let mut tag: u32 = read::util::read_tag_from_dataset(&mut cursor, ts.is_big_endian())?;

        if tag == 0 {
            // if tag is zero then assume preamble, jump forward and attempt to detect tag after it

            // if file preamble was already read then flip into Element mode and let it fail
            if already_read_preamble {
                self.detected_ts = &ts::ImplicitVRLittleEndian;
                self.partial_tag = Some(tag);
                self.bytes_read += bytes_read as u64;
                self.state = ParseState::Element;
                return Ok(());
            }

            // read the remainder of the preamble, the prefix
            self.dataset
                .read_exact(&mut file_preamble[bytes_read..FILE_PREAMBLE_LENGTH])?;
            self.bytes_read += file_preamble.len() as u64;
            self.file_preamble = Some(file_preamble);
            self.iterate_prefix()?;
            self.state = ParseState::DetectTransferSyntax;
            return Ok(());
        } else if !(tags::FILE_META_INFORMATION_GROUP_LENGTH..=tags::SOP_INSTANCE_UID)
            .contains(&tag)
        {
            cursor.set_position(0);
            ts = &ts::ExplicitVRBigEndian;
            tag = read::util::read_tag_from_dataset(&mut cursor, ts.is_big_endian())?;

            // if switching endian didn't result in a valid tag then try skipping preamble/prefix
            if !(tags::FILE_META_INFORMATION_GROUP_LENGTH..=tags::SOP_INSTANCE_UID).contains(&tag) {
                // if file preamble was already read then flip into Element mode and let it fail
                if already_read_preamble {
                    self.detected_ts = &ts::ImplicitVRLittleEndian;
                    self.partial_tag = Some(tag);
                    self.bytes_read += bytes_read as u64;
                    self.state = ParseState::Element;
                    return Ok(());
                }

                // read the remainder of the preamble, the prefix
                self.dataset
                    .read_exact(&mut file_preamble[bytes_read..FILE_PREAMBLE_LENGTH])?;
                self.bytes_read += file_preamble.len() as u64;
                self.file_preamble = Some(file_preamble);
                self.iterate_prefix()?;
                self.state = ParseState::DetectTransferSyntax;
                return Ok(());
            }
        }

        // if not an expected non-file-meta tag then try big-endian
        if !ts.is_big_endian() && tag < tags::FILE_META_INFORMATION_GROUP_LENGTH
            || tag > tags::SOP_INSTANCE_UID
        {
            cursor.set_position(0);
            ts = &ts::ExplicitVRBigEndian;
            tag = read::util::read_tag_from_dataset(&mut cursor, ts.is_big_endian())?;
        }

        // doesn't appear to be a valid tag in either big or little endian
        if tag < tags::FILE_META_INFORMATION_GROUP_LENGTH
            || tag > tags::SOP_INSTANCE_UID && already_read_preamble
        {
            // testing tag in either endian didn't seem to work, set as DICOM default
            self.detected_ts = &ts::ImplicitVRLittleEndian;
            self.partial_tag = Some(tag);
            self.bytes_read += bytes_read as u64;
            self.state = ParseState::Element;
            return Ok(());
        }

        // read in 4 bytes. for implicit vr 4 bytes are used for value length. if it's not implicit
        // then the first two bytes are re-parsed as vr and the later two bytes are the value length
        // note: we could attempt to match the first two bytes as a valid VR to determine implicit
        // vs. explicit vr however this current approach also works because all VR options have
        // really high binary values.
        let mut buf: [u8; 4] = [0; 4];
        self.dataset.read_exact(&mut buf)?;
        // if we haven't already skipped preamble/prefix then continue to copy bytes read from the
        // dataset into the buffer to be stored as preamble we discover that we are in a preamble
        if !already_read_preamble {
            file_preamble[bytes_read..(bytes_read + buf.len())].copy_from_slice(&buf);
        }
        bytes_read += buf.len();
        cursor = Cursor::new(&buf);

        let mut vr_is_explicit_padded: bool = false;
        match read::util::read_vr_from_dataset(&mut cursor) {
            Ok(vr) => {
                self.partial_vr = Some(vr);
                if vr.has_explicit_2byte_pad {
                    vr_is_explicit_padded = true;
                    // if explict & padded then the padding was read-in already and we have to read
                    // in the next 4 bytes for the value length.
                    self.dataset.read_exact(&mut buf)?;
                    if !already_read_preamble {
                        file_preamble[bytes_read..(bytes_read + buf.len())].copy_from_slice(&buf);
                    }
                    bytes_read += buf.len();
                    cursor = Cursor::new(&buf);
                } else {
                    // with no padding value length is only 2 bytes and was read-in as part of the
                    // buffer for vr above.
                    cursor.set_position(2);
                }

                if ts.is_big_endian() {
                    ts = &ts::ExplicitVRBigEndian;
                } else {
                    ts = &ts::ExplicitVRLittleEndian;
                }
            }
            Err(ParseError::UnknownExplicitVR(_)) => {
                // unknown VR so this was likely a value length read in, reset the 4-byte buffer
                // read in as vr and next read it as value length.
                cursor.set_position(0);
                if ts.is_big_endian() {
                    ts = &ts::ImplicitVRBigEndian;
                } else {
                    ts = &ts::ImplicitVRLittleEndian;
                }
            }
            Err(e) => {
                return Err(e);
            }
        }

        let vl_read_4bytes: bool = !ts.is_explicit_vr() || vr_is_explicit_padded;

        // assume implicit VR so read a value length and and if it's reasonably low then this is
        // likely implicit
        let vl: ValueLength = read::util::read_value_length_from_dataset(
            &mut cursor,
            vl_read_4bytes,
            ts.is_big_endian(),
        )?;
        if let ValueLength::Explicit(len) = vl {
            // if a value length is read which makes sense for file-meta elements then assume this
            // is implicit endian and let regular parsing continue
            if len < MAX_VALUE_LENGTH_IN_DETECT {
                self.detected_ts = ts;
                self.partial_tag = Some(tag);
                self.partial_vl = Some(vl);
                self.bytes_read += bytes_read as u64;
                // FileMeta is coded to read as ExplicitVRLittleEndian and since we've determined
                // this is implicit we skip to Element which will follow self.ts
                if tag < tags::FILE_META_GROUP_END {
                    if tag == tags::FILE_META_INFORMATION_GROUP_LENGTH {
                        self.state = ParseState::GroupLength;
                    } else {
                        self.state = ParseState::FileMeta;
                    }
                } else {
                    self.state = ParseState::Element;
                }
                return Ok(());
            }
        }

        if already_read_preamble {
            self.detected_ts = &ts::ImplicitVRLittleEndian;
            self.partial_tag = Some(tag);
            self.partial_vl = Some(vl);
            self.bytes_read += bytes_read as u64;
            self.state = ParseState::Element;
            return Ok(());
        }

        // garbage data so likely in preamble, finish reading preamble and prefix, restart detect
        self.dataset
            .read_exact(&mut file_preamble[bytes_read..FILE_PREAMBLE_LENGTH])?;
        self.bytes_read += file_preamble.len() as u64;
        self.file_preamble = Some(file_preamble);
        self.partial_tag = None;
        self.partial_vr = None;
        self.partial_vl = None;
        self.iterate_prefix()?;
        self.state = ParseState::DetectTransferSyntax;
        Ok(())
    }

    /// Performs the `ParserState::Preamble` iteration
    fn iterate_preamble(&mut self) -> Result<()> {
        let mut file_preamble: [u8; FILE_PREAMBLE_LENGTH] = [0; FILE_PREAMBLE_LENGTH];
        self.dataset.read_exact(&mut file_preamble)?;
        self.bytes_read += file_preamble.len() as u64;
        self.file_preamble = Some(file_preamble);
        self.state = ParseState::Prefix;
        Ok(())
    }

    /// Performs the `ParserState::Prefix` iteration
    fn iterate_prefix(&mut self) -> Result<()> {
        let mut dicom_prefix: [u8; DICOM_PREFIX_LENGTH] = [0; DICOM_PREFIX_LENGTH];
        self.dataset.read_exact(&mut dicom_prefix)?;
        self.bytes_read += dicom_prefix.len() as u64;
        for (n, prefix_item) in DICOM_PREFIX.iter().enumerate() {
            if dicom_prefix[n] != *prefix_item {
                return Err(ParseError::BadDICOMPrefix(dicom_prefix));
            }
        }
        self.dicom_prefix = Some(dicom_prefix);
        self.state = ParseState::GroupLength;
        Ok(())
    }

    /// Performs the `ParserState::GroupLength` iteration
    fn iterate_group_length(&mut self) -> Result<Option<DicomElement>> {
        let ts: TSRef = self.detected_ts;
        let tag: u32 = self.read_tag(ts)?;
        if self.is_at_parse_stop() {
            return Ok(None);
        }

        if tag != tags::FILE_META_INFORMATION_GROUP_LENGTH {
            if tag > tags::FILE_META_INFORMATION_GROUP_LENGTH && tag < tags::FILE_META_GROUP_END {
                self.state = ParseState::FileMeta;
            } else {
                self.state = ParseState::Element;
            }
            return Ok(None);
        }

        let grouplength: DicomElement = self.read_dicom_element(tag, ts)?;
        self.fmi_grouplength = u32::try_from(&grouplength)?;
        self.fmi_start = self.bytes_read;
        self.state = ParseState::FileMeta;
        // reset partial_tag to None
        self.partial_tag.take();

        Ok(Some(grouplength))
    }

    /// Performs the `ParserState::FileMeta` iteration
    fn iterate_file_meta(&mut self) -> Result<Option<DicomElement>> {
        // check if we're about to read an element which is outside the file meta section, if so
        // then change states outside of this one.
        if self.fmi_grouplength > 0
            && (self.bytes_read >= self.fmi_start + u64::from(self.fmi_grouplength))
        {
            // if we never read a transfer syntax in the file-meta then jump back to detecting the
            // transfer syntax of the main dataset.
            self.state = if self.dataset_ts.is_some() {
                ParseState::Element
            } else {
                ParseState::DetectTransferSyntax
            };
            return Ok(None);
        }

        let tag: u32 = self.read_tag(&ts::ExplicitVRLittleEndian)?;
        if self.is_at_parse_stop() {
            return Ok(None);
        }

        let element: DicomElement = self.read_dicom_element(tag, self.detected_ts)?;
        if element.tag == tags::TRANSFER_SYNTAX_UID {
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
                ParseState::Element
            } else {
                ParseState::DetectTransferSyntax
            };
        }

        // reset partial_tag to None
        self.partial_tag.take();

        Ok(Some(element))
    }

    /// Performs the `ParserState::Element` iteration
    fn iterate_element(&mut self) -> Result<Option<DicomElement>> {
        // if we're in a sequence we need to use the sequence's transfer syntax
        let ts: TSRef = self
            .current_path
            .last()
            .map(SequenceElement::get_ts)
            .or(self.dataset_ts)
            .unwrap_or(self.detected_ts);

        #[cfg(feature = "deflate")]
        if ts.deflated {
            self.dataset.set_read_deflated(true);
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
                if seq_elem.get_seq_tag() == tags::ITEM {
                    self.current_path.pop();
                }
            }
        }
        self.pop_sequence_items_base_on_byte_pos();

        // reading element clones the current path so update prior to reading element
        if tag == tags::ITEM {
            // get the sequence this item is for and increment its item number
            if let Some(seq_elem) = self.current_path.last_mut() {
                seq_elem.increment_item_num();
            }
        }

        let element: DicomElement = self.read_dicom_element(tag, ts)?;
        // if the file-meta state was skipped due to the initial detection we may still need to
        // switch transfer syntax -- only do this if the element is at the root of the dataset
        if element.tag == tags::TRANSFER_SYNTAX_UID && element.get_sequence_path().is_empty() {
            self.dataset_ts = self
                .parse_transfer_syntax(&element)?
                .or(Some(&ts::ImplicitVRLittleEndian));
        } else if element.tag == tags::SPECIFIC_CHARACTER_SET {
            let cs: CSRef = self.parse_specific_character_set(&element)?;
            if element.get_sequence_path().is_empty() {
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
                if seq_elem.get_seq_tag() == tags::ITEM {
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
            let sq_ts: TSRef = if tag == tags::ITEM {
                // item elements will have a TS of IVRLE but since this is indicates what contents
                // should parse as use the original ts determined to parse regular elements
                ts
            } else {
                // otherwise the element will have the transfer syntax for the sequence
                element.get_ts()
            };

            let seq_end_pos: Option<u64> = if let ValueLength::Explicit(len) = element.vl {
                Some(self.bytes_read + u64::from(len))
            } else {
                None
            };

            let sq_cs: CSRef = if let Some(sq) = self.current_path.last() {
                sq.get_cs()
            } else {
                self.cs
            };

            self.current_path
                .push(SequenceElement::new(tag, seq_end_pos, sq_ts, sq_cs));
        }

        Ok(Some(element))
    }
}
