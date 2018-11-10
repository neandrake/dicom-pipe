use byteorder::{ByteOrder, ReadBytesExt};

use core::dict::lookup::TAG_BY_VALUE;
use core::tag::Tag;
use core::vl::ValueLength;
use core::vr;
use core::vr::{CHARACTER_STRING_SEPARATOR, VRRef};

use encoding::types::DecoderTrap;

use read::dcmdataset::{DicomDataSet, DicomDataSetContainer};
use read::CSRef;

use std::borrow::Cow;
use std::fmt;
use std::io::{Cursor, Error, ErrorKind};

/// Represents a DICOM Element including its Tag, VR, and Value
/// Provides methods for parsing the element value as different native types
pub struct DicomElement {
    pub tag: u32,
    pub vr: VRRef,
    pub vl: ValueLength,

    value: Cursor<Vec<u8>>,
    items: DicomDataSet,
}

/// A nice user-readable display of the element such as
/// (0002,0001) OB FileMetaInformationVersion
impl fmt::Debug for DicomElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tag_num: String = Tag::format_tag_to_display(self.tag);

        let tag_name: &str = if let Some(tag) = TAG_BY_VALUE.get(&self.tag) {
            &tag.ident
        } else {
            "{{Unknown Tag}}"
        };

        write!(f, "{} {} {}", tag_num, self.vr.ident, tag_name)
    }
}

impl DicomElement {
    pub fn new(tag: u32, vr: VRRef, vl: ValueLength, value: Vec<u8>) -> DicomElement {
        DicomElement {
            tag,
            vr,
            vl,

            value: Cursor::new(value),
            items: DicomDataSet::new(),
        }
    }

    pub fn get_value(&self) -> &Cursor<Vec<u8>> {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.value
    }

    pub fn add_item(&mut self, index: u32, item: DicomElement) -> Option<DicomElement> {
        self.items.put_element(index, item)
    }

    /// Resets the internal byte stream to be parsed from the beginning
    fn reset_value_read(&mut self) {
        self.value.set_position(0);
    }

    /// Returns whether the the size of the value for this element is zero
    pub fn is_empty(&self) -> bool {
        self.value.get_ref().len() == 0
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

        if is_ui {
            if data[data.len() - 1] == padding {
                data = &data[0..data.len() - 1];
            }
        }

        cs.decode(data, DecoderTrap::Strict)
            .map(|v: String| {
                if !is_ui && is_char_str {
                    v.trim_end_matches(|c: char| c == padding as char).to_string()
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

        if is_ui {
            if data[data.len() - 1] == padding {
                data = &data[0..data.len() - 1];
            }
        }

        cs.decode(data, DecoderTrap::Strict)
            .map(|v: String| {
                if !is_ui && is_char_str {
                    v.trim_end_matches(|c: char| c == padding as char).to_string()
                } else {
                    v
                }
            })
            .map_err(|e: Cow<'static, str>| Error::new(ErrorKind::InvalidData, e.into_owned()))
            .map(|multivalue: String| {
                multivalue
                    .split(CHARACTER_STRING_SEPARATOR)
                    .map(|singlevalue: &str| singlevalue.to_owned())
                    .collect::<Vec<String>>()
            })
    }

    /// Returns the value as a slice with the padding character
    /// removed per the specification of whether the VR indicates leading/trailing
    /// padding is significant.
    fn get_string_bytes_without_padding(&mut self) -> &[u8] {
        self.reset_value_read();

        // grab the position to start reading bytes from prior to computing the new bytes_read
        let mut lindex: usize = 0;

        let data: &[u8] = self.get_value().get_ref();

        let mut rindex: usize = data.len() - 1;
        if self.vr.should_trim_trailing_space {
            while rindex >= lindex {
                if data[rindex] == 0x20 {
                    rindex = rindex - 1;
                } else {
                    break;
                }
            }
        }
        if self.vr.should_trim_leading_space {
            while lindex < rindex {
                if data[lindex] == 0x20 {
                    lindex = lindex + 1;
                } else {
                    break;
                }
            }
        }
        &data[lindex .. (rindex + 1)]
    }

    /// Parses the value for this element as an attribute (aka a tag)
    /// Associated VRs: AT
    pub fn parse_attribute<Endian: ByteOrder>(&mut self) -> Result<u32, Error> {
        self.reset_value_read();
        let first: u32 = (self.value.read_u16::<Endian>()? as u32) << 16;
        let second: u32 = self.value.read_u16::<Endian>()? as u32;
        let result: u32 = first + second;
        Ok(result)
    }

    /// Parses the value for this element as a 32bit floating point
    /// Associated VRs: FL
    pub fn parse_f32<Endian: ByteOrder>(&mut self) -> Result<f32, Error> {
        self.reset_value_read();
        let result: f32 = self.value.read_f32::<Endian>()?;
        Ok(result)
    }

    /// Parses the value for this element as a list of 32bit floating point values
    /// Associated VRs: OF
    pub fn parse_f32s<Endian: ByteOrder>(&mut self) -> Result<Vec<f32>, Error> {
        self.reset_value_read();
        let num_bytes: usize = self.value.get_ref().len();
        let num_floats: usize = num_bytes / 4;
        let mut result: Vec<f32> = Vec::with_capacity(num_floats);
        for _ in 0..num_floats {
            let val: f32 = self.value.read_f32::<Endian>()?;
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as a 64bit floating point
    /// Associated VRs: FD
    pub fn parse_f64<Endian: ByteOrder>(&mut self) -> Result<f64, Error> {
        self.reset_value_read();
        let result: f64 = self.value.read_f64::<Endian>()?;
        Ok(result)
    }

    /// Parses the value for this element as a list of 64bit floating point values
    /// Associated VRs: OD
    pub fn parse_f64s<Endian: ByteOrder>(&mut self) -> Result<Vec<f64>, Error> {
        self.reset_value_read();
        let num_bytes: usize = self.value.get_ref().len();
        let num_doubles: usize = num_bytes / 8;
        let mut result: Vec<f64> = Vec::with_capacity(num_doubles);
        for _ in 0..num_doubles {
            let val: f64 = self.value.read_f64::<Endian>()?;
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as a signed 16bit integer
    /// Associated VRs: SS
    pub fn parse_i16<Endian: ByteOrder>(&mut self) -> Result<i16, Error> {
        self.reset_value_read();
        // TODO: Verify that we're parsing as 2s complement (not sure Endian should be considered?)
        let result: i16 = self.value.read_i16::<Endian>()?;
        Ok(result)
    }

    /// Parses the value for this element as a list of signed 16bit integer values
    /// Associated VRs: OW
    pub fn parse_i16s<Endian: ByteOrder>(&mut self) -> Result<Vec<i16>, Error> {
        self.reset_value_read();
        let num_bytes: usize = self.value.get_ref().len();
        let num_words: usize = num_bytes / 4;
        let mut result: Vec<i16> = Vec::with_capacity(num_words);
        for _ in 0..num_words {
            let val: i16 = self.value.read_i16::<Endian>()?;
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as a signed 32bit integer
    /// Associated VRs: SL
    pub fn parse_i32<Endian: ByteOrder>(&mut self) -> Result<i32, Error> {
        self.reset_value_read();
        // TODO: Verify that we're parsing as 2s complement (not sure Endian should be considered?)
        let result: i32 = self.value.read_i32::<Endian>()?;
        Ok(result)
    }

    /// Parses the value for this element as a list of signed 32bit integer values
    /// Associated VRs: OL
    pub fn parse_i32s<Endian: ByteOrder>(&mut self) -> Result<Vec<i32>, Error> {
        self.reset_value_read();
        let num_bytes: usize = self.value.get_ref().len();
        let num_longs: usize = num_bytes / 4;
        let mut result: Vec<i32> = Vec::with_capacity(num_longs);
        for _ in 0..num_longs {
            let val: i32 = self.value.read_i32::<Endian>()?;
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the value for this element as an unsigned 32bit integer
    /// Associated VRs: UL
    pub fn parse_u32<Endian: ByteOrder>(&mut self) -> Result<u32, Error> {
        self.reset_value_read();
        let result: u32 = self.value.read_u32::<Endian>()?;
        Ok(result)
    }

    /// Parses the value for this element as an unsigned 16bit integer
    /// Associated VRs: US
    pub fn parse_u16<Endian: ByteOrder>(&mut self) -> Result<u16, Error> {
        self.reset_value_read();
        let result: u16 = self.value.read_u16::<Endian>()?;
        Ok(result)
    }
}
