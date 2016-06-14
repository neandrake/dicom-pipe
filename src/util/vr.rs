#![allow(dead_code)]
#![allow(non_camel_case_types)]


pub struct VR {
	pub code: u32,
	pub padding: u32,
	pub explicit_vr_header_length: u32,
}

impl VR {
	pub fn code_to_vr(code: u16) -> Option<&'static VR> {
		match code {
			0x4145 => Option::Some(AE),
			0x4153 => Option::Some(AS),
			0x4154 => Option::Some(AT),
			0x4353 => Option::Some(CS),
			0x4441 => Option::Some(DA),
			0x4453 => Option::Some(DS),
			0x4454 => Option::Some(DT),
			0x4644 => Option::Some(FD),
			0x464C => Option::Some(FL),
			0x4953 => Option::Some(IS),
			0x4C4F => Option::Some(LO),
			0x4C54 => Option::Some(LT),
			0x4F42 => Option::Some(OB),
			0x4F46 => Option::Some(OF),
			0x4F57 => Option::Some(OW),
			0x504E => Option::Some(PN),
			0x5348 => Option::Some(SH),
			0x534C => Option::Some(SL),
			0x5351 => Option::Some(SQ),
			0x5353 => Option::Some(SS),
			0x5354 => Option::Some(ST),
			0x544D => Option::Some(TM),
			0x5549 => Option::Some(UI),
			0x554c => Option::Some(UL),
			0x544E => Option::Some(UN),
			0x5553 => Option::Some(US),
			0x5554 => Option::Some(UT),
			_ => Option::None,
		}
	}
}

/// Application Entity (<= 16 chars/u16)
pub static AE: &'static VR = &VR {
	code: 0x4145,
	padding: 0x20,
	explicit_vr_header_length: 8,
};

/// Age String
pub static AS: &'static VR = &VR {
	code: 0x4153,
	padding: 0x20,
	explicit_vr_header_length: 8,
};

/// Attribute Tag
pub static AT: &'static VR = &VR {
	code: 0x4154,
	padding: 0x0,
	explicit_vr_header_length: 8,
};

/// Code String (<= 16 chars/u16)
pub static CS: &'static VR = &VR {
	code: 0x4353,
	padding: 0x20,
	explicit_vr_header_length: 8,
};

/// Date
pub static DA: &'static VR = &VR {
	code: 0x4441,
	padding: 0x20,
	explicit_vr_header_length: 8,
};

/// Decimal String (=> f32)
pub static DS: &'static VR = &VR {
	code: 0x4453,
	padding: 0x20,
	explicit_vr_header_length: 8,
};

/// Date Time
pub static DT: &'static VR = &VR {
	code: 0x4454,
	padding: 0x20,
	explicit_vr_header_length: 8,
};

/// Floating Point Double (=> f64)
pub static FD: &'static VR = &VR {
	code: 0x4644,
	padding: 0x0,
	explicit_vr_header_length: 8,
};

/// Floating Point Single (=> f32)
pub static FL: &'static VR = &VR {
	code: 0x464C,
	padding: 0x0,
	explicit_vr_header_length: 8,
};

/// Integer String (=> i32)
pub static IS: &'static VR = &VR {
	code: 0x4953,
	padding: 0x20,
	explicit_vr_header_length: 8,
};

/// Long String (<= 64 chars/u16)
pub static LO: &'static VR = &VR {
	code: 0x4C4F,
	padding: 0x20,
	explicit_vr_header_length: 8,
};

/// Long Text (<= 10240 chars/u16)
pub static LT: &'static VR = &VR {
	code: 0x4C54,
	padding: 0x20,
	explicit_vr_header_length: 8,
};

/// Other Byte String
pub static OB: &'static VR = &VR {
	code: 0x4F42,
	padding: 0x0,
	explicit_vr_header_length: 12,
};

/// Other Float String
pub static OF: &'static VR = &VR {
	code: 0x4F46,
	padding: 0x0,
	explicit_vr_header_length: 12,
};

/// Other Word String
pub static OW: &'static VR = &VR {
	code: 0x4F57,
	padding: 0x0,
	explicit_vr_header_length: 12,
};

/// Person Name (component group <= 64 chars/u16)
pub static PN: &'static VR = &VR {
	code: 0x504E,
	padding: 0x20,
	explicit_vr_header_length: 8,
};

/// Short String (<= 16 chars/u16)
pub static SH: &'static VR = &VR {
	code: 0x5348,
	padding: 0x20,
	explicit_vr_header_length: 8,
};

/// Signed Long (-2147483648..+2147483647)
pub static SL: &'static VR = &VR {
	code: 0x534C,
	padding: 0x20,
	explicit_vr_header_length: 8
};

/// Sequence of Items
pub static SQ: &'static VR = &VR {
	code: 0x5351,
	padding: 0x0,
	explicit_vr_header_length: 12,
};

/// Signed Short (-32768..+32767)
pub static SS: &'static VR = &VR {
	code: 0x5353,
	padding: 0x0,
	explicit_vr_header_length: 8,
};

/// Short Text (<= 1024 chars/u16)
pub static ST: &'static VR = &VR {
	code: 0x5354,
	padding: 0x20,
	explicit_vr_header_length: 8,
};

/// Time
pub static TM: &'static VR = &VR {
	code: 0x544D,
	padding: 0x20,
	explicit_vr_header_length: 8,
};

/// Unique Identifier (UID) (<= 64 chars/u16)
pub static UI: &'static VR = &VR {
	code: 0x5549,
	padding: 0x0,
	explicit_vr_header_length: 8,
};

/// Unsigned Long (0..4294967295)
pub static UL: &'static VR = &VR {
	code: 0x554C,
	padding: 0x0,
	explicit_vr_header_length: 8,
};

/// Unkown
pub static UN: &'static VR = &VR {
	code: 0x544E,
	padding: 0x0,
	explicit_vr_header_length: 12,
};

/// Unsigned Short (0..65535)
pub static US: &'static VR = &VR {
	code: 0x5553,
	padding: 0,
	explicit_vr_header_length: 8,
};

/// Unlimited Text (<= 4294967294 chars/u16)
pub static UT: &'static VR = &VR {
	code: 0x5554,
	padding: 0x20,
	explicit_vr_header_length: 12,
};
