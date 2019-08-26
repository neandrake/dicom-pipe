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

/// The different parsing behaviors of the stream
pub enum ParseState {
    /// An initial state in which we're trying to detect if there is a preamble at all
    DetectState,
    /// The File Preamble. May only be present on file media and possibly not present over network
    Preamble,
    /// The DICOM prefix. May be required for all media transfer.
    Prefix,
    /// The first official dicom element to be parsed as `ExplicitVRLittleEndian`. The value of
    /// this first element is the remaining bytes of the File Meta group.
    GroupLength,
    /// These are elements that are always parsed using `ExplicitVRLittleEndian`
    FileMeta,
    /// These are the main dicom elements being parsed. They are parsed using the transfer
    /// syntax specified in the File Meta group
    Element,
}

pub struct ParserBuilder<StreamType: Read> {
    stream: StreamType,
    state: Option<ParseState>,
    tagstop: Option<TagStop>,
    tag_by_value: Option<TagByValueLookup>,
    ts_by_uid: Option<TsByUidLookup>,
}

impl<StreamType: Read> ParserBuilder<StreamType> {
    pub fn new(stream: StreamType) -> ParserBuilder<StreamType> {
        ParserBuilder {
            stream,
            state: None,
            tagstop: None,
            tag_by_value: None,
            ts_by_uid: None,
        }
    }

    pub fn tagstop(mut self, tagstop: TagStop) -> Self {
        self.tagstop = Some(tagstop);
        self
    }

    pub fn tag_by_value(mut self, tag_by_value: TagByValueLookup) -> Self {
        self.tag_by_value = Some(tag_by_value);
        self
    }

    pub fn ts_by_uid(mut self, ts_by_uid: TsByUidLookup) -> Self {
        self.ts_by_uid = Some(ts_by_uid);
        self
    }

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
        }
    }
}

/// Provides an iterator that parses through a dicom stream returning top-level elements
pub struct Parser<StreamType: Read> {
    /// The stream to parse dicom from.
    stream: StreamType,

    /// The current state of reading items from the stream, which represents the different types
    /// of items that can be parsed from the stream.
    state: ParseState,

    /// The condition under which this iterator should stop parsing elements from the stream.
    /// This allows for partially parsing through the stream instead of having to read the entire
    /// thing.
    tagstop: TagStop,

    /// Lookup map for identifying tags by their tag number. Needed for resolving implicit VRs.
    tag_by_value: Option<TagByValueLookup>,

    /// Lookup map for identifying transfer sytnax by their UID.
    ts_by_uid: Option<TsByUidLookup>,

    /// Tracks the number of bytes read from the stream. Since we don't require that the stream
    /// implement `Seek` we count bytes read from the stream in order to check relative positioning
    /// which currently is used for determining whether we're still parsing File Meta elements vs.
    /// switch to parsing regular dicom elements. This is also used for tracking when sequences
    /// of explicit length begin/end.
    bytes_read: u64,

    /// The file preamble read from the stream. This may only be present when parsing from files
    /// and may need to be skipped when reading from network or elsewhere.
    file_preamble: [u8; FILE_PREAMBLE_LENGTH],

    /// The prefix of the stream. Not yet clear if this is always expected to be present depending
    /// on which media format (file, network, etc.) the dicom object is being read from.
    dicom_prefix: [u8; DICOM_PREFIX_LENGTH],

    /// The number of bytes read just after having read the `FileMetaInformationGroupLength`. This
    /// is used to determine how many bytes to continue parsing until we switch to reading regular
    /// DICOM elements, by checking `bytes_read` against `fmi_start + fmi_grouplength`.
    fmi_start: u64,

    /// The value of the FileMetaInformationGroupLength tag, which is the number of bytes remaining
    /// in the File Meta Information section until the non-meta section of the DICOM stream starts.
    /// Only after the File Meta Information section does the transfer syntax and character encoding
    /// take effect.
    fmi_grouplength: u32,

    /// This is the last element tag successfully read from the stream, regardless of whether
    /// the element it's for successfully finished parsing.
    tag_last_read: u32,

    /// This field represents an element tag being successfully read from the stream however the
    /// remainder of the element did not finish parsing, either due to `TagStop` or errors reading
    /// the stream. It is set after successfully parsing an element tag and unset when the reaminder
    /// of the element is successfully parsed and returned by the iterator.
    partial_tag: Option<u32>,

    /// The transfer syntax used for this stream. This defaults to `ExplicitVRLittleEndian` which is
    /// the transfer syntax used for parsing File Meta section. This default is not relied upon being
    /// set however as the iteration hardcodes the explicitness and endianness for those elements.
    /// This will only be set after having successully parsed the transfer sytnax element.
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
}

impl<StreamType: Read> Parser<StreamType> {
    pub fn bytes_read(&self) -> u64 {
        self.bytes_read
    }

    pub fn partial_tag(&self) -> Option<u32> {
        self.partial_tag
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

    /// This needs to be checked multiple times during parsing of element
    /// 1. Just before reading the next element will catch `TagStop::AfterTag`
    /// and `TagStop::AfterBytePos`
    /// 2. Just after reading the tag for the element about to be parsed
    /// will catch `TagStop::BeforeTag` as well as `TagStop::AfterBytePos`
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

    /// Reads the remainder of the dicom element from the stream.
    /// This assumes that just prior to calling this `self.read_tag()` was called
    /// and the result is passed as parameter here.
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
        let mut vr: VRRef = vr_ts.0;
        // If VR is explicitly UN but we can tell it's SQ then the inner elements are encoded as
        // IVRLE -- but only the contents should be parsed as such, do not switch transfer syntax
        // prior to reading in the value length.
        let vl: ValueLength = self.read_value_length(vr, ts)?;

        // If the VR is not SQ and ValueLength is UndefinedLength then this should be interpreted as
        // a private-tag SQ element.
        let ts: TSRef = if tag != tags::ITEM
            && (vr == &vr::UN || vr == &vr::OB || vr == &vr::OW)
            && vl == ValueLength::UndefinedLength
        {
            vr = &vr::SQ;
            &ts::ImplicitVRLittleEndian
        } else {
            vr_ts.1
        };

        let bytes: Vec<u8> = if vr == &vr::SQ {
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

    /// Reads a VR attribute from the stream. If `force_explicit` is false then
    /// the transfer syntax specified from the stream is used to determine if the VR
    /// should be read explicitly or implicitly determined from the dataset dictionary.
    /// If `force_explicit` is true then the VR is explicitly read from the stream.
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
        self.tag_by_value
            .and_then(|map| {
                map.get(&tag)
                    .and_then(|read_tag: &&Tag| read_tag.implicit_vr)
            })
            .or_else(|| Some(&vr::UN))
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("ImplicitVR TS but VR is unknown for tag: {}", tag),
                )
            })
    }

    /// Reads a Value Length attribute from the stream. If `force_explicit` is false then
    /// the transfer syntax specified from the stream is used to determine how to read the attribute
    /// otherwise it forces reading as explicit VR definition.
    fn read_value_length(&mut self, vr: VRRef, ts: TSRef) -> Result<ValueLength, Error> {
        let value_length: u32;
        if !ts.explicit_vr || vr.has_explicit_2byte_pad {
            let mut buf: [u8; 4] = [0; 4];
            self.stream.read_exact(&mut buf)?;
            self.bytes_read += 4;

            value_length = if ts.is_big_endian() {
                u32::from_be_bytes(buf)
            } else {
                u32::from_le_bytes(buf)
            };
        } else {
            let mut buf: [u8; 2] = [0; 2];
            self.stream.read_exact(&mut buf)?;
            self.bytes_read += 2;

            value_length = if ts.is_big_endian() {
                u32::from(u16::from_be_bytes(buf))
            } else {
                u32::from(u16::from_le_bytes(buf))
            };
        }
        Ok(vl::from_value_length(value_length))
    }

    /// Reads the value field of the dicom element into a byte array
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
    /// iterator to affect the reading of further dicom elements.
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

    fn finalize_preamble(&mut self, buf: &[u8]) -> Result<(), Error> {
        self.file_preamble[..buf.len()].copy_from_slice(&buf);
        let file_preamble_len: usize = self.file_preamble.len();
        self.stream
            .read_exact(&mut self.file_preamble[buf.len()..file_preamble_len])?;
        self.bytes_read += self.file_preamble.len() as u64;
        self.state = ParseState::Prefix;
        Ok(())
    }
}

type DicomElementResult = Result<DicomElement, Error>;

impl<StreamType: Read> Iterator for Parser<StreamType> {
    type Item = DicomElementResult;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        loop {
            let at_tagstop: Result<bool, Error> = self.is_at_tag_stop();
            if let Ok(true) = at_tagstop {
                return None;
            } else if let Err(e) = at_tagstop {
                return Some(Err(e));
            }

            match self.state {
                ParseState::DetectState => {
                    // Read in enough bytes to determine if we're reading valid dicom.
                    // Right now this is determined by reading a tag using the default transfer
                    // syntax and verifying we know it's VR in the VR lookup table.
                    let mut buf: [u8; 4] = [0; 4];
                    if let Err(e) = self.stream.read_exact(&mut buf) {
                        return Some(Err(e));
                    }
                    let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);

                    let tag_result: Result<u32, Error> =
                        self.read_tag_from_stream(&mut cursor, &ts::ImplicitVRLittleEndian);
                    if let Err(e) = tag_result {
                        return Some(Err(e));
                    }
                    let tag: u32 = tag_result.unwrap();
                    // quick check for common case of being zeroed-out data
                    if tag == 0 {
                        if let Err(e) = self.finalize_preamble(&buf) {
                            return Some(Err(e));
                        }
                        continue;
                    }

                    // quick check if we're reading beginning of file meta, continue from there
                    if tag == tags::FILE_META_INFORMATION_GROUP_LENGTH {
                        self.partial_tag = Some(tag);
                        self.state = ParseState::GroupLength;
                        continue;
                    }

                    let vr_result: Result<VRRef, Error> = self.lookup_vr(tag);
                    if let Err(e) = vr_result {
                        return Some(Err(e));
                    }
                    let vr: VRRef = vr_result.unwrap();

                    // unknown tag and not a group-length tag then assume it's not dicom encoded
                    if vr == &vr::UN && tag.trailing_zeros() < 16 {
                        if let Err(e) = self.finalize_preamble(&buf) {
                            return Some(Err(e));
                        }
                        continue;
                    }

                    // known tag that's not file meta, use default transfer syntax
                    self.partial_tag = Some(tag);
                    self.ts = &ts::ImplicitVRLittleEndian;
                    self.state = ParseState::Element;
                }
                ParseState::Preamble => {
                    let result: Result<(), Error> = self.stream.read_exact(&mut self.file_preamble);
                    if let Err(e) = result {
                        return Some(Err(e));
                    }
                    self.bytes_read += self.file_preamble.len() as u64;
                    self.state = ParseState::Prefix;
                }
                ParseState::Prefix => {
                    let result: Result<(), Error> = self.stream.read_exact(&mut self.dicom_prefix);
                    if let Err(e) = result {
                        return Some(Err(e));
                    }
                    self.bytes_read += self.dicom_prefix.len() as u64;

                    for (n, prefix_item) in DICOM_PREFIX.iter().enumerate() {
                        if self.dicom_prefix[n] != *prefix_item {
                            return Some(Err(Error::new(
                                ErrorKind::InvalidData,
                                format!("Invalid DICOM Prefix: {:?}", self.dicom_prefix),
                            )));
                        }
                    }

                    self.state = ParseState::GroupLength;
                }
                ParseState::GroupLength => {
                    let tag: u32 = if let Some(partial_tag) = self.partial_tag {
                        partial_tag
                    } else {
                        let tag: Result<u32, Error> = self.read_tag(&ts::ExplicitVRLittleEndian);
                        if let Err(e) = tag {
                            return Some(Err(e));
                        }
                        let tag: u32 = tag.unwrap();
                        self.partial_tag.replace(tag);
                        tag
                    };
                    self.tag_last_read = tag;

                    let at_tagstop: Result<bool, Error> = self.is_at_tag_stop();
                    if let Ok(true) = at_tagstop {
                        return None;
                    } else if let Err(e) = at_tagstop {
                        return Some(Err(e));
                    }

                    if tag != tags::FILE_META_INFORMATION_GROUP_LENGTH {
                        return Some(Err(Error::new(
                            ErrorKind::InvalidData,
                            format!(
                                "Expected FileMetaInformationGroupLength but read: {:?}",
                                Tag::format_tag_to_display(tag)
                            ),
                        )));
                    }

                    let grouplength: DicomElementResult =
                        self.read_dicom_element(tag, &ts::ExplicitVRLittleEndian);
                    if grouplength.is_err() {
                        return Some(grouplength);
                    }

                    let grouplength: DicomElement = grouplength.unwrap();
                    let grouplength_val: Result<u32, Error> = grouplength.parse_u32();
                    if let Err(e) = grouplength_val {
                        return Some(Err(e));
                    }

                    self.fmi_grouplength = grouplength_val.unwrap();
                    self.fmi_start = self.bytes_read;
                    self.state = ParseState::FileMeta;
                    // reset partial_tag to None
                    self.partial_tag.take();

                    return Some(Ok(grouplength));
                }
                ParseState::FileMeta => {
                    let tag: u32 = if let Some(partial_tag) = self.partial_tag {
                        partial_tag
                    } else {
                        let tag: Result<u32, Error> = self.read_tag(&ts::ExplicitVRLittleEndian);
                        if let Err(e) = tag {
                            return Some(Err(e));
                        }
                        let tag: u32 = tag.unwrap();
                        self.partial_tag.replace(tag);
                        tag
                    };
                    self.tag_last_read = tag;

                    let at_tagstop: Result<bool, Error> = self.is_at_tag_stop();
                    if let Ok(true) = at_tagstop {
                        return None;
                    } else if let Err(e) = at_tagstop {
                        return Some(Err(e));
                    }

                    let element: DicomElementResult =
                        self.read_dicom_element(tag, &ts::ExplicitVRLittleEndian);
                    if element.is_err() {
                        return Some(element);
                    }

                    let element: DicomElement = element.unwrap();
                    if element.tag == tags::TRANSFER_SYNTAX_UID {
                        let result: Result<(), Error> = self.parse_transfer_syntax(&element);
                        if let Err(e) = result {
                            return Some(Err(e));
                        }
                    }

                    if self.bytes_read >= self.fmi_start + u64::from(self.fmi_grouplength) {
                        self.state = ParseState::Element;
                    }

                    // reset partial_tag to None
                    self.partial_tag.take();

                    return Some(Ok(element));
                }
                ParseState::Element => {
                    // if we're in a sequence we need to use the sequence's transfer syntax
                    let ts: TSRef = self
                        .current_path
                        .last()
                        .map(SequenceElement::get_ts)
                        .unwrap_or(self.ts);

                    let tag: u32 = if let Some(partial_tag) = self.partial_tag {
                        partial_tag
                    } else {
                        let tag: Result<u32, Error> = self.read_tag(ts);
                        if let Err(e) = tag {
                            // only check EOF when reading beginning of elements as it would actually
                            // be expected in this scenario since the DICOM format provides no determination
                            // for end of the dicom object
                            if e.kind() == ErrorKind::UnexpectedEof {
                                return None;
                            }
                            return Some(Err(e));
                        }
                        let tag: u32 = tag.unwrap();
                        self.partial_tag.replace(tag);
                        tag
                    };
                    self.tag_last_read = tag;

                    let at_tagstop: Result<bool, Error> = self.is_at_tag_stop();
                    if let Ok(true) = at_tagstop {
                        return None;
                    } else if let Err(e) = at_tagstop {
                        return Some(Err(e));
                    }

                    // reading element clones the current path so update prior to reading element
                    if tag == tags::ITEM {
                        // get the sequence this item is for and increment its item number
                        if let Some(seq_elem) = self.current_path.last_mut() {
                            seq_elem.increment_item_number();
                        }
                    }

                    let element: DicomElementResult = self.read_dicom_element(tag, ts);
                    if element.is_err() {
                        return Some(element);
                    }

                    let element: DicomElement = element.unwrap();
                    if element.tag == tags::SPECIFIC_CHARACTER_SET {
                        let result: Result<(), Error> = self.parse_specific_character_set(&element);
                        if let Err(e) = result {
                            return Some(Err(e));
                        }
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

                    if element.is_seq() {
                        let seq_end_pos: Option<u64> =
                            if let ValueLength::Explicit(len) = element.vl {
                                Some(self.bytes_read + u64::from(len))
                            } else {
                                None
                            };
                        self.current_path.push(SequenceElement::new(
                            tag,
                            seq_end_pos,
                            element.get_ts(),
                        ));
                    }

                    return Some(Ok(element));
                }
            }
        }
    }
}
