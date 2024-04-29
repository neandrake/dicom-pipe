use crate::core::charset::CSRef;
use crate::core::tagpath::{TagPath, TagPathElement};
use crate::defn::ts::TSRef;
use crate::defn::vl::ValueLength;
use crate::defn::vr::{self, VRRef, CHARACTER_STRING_SEPARATOR};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use encoding::types::DecoderTrap;
use std::borrow::Cow;
use std::io::{Cursor, Error, ErrorKind};

/// Represents the sequence/item position of an element.
/// For elements to track which sequence they are a part of. When an SQ element is parsed the parser
/// adds a new `SequenceElement` to its current path which subsequent elements will clone for
/// themselves. This allows elements to know how they exist within a dicom object.
#[derive(Clone, PartialEq, Eq)]
pub struct SequenceElement {
    /// The SQ element tag.
    seq_tag: u32,
    /// The byte position where the parent sequence ends. This value is set as
    /// `bytes_read + value_length` during parsing. If the sequence has undefined length this is set
    /// to None.
    seq_end_pos: Option<u64>,
    /// See Part 5 Section 6.2.2 Note 2
    /// If a sequence is encoded with explicit VR but data dictionary defines it as SQ then we
    /// should interpret the contents of the sequence as ImplicitVRLittleEndian. SQ elements need to
    /// track what transfer syntax their contents are encoded with.
    ts: TSRef,
    /// See Part 5 Section 7.5
    /// Items present in an SQ Data Element shall be an ordered set where each Item may be
    /// referenced by its ordinal position. Each Item shall be implicitly assigned an ordinal
    /// position starting with the value 1 for the first Item in the Sequence, and incremented by 1
    /// with each subsequent Item. The last Item in the Sequence shall have an ordinal position
    /// equal to the number of Items in the Sequence.
    ///
    /// This is initialized/incremented whenever an Item tag is parsed. Sequences are not required
    /// to have their contents encoded within items so this cannot be used as an index into a
    /// sequence's total listing of top-level children.
    item_number: Option<u32>,
}

impl SequenceElement {
    pub fn new(seq_tag: u32, seq_end_pos: Option<u64>, ts: TSRef) -> SequenceElement {
        SequenceElement {
            seq_tag,
            seq_end_pos,
            ts,
            item_number: None,
        }
    }

    pub fn get_seq_tag(&self) -> u32 {
        self.seq_tag
    }

    pub fn get_seq_end_pos(&self) -> Option<u64> {
        self.seq_end_pos
    }

    pub fn get_item_number(&self) -> Option<u32> {
        self.item_number
    }

    pub fn get_ts(&self) -> TSRef {
        self.ts
    }

    pub fn increment_item_number(&mut self) {
        match self.item_number {
            None => self.item_number.replace(1),
            Some(val) => self.item_number.replace(val + 1),
        };
    }

    pub fn decrement_item_num(&mut self) {
        match self.item_number {
            None => {}
            Some(val) => {
                if val > 1 {
                    self.item_number.replace(val - 1);
                } else {
                    self.item_number.take();
                }
            }
        }
    }
}

/// Represents a DICOM Element including its Tag, VR, and Value
/// Provides methods for parsing the element value as different native types
#[derive(Clone)]
pub struct DicomElement {
    pub tag: u32,
    pub vr: VRRef,
    pub vl: ValueLength,

    data: Vec<u8>,
    sequence_path: Vec<SequenceElement>,

    ts: TSRef,
    cs: CSRef,
}

impl DicomElement {
    pub fn new(
        tag: u32,
        vr: VRRef,
        vl: ValueLength,
        ts: TSRef,
        cs: CSRef,
        data: Vec<u8>,
        sequence_path: Vec<SequenceElement>,
    ) -> DicomElement {
        let cs: CSRef = vr.get_proper_cs(cs);
        DicomElement {
            tag,
            vr,
            vl,
            ts,
            cs,
            data,
            sequence_path,
        }
    }

    pub fn get_ts(&self) -> TSRef {
        self.ts
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_sequence_path(&self) -> &Vec<SequenceElement> {
        &self.sequence_path
    }

    pub fn get_tag_path(&self) -> TagPath {
        let mut path: Vec<TagPathElement> = self
            .sequence_path
            .iter()
            .map(|dse: &SequenceElement| TagPathElement::new(dse.get_seq_tag(), None))
            .collect::<Vec<TagPathElement>>();
        path.push(TagPathElement::new(self.tag, None));
        TagPath::new_from_vec(path)
    }

    pub fn is_seq(&self) -> bool {
        self.vr == &vr::SQ
    }

    /// Returns whether the the size of the value for this element is zero
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Parses the value of this element as a string using the given encoding
    /// Associated VRs:
    /// All character string AE's -- subsequent interpretation of String is necessary based on VR
    /// AE, AS, CS, DA, DS, DT, IS, LO, LT, PN, SH, ST, TM, UC, UI, UR, UT
    pub fn parse_string(&self) -> Result<String, Error> {
        let is_ui: bool = self.vr == &vr::UI;
        let is_char_str: bool = self.vr.is_character_string;
        let padding: u8 = self.vr.padding;
        let cs: CSRef = self.cs;

        let mut data: &[u8] = self.get_string_bytes_without_padding();

        if is_ui && data[data.len() - 1] == padding {
            data = &data[0..data.len() - 1];
        }

        cs.decode(data, DecoderTrap::Strict)
            .map(|v: String| {
                if !is_ui && is_char_str {
                    v.trim_end_matches(|c: char| c == padding as char)
                        .to_string()
                } else {
                    v
                }
            })
            .map_err(|e: Cow<'static, str>| Error::new(ErrorKind::InvalidData, e.into_owned()))
    }

    /// Parses the value of this element as a list of strings using the given encoding
    /// Associated VRs:
    /// All character string AE's -- subsequent interpretation of String is necessary based on VR
    /// AE, AS, CS, DA, DS, DT, IS, LO, LT, PN, SH, ST, TM, UC, UI, UR, UT
    pub fn parse_strings(&self) -> Result<Vec<String>, Error> {
        let is_ui: bool = self.vr == &vr::UI;
        let is_char_str: bool = self.vr.is_character_string;
        let padding: u8 = self.vr.padding;
        let cs: CSRef = self.cs;

        let mut data: &[u8] = self.get_string_bytes_without_padding();

        if is_ui && data[data.len() - 1] == padding {
            data = &data[0..data.len() - 1];
        }

        cs.decode(data, DecoderTrap::Strict)
            .map(|v: String| {
                if !is_ui && is_char_str {
                    v.trim_end_matches(|c: char| c == padding as char)
                        .to_string()
                } else {
                    v
                }
            })
            .map_err(|e: Cow<'static, str>| Error::new(ErrorKind::InvalidData, e.into_owned()))
            .map(|multivalue: String| {
                multivalue
                    .split(CHARACTER_STRING_SEPARATOR)
                    .map(str::to_owned)
                    .collect::<Vec<String>>()
            })
    }

    /// Returns the value as a slice with the padding character
    /// removed per the specification of whether the VR indicates leading/trailing
    /// padding is significant.
    fn get_string_bytes_without_padding(&self) -> &[u8] {
        // grab the position to start reading bytes from prior to computing the new bytes_read
        let mut lindex: usize = 0;

        let data: &[u8] = self.data.as_slice();
        if data.is_empty() {
            return data;
        }

        let mut rindex: usize = data.len() - 1;
        if self.vr.should_trim_trailing_space {
            while rindex > lindex {
                if data[rindex] == 0x20 {
                    rindex -= 1;
                } else {
                    break;
                }
            }
        }
        if self.vr.should_trim_leading_space {
            while lindex < rindex {
                if data[lindex] == 0x20 {
                    lindex += 1;
                } else {
                    break;
                }
            }
        }
        &data[lindex..=rindex]
    }

    /// Parses the value for this element as an attribute (aka a tag)
    /// Associated VRs: AT
    pub fn parse_attribute(&self) -> Result<u32, Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(self.data.as_slice());
        let first: u32 = if self.ts.is_big_endian() {
            u32::from(cursor.read_u16::<BigEndian>()?) << 16
        } else {
            u32::from(cursor.read_u16::<LittleEndian>()?) << 16
        };
        let second: u32 = if self.ts.is_big_endian() {
            u32::from(cursor.read_u16::<BigEndian>()?)
        } else {
            u32::from(cursor.read_u16::<LittleEndian>()?)
        };
        let result: u32 = first + second;
        Ok(result)
    }

    /// Parses the value for this element as a 32bit floating point
    /// Associated VRs: FL
    pub fn parse_f32(&self) -> Result<f32, Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(self.data.as_slice());
        let result: f32 = if self.ts.is_big_endian() {
            cursor.read_f32::<BigEndian>()?
        } else {
            cursor.read_f32::<LittleEndian>()?
        };
        Ok(result)
    }

    /// Parses the value for this element as a list of 32bit floating point values
    /// Associated VRs: OF
    pub fn parse_f32s(&self) -> Result<Vec<f32>, Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(self.data.as_slice());
        let num_bytes: usize = self.data.len();
        let num_floats: usize = num_bytes / 4;
        let mut result: Vec<f32> = Vec::with_capacity(num_floats);
        for _ in 0..num_floats {
            let val: f32 = if self.ts.is_big_endian() {
                cursor.read_f32::<BigEndian>()?
            } else {
                cursor.read_f32::<LittleEndian>()?
            };
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as a 64bit floating point
    /// Associated VRs: FD
    pub fn parse_f64(&self) -> Result<f64, Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(self.data.as_slice());
        let result: f64 = if self.ts.is_big_endian() {
            cursor.read_f64::<BigEndian>()?
        } else {
            cursor.read_f64::<LittleEndian>()?
        };
        Ok(result)
    }

    /// Parses the value for this element as a list of 64bit floating point values
    /// Associated VRs: OD
    pub fn parse_f64s(&self) -> Result<Vec<f64>, Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(self.data.as_slice());
        let num_bytes: usize = self.data.len();
        let num_doubles: usize = num_bytes / 8;
        let mut result: Vec<f64> = Vec::with_capacity(num_doubles);
        for _ in 0..num_doubles {
            let val: f64 = if self.ts.is_big_endian() {
                cursor.read_f64::<BigEndian>()?
            } else {
                cursor.read_f64::<LittleEndian>()?
            };
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as a signed 16bit integer
    /// Associated VRs: SS
    pub fn parse_i16(&self) -> Result<i16, Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(self.data.as_slice());
        // TODO: Verify that we're parsing as 2s complement (not sure Endian should be considered?)
        let result: i16 = if self.ts.is_big_endian() {
            cursor.read_i16::<BigEndian>()?
        } else {
            cursor.read_i16::<LittleEndian>()?
        };
        Ok(result)
    }

    /// Parses the value for this element as a list of signed 16bit integer values
    /// Associated VRs: OW
    pub fn parse_i16s(&self) -> Result<Vec<i16>, Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(self.data.as_slice());
        let num_bytes: usize = self.data.len();
        let num_words: usize = num_bytes / 4;
        let mut result: Vec<i16> = Vec::with_capacity(num_words);
        for _ in 0..num_words {
            let val: i16 = if self.ts.is_big_endian() {
                cursor.read_i16::<BigEndian>()?
            } else {
                cursor.read_i16::<LittleEndian>()?
            };
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as a signed 32bit integer
    /// Associated VRs: SL
    pub fn parse_i32(&self) -> Result<i32, Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(self.data.as_slice());
        // TODO: Verify that we're parsing as 2s complement (not sure Endian should be considered?)
        let result: i32 = if self.ts.is_big_endian() {
            cursor.read_i32::<BigEndian>()?
        } else {
            cursor.read_i32::<LittleEndian>()?
        };
        Ok(result)
    }

    /// Parses the value for this element as a list of signed 32bit integer values
    /// Associated VRs: OL
    pub fn parse_i32s(&self) -> Result<Vec<i32>, Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(self.data.as_slice());
        let num_bytes: usize = self.data.len();
        let num_longs: usize = num_bytes / 4;
        let mut result: Vec<i32> = Vec::with_capacity(num_longs);
        for _ in 0..num_longs {
            let val: i32 = if self.ts.is_big_endian() {
                cursor.read_i32::<BigEndian>()?
            } else {
                cursor.read_i32::<LittleEndian>()?
            };
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as an unsigned 32bit integer
    /// Associated VRs: UL
    pub fn parse_u32(&self) -> Result<u32, Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(self.data.as_slice());
        let result: u32 = if self.ts.is_big_endian() {
            cursor.read_u32::<BigEndian>()?
        } else {
            cursor.read_u32::<LittleEndian>()?
        };
        Ok(result)
    }

    /// Parses the value for this element as an unsigned 16bit integer
    /// Associated VRs: US
    pub fn parse_u16(&self) -> Result<u16, Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(self.data.as_slice());
        let result: u16 = if self.ts.is_big_endian() {
            cursor.read_u16::<BigEndian>()?
        } else {
            cursor.read_u16::<LittleEndian>()?
        };
        Ok(result)
    }
}
