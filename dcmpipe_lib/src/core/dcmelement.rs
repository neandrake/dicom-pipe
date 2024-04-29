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
/// adds a new DicomSequenceElement to its current path which subsequent elements will clone for
/// themselves. This allows elements to know how they exist within a dicom object.
#[derive(Clone, PartialEq, Eq)]
pub struct DicomSequenceElement {
    /// The SQ element tag.
    seq_tag: u32,
    /// The byte position where the parent sequence ends. This value is set as
    /// `bytes_read + value_length` during parsing. If the sequence has undefined length this is set
    /// to None.
    seq_end_pos: Option<u64>,
    /// The item number within the sequence. This is initialized/incremented whenever an Item tag is
    /// parsed. Elements within a sequence are not required to be contained within an Item element
    /// so this is not an element index into the sequence. This is an option which will be `None`
    /// between reading the sequence element and reading the first item element, and is then
    /// incremented whenever an Item tag is read.
    item_number: Option<u32>,
}

impl DicomSequenceElement {
    pub fn new(seq_tag: u32, seq_end_pos: Option<u64>) -> DicomSequenceElement {
        DicomSequenceElement {
            seq_tag,
            seq_end_pos,
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

    pub fn increment_item_number(&mut self) {
        match self.item_number {
            None => self.item_number.replace(0),
            Some(val) => self.item_number.replace(val + 1),
        };
    }
}

/// Represents a DICOM Element including its Tag, VR, and Value
/// Provides methods for parsing the element value as different native types
#[derive(Clone)]
pub struct DicomElement {
    pub tag: u32,
    pub vr: VRRef,
    pub vl: ValueLength,

    data: Cursor<Vec<u8>>,
    sequence_path: Vec<DicomSequenceElement>,

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
        sequence_path: Vec<DicomSequenceElement>,
    ) -> DicomElement {
        let cs: CSRef = vr.get_proper_cs(cs);
        DicomElement {
            tag,
            vr,
            vl,
            ts,
            cs,

            data: Cursor::new(data),
            sequence_path,
        }
    }

    pub fn get_data(&self) -> &Cursor<Vec<u8>> {
        &self.data
    }

    pub fn get_sequence_path(&self) -> &Vec<DicomSequenceElement> {
        &self.sequence_path
    }

    pub fn get_tag_path(&self) -> TagPath {
        let mut path: Vec<TagPathElement> = self
            .sequence_path
            .iter()
            .map(|dse: &DicomSequenceElement| TagPathElement::new(dse.get_seq_tag(), None))
            .collect::<Vec<TagPathElement>>();
        path.push(TagPathElement::new(self.tag, None));
        TagPath::new_from_vec(path)
    }

    pub fn is_seq(&self) -> bool {
        self.vr == &vr::SQ
    }

    /// Resets the internal byte stream to be parsed from the beginning
    fn reset_data_read(&mut self) {
        self.data.set_position(0);
    }

    /// Returns whether the the size of the value for this element is zero
    pub fn is_empty(&self) -> bool {
        self.data.get_ref().len() == 0
    }

    /// Parses the value of this element as a string using the given encoding
    /// Associated VRs:
    /// All character string AE's -- subsequent interpretation of String is necessary based on VR
    /// AE, AS, CS, DA, DS, DT, IS, LO, LT, PN, SH, ST, TM, UC, UI, UR, UT
    pub fn parse_string(&mut self) -> Result<String, Error> {
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
    pub fn parse_strings(&mut self) -> Result<Vec<String>, Error> {
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
    fn get_string_bytes_without_padding(&mut self) -> &[u8] {
        self.reset_data_read();

        // grab the position to start reading bytes from prior to computing the new bytes_read
        let mut lindex: usize = 0;

        let data: &[u8] = self.get_data().get_ref();
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
    pub fn parse_attribute(&mut self) -> Result<u32, Error> {
        self.reset_data_read();
        let first: u32 = if self.ts.is_big_endian() {
            u32::from(self.data.read_u16::<BigEndian>()?) << 16
        } else {
            u32::from(self.data.read_u16::<LittleEndian>()?) << 16
        };
        let second: u32 = if self.ts.is_big_endian() {
            u32::from(self.data.read_u16::<BigEndian>()?)
        } else {
            u32::from(self.data.read_u16::<LittleEndian>()?)
        };
        let result: u32 = first + second;
        Ok(result)
    }

    /// Parses the value for this element as a 32bit floating point
    /// Associated VRs: FL
    pub fn parse_f32(&mut self) -> Result<f32, Error> {
        self.reset_data_read();
        let result: f32 = if self.ts.is_big_endian() {
            self.data.read_f32::<BigEndian>()?
        } else {
            self.data.read_f32::<LittleEndian>()?
        };
        Ok(result)
    }

    /// Parses the value for this element as a list of 32bit floating point values
    /// Associated VRs: OF
    pub fn parse_f32s(&mut self) -> Result<Vec<f32>, Error> {
        self.reset_data_read();
        let num_bytes: usize = self.data.get_ref().len();
        let num_floats: usize = num_bytes / 4;
        let mut result: Vec<f32> = Vec::with_capacity(num_floats);
        for _ in 0..num_floats {
            let val: f32 = if self.ts.is_big_endian() {
                self.data.read_f32::<BigEndian>()?
            } else {
                self.data.read_f32::<LittleEndian>()?
            };
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as a 64bit floating point
    /// Associated VRs: FD
    pub fn parse_f64(&mut self) -> Result<f64, Error> {
        self.reset_data_read();
        let result: f64 = if self.ts.is_big_endian() {
            self.data.read_f64::<BigEndian>()?
        } else {
            self.data.read_f64::<LittleEndian>()?
        };
        Ok(result)
    }

    /// Parses the value for this element as a list of 64bit floating point values
    /// Associated VRs: OD
    pub fn parse_f64s(&mut self) -> Result<Vec<f64>, Error> {
        self.reset_data_read();
        let num_bytes: usize = self.data.get_ref().len();
        let num_doubles: usize = num_bytes / 8;
        let mut result: Vec<f64> = Vec::with_capacity(num_doubles);
        for _ in 0..num_doubles {
            let val: f64 = if self.ts.is_big_endian() {
                self.data.read_f64::<BigEndian>()?
            } else {
                self.data.read_f64::<LittleEndian>()?
            };
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as a signed 16bit integer
    /// Associated VRs: SS
    pub fn parse_i16(&mut self) -> Result<i16, Error> {
        self.reset_data_read();
        // TODO: Verify that we're parsing as 2s complement (not sure Endian should be considered?)
        let result: i16 = if self.ts.is_big_endian() {
            self.data.read_i16::<BigEndian>()?
        } else {
            self.data.read_i16::<LittleEndian>()?
        };
        Ok(result)
    }

    /// Parses the value for this element as a list of signed 16bit integer values
    /// Associated VRs: OW
    pub fn parse_i16s(&mut self) -> Result<Vec<i16>, Error> {
        self.reset_data_read();
        let num_bytes: usize = self.data.get_ref().len();
        let num_words: usize = num_bytes / 4;
        let mut result: Vec<i16> = Vec::with_capacity(num_words);
        for _ in 0..num_words {
            let val: i16 = if self.ts.is_big_endian() {
                self.data.read_i16::<BigEndian>()?
            } else {
                self.data.read_i16::<LittleEndian>()?
            };
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as a signed 32bit integer
    /// Associated VRs: SL
    pub fn parse_i32(&mut self) -> Result<i32, Error> {
        self.reset_data_read();
        // TODO: Verify that we're parsing as 2s complement (not sure Endian should be considered?)
        let result: i32 = if self.ts.is_big_endian() {
            self.data.read_i32::<BigEndian>()?
        } else {
            self.data.read_i32::<LittleEndian>()?
        };
        Ok(result)
    }

    /// Parses the value for this element as a list of signed 32bit integer values
    /// Associated VRs: OL
    pub fn parse_i32s(&mut self) -> Result<Vec<i32>, Error> {
        self.reset_data_read();
        let num_bytes: usize = self.data.get_ref().len();
        let num_longs: usize = num_bytes / 4;
        let mut result: Vec<i32> = Vec::with_capacity(num_longs);
        for _ in 0..num_longs {
            let val: i32 = if self.ts.is_big_endian() {
                self.data.read_i32::<BigEndian>()?
            } else {
                self.data.read_i32::<LittleEndian>()?
            };
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as an unsigned 32bit integer
    /// Associated VRs: UL
    pub fn parse_u32(&mut self) -> Result<u32, Error> {
        self.reset_data_read();
        let result: u32 = if self.ts.is_big_endian() {
            self.data.read_u32::<BigEndian>()?
        } else {
            self.data.read_u32::<LittleEndian>()?
        };
        Ok(result)
    }

    /// Parses the value for this element as an unsigned 16bit integer
    /// Associated VRs: US
    pub fn parse_u16(&mut self) -> Result<u16, Error> {
        self.reset_data_read();
        let result: u16 = if self.ts.is_big_endian() {
            self.data.read_u16::<BigEndian>()?
        } else {
            self.data.read_u16::<LittleEndian>()?
        };
        Ok(result)
    }
}
