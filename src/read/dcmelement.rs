use byteorder::{ByteOrder, ReadBytesExt};

use core::dict::lookup::TAG_BY_VALUE;
use core::vl::ValueLength;
use core::vr::VRRef;

use encoding::types::{DecoderTrap, EncodingRef};

use std::borrow::Cow;
use std::fmt;
use std::io::{Cursor, Error, ErrorKind};

pub struct DicomElement {
    pub tag: u32,
    pub vr: VRRef,
    pub vl: ValueLength,
    value: Cursor<Vec<u8>>,
}

impl DicomElement {
    pub fn new(tag: u32, vr: VRRef, vl: ValueLength, value: Vec<u8>) -> DicomElement {
        DicomElement {
            tag: tag,
            vr: vr,
            vl: vl,
            value: Cursor::new(value),
        }
    }

    pub fn get_value(&self) -> &Cursor<Vec<u8>> {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.value
    }

    /// All character string AE's
    pub fn parse_string(&self, cs: EncodingRef) -> Result<String, Error> {
        let data: &Vec<u8> = self.get_value().get_ref();
        cs.decode(data, DecoderTrap::Strict)
            .map_err(|e: Cow<'static, str>| Error::new(ErrorKind::InvalidData, e.into_owned()))
    }

    /// AT
    pub fn parse_attribute<Endian: ByteOrder>(&mut self) -> Result<u32, Error> {
        let first: u32 = (self.value.read_u16::<Endian>()? as u32) << 16;
        let second: u32 = self.value.read_u16::<Endian>()? as u32;
        let result: u32 = first + second;
        self.value.set_position(0);
        Ok(result)
    }

    /// FL
    pub fn parse_f32<Endian: ByteOrder>(&mut self) -> Result<f32, Error> {
        let result: f32 = self.value.read_f32::<Endian>()?;
        self.value.set_position(0);
        Ok(result)
    }

    /// OF
    pub fn parse_f32s<Endian: ByteOrder>(&mut self) -> Result<Vec<f32>, Error> {
        let num_bytes: usize = self.value.get_ref().len();
        let num_floats: usize = num_bytes / 4;
        let mut result: Vec<f32> = Vec::with_capacity(num_floats);
        for _ in 0..num_floats {
            let val: f32 = self.value.read_f32::<Endian>()?;
            result.push(val);
        }
        self.value.set_position(0);
        Ok(result)
    }

    /// FD
    pub fn parse_f64<Endian: ByteOrder>(&mut self) -> Result<f64, Error> {
        let result: f64 = self.value.read_f64::<Endian>()?;
        self.value.set_position(0);
        Ok(result)
    }

    /// OD
    pub fn parse_f64s<Endian: ByteOrder>(&mut self) -> Result<Vec<f64>, Error> {
        let num_bytes: usize = self.value.get_ref().len();
        let num_doubles: usize = num_bytes / 8;
        let mut result: Vec<f64> = Vec::with_capacity(num_doubles);
        for _ in 0..num_doubles {
            let val: f64 = self.value.read_f64::<Endian>()?;
            result.push(val);
        }
        self.value.set_position(0);
        Ok(result)
    }

    /// SS
    pub fn parse_i16<Endian: ByteOrder>(&mut self) -> Result<i16, Error> {
        // TODO: Verify that we're parsing as 2s complement (not sure Endian should be considered?)
        let result: i16 = self.value.read_i16::<Endian>()?;
        self.value.set_position(0);
        Ok(result)
    }

    /// OW
    pub fn parse_i16s<Endian: ByteOrder>(&mut self) -> Result<Vec<i16>, Error> {
        let num_bytes: usize = self.value.get_ref().len();
        let num_words: usize = num_bytes / 4;
        let mut result: Vec<i16> = Vec::with_capacity(num_words);
        for _ in 0..num_words {
            let val: i16 = self.value.read_i16::<Endian>()?;
            result.push(val);
        }
        self.value.set_position(0);
        Ok(result)
    }

    /// SL
    pub fn parse_i32<Endian: ByteOrder>(&mut self) -> Result<i32, Error> {
        // TODO: Verify that we're parsing as 2s complement (not sure Endian should be considered?)
        let result: i32 = self.value.read_i32::<Endian>()?;
        self.value.set_position(0);
        Ok(result)
    }

    /// OL
    pub fn parse_i32s<Endian: ByteOrder>(&mut self) -> Result<Vec<i32>, Error> {
        let num_bytes: usize = self.value.get_ref().len();
        let num_longs: usize = num_bytes / 4;
        let mut result: Vec<i32> = Vec::with_capacity(num_longs);
        for _ in 0..num_longs {
            let val: i32 = self.value.read_i32::<Endian>()?;
            result.push(val);
        }
        self.value.set_position(0);
        Ok(result)
    }

    /// UL
    pub fn parse_u32<Endian: ByteOrder>(&mut self) -> Result<u32, Error> {
        let result: u32 = self.value.read_u32::<Endian>()?;
        self.value.set_position(0);
        Ok(result)
    }

    /// US
    pub fn parse_u16<Endian: ByteOrder>(&mut self) -> Result<u16, Error> {
        let result: u16 = self.value.read_u16::<Endian>()?;
        self.value.set_position(0);
        Ok(result)
    }
}

impl fmt::Debug for DicomElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tag_display: String = if let Some(tag) = TAG_BY_VALUE.get(&self.tag) {
            format!("{}", tag.ident)
        } else {
            format!("{:08X}", self.tag)
        };

        write!(f, "DicomElement {{ tag: {}, vr: {}, vl: {:?} }}", tag_display, self.vr.ident, self.vl)
    }
}