use crate::core::charset::{self, CSRef, DEFAULT_CHARACTER_SET};
use crate::core::dcmelement::{DicomElement, SequenceElement};
use crate::core::dcmparser_util;
use crate::core::tagstop::TagStop;
use crate::defn::constants::lookup::MINIMAL_DICOM_DICTIONARY;
use crate::defn::constants::{tags, ts};
use crate::defn::dcmdict::DicomDictionary;
use crate::defn::tag::Tag;
use crate::defn::ts::TSRef;
use crate::defn::vl::ValueLength;
use crate::defn::vr::{self, VRRef};
use std::io::{Cursor, Error, ErrorKind, Read};

pub const FILE_PREAMBLE_LENGTH: usize = 128;
pub const DICOM_PREFIX_LENGTH: usize = 4;
const MAX_VALUE_LENGTH_IN_DETECT: u32 = 100;

pub static DICOM_PREFIX: &[u8; DICOM_PREFIX_LENGTH] = b"DICM";

/// The different parsing behaviors of the dataset.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ParseState {
    /// An initial state in which we're trying to detect if there is a preamble/prefix
    DetectState,
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

/// A builder for constructing `Parser` with common default states.
pub struct ParserBuilder<'dict, DatasetType: Read> {
    /// The dataset to parse dicom from.
    dataset: DatasetType,
    /// Initial parse state. Default is `ParseState::DetectState`.
    state: Option<ParseState>,
    /// When to stop parsing the dataset. Default is `TagStop::EndOfDataset`.
    tagstop: Option<TagStop>,
    /// The `DicomDictionary` to be used when parsing elements. Default is `MinimalDicomDictionary`.
    dictionary: &'dict dyn DicomDictionary,
}

impl<'dict, DatasetType: Read> ParserBuilder<'dict, DatasetType> {
    /// Start a new default builder for the given dataset.
    pub fn new(dataset: DatasetType) -> ParserBuilder<'dict, DatasetType> {
        ParserBuilder {
            dataset,
            state: None,
            tagstop: None,
            dictionary: &MINIMAL_DICOM_DICTIONARY,
        }
    }

    /// Sets the `TagStop` for when to stop parsing the dataset.
    pub fn tagstop(mut self, tagstop: TagStop) -> Self {
        self.tagstop = Some(tagstop);
        self
    }

    /// Sets the DICOM dictionary. The parser uses `get_ts_by_uid` to identify transfer syntax for
    /// parsing through the stream, and `get_tag_by_number` for resolving VR of parsed elements. The
    /// VR is not strictly necessary for parsing elements however there is potential for sequences
    /// to not have their sub-elements parsed properly without this.
    pub fn dictionary(mut self, dictionary: &'dict dyn DicomDictionary) -> Self {
        self.dictionary = dictionary;
        self
    }

    /// Constructs the parser from this builder.
    pub fn build(self) -> Parser<'dict, DatasetType> {
        Parser {
            dataset: self.dataset,
            tagstop: self.tagstop.unwrap_or(TagStop::EndOfDataset),
            dictionary: self.dictionary,
            state: self.state.unwrap_or(ParseState::DetectState),

            bytes_read: 0,
            file_preamble: None,
            dicom_prefix: None,
            fmi_start: 0,
            fmi_grouplength: 0,
            tag_last_read: 0,
            partial_tag: None,
            partial_vr: None,
            partial_vl: None,
            ts: &ts::ExplicitVRLittleEndian,
            cs: DEFAULT_CHARACTER_SET,
            current_path: Vec::new(),
            iterator_ended: false,
        }
    }
}

/// Provides an iterator that parses through a dicom dataset returning dicom elements.
pub struct Parser<'dict, DatasetType: Read> {
    /// The dataset to parse dicom from.
    dataset: DatasetType,

    /// The current state of reading elements from the dataset.
    state: ParseState,

    /// The condition under which this iterator should stop parsing elements from the dataset. This
    /// can be used for only partially parsing through a dataset.
    tagstop: TagStop,

    /// The DICOM dictionary. Parsing uses `get_ts_by_uid` to identify transfer syntax for parsing
    /// through the stream, and `get_tag_by_number` for resolving VR of parsed elements. The VR is
    /// not strictly necessary for parsing elements however there is potential for sequences to not
    /// have their sub-elements parsed properly without this.
    dictionary: &'dict dyn DicomDictionary,

    /// Tracks the number of bytes read from the dataset. It's not required that the dataset
    /// implement `Seek` (network streams won't implement `Seek` without a buffer). Bytes read from
    /// the dataset are counted in order to track relative positioning for allocating elements with
    /// defined value lengths. Used to determine if File Meta elements are being parsed vs. switch
    /// to regular elements. Also used to track when sequences of explicit length begin/end.
    bytes_read: u64,

    /// The file preamble read from the dataset. Not all datasets may have a preamble.
    file_preamble: Option<[u8; FILE_PREAMBLE_LENGTH]>,

    /// The prefix read from the dataset. This should be a value of `"DICM"` but not all datasets
    /// have a prefix. If the dataset has a file preamble then it should also have a prefix.
    dicom_prefix: Option<[u8; DICOM_PREFIX_LENGTH]>,

    /// The number of bytes read just after having read the `FileMetaInformationGroupLength`. This
    /// is used to determine how many bytes to continue parsing until we switch to reading regular
    /// DICOM elements, by checking `bytes_read` against `fmi_start + fmi_grouplength`.
    fmi_start: u64,

    /// The value of the `FileMetaInformationGroupLength` tag if one is present. If present this is
    /// is the number of bytes remaining in the File Meta Information section until the non-meta
    /// section of the DICOM dataset starts. Only after the File Meta Information section does the
    /// transfer syntax and character encoding take effect. If the dataset does not contain the
    /// `FileMetaInformationGroupLength` element then this will have a value of zero and unused.
    fmi_grouplength: u32,

    /// This is the last element tag successfully read from the dataset, regardless of whether
    /// the element it's for was successfully finished parsing.
    tag_last_read: u32,

    /// This is the element tag currently being read from the dataset. It will be `Some` once the
    /// element starts parsing and will be `None` after the element has completed parsing. Elements
    /// may be partially parsed either due to parsing errors or `TagStop`.
    partial_tag: Option<u32>,

    /// This is the element's VR read from the dataset when in `ParseState::DetectState`. This will
    /// only ever be used once in this regard. Since bytes need to be parsed from the dataset in
    /// order to detect the transfer syntax, if the file preamble is missing then this will be
    /// set as the vr parsed from the dataset of the first valid dicom element (or it will be the
    /// look-up if implicit vr is determined).
    partial_vr: Option<VRRef>,

    /// This is the element's value length read from the dataset when in `ParseState::DetectState`.
    /// This wll only ever be used once in this regard. Since bytes need to be parsed from the
    /// dataset in order to detect the transfer syntax, if the file preamble is missing then this
    /// will be set as the value length parsed from the dataset of the first valid dicom element.
    partial_vl: Option<ValueLength>,

    /// The transfer syntax used for this dataset. This defaults to `ExplicitVRLittleEndian` which
    /// is the transfer syntax used for parsing File Meta section despite the default DICOM transfer
    /// syntax being `ImplicitVRLittleEndian`.
    ts: TSRef,

    /// The specific character set used for this dataset. This defaults to the dicom default which
    /// is `WINDOWS_1252` but is changed after having successully parsed the specific character set
    /// element.
    cs: CSRef,

    /// The current sequence stack. Whenever an SQ element is parsed a new `SequenceElement` is
    /// appened to this stack. The last element is popped of when the sequence ends (via byte
    /// position or `SequenceDelimitationItem`). This also tracks the current `Item` within a
    /// sequence. Whenever an `Item` element is read the last element in this list has its item
    /// count initialized/incremented. Every element parsed from the dataset clones this stack.
    current_path: Vec<SequenceElement>,

    /// When the `next()` returns an `Error` or `None` future calls to `next()` should not attempt
    /// to read from the dataset. This is used to track when the iterator should be considered fully
    /// consumed in those cases and prevent further attempts at reading from the dataset.
    iterator_ended: bool,
}

impl<'dict, DatasetType: Read> Parser<'dict, DatasetType> {
    pub fn get_bytes_read(&self) -> u64 {
        self.bytes_read
    }

    pub fn get_partial_tag(&self) -> Option<u32> {
        self.partial_tag
    }

    pub fn get_parser_state(&self) -> ParseState {
        self.state
    }

    pub fn get_ts(&self) -> TSRef {
        self.ts
    }

    pub fn get_cs(&self) -> CSRef {
        self.cs
    }

    pub fn get_dictionary(&self) -> &'dict dyn DicomDictionary {
        self.dictionary
    }

    pub fn get_file_preamble(&self) -> &Option<[u8; FILE_PREAMBLE_LENGTH]> {
        &self.file_preamble
    }

    pub fn get_dicom_prefix(&self) -> &Option<[u8; DICOM_PREFIX_LENGTH]> {
        &self.dicom_prefix
    }

    /// This needs to be checked multiple times during parsing of an element
    /// 1. Before reading an element will catch `TagStop::AfterTag` and `TagStop::AfterBytePos`
    /// 2. After reading the tag value will catch `TagStop::BeforeTag` and `TagStop::AfterBytePos`
    fn is_at_tag_stop(&self) -> Result<bool, Error> {
        let is_at_tag_stop: bool = match self.tagstop {
            TagStop::EndOfDataset => false,
            TagStop::BeforeTag(before_tag) => {
                self.current_path.is_empty() && self.tag_last_read >= before_tag
            }
            TagStop::AfterTag(after_tag) => {
                self.current_path.is_empty() && self.tag_last_read > after_tag
            }
            TagStop::AfterBytePos(byte_pos) => self.bytes_read > byte_pos,
        };

        Ok(is_at_tag_stop)
    }

    /// Reads a tag attribute from the dataset
    fn read_tag(&mut self, ts: TSRef) -> Result<u32, Error> {
        let result: Result<u32, Error> =
            dcmparser_util::read_tag_from_dataset(&mut self.dataset, ts.is_big_endian());
        if result.is_ok() {
            self.bytes_read += 4;
        }
        result
    }

    /// Reads the remainder of the dicom element from the dataset. This assumes `self.read_tag()`
    /// was called just prior and its result passed as the tag parameter here.
    fn read_dicom_element(&mut self, tag: u32, ts: TSRef) -> Result<DicomElement, Error> {
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
        } else {
            self.read_vr(tag, ts)?
        };

        let vr: VRRef = vr_ts.0;
        // If VR is explicitly UN but we can tell it's SQ then the inner elements are encoded as
        // IVRLE -- but only the contents should be parsed as such, do not switch transfer syntax
        // prior to reading in the value length.
        let vl: ValueLength = if let Some(partial_vl) = self.partial_vl {
            self.partial_vl.take();
            partial_vl
        } else {
            self.read_value_length(vr, ts)?
        };

        let parse_as_seq: bool = dcmparser_util::is_non_standard_seq(tag, vr, vl);
        let ts: TSRef = if parse_as_seq {
            &ts::ImplicitVRLittleEndian
        } else {
            vr_ts.1
        };

        let bytes: Vec<u8> = if vr == &vr::SQ || parse_as_seq || tag == tags::ITEM {
            // Sequence and item elements should let the iterator handle parsing its contents and
            // not associate bytes to the element's value
            Vec::new()
        } else {
            self.read_value_field(vl)?
        };

        let ancestors: Vec<SequenceElement> = self.current_path.clone();
        Ok(DicomElement::new(
            tag, vr, vl, ts, self.cs, bytes, ancestors,
        ))
    }

    /// If the given transfer syntax has Explicit VR then this reads a VR attribute from the dataset
    /// using the given transfer syntax. This returns a tuple of `(VRRef, TSRef)` as if the VR is
    /// written explicitly as `UN` but the tag dictionary being used for parsing knows the VR is
    /// non-`UN` then the element value should be read with IVRLE.
    ///
    /// If the given transfer syntax is Implicit VR then this does not read from the dataset but
    /// does a lookup of the VR based on the tag dictionary used for parsing.
    fn read_vr(&mut self, tag: u32, ts: TSRef) -> Result<(VRRef, TSRef), Error> {
        if !ts.explicit_vr {
            return Ok((self.lookup_vr(tag)?, ts));
        }

        let result: Result<Option<VRRef>, Error> =
            dcmparser_util::read_vr_from_dataset(&mut self.dataset);
        if let Ok(Some(vr)) = result {
            self.bytes_read += 2;
            if vr.has_explicit_2byte_pad {
                self.bytes_read += 2;
            }
        }
        let mut vr: VRRef = result?.unwrap_or(&vr::UN);

        // Part 5 Section 6.2.2 Note 2
        // If at some point an application knows the actual VR for an Attribute of VR UN (e.g., has
        // has its own applicable data dictionary), it can assume that the Value Field of the
        // Attribute is encoded in Little Endian byte ordering with implicit VR encoding,
        // irrespective of the current Transfer Syntax.
        let mut ts: TSRef = ts;
        if vr == &vr::UN {
            vr = self.lookup_vr(tag)?;
            if vr != &vr::UN {
                ts = &ts::ImplicitVRLittleEndian;
            }
        }

        Ok((vr, ts))
    }

    /// Looks up the VR of the given tag in the current lookup dictionary, or `UN` if not present.
    fn lookup_vr(&self, tag: u32) -> Result<VRRef, Error> {
        Ok(self
            .dictionary
            .get_tag_by_number(tag)
            .and_then(|read_tag: &Tag| read_tag.implicit_vr)
            .unwrap_or(&vr::UN))
    }

    /// Reads a Value Length attribute from the dataset using the given transfer syntax. The number
    /// of bytes representing the value length depends on transfer syntax. If the VR has a 2-byte
    /// padding then those bytes are also read from the dataset.
    fn read_value_length(&mut self, vr: VRRef, ts: TSRef) -> Result<ValueLength, Error> {
        let read_4bytes: bool = !ts.explicit_vr || vr.has_explicit_2byte_pad;
        let result: Result<ValueLength, Error> = dcmparser_util::read_value_length_from_dataset(
            &mut self.dataset,
            read_4bytes,
            ts.big_endian,
        );
        if result.is_ok() {
            if !ts.explicit_vr || vr.has_explicit_2byte_pad {
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
    fn read_value_field(&mut self, vl: ValueLength) -> Result<Vec<u8>, Error> {
        match vl {
            // Undefined length means that the contents of the element are other dicom elements to
            // be parsed. Don't read data from the dataset in this case.
            ValueLength::Explicit(0) | ValueLength::UndefinedLength => Ok(Vec::new()),
            ValueLength::Explicit(value_length) => {
                let mut bytes: Vec<u8> = vec![0; value_length as usize];
                self.dataset.read_exact(bytes.as_mut_slice())?;
                self.bytes_read += u64::from(value_length);
                Ok(bytes)
            }
        }
    }

    /// Parses the value of the given element as the transfer syntax and sets the `ts` value on this
    /// iterator to affect the reading of further dicom elements. If the transfer syntax cannot be
    /// resolved then this sets it to the default DICOM transfer syntax which is IVRLE.
    fn parse_transfer_syntax(&mut self, element: &DicomElement) -> Result<(), Error> {
        let ts_uid: String = element.parse_string()?;

        self.ts = self
            .dictionary
            .get_ts_by_uid(ts_uid.as_ref())
            .unwrap_or(&ts::ImplicitVRLittleEndian);

        Ok(())
    }

    /// Parses the value of the given element as the specific character set and sets the `cs` value
    /// on this iterator to affect the parsing of further text-type element values.
    fn parse_specific_character_set(&mut self, element: &DicomElement) -> Result<(), Error> {
        let new_cs: Option<String> = element
            .parse_strings()?
            .into_iter()
            .filter(|cs_entry: &String| !cs_entry.is_empty())
            .nth(0);

        // TODO: There are options for what to do if we can't support the character repertoire
        //       See note on Ch 5 Part 6.1.2.3 under "Considerations on the Handling of
        //       Unsupported Character Sets"

        self.cs = new_cs
            .and_then(|cs: String| charset::lookup_charset(&cs))
            .unwrap_or(charset::DEFAULT_CHARACTER_SET);
        Ok(())
    }

    /// Performs the primary iteration for the parser but the return type is consistent for error
    /// handling and not iteration. This should be called once for each invocation of `next()`.
    fn iterate(&mut self) -> Result<Option<DicomElement>, Error> {
        // The earlier parse states will read non-elements from the dataset and move to another
        // state. A loop is used so once those succeed they continue the loop and move to next
        // states which will eventually return a dicom element.
        loop {
            if self.is_at_tag_stop()? {
                return Ok(None);
            }

            match self.state {
                ParseState::DetectState => {
                    self.iterate_detect_state()?;
                    continue;
                }
                ParseState::Preamble => {
                    self.iterate_preamble()?;
                    continue;
                }
                ParseState::Prefix => {
                    self.iterate_prefix()?;
                    continue;
                }
                ParseState::GroupLength => match self.iterate_group_length()? {
                    None => {
                        // if none is returned and the state changed then let the iterator go to the
                        // next state -- it's likely group length wasn't read but another tag was
                        if self.state != ParseState::GroupLength {
                            continue;
                        }
                        return Ok(None);
                    }
                    Some(element) => return Ok(Some(element)),
                },
                ParseState::FileMeta => match self.iterate_file_meta()? {
                    None => {
                        // if none is returned and the state changed then let the iterator go to the
                        // next state -- it's likely file meta wasn't read but another tag was
                        if self.state != ParseState::FileMeta {
                            continue;
                        }
                        return Ok(None);
                    }
                    Some(element) => return Ok(Some(element)),
                },
                ParseState::Element => match self.iterate_element() {
                    Err(e) => {
                        // only check EOF when reading beginning of elements as it would
                        // actually be expected in this scenario since the DICOM format provides
                        // no determination for end of the dicom object
                        if e.kind() == ErrorKind::UnexpectedEof {
                            return Ok(None);
                        }
                        return Err(e);
                    }
                    element_result => return element_result,
                },
            }
        }
    }

    /// Performs the `ParserState::DetectState` iteration.
    /// The strategy this uses is to parse just a few bytes from the dataset to see if it looks like
    /// the start of a dicom element.
    /// 1. Parse a tag and check whether it looks like a valid tag value (`<= SOP_INSTANCE_UID`).
    ///    The tag is parsed as both little-endian and as big-endian.
    /// 2. Parse a value length, assuming implicit vr. If the value length parsed is reasonable
    ///    (`<MAX_VALUE_LENGTH_IN_DETECT`) then implicit vr is assumed. The elements at the
    ///    beginning of a dataset should have fairly small values (for things like UIDs, etc.). The
    ///    Pixel-med library also uses this same value comparison.
    /// 3. If value length is too large then it is re-parsed as a VR. If a valid VR was not found
    ///    then we assume it's proprietary data in the preamble.
    /// 4. If the VR is valid then attempt reading another value length (only 2-bytes since we know
    ///    it's implicit vr and any valid vr this early in dataset should not have a 2-byte
    ///    padding). If the value length is reasonable (`<MAX_VALUE_LENGTH_IN_DETECT`) then explict
    ///    vr is assumed.
    /// 5. Otherwise it's assumed the start of the file is proprietary file preamble.
    fn iterate_detect_state(&mut self) -> Result<(), Error> {
        // start off assuming IVRLE
        let mut ts: TSRef = &ts::ImplicitVRLittleEndian;

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

        let mut tag: u32 = dcmparser_util::read_tag_from_dataset(&mut cursor, ts.is_big_endian())?;

        // quick check for common case of being zeroed-out data
        if tag == 0 {
            // read the remainder of the preamble, move onto reading prefix
            self.dataset
                .read_exact(&mut file_preamble[bytes_read..FILE_PREAMBLE_LENGTH])?;
            self.bytes_read += file_preamble.len() as u64;
            self.file_preamble = Some(file_preamble);
            self.state = ParseState::Prefix;
            return Ok(());
        }

        // quick check if we're reading beginning of file meta, continue from there
        if tag >= tags::FILE_META_INFORMATION_GROUP_LENGTH && tag < tags::FILE_META_GROUP_END {
            // there's no preamble/prefix, set the partial tag read, jump right to file-meta
            self.partial_tag = Some(tag);
            self.bytes_read += bytes_read as u64;
            if tag == tags::FILE_META_INFORMATION_GROUP_LENGTH {
                self.state = ParseState::GroupLength;
            } else {
                self.state = ParseState::FileMeta;
            }
            return Ok(());
        }

        // if not an expected non-file-meta tag then try big-endian
        if tag > tags::SOP_INSTANCE_UID {
            cursor.set_position(0);
            ts = &ts::ExplicitVRBigEndian;
            tag = dcmparser_util::read_tag_from_dataset(&mut cursor, ts.is_big_endian())?;
        }

        // doesn't appear to be a valid tag in either big or little endian, assume it's preamble
        if tag > tags::SOP_INSTANCE_UID {
            // read the remainder of the preamble, move onto reading prefix
            self.dataset
                .read_exact(&mut file_preamble[bytes_read..FILE_PREAMBLE_LENGTH])?;
            self.bytes_read += file_preamble.len() as u64;
            self.file_preamble = Some(file_preamble);
            self.state = ParseState::Prefix;
            return Ok(());
        }

        // read in 4 bytes. for implicit vr 4 bytes are used for value length. if it's not implicit
        // then the first two bytes are re-parsed as vr and the later two bytes are the value length
        let mut buf: [u8; 4] = [0; 4];
        self.dataset.read_exact(&mut buf)?;

        file_preamble[bytes_read..(bytes_read + buf.len())].copy_from_slice(&buf);
        bytes_read += buf.len();
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);

        // read value length and if it has a reasonably low number assume implicit VR
        let vl: ValueLength =
            dcmparser_util::read_value_length_from_dataset(&mut cursor, true, ts.big_endian)?;
        if let ValueLength::Explicit(len) = vl {
            if len < MAX_VALUE_LENGTH_IN_DETECT {
                self.ts = ts;
                self.partial_tag = Some(tag);
                self.partial_vr = Some(self.lookup_vr(tag)?);
                self.partial_vl = Some(vl);
                self.bytes_read += bytes_read as u64;
                self.state = ParseState::Element;
                return Ok(());
            }
        }

        // otherwise backtrack and try to parse a vr and then vl
        cursor.set_position(0);
        let vr: VRRef = match dcmparser_util::read_vr_from_dataset(&mut cursor)? {
            Some(vr) => vr,
            None => {
                // garbage data so likely in preamble, finish reading preamble and jump to prefix
                self.dataset
                    .read_exact(&mut file_preamble[bytes_read..FILE_PREAMBLE_LENGTH])?;
                self.bytes_read += file_preamble.len() as u64;
                self.file_preamble = Some(file_preamble);
                self.state = ParseState::Prefix;
                return Ok(());
            }
        };

        // read value length and if it has a reasonably low number assume explicit VR
        let vl: ValueLength =
            dcmparser_util::read_value_length_from_dataset(&mut cursor, false, ts.big_endian)?;
        if let ValueLength::Explicit(len) = vl {
            if len < MAX_VALUE_LENGTH_IN_DETECT {
                self.ts = if ts.big_endian {
                    &ts::ExplicitVRBigEndian
                } else {
                    &ts::ExplicitVRLittleEndian
                };
                self.partial_tag = Some(tag);
                self.partial_vr = Some(vr);
                self.partial_vl = Some(vl);
                self.bytes_read += bytes_read as u64;
                self.state = ParseState::Element;
                return Ok(());
            }
        }

        // garbage data so likely in preamble, finish reading preamble and jump to prefix
        self.dataset
            .read_exact(&mut file_preamble[bytes_read..FILE_PREAMBLE_LENGTH])?;
        self.bytes_read += file_preamble.len() as u64;
        self.file_preamble = Some(file_preamble);
        self.state = ParseState::Prefix;
        Ok(())
    }

    /// Performs the `ParserState::Preamble` iteration
    fn iterate_preamble(&mut self) -> Result<(), Error> {
        let mut file_preamble: [u8; FILE_PREAMBLE_LENGTH] = [0; FILE_PREAMBLE_LENGTH];
        self.dataset.read_exact(&mut file_preamble)?;
        self.bytes_read += file_preamble.len() as u64;
        self.file_preamble = Some(file_preamble);
        self.state = ParseState::Prefix;
        Ok(())
    }

    /// Performs the `ParserState::Prefix` iteration
    fn iterate_prefix(&mut self) -> Result<(), Error> {
        let mut dicom_prefix: [u8; DICOM_PREFIX_LENGTH] = [0; DICOM_PREFIX_LENGTH];
        self.dataset.read_exact(&mut dicom_prefix)?;
        self.bytes_read += dicom_prefix.len() as u64;
        for (n, prefix_item) in DICOM_PREFIX.iter().enumerate() {
            if dicom_prefix[n] != *prefix_item {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid DICOM Prefix: {:?}", dicom_prefix),
                ));
            }
        }
        self.dicom_prefix = Some(dicom_prefix);
        self.state = ParseState::GroupLength;
        Ok(())
    }

    /// Performs the `ParserState::GroupLength` iteration
    fn iterate_group_length(&mut self) -> Result<Option<DicomElement>, Error> {
        let tag: u32 = if let Some(partial_tag) = self.partial_tag {
            partial_tag
        } else {
            let tag: u32 = self.read_tag(&ts::ExplicitVRLittleEndian)?;
            self.partial_tag.replace(tag);
            tag
        };
        self.tag_last_read = tag;

        if self.is_at_tag_stop()? {
            return Ok(None);
        }

        if tag != tags::FILE_META_INFORMATION_GROUP_LENGTH {
            if tag > tags::FILE_META_INFORMATION_GROUP_LENGTH && tag < tags::FILE_META_GROUP_END {
                self.state = ParseState::FileMeta;
                return Ok(None);
            } else {
                self.state = ParseState::Element;
                return Ok(None);
            }
        }

        let grouplength: DicomElement =
            self.read_dicom_element(tag, &ts::ExplicitVRLittleEndian)?;
        self.fmi_grouplength = grouplength.parse_u32()?;
        self.fmi_start = self.bytes_read;
        self.state = ParseState::FileMeta;
        // reset partial_tag to None
        self.partial_tag.take();

        Ok(Some(grouplength))
    }

    /// Performs the `ParserState::FileMeta` iteration
    fn iterate_file_meta(&mut self) -> Result<Option<DicomElement>, Error> {
        let tag: u32 = if let Some(partial_tag) = self.partial_tag {
            partial_tag
        } else {
            let tag: u32 = self.read_tag(&ts::ExplicitVRLittleEndian)?;
            self.partial_tag.replace(tag);
            tag
        };
        self.tag_last_read = tag;

        if self.is_at_tag_stop()? {
            return Ok(None);
        }

        // if group length wasn't read then check the group value of the tag just read before
        // parsing the element
        if self.fmi_grouplength == 0
            && (tag < tags::FILE_META_INFORMATION_GROUP_LENGTH || tag > tags::FILE_META_GROUP_END)
        {
            self.state = ParseState::Element;
            return Ok(None);
        }

        let element: DicomElement = self.read_dicom_element(tag, &ts::ExplicitVRLittleEndian)?;
        if element.tag == tags::TRANSFER_SYNTAX_UID {
            self.parse_transfer_syntax(&element)?;
        }

        // if group length was read use the byte position to determine if we're out of file-meta
        if self.fmi_grouplength > 0
            && (self.bytes_read >= self.fmi_start + u64::from(self.fmi_grouplength))
        {
            self.state = ParseState::Element;
        }

        // reset partial_tag to None
        self.partial_tag.take();

        Ok(Some(element))
    }

    /// Performs the `ParserState::Element` iteration
    fn iterate_element(&mut self) -> Result<Option<DicomElement>, Error> {
        // if we're in a sequence we need to use the sequence's transfer syntax
        let ts: TSRef = self
            .current_path
            .last()
            .map(SequenceElement::get_ts)
            .unwrap_or(self.ts);

        let tag: u32 = if let Some(partial_tag) = self.partial_tag {
            partial_tag
        } else {
            let tag: u32 = self.read_tag(ts)?;
            self.partial_tag.replace(tag);
            tag
        };
        self.tag_last_read = tag;

        if self.is_at_tag_stop()? {
            return Ok(None);
        }

        // reading element clones the current path so update prior to reading element
        if tag == tags::ITEM {
            // get the sequence this item is for and increment its item number
            if let Some(seq_elem) = self.current_path.last_mut() {
                seq_elem.increment_item_number();
            }
        }

        let element: DicomElement = self.read_dicom_element(tag, ts)?;
        if element.tag == tags::SPECIFIC_CHARACTER_SET {
            self.parse_specific_character_set(&element)?;
        }

        // reset partial_tag to None
        self.partial_tag.take();

        // check for exiting a sequence based on being sequence delimiter
        // do this before checking against byte position
        if element.tag == tags::SEQUENCE_DELIMITATION_ITEM {
            self.current_path.pop();
        } else if element.tag == tags::ITEM_DELIMITATION_ITEM {
            if let Some(seq_elem) = self.current_path.last_mut() {
                seq_elem.decrement_item_num();
            }
        }

        // sequence may not have a delimiter and should end based on byte position
        // multiple sequences may have been exited based off byte position
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

        if element.is_seq_like() {
            let seq_end_pos: Option<u64> = if let ValueLength::Explicit(len) = element.vl {
                Some(self.bytes_read + u64::from(len))
            } else {
                None
            };
            self.current_path
                .push(SequenceElement::new(tag, seq_end_pos, element.get_ts()));
        }

        Ok(Some(element))
    }
}

impl<'dict, DatasetType: Read> Iterator for Parser<'dict, DatasetType> {
    type Item = Result<DicomElement, Error>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.iterator_ended {
            return None;
        }

        match self.iterate() {
            Err(e) => {
                self.iterator_ended = true;
                Some(Err(e))
            }
            Ok(None) => {
                self.iterator_ended = true;
                None
            }
            Ok(Some(element)) => Some(Ok(element)),
        }
    }
}
