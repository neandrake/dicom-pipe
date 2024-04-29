#![allow(dead_code)]
#![allow(non_camel_case_types)]

//! Value Representation

use std::hash::{Hash, Hasher};

pub type VRRef = &'static VR;

/// Value Representation
#[derive(Debug, Eq)]
pub struct VR {
	/// The two-letter identifer, "AE", "IS", etc.
	pub ident: &'static str,
	/// A display name
	pub name: &'static str,
	/// The 16-bit code for this AE. In most (all?) cases this is the ASCII representation of the ident.
	pub code: u32,
	/// If the value is padded, what value/code is used to pad. Generally either zeroes or the space (0x20)
	pub padding: u8,

	/// If this VR is encoded explicitly, then depending on VR there might be a 2-byte padding after the VR encoding
	/// Part 5 Ch 7.1.2:
	/// For VRs of OB, OD, OF, OL, OW, SQ, UC, UR, UT or UN the 16 bits
	/// following the two byte VR Field are reserved for use by later
	/// versions of the DICOM Standard. These reserved bytes shall be
	/// set to 0000H and shall not be used or decoded.
	pub has_explicit_2byte_pad: bool,
}

impl PartialEq for VR {
	fn eq(&self, other: &VR) -> bool {
		self.code.eq(&other.code)
	}
}

impl Hash for VR {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.code.hash(state)
	}
}

pub fn code_to_vr(code: u16) -> Option<&'static VR> {
	match code {
		0x4145 => Some(&AE),
		0x4153 => Some(&AS),
		0x4154 => Some(&AT),
		0x4353 => Some(&CS),
		0x4441 => Some(&DA),
		0x4453 => Some(&DS),
		0x4454 => Some(&DT),
		0x4644 => Some(&FD),
		0x464C => Some(&FL),
		0x4953 => Some(&IS),
		0x4C4F => Some(&LO),
		0x4C54 => Some(&LT),
		0x4F42 => Some(&OB),
		0x4F44 => Some(&OD),
		0x4F46 => Some(&OF),
		0x4F4C => Some(&OL),
		0x4F57 => Some(&OW),
		0x504E => Some(&PN),
		0x5348 => Some(&SH),
		0x534C => Some(&SL),
		0x5351 => Some(&SQ),
		0x5353 => Some(&SS),
		0x5354 => Some(&ST),
		0x544D => Some(&TM),
		0x5443 => Some(&UC),
		0x5549 => Some(&UI),
		0x554c => Some(&UL),
		0x544E => Some(&UN),
		0x5452 => Some(&UR),
		0x5553 => Some(&US),
		0x5554 => Some(&UT),
		_ => None,
	}
}


/// Application Entity (<= 16 chars/u16)
pub static AE: VR = VR {
	ident: "AE",
	name: "Application Entity",
	code: 0x4145,
	padding: 0x20,
	has_explicit_2byte_pad: false,
};

/// Age String
pub static AS: VR = VR {
	ident: "AS",
	name: "Age String",
	code: 0x4153,
	padding: 0x20,
	has_explicit_2byte_pad: false,
};

/// Attribute Tag
pub static AT: VR = VR {
	ident: "AT",
	name: "Attribute Tag",
	code: 0x4154,
	padding: 0x0,
	has_explicit_2byte_pad: false,
};

/// Code String (<= 16 chars/u16)
pub static CS: VR = VR {
	ident: "CS",
	name: "Code String",
	code: 0x4353,
	padding: 0x20,
	has_explicit_2byte_pad: false,
};

/// Date
pub static DA: VR = VR {
	ident: "DA",
	name: "Date",
	code: 0x4441,
	padding: 0x20,
	has_explicit_2byte_pad: false,
};

/// Decimal String (=> f32)
pub static DS: VR = VR {
	ident: "DS",
	name: "Decimal String",
	code: 0x4453,
	padding: 0x20,
	has_explicit_2byte_pad: false,
};

/// Date Time
pub static DT: VR = VR {
	ident: "DT",
	name: "Date Time",
	code: 0x4454,
	padding: 0x20,
	has_explicit_2byte_pad: false,
};

/// Floating Point Double (=> f64)
pub static FD: VR = VR {
	ident: "FD",
	name: "Floating Point Double",
	code: 0x4644,
	padding: 0x0,
	has_explicit_2byte_pad: false,
};

/// Floating Point Single (=> f32)
pub static FL: VR = VR {
	ident: "FL",
	name: "Floating Point Single",
	code: 0x464C,
	padding: 0x0,
	has_explicit_2byte_pad: false,
};

/// Integer String (=> i32)
pub static IS: VR = VR {
	ident: "IS",
	name: "Integer String",
	code: 0x4953,
	padding: 0x20,
	has_explicit_2byte_pad: false,
};

/// Long String (<= 64 chars/u16)
pub static LO: VR = VR {
	ident: "LO",
	name: "Long String",
	code: 0x4C4F,
	padding: 0x20,
	has_explicit_2byte_pad: false,
};

/// Long Text (<= 10240 chars/u16)
pub static LT: VR = VR {
	ident: "LT",
	name: "Long Text",
	code: 0x4C54,
	padding: 0x20,
	has_explicit_2byte_pad: false,
};

/// Other Byte
pub static OB: VR = VR {
	ident: "OB",
	name: "Other Byte",
	code: 0x4F42,
	padding: 0x0,
	has_explicit_2byte_pad: true,
};

/// Other Double
pub static OD: VR = VR {
	ident: "OD",
	name: "Other Byte",
	code: 0x4F44,
	padding: 0x0,
	has_explicit_2byte_pad: true,
};

/// Other Float
pub static OF: VR = VR {
	ident: "OF",
	name: "Other Float",
	code: 0x4F46,
	padding: 0x0,
	has_explicit_2byte_pad: true,
};

/// Other Long
pub static OL: VR = VR {
	ident: "OL",
	name: "Other Long",
	code: 0x4F4C,
	padding: 0x0,
	has_explicit_2byte_pad: true,
};

/// Other Word
pub static OW: VR = VR {
	ident: "OW",
	name: "Other Float",
	code: 0x4F57,
	padding: 0x0,
	has_explicit_2byte_pad: true,
};

/// Person Name (component group <= 64 chars/u16)
pub static PN: VR = VR {
	ident: "PN",
	name: "Person Name",
	code: 0x504E,
	padding: 0x20,
	has_explicit_2byte_pad: false,
};

/// Short String (<= 16 chars/u16)
pub static SH: VR = VR {
	ident: "SH",
	name: "Short String",
	code: 0x5348,
	padding: 0x20,
	has_explicit_2byte_pad: false,
};

/// Signed Long (-2147483648..+2147483647)
pub static SL: VR = VR {
	ident: "SL",
	name: "Signed Long",
	code: 0x534C,
	padding: 0x20,
	has_explicit_2byte_pad: false
};

/// Sequence of Items
pub static SQ: VR = VR {
	ident: "SQ",
	name: "Sequence of Items",
	code: 0x5351,
	padding: 0x0,
	has_explicit_2byte_pad: true,
};

/// Signed Short (-32768..+32767)
pub static SS: VR = VR {
	ident: "SS",
	name: "Signed Short",
	code: 0x5353,
	padding: 0x0,
	has_explicit_2byte_pad: false,
};

/// Short Text (<= 1024 chars/u16)
pub static ST: VR = VR {
	ident: "ST",
	name: "Short Text",
	code: 0x5354,
	padding: 0x20,
	has_explicit_2byte_pad: false,
};

/// Time
pub static TM: VR = VR {
	ident: "TM",
	name: "Time",
	code: 0x544D,
	padding: 0x20,
	has_explicit_2byte_pad: false,
};

/// Unlimited Characters
pub static UC: VR = VR {
	ident: "UC",
	name: "Time",
	code: 0x5443,
	padding: 0x20,
	has_explicit_2byte_pad: true
};

/// Unique Identifier (UID) (<= 64 chars/u16)
pub static UI: VR = VR {
	ident: "UI",
	name: "Unique Identifier (UID)",
	code: 0x5549,
	padding: 0x0,
	has_explicit_2byte_pad: false,
};

/// Unsigned Long (0..4294967295)
pub static UL: VR = VR {
	ident: "UL",
	name: "Unsigned Long",
	code: 0x554C,
	padding: 0x0,
	has_explicit_2byte_pad: false,
};

/// Unknown
pub static UN: VR = VR {
	ident: "UN",
	name: "Unknown",
	code: 0x544E,
	padding: 0x0,
	has_explicit_2byte_pad: true,
};

/// URI/URL
pub static UR: VR = VR {
	ident: "UR",
	name: "Universal Resource Identifier / Universal Resource Locator",
	code: 0x5452,
	padding: 0x20,
	has_explicit_2byte_pad: true,
};

/// Unsigned Short (0..65535)
pub static US: VR = VR {
	ident: "US",
	name: "Unsigned Short",
	code: 0x5553,
	padding: 0,
	has_explicit_2byte_pad: false,
};

/// Unlimited Text (<= 4294967294 chars/u16)
pub static UT: VR = VR {
	ident: "UT",
	name: "Unlimited Text",
	code: 0x5554,
	padding: 0x20,
	has_explicit_2byte_pad: true,
};
