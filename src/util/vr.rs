#![allow(dead_code)]
#![allow(non_camel_case_types)]

extern crate byteorder;
extern crate encoding;

use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};

use encoding::{DecoderTrap, EncoderTrap, Encoding};

use std::borrow::Cow;
use std::io::{Error, ErrorKind};

pub trait VR : ReadBytesExt + WriteBytesExt {
	fn get_code(&self) -> u32;
	fn get_pading(&self) -> u32;
	fn get_explicit_vr_header_length() -> u32;

	/* to_bytes/to_ints/to_shorts/to_strings/to_dates/to_etc. */

	// fn u32_to_bytes(&self, value: u32, big_endian: bool) -> Vec<u8>;
	// fn u32s_to_bytes(&self, values: Vec<u32>, big_endian: bool) -> Vec<u8>;

	// fn bytes_to_u32(&self, value: Vec<u8>, big_endian: bool) -> u32;
	// fn bytes_to_u32s(&self, value: Vec<u8>, big_endian: bool) -> Vec<u32>;

	// fn str_to_bytes(&self, value: &str, big_endian: bool, cs: &Encoding) -> Vec<u8>;
	// fn strs_to_bytes(&self, values: Vec<&str>, big_endian: bool, cs: &Encoding) -> Vec<u8>;

	// fn bytes_to_str(&self, value: Vec<u8>, big_endian: bool, cs: &Encoding) -> String;
	// fn bytes_to_strs(&self, values: Vec<u8>, big_endian: bool, cs: &Encoding) -> Vec<String>;
}

pub trait IntVR : VR {
	fn write_u32s<Endian: ByteOrder>(&mut self, values: Vec<u32>) -> Result<(), Error> {
		for value in values {
			try!(self.write_u32::<Endian>(value));
		}
		Result::Ok(())
	}

	fn read_u32s<Endian: ByteOrder>(&mut self, num_values: usize) -> Result<Vec<u32>, Error> {
		let mut values: Vec<u32> = Vec::with_capacity(num_values);
		for n in 0..num_values {
			values[n] = try!(self.read_u32::<Endian>());
		}
		Result::Ok(values)
	}

	fn write_string<Endian: ByteOrder>(&mut self, value: &str, cs: &Encoding) -> Result<(), Error> {
		let int_val: u32 = try!(u32::from_str_radix(value, 10).map_err(|_| Error::new(ErrorKind::InvalidData, "Unable to convert string to u32")));
		try!(self.write_u32::<Endian>(int_val));
		Result::Ok(())
	}

	// fn u32_to_bytes(&self, value: u32, big_endian: bool) -> Result<Vec<u8>, Error> {
	// 	let mut bytes: Vec<u8> = Vec::with_capacity(4);
	// 	if big_endian {
	// 		try!(bytes.write_u32::<BigEndian>(value));
	// 	} else {
	// 		try!(bytes.write_u32::<LittleEndian>(value));
	// 	}
	// 	Result::Ok(bytes)
	// }
}


pub enum VRType {
	/// Illegal VR='??' used by old SIEMENS Modalities
	UN_SIEMENS,

	/// Application Entity (<= 16 chars/u16)
	AE,

	/// Age String
	AS,

	/// Attribute Tag
	AT,

	/// Code String (<= 16 chars/u16) 
	CS,

	/// Date
	DA,

	/// Decimal String (=> f32)
	DS,

	/// Date Time
	DT,

	/// Floating Point Single (=> f32)
	FL,

	/// Floating Point Double (=> f64)
	FD,

	/// Integer String (=> i32)
	IS,

	/// Long String (<= 64 chars/u16)
	LO,

	/// Long Text (<= 10240 chars/u16)
	LT,

	/// Other Byte String
	OB,

	/// Other Float String
	OF,

	/// Other Word String
	OW,

	/// Person Name (component group <= 64 chars/u16)
	PN,

	/// Short String (<= 16 chars/u16)
	SH,

	/// Signed Long (-2147483648..+2147483647)
	SL,

	/// Sequence of Items
	SQ,

	/// Signed Short (-32768..+32767)
	SS,

	/// Short Text (<= 1024 chars/u16)
	ST,

	/// Time
	TM,

	/// Unique Identifier (UID) (<= 64 chars/u16)
	UI,

	/// Unsigned Long (0..4294967295)
	UL,

	/// Unkown
	UN,

	/// Unsigned Short (0..65535)
	US,

	/// Unlimited Text (<= 4294967294 chars/u16)
	UT,
}

fn str_to_bytes(value: &str, cs: &Encoding) -> Result<Vec<u8>, Cow<'static, str>> {
	cs.encode(value, EncoderTrap::Strict)
}

fn bytes_to_str(value: Vec<u8>, cs: &Encoding) -> Result<String, Cow<'static, str>> {
	cs.decode(value.as_slice(), DecoderTrap::Strict)
}

fn strs_to_bytes(values: Vec<&str>, cs: &Encoding) -> Result<Vec<u8>, Cow<'static, str>> {
	str_to_bytes(&values.join("\\"), cs)
}

fn bytes_to_strs(values: Vec<u8>, cs: &Encoding) -> Result<Vec<String>, Cow<'static, str>> {
	let as_str: String = try!(bytes_to_str(values, cs));
	let as_strs: Vec<String> = as_str.split('\\').map(|s| s.to_owned()).collect();
	Result::Ok(as_strs)
}

fn bytes_to_str_1(values: Vec<u8>, cs: &Encoding) -> Result<String, Cow<'static, str>> {
	let as_str: String = try!(bytes_to_str(values, cs));
	let first_str: Option<String> = as_str.split('\\').map(|s| s.to_owned()).nth(0);
	first_str.ok_or(Cow::Borrowed("Unable to decode"))
}



pub fn code_to_type(code: u32) -> Option<VRType> {
    match code {
        0x3F3F => Option::Some(VRType::UN_SIEMENS),
        0x4145 => Option::Some(VRType::AE),
        0x4153 => Option::Some(VRType::AS),
        0x4154 => Option::Some(VRType::AT),
        0x4353 => Option::Some(VRType::CS),
        0x4441 => Option::Some(VRType::DA),
        0x4453 => Option::Some(VRType::DS),
        0x4454 => Option::Some(VRType::DT),
        0x4644 => Option::Some(VRType::FD),
        0x464c => Option::Some(VRType::FL),
        0x4953 => Option::Some(VRType::IS),
        0x4c4f => Option::Some(VRType::LO),
        0x4c54 => Option::Some(VRType::LT),
        0x4f42 => Option::Some(VRType::OB),
        0x4f46 => Option::Some(VRType::OF),
        0x4f57 => Option::Some(VRType::OW),
        0x504e => Option::Some(VRType::PN),
        0x5348 => Option::Some(VRType::SH),
        0x534c => Option::Some(VRType::SL),
        0x5351 => Option::Some(VRType::SQ),
        0x5353 => Option::Some(VRType::SS),
        0x5354 => Option::Some(VRType::ST),
        0x544d => Option::Some(VRType::TM),
        0x5549 => Option::Some(VRType::UI),
        0x554c => Option::Some(VRType::UL),
        0x554E => Option::Some(VRType::UN),
        0x5553 => Option::Some(VRType::US),
        0x5554 => Option::Some(VRType::UT),
        _ => Option::None,
    }
}