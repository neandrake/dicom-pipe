use crate::core::charset::CSRef;
use crate::core::dict::lookup::TAG_BY_VALUE;
use crate::core::tag::Tag;
use crate::core::vl::ValueLength;
use crate::core::vr;
use crate::core::vr::{VRRef, CHARACTER_STRING_SEPARATOR};
use byteorder::{ByteOrder, ReadBytesExt};
use encoding::types::DecoderTrap;
use std::borrow::Cow;
use std::fmt;
use std::io::{Cursor, Error, ErrorKind};

#[derive(Clone)]
pub struct DicomSequencePosition {
    /// The tag of the sequence.
    seq_tag: u32,
    /// The byte position where this sequence ends. This value is set as `bytes_read + value_length`
    /// during parsing. If the sequence has undefined length this is set to None.
    seq_end_pos: Option<u64>,
    /// The item number within the sequence. Note that an item can contain multiple elements so
    /// this is not an element index into a sequence. This is an option which will be `None` between
    /// reading the sequence element and reading an item element, and is then incremented for each
    /// item read.
    item_number: Option<u32>,
}

impl DicomSequencePosition {
    pub fn new(seq_tag: u32, seq_end_pos: Option<u64>) -> DicomSequencePosition {
        DicomSequencePosition {
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
pub struct DicomElement {
    pub tag: u32,
    pub vr: VRRef,
    pub vl: ValueLength,

    data: Cursor<Vec<u8>>,
    sequence_path: Vec<DicomSequencePosition>,
}

/// A nice user-readable display of the element such as
/// (0002,0001) OB FileMetaInformationVersion
impl fmt::Debug for DicomElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tag_num: String = Tag::format_tag_to_display(self.tag);

        let tag_name: &str = if let Some(tag) = TAG_BY_VALUE.get(&self.tag) {
            &tag.ident
        } else {
            "<Private Tag>"
        };

        write!(f, "{} {} {}", tag_num, self.vr.ident, tag_name)
    }
}

impl DicomElement {
    pub fn new(
        tag: u32,
        vr: VRRef,
        vl: ValueLength,
        data: Vec<u8>,
        sequence_path: Vec<DicomSequencePosition>,
    ) -> DicomElement {
        DicomElement {
            tag,
            vr,
            vl,

            data: Cursor::new(data),
            sequence_path,
        }
    }

    pub fn get_data(&self) -> &Cursor<Vec<u8>> {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.data
    }

    pub fn get_sequence_path(&self) -> &Vec<DicomSequencePosition> {
        &self.sequence_path
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
    pub fn parse_string(&mut self, cs: CSRef) -> Result<String, Error> {
        let is_ui: bool = self.vr == &vr::UI;
        let is_char_str: bool = self.vr.is_character_string;
        let padding: u8 = self.vr.padding;

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
    pub fn parse_strings(&mut self, cs: CSRef) -> Result<Vec<String>, Error> {
        let is_ui: bool = self.vr == &vr::UI;
        let is_char_str: bool = self.vr.is_character_string;
        let padding: u8 = self.vr.padding;

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

        let mut rindex: usize = data.len() - 1;
        if self.vr.should_trim_trailing_space {
            while rindex >= lindex {
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
    pub fn parse_attribute<Endian: ByteOrder>(&mut self) -> Result<u32, Error> {
        self.reset_data_read();
        let first: u32 = u32::from(self.data.read_u16::<Endian>()?) << 16;
        let second: u32 = u32::from(self.data.read_u16::<Endian>()?);
        let result: u32 = first + second;
        Ok(result)
    }

    /// Parses the value for this element as a 32bit floating point
    /// Associated VRs: FL
    pub fn parse_f32<Endian: ByteOrder>(&mut self) -> Result<f32, Error> {
        self.reset_data_read();
        let result: f32 = self.data.read_f32::<Endian>()?;
        Ok(result)
    }

    /// Parses the value for this element as a list of 32bit floating point values
    /// Associated VRs: OF
    pub fn parse_f32s<Endian: ByteOrder>(&mut self) -> Result<Vec<f32>, Error> {
        self.reset_data_read();
        let num_bytes: usize = self.data.get_ref().len();
        let num_floats: usize = num_bytes / 4;
        let mut result: Vec<f32> = Vec::with_capacity(num_floats);
        for _ in 0..num_floats {
            let val: f32 = self.data.read_f32::<Endian>()?;
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as a 64bit floating point
    /// Associated VRs: FD
    pub fn parse_f64<Endian: ByteOrder>(&mut self) -> Result<f64, Error> {
        self.reset_data_read();
        let result: f64 = self.data.read_f64::<Endian>()?;
        Ok(result)
    }

    /// Parses the value for this element as a list of 64bit floating point values
    /// Associated VRs: OD
    pub fn parse_f64s<Endian: ByteOrder>(&mut self) -> Result<Vec<f64>, Error> {
        self.reset_data_read();
        let num_bytes: usize = self.data.get_ref().len();
        let num_doubles: usize = num_bytes / 8;
        let mut result: Vec<f64> = Vec::with_capacity(num_doubles);
        for _ in 0..num_doubles {
            let val: f64 = self.data.read_f64::<Endian>()?;
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as a signed 16bit integer
    /// Associated VRs: SS
    pub fn parse_i16<Endian: ByteOrder>(&mut self) -> Result<i16, Error> {
        self.reset_data_read();
        // TODO: Verify that we're parsing as 2s complement (not sure Endian should be considered?)
        let result: i16 = self.data.read_i16::<Endian>()?;
        Ok(result)
    }

    /// Parses the value for this element as a list of signed 16bit integer values
    /// Associated VRs: OW
    pub fn parse_i16s<Endian: ByteOrder>(&mut self) -> Result<Vec<i16>, Error> {
        self.reset_data_read();
        let num_bytes: usize = self.data.get_ref().len();
        let num_words: usize = num_bytes / 4;
        let mut result: Vec<i16> = Vec::with_capacity(num_words);
        for _ in 0..num_words {
            let val: i16 = self.data.read_i16::<Endian>()?;
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as a signed 32bit integer
    /// Associated VRs: SL
    pub fn parse_i32<Endian: ByteOrder>(&mut self) -> Result<i32, Error> {
        self.reset_data_read();
        // TODO: Verify that we're parsing as 2s complement (not sure Endian should be considered?)
        let result: i32 = self.data.read_i32::<Endian>()?;
        Ok(result)
    }

    /// Parses the value for this element as a list of signed 32bit integer values
    /// Associated VRs: OL
    pub fn parse_i32s<Endian: ByteOrder>(&mut self) -> Result<Vec<i32>, Error> {
        self.reset_data_read();
        let num_bytes: usize = self.data.get_ref().len();
        let num_longs: usize = num_bytes / 4;
        let mut result: Vec<i32> = Vec::with_capacity(num_longs);
        for _ in 0..num_longs {
            let val: i32 = self.data.read_i32::<Endian>()?;
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as an unsigned 32bit integer
    /// Associated VRs: UL
    pub fn parse_u32<Endian: ByteOrder>(&mut self) -> Result<u32, Error> {
        self.reset_data_read();
        let result: u32 = self.data.read_u32::<Endian>()?;
        Ok(result)
    }

    /// Parses the value for this element as an unsigned 16bit integer
    /// Associated VRs: US
    pub fn parse_u16<Endian: ByteOrder>(&mut self) -> Result<u16, Error> {
        self.reset_data_read();
        let result: u16 = self.data.read_u16::<Endian>()?;
        Ok(result)
    }
}
