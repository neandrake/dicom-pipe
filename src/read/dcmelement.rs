use byteorder::{ByteOrder, ReadBytesExt};

use core::dict::dicom_elements as tags;
use core::dict::lookup::TAG_BY_VALUE;
use core::tag::Tag;
use core::vl;
use core::vl::ValueLength;
use core::vr;
use core::vr::{CHARACTER_STRING_SEPARATOR, VRRef};

use encoding::types::{DecoderTrap, EncodingRef};

use read::dcmitem::DicomItem;

use std::borrow::Cow;
use std::fmt;
use std::io::{Cursor, Error, ErrorKind, Read};

static MAX_BYTES_DISPLAY: usize = 16;

/// Represents a DICOM Element including its Tag, VR, and Value
/// Provides methods for parsing the element value as different native types
pub struct DicomElement {
    pub tag: u32,
    pub vr: VRRef,
    pub vl: ValueLength,
    value: Cursor<Vec<u8>>,

    bytes_read: usize,
    tag_peek: Option<u32>,
}

/// A nice user-readable display of the element such as
/// (0002,0001) OB FileMetaInformationVersion
impl fmt::Debug for DicomElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tag_num: String = Tag::format_tag_to_display(self.tag);

        let tag_name: String = if let Some(tag) = TAG_BY_VALUE.get(&self.tag) {
            format!("{}", tag.ident)
        } else {
            "{{Unknown Tag}}".to_owned()
        };

        write!(f, "{} {} {}", tag_num, self.vr.ident, tag_name)
    }
}

impl DicomElement {
    pub fn new(tag: u32, vr: VRRef, vl: ValueLength, value: Vec<u8>) -> DicomElement {
        DicomElement {
            tag: tag,
            vr: vr,
            vl: vl,
            value: Cursor::new(value),

            bytes_read: 0_usize,
            tag_peek: None,
        }
    }

    pub fn get_value(&self) -> &Cursor<Vec<u8>> {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.value
    }

    /// Resets the internal byte stream to be parsed from the beginning
    fn reset_value_read(&mut self) {
        self.bytes_read = 0;
        self.value.set_position(0);
    }

    /// Returns whether the the size of the value for this element is zero
    pub fn is_empty(&self) -> bool {
        self.value.get_ref().len() == 0
    }

    /// Returns the previous tag/attribute that was parsed from the value of this element,
    /// or if no previous tag was parses the attribute from the current position in the stream
    fn read_tag<Endian: ByteOrder>(&mut self) -> Result<u32, Error> {
        if let Some(last_tag) = self.tag_peek {
            return Ok(last_tag);
        }
        let value: u32 = self.read_attribute::<Endian>()?;
        self.tag_peek = Some(value);
        Ok(value)
    }

    /// Parses the remainder of the value of this element as a string using the given encoding
    /// Associated VRs:
    /// All character string AE's -- subsequent interpretation of String is necessary based on VR
    /// AE, AS, CS, DA, DS, DT, IS, LO, LT, PN, SH, ST, TM, UC, UI, UR, UT
    pub fn read_string(&mut self, cs: EncodingRef) -> Result<String, Error> {
        let data: &[u8] = self.get_string_bytes_without_padding();
        cs.decode(data, DecoderTrap::Strict)
            .map_err(|e: Cow<'static, str>| Error::new(ErrorKind::InvalidData, e.into_owned()))
    }

    /// Parses the remainder of the value of this element as a list of strings using the given encoding
    /// Associated VRs:
    /// All character string AE's -- subsequent interpretation of String is necessary based on VR
    /// AE, AS, CS, DA, DS, DT, IS, LO, LT, PN, SH, ST, TM, UC, UI, UR, UT
    pub fn read_strings(&mut self, cs: EncodingRef) -> Result<Vec<String>, Error> {
        let data: &[u8] = self.get_string_bytes_without_padding();
        cs.decode(data, DecoderTrap::Strict)
            .map_err(|e: Cow<'static, str>| Error::new(ErrorKind::InvalidData, e.into_owned()))
            .map(|multivalue: String| {
                multivalue
                    .split(CHARACTER_STRING_SEPARATOR)
                    .map(|singlevalue: &str| singlevalue.to_owned())
                    .collect::<Vec<String>>()
            })
    }

    /// Returns the remainder of the value as a slice with the padding character
    /// removed per the specification of whether the VR indicates leading/trailing
    /// padding is significant.
    fn get_string_bytes_without_padding(&mut self) -> &[u8] {
        // grab the position to start reading bytes from prior to computing the new bytes_read
        let mut lindex: usize = self.bytes_read;

        // scoped so the mutable borrow of self is released
        {
            self.bytes_read += self.get_value().get_ref().len() - self.bytes_read;
        }

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

    /// Parses the next 8 bytes of the value for this element as an attribute (aka a tag)
    /// Associated VRs: AT
    pub fn read_attribute<Endian: ByteOrder>(&mut self) -> Result<u32, Error> {
        let first: u32 = (self.value.read_u16::<Endian>()? as u32) << 16;
        self.bytes_read += 4;
        let second: u32 = self.value.read_u16::<Endian>()? as u32;
        self.bytes_read += 4;
        let result: u32 = first + second;
        Ok(result)
    }

    /// Parses the next 4 bytes of the value for this element as a 32bit floating point
    /// Associated VRs: FL
    pub fn read_f32<Endian: ByteOrder>(&mut self) -> Result<f32, Error> {
        let result: f32 = self.value.read_f32::<Endian>()?;
        self.bytes_read += 4;
        Ok(result)
    }

    /// Parses the remainder of the value for this element as a list of 32bit floating point values
    /// Associated VRs: OF
    pub fn read_f32s<Endian: ByteOrder>(&mut self) -> Result<Vec<f32>, Error> {
        let num_bytes: usize = self.value.get_ref().len() - self.bytes_read;
        let num_floats: usize = num_bytes / 4;
        let mut result: Vec<f32> = Vec::with_capacity(num_floats);
        for _ in 0..num_floats {
            let val: f32 = self.value.read_f32::<Endian>()?;
            self.bytes_read += 4;
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the next 8 bytes of the value for this element as a 64bit floating point
    /// Associated VRs: FD
    pub fn read_f64<Endian: ByteOrder>(&mut self) -> Result<f64, Error> {
        let result: f64 = self.value.read_f64::<Endian>()?;
        self.bytes_read += 8;
        Ok(result)
    }

    /// Parses the remainder of the value for this element as a list of 64bit floating point values
    /// Associated VRs: OD
    pub fn read_f64s<Endian: ByteOrder>(&mut self) -> Result<Vec<f64>, Error> {
        let num_bytes: usize = self.value.get_ref().len() - self.bytes_read;
        let num_doubles: usize = num_bytes / 8;
        let mut result: Vec<f64> = Vec::with_capacity(num_doubles);
        for _ in 0..num_doubles {
            let val: f64 = self.value.read_f64::<Endian>()?;
            self.bytes_read += 8;
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the next 2 bytes of the value for this element as a signed 16bit integer
    /// Associated VRs: SS
    pub fn read_i16<Endian: ByteOrder>(&mut self) -> Result<i16, Error> {
        // TODO: Verify that we're parsing as 2s complement (not sure Endian should be considered?)
        let result: i16 = self.value.read_i16::<Endian>()?;
        self.bytes_read += 2;
        Ok(result)
    }

    /// Parses the remainder of the value for this element as a list of signed 16bit integer values
    /// Associated VRs: OW
    pub fn read_i16s<Endian: ByteOrder>(&mut self) -> Result<Vec<i16>, Error> {
        let num_bytes: usize = self.value.get_ref().len() - self.bytes_read;
        let num_words: usize = num_bytes / 4;
        let mut result: Vec<i16> = Vec::with_capacity(num_words);
        for _ in 0..num_words {
            let val: i16 = self.value.read_i16::<Endian>()?;
            self.bytes_read += 2;
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the next 4 bytes of the value for this element as a signed 32bit integer
    /// Associated VRs: SL
    pub fn read_i32<Endian: ByteOrder>(&mut self) -> Result<i32, Error> {
        // TODO: Verify that we're parsing as 2s complement (not sure Endian should be considered?)
        let result: i32 = self.value.read_i32::<Endian>()?;
        self.bytes_read += 4;
        Ok(result)
    }

    /// Parses the remainder of the value for this element as a list of signed 32bit integer values
    /// Associated VRs: OL
    pub fn read_i32s<Endian: ByteOrder>(&mut self) -> Result<Vec<i32>, Error> {
        let num_bytes: usize = self.value.get_ref().len();
        let num_longs: usize = num_bytes / 4;
        let mut result: Vec<i32> = Vec::with_capacity(num_longs);
        for _ in 0..num_longs {
            let val: i32 = self.value.read_i32::<Endian>()?;
            self.bytes_read += 4;
            result.push(val);
        }
        Ok(result)
    }

    /// Parses the next 4 bytes of the value for this element as an unsigned 32bit integer
    /// Associated VRs: UL
    pub fn read_u32<Endian: ByteOrder>(&mut self) -> Result<u32, Error> {
        let result: u32 = self.value.read_u32::<Endian>()?;
        self.bytes_read += 4;
        Ok(result)
    }

    /// Parses the next 2 bytes of the value for this element as an unsigned 16bit integer
    /// Associated VRs: US
    pub fn read_u16<Endian: ByteOrder>(&mut self) -> Result<u16, Error> {
        let result: u16 = self.value.read_u16::<Endian>()?;
        self.bytes_read += 2;
        Ok(result)
    }

    /// Parses the remainder of the value for this element as a sequence of elements
    /// Associated VRs: SQ
    pub fn read_sequence<Endian: ByteOrder>(&mut self) -> Result<Vec<DicomItem>, Error> {
        let mut items: Vec<DicomItem> = Vec::new();
        loop {
            let tag: u32 = self.read_tag::<Endian>()?;
            if tag != tags::Item.tag {
                return Err(Error::new(ErrorKind::InvalidData, format!("Expected Item element when parsing as SQ but instead found: {}", tag)));
            }

            let vl: ValueLength = self.read_item_value_length::<Endian>()?;
            let bytes: Vec<u8> = self.read_item_value_field(&vl)?;

            // clear `self.tag_peek` as we've now read the entire element and the next
            // read should advance to the next tag
            self.tag_peek = None;

            items.push(DicomItem::new(tag, vl, bytes));

            // if the sequence delimiter tag was just read then it's the end of the sequence
            if tag == tags::SequenceDelimitationItem.tag {
                break;
            }

            // if there's no more data to read then it's the end of the sequence
            if let ValueLength::Explicit(len) = self.vl {
                if self.value.position() == (len as u64) - 1 {
                    break;
                }
            }
        }
        Ok(items)
    }

    /// Parses the next 4 bytes as an unsigned 32bit integer value length
    fn read_item_value_length<Endian: ByteOrder>(&mut self) -> Result<ValueLength, Error> {
        let value_length: u32 = self.read_u32::<Endian>()?;
        Ok(vl::from_value_length(value_length))
    }

    /// Extracts the next set of bytes. The number of bytes returned is based on the given value length
    fn read_item_value_field(&mut self, vl: &ValueLength) -> Result<Vec<u8>, Error> {
        match *vl {
            ValueLength::Explicit(value_length) => {
                let mut bytes: Vec<u8> = vec![0;value_length as usize];
                self.value.read_exact(bytes.as_mut_slice())?;
                self.bytes_read += value_length as usize;
                Ok(bytes)
            },
            ValueLength::UndefinedLength => {
                // TODO: Read until Sequence Delimitation Item
                // Part 5 Ch. 7.1.3
                // The Value Field has an Undefined Length and a Sequence Delimitation Item marks the end of the Value Field.
                unimplemented!("Undefined length values not yet implemented");
            },
        }
    }

    /// Formats the value of this element as a string based on the VR
    /// XXX: This re-parses the value! This should probably not be here?
    pub fn fmt_string_value<Endian: ByteOrder>(&mut self, cs: EncodingRef) -> Result<String, Error> {
        if self.is_empty() {
            return Ok("<EMPTY VALUE>".to_owned());
        }
        self.reset_value_read();
        if self.vr.is_character_string {
            Ok(self.read_strings(cs)?
                .iter_mut()
                .map(|val: &mut String| {
                    format!("\"{}\"", val)
                })
                .collect::<Vec<String>>()
                .join(", "))
        } else if self.vr == &vr::AT {
            Ok(Tag::format_tag_to_display(self.read_attribute::<Endian>()?))
        } else if self.vr == &vr::FL {
            Ok(format!("{}", self.read_f32::<Endian>()?))
        } else if self.vr == &vr::OF {
            let values: Vec<f32> = self.read_f32s::<Endian>()?;
            let mut string_values: String = String::new();
            for value in values {
                string_values.push_str(&format!("{} / ", value));
            }
            Ok(string_values)
        } else if self.vr == &vr::FD {
            Ok(format!("{}", self.read_f64::<Endian>()?))
        } else if self.vr == &vr::OD {
            let values: Vec<f64> = self.read_f64s::<Endian>()?;
            let mut string_values: String = String::new();
            for value in values {
                string_values.push_str(&format!("{} / ", value));
            }
            Ok(string_values)
        } else if self.vr == &vr::SS {
            Ok(format!("{}", self.read_i16::<Endian>()?))
        } else if self.vr == &vr::OW {
            let values: Vec<i16> = self.read_i16s::<Endian>()?;
            let mut string_values: String = String::new();
            for value in values {
                string_values.push_str(&format!("{} / ", value));
            }
            Ok(string_values)
        } else if self.vr == &vr::SL {
            Ok(format!("{}", self.read_i32::<Endian>()?))
        } else if self.vr == &vr::OL {
            let values: Vec<i32> = self.read_i32s::<Endian>()?;
            let mut string_values: String = String::new();
            for value in values {
                string_values.push_str(&format!("{} / ", value));
            }
            Ok(string_values)
        } else if self.vr == &vr::UL {
            Ok(format!("{}", self.read_u32::<Endian>()?))
        } else if self.vr == &vr::US {
            Ok(format!("{}", self.read_u16::<Endian>()?))
        } else {
            let byte_vec: &Vec<u8> = self.value.get_ref();
            if byte_vec.len() <= MAX_BYTES_DISPLAY {
                Ok(format!("{:?}", byte_vec))
            } else {
                let byte_display: String = byte_vec
                    .iter()
                    .take(MAX_BYTES_DISPLAY)
                    .map(|val: &u8| format!("{}", val))
                    .collect::<Vec<String>>()
                    .join(", ");
                Ok(format!("[{}, ..]", byte_display))
            }
        }
    }
}
