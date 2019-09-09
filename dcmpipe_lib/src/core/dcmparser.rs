use crate::core::charset::{self, CSRef, DEFAULT_CHARACTER_SET};
use crate::core::dcmelement::{DicomElement, SequenceElement};
use crate::core::tagstop::TagStop;
use crate::defn::constants::{lookup, tags, ts};
use crate::defn::tag::{Tag, TagRef};
use crate::defn::ts::TSRef;
use crate::defn::vl::{self, ValueLength};
use crate::defn::vr::{self, VRRef, VR};
use std::io::{Cursor, Error, ErrorKind, Read};

pub const FILE_PREAMBLE_LENGTH: usize = 128;
pub const DICOM_PREFIX_LENGTH: usize = 4;

pub static DICOM_PREFIX: &[u8; DICOM_PREFIX_LENGTH] = b"DICM";

pub type TagByValueLookup = &'static phf::Map<u32, TagRef>;

pub type TsByUidLookup = &'static phf::Map<&'static str, TSRef>;

/// If the tag isn't Item and VR isn't SQ but ValueLength is Undefined then element should be
/// considered a private-tag sequence whose contents are encoded as IVRLE.
pub fn should_parse_as_seq(tag: u32, vr: VRRef, vl: ValueLength) -> bool {
    tag != tags::ITEM
        && (vr == &vr::UN || vr == &vr::OB || vr == &vr::OW)
        && vl == ValueLength::UndefinedLength
}

/// The different parsing behaviors of the stream
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ParseState {
    /// An initial state in which we're trying to detect if there is a preamble/prefix
    DetectState,
    /// The File Preamble. This is not required for all dicom streams but is commonly present in
    /// file media.
    Preamble,
    /// The DICOM prefix. This is only present if Preamble is present.
    Prefix,
    /// The first element of a valid dicom stream which is from the File Meta group of tags which
    /// are always encoded as `ExplicitVRLittleEndian`. The value of this first element is the
    /// remaining bytes of the File Meta group.
    GroupLength,
    /// The first set of elements of a valid dicom stream which provide details on how the dicom
    /// stream is encoded. These elements are always encoded using `ExplicitVRLittleEndian`.
    FileMeta,
    /// The primary content of a dicom stream. They are parsed using the transfer syntax specified
    /// in the File Meta group
    Element,
}

/// A builder for constructing `Parser` with common default states
pub struct ParserBuilder<StreamType: Read> {
    /// The stream to parse dicom from.
    stream: StreamType,
    /// Initial parse state. Default is `ParseState::DetectState`.
    state: Option<ParseState>,
    /// When to stop parsing the stream. Default is `TagStop::EndOfStream`.
    tagstop: Option<TagStop>,
    /// Lookup of tags by tag number.
    tag_by_value: Option<TagByValueLookup>,
    /// Lookup of transfer syntax by uid.
    ts_by_uid: Option<TsByUidLookup>,
}

impl<StreamType: Read> ParserBuilder<StreamType> {
    /// Start a new default builder for the given stream.
    pub fn new(stream: StreamType) -> ParserBuilder<StreamType> {
        ParserBuilder {
            stream,
            state: None,
            tagstop: None,
            tag_by_value: None,
            ts_by_uid: None,
        }
    }

    /// Sets the `TagStop` for when to stop parsing the stream.
    pub fn tagstop(mut self, tagstop: TagStop) -> Self {
        self.tagstop = Some(tagstop);
        self
    }

    /// Sets the tag lookup map.
    pub fn tag_by_value(mut self, tag_by_value: TagByValueLookup) -> Self {
        self.tag_by_value = Some(tag_by_value);
        self
    }

    /// Sets the transfer syntax lookup map.
    pub fn ts_by_uid(mut self, ts_by_uid: TsByUidLookup) -> Self {
        self.ts_by_uid = Some(ts_by_uid);
        self
    }

    /// Constructs the parser from this builder.
    pub fn build(self) -> Parser<StreamType> {
        Parser {
            stream: self.stream,
            tagstop: self.tagstop.unwrap_or(TagStop::EndOfStream),
            tag_by_value: self.tag_by_value,
            ts_by_uid: self.ts_by_uid,
            state: self.state.unwrap_or(ParseState::DetectState),

            bytes_read: 0,
            file_preamble: [0u8; FILE_PREAMBLE_LENGTH],
            dicom_prefix: [0u8; DICOM_PREFIX_LENGTH],
            fmi_start: 0,
            fmi_grouplength: 0,
            tag_last_read: 0,
            partial_tag: None,
            ts: &ts::ExplicitVRLittleEndian,
            cs: DEFAULT_CHARACTER_SET,
            current_path: Vec::new(),
            iterator_ended: false,
        }
    }
}

/// Provides an iterator that parses through a dicom stream returning top-level elements
pub struct Parser<StreamType: Read> {
    /// The stream to parse dicom from.
    stream: StreamType,

    /// The current state of reading elements from the stream.
    state: ParseState,

    /// The condition under which this iterator should stop parsing elements from the stream.
    /// This allows for partially parsing through the stream.
    tagstop: TagStop,

    /// Lookup map for identifying tags by their tag number. Needed for resolving implicit VRs.
    tag_by_value: Option<TagByValueLookup>,

    /// Lookup map for identifying transfer sytnax by their UID.
    ts_by_uid: Option<TsByUidLookup>,

    /// Tracks the number of bytes read from the stream. We don't require that the stream implement
    /// `Seek` (network streams won't implement `Seek` without a buffer). Bytes read from the stream
    /// are counted in order to track relative positioning for allocating elements with defined
    /// value lengths. Currently used for determining if still parsing File Meta elements vs.
    /// switching to parsing regular dicom elements. This is also used for tracking when sequences
    /// of explicit length begin/end.
    bytes_read: u64,

    /// The file preamble read from the stream. Currently this doesn't differentiate between the
    /// stream having a preamble of all zeros or not having a preamble at all.
    file_preamble: [u8; FILE_PREAMBLE_LENGTH],

    /// The prefix read from the stream. This should be a value of `"DICM"` but not all streams have
    /// a prefix.
    dicom_prefix: [u8; DICOM_PREFIX_LENGTH],

    /// The number of bytes read just after having read the `FileMetaInformationGroupLength`. This
    /// is used to determine how many bytes to continue parsing until we switch to reading regular
    /// DICOM elements, by checking `bytes_read` against `fmi_start + fmi_grouplength`.
    fmi_start: u64,

    /// The value of the `FileMetaInformationGroupLength` tag, which is the number of bytes
    /// remaining in the File Meta Information section until the non-meta section of the DICOM
    /// stream starts. Only after the File Meta Information section does the transfer syntax and
    /// character encoding take effect.
    fmi_grouplength: u32,

    /// This is the last element tag successfully read from the stream, regardless of whether
    /// the element it's for was successfully finished parsing.
    tag_last_read: u32,

    /// This is the element tag currently being read from the stream. It will be `Some` once the
    /// element starts parsing and will be `None` after the element has completed parsing. Elements
    /// may be only partially parsed either due to IO errors or `TagStop`.
    partial_tag: Option<u32>,

    /// The transfer syntax used for this stream. This defaults to `ExplicitVRLittleEndian` which is
    /// the transfer syntax used for parsing File Meta section despite the default DICOM transfer
    /// syntax being `ImplicitVRLittleEndian`.
    ts: TSRef,

    /// The specific character set used for this stream. This defaults to the dicom default which
    /// is `WINDOWS_1252` but is changed after having successully parsed the specific character
    /// set element.
    cs: CSRef,

    /// The current sequence stack. Whenever an SQ element is parsed a new `SequenceElement` is
    /// appened to this stack. When the sequence ends (via byte position or
    /// `SequenceDelimitationItem`) then the last element is popped off. This also tracks the
    /// current `Item` within a sequence. Whenever an `Item` element is read the last element in
    /// this list has its item count initialized/incremented. Every element parsed from the stream
    /// clones this stack.
    current_path: Vec<SequenceElement>,

    /// When the `next()` returns an `Error` or `None` future calls to `next()` should not attempt
    /// to read from the stream. This is used to track when the iterator should be considered fully
    /// consumed in those cases and prevent further attempts at reading from the stream.
    iterator_ended: bool,
}

impl<StreamType: Read> Parser<StreamType> {
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

    pub fn get_file_preamble(&self) -> &[u8] {
        &self.file_preamble
    }

    pub fn get_dicom_prefix(&self) -> &[u8] {
        &self.dicom_prefix
    }

    /// This needs to be checked multiple times during parsing of an element
    /// 1. Before reading an element will catch `TagStop::AfterTag` and `TagStop::AfterBytePos`
    /// 2. After reading the tag value will catch `TagStop::BeforeTag` and `TagStop::AfterBytePos`
    fn is_at_tag_stop(&self) -> Result<bool, Error> {
        let is_at_tag_stop: bool = match self.tagstop {
            TagStop::EndOfStream => false,
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

    /// Reads a tag attribute from the stream
    fn read_tag(&mut self, ts: TSRef) -> Result<u32, Error> {
        let mut buf: [u8; 2] = [0; 2];
        self.stream.read_exact(&mut buf)?;
        self.bytes_read += 2;

        let group_number: u32 = if ts.is_big_endian() {
            u32::from(u16::from_be_bytes(buf)) << 16
        } else {
            u32::from(u16::from_le_bytes(buf)) << 16
        };

        self.stream.read_exact(&mut buf)?;
        self.bytes_read += 2;
        let element_number: u32 = if ts.is_big_endian() {
            u32::from(u16::from_be_bytes(buf))
        } else {
            u32::from(u16::from_le_bytes(buf))
        };

        let tag: u32 = group_number + element_number;
        Ok(tag)
    }

    /// Reads a tag attribute from a given buffer instead of the parser's stream
    fn read_tag_from_stream(
        &mut self,
        stream: &mut Cursor<&[u8]>,
        ts: TSRef,
    ) -> Result<u32, Error> {
        let mut buf: [u8; 2] = [0; 2];

        stream.read_exact(&mut buf)?;
        let group_number: u32 = if ts.is_big_endian() {
            u32::from(u16::from_be_bytes(buf)) << 16
        } else {
            u32::from(u16::from_le_bytes(buf)) << 16
        };

        stream.read_exact(&mut buf)?;
        let element_number: u32 = if ts.is_big_endian() {
            u32::from(u16::from_be_bytes(buf))
        } else {
            u32::from(u16::from_le_bytes(buf))
        };

        let tag: u32 = group_number + element_number;
        Ok(tag)
    }

    /// Reads the remainder of the dicom element from the stream. This assumes `self.read_tag()` was
    /// called just prior and its result passed as the tag parameter here.
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

        let vr_ts: (VRRef, TSRef) = self.read_vr(tag, ts)?;
        let vr: VRRef = vr_ts.0;
        // If VR is explicitly UN but we can tell it's SQ then the inner elements are encoded as
        // IVRLE -- but only the contents should be parsed as such, do not switch transfer syntax
        // prior to reading in the value length.
        let vl: ValueLength = self.read_value_length(vr, ts)?;

        let parse_as_seq: bool = should_parse_as_seq(tag, vr, vl);
        let ts: TSRef = if parse_as_seq {
            &ts::ImplicitVRLittleEndian
        } else {
            vr_ts.1
        };

        let bytes: Vec<u8> = if vr == &vr::SQ || parse_as_seq {
            // Sequence elements should let the iterator handle parsing its contents
            // and not associate bytes to the element's value
            Vec::new()
        } else {
            self.read_value_field(vl)?
        };

        let ancestors: Vec<SequenceElement> = self.current_path.clone();
        Ok(DicomElement::new(
            tag, vr, vl, ts, self.cs, bytes, ancestors,
        ))
    }

    /// If the given transfer syntax has Explicit VR then this reads a VR attribute from the stream
    /// using the given transfer syntax. This returns a tuple of `(VRRef, TSRef)` as if the VR is
    /// written explicitly as `UN` but the tag dictionary being used for parsing knows the VR is
    /// non-`UN` then the element value should be read with IVRLE.
    ///
    /// If the given transfer syntax is Implicit VR then this does not read from the stream but does
    /// a lookup of the VR based on the tag dictionary used for parsing.
    fn read_vr(&mut self, tag: u32, ts: TSRef) -> Result<(VRRef, TSRef), Error> {
        if ts.explicit_vr {
            let mut buf: [u8; 2] = [0; 2];
            self.stream.read_exact(&mut buf)?;
            self.bytes_read += 2;
            let first_char: u8 = buf[0];
            let second_char: u8 = buf[1];

            let code: u16 = (u16::from(first_char) << 8) + u16::from(second_char);
            let mut vr: VRRef = VR::from_code(code).unwrap_or(&vr::UN);

            if vr.has_explicit_2byte_pad {
                self.stream.read_exact(&mut buf)?;
                self.bytes_read += 2;
            }

            // Part 5 Section 6.2.2 Note 2
            // If at some point an application knows the actual VR for an Attribute of VR UN
            // (e.g., has its own applicable data dictionary), it can assume that the Value Field of
            // the Attribute is encoded in Little Endian byte ordering with implicit VR encoding,
            // irrespective of the current Transfer Syntax.
            let mut ts: TSRef = ts;
            if vr == &vr::UN {
                vr = self.lookup_vr(tag)?;
                if vr != &vr::UN {
                    ts = &ts::ImplicitVRLittleEndian;
                }
            }

            Ok((vr, ts))
        } else {
            Ok((self.lookup_vr(tag)?, ts))
        }
    }

    /// Looks up the VR of the given tag in the current lookup dictionary, or `UN` if not present
    fn lookup_vr(&self, tag: u32) -> Result<VRRef, Error> {
        Ok(self
            .tag_by_value
            .and_then(|map| {
                map.get(&tag)
                    .and_then(|read_tag: &&Tag| read_tag.implicit_vr)
            })
            .unwrap_or(&vr::UN))
    }

    /// Reads a Value Length attribute from the stream using the given transfer syntax.
    fn read_value_length(&mut self, vr: VRRef, ts: TSRef) -> Result<ValueLength, Error> {
        let value_length: u32 = if !ts.explicit_vr || vr.has_explicit_2byte_pad {
            let mut buf: [u8; 4] = [0; 4];
            self.stream.read_exact(&mut buf)?;
            self.bytes_read += 4;

            if ts.is_big_endian() {
                u32::from_be_bytes(buf)
            } else {
                u32::from_le_bytes(buf)
            }
        } else {
            let mut buf: [u8; 2] = [0; 2];
            self.stream.read_exact(&mut buf)?;
            self.bytes_read += 2;

            if ts.is_big_endian() {
                u32::from(u16::from_be_bytes(buf))
            } else {
                u32::from(u16::from_le_bytes(buf))
            }
        };
        Ok(vl::from_value_length(value_length))
    }

    /// Reads the value field of the dicom element into a byte array. If the given `ValueLength` is
    /// undefined then this returns an empty array as elements with undefined length should have its
    /// contents parsed as dicom elements.
    fn read_value_field(&mut self, vl: ValueLength) -> Result<Vec<u8>, Error> {
        match vl {
            // undefined length means that the contents of the element are other
            // dicom elements to be parsed, so don't read data from the stream.
            ValueLength::Explicit(0) | ValueLength::UndefinedLength => Ok(Vec::new()),
            ValueLength::Explicit(value_length) => {
                let mut bytes: Vec<u8> = vec![0; value_length as usize];
                self.stream.read_exact(bytes.as_mut_slice())?;
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
            .ts_by_uid
            .and_then(|map| map.get::<str>(ts_uid.as_ref()))
            .cloned()
            .or_else(|| lookup::get_ts_by_uid(ts_uid.as_ref()))
            // The default encoding
            .unwrap_or(&ts::ImplicitVRLittleEndian);

        Ok(())
    }

    /// Parses the value of the given element as the specific character set and sets the `cs` value
    /// on this iterator to affect the parsing of further text-type element values.
    fn parse_specific_character_set(&mut self, element: &DicomElement) -> Result<(), Error> {
        let new_cs: Option<String> = element
            .parse_strings()?
            .into_iter()
            .find(|cs_entry: &String| !cs_entry.is_empty());

        // TODO: There are options for what to do if we can't support the character repertoire
        // See note on Ch 5 Part 6.1.2.3 under "Considerations on the Handling of Unsupported Character Sets"

        match new_cs {
            Some(cs) => {
                self.cs = charset::lookup_charset(&cs).unwrap_or(charset::DEFAULT_CHARACTER_SET);
            }
            None => {
                self.cs = charset::DEFAULT_CHARACTER_SET;
            }
        }
        Ok(())
    }

    /// Performs the primary iteration for the parser but the return type is consistent for error
    /// handling and not iteration. This should be called once for each invocation of `next()`.
    fn iterate(&mut self) -> Result<Option<DicomElement>, Error> {
        // The earlier parse states will read non-elements from the stream and move to another
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
                    None => return Ok(None),
                    Some(element) => return Ok(Some(element)),
                },
                ParseState::FileMeta => match self.iterate_file_meta()? {
                    None => return Ok(None),
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

    /// Performs the `ParserState::DetectState` iteration
    fn iterate_detect_state(&mut self) -> Result<(), Error> {
        // Read in enough bytes to determine if we're reading valid dicom.
        // Right now this is determined by reading a tag using the default transfer
        // syntax and verifying we know it's VR in the VR lookup table.
        let mut buf: [u8; 4] = [0; 4];
        self.stream.read_exact(&mut buf)?;
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);

        let tag: u32 = self.read_tag_from_stream(&mut cursor, &ts::ImplicitVRLittleEndian)?;
        // quick check for common case of being zeroed-out data
        if tag == 0 {
            self.finalize_preamble(&buf)?;
            return Ok(());
        }

        // quick check if we're reading beginning of file meta, continue from there
        if tag == tags::FILE_META_INFORMATION_GROUP_LENGTH {
            self.partial_tag = Some(tag);
            self.state = ParseState::GroupLength;
            return Ok(());
        }

        let vr: VRRef = self.lookup_vr(tag)?;

        // unknown tag and not a group-length tag then assume it's not dicom encoded
        if vr == &vr::UN && tag.trailing_zeros() < 16 {
            self.finalize_preamble(&buf)?;
            return Ok(());
        }

        // known tag that's not file meta, use DICOM default transfer syntax
        self.partial_tag = Some(tag);
        self.ts = &ts::ImplicitVRLittleEndian;
        self.state = ParseState::Element;
        Ok(())
    }

    /// Used when detecting if the stream has a file preamble. In the event the parser detects that
    /// there is a preamble then this takes the currently parsed bytes as input, copies them into
    /// the preamble field and reads in the remainder of the preamble from the stream.
    fn finalize_preamble(&mut self, buf: &[u8]) -> Result<(), Error> {
        self.file_preamble[..buf.len()].copy_from_slice(&buf);
        let file_preamble_len: usize = self.file_preamble.len();
        self.stream
            .read_exact(&mut self.file_preamble[buf.len()..file_preamble_len])?;
        self.bytes_read += self.file_preamble.len() as u64;
        self.state = ParseState::Prefix;
        Ok(())
    }

    /// Performs the `ParserState::Preamble` iteration
    fn iterate_preamble(&mut self) -> Result<(), Error> {
        self.stream.read_exact(&mut self.file_preamble)?;
        self.bytes_read += self.file_preamble.len() as u64;
        self.state = ParseState::Prefix;
        Ok(())
    }

    /// Performs the `ParserState::Prefix` iteration
    fn iterate_prefix(&mut self) -> Result<(), Error> {
        self.stream.read_exact(&mut self.dicom_prefix)?;
        self.bytes_read += self.dicom_prefix.len() as u64;
        for (n, prefix_item) in DICOM_PREFIX.iter().enumerate() {
            if self.dicom_prefix[n] != *prefix_item {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid DICOM Prefix: {:?}", self.dicom_prefix),
                ));
            }
        }
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
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Expected FileMetaInformationGroupLength but read: {:?}",
                    Tag::format_tag_to_display(tag)
                ),
            ));
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

        let element: DicomElement = self.read_dicom_element(tag, &ts::ExplicitVRLittleEndian)?;
        if element.tag == tags::TRANSFER_SYNTAX_UID {
            self.parse_transfer_syntax(&element)?;
        }

        if self.bytes_read >= self.fmi_start + u64::from(self.fmi_grouplength) {
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

impl<StreamType: Read> Iterator for Parser<StreamType> {
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
