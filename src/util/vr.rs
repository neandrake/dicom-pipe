#![allow(dead_code)]
#![allow(non_camel_case_types)]

pub enum VR {
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

pub fn value_of(code: u32) -> Option<VR> {
    match code {
        0x3F3F => Option::Some(VR::UN_SIEMENS),
        0x4145 => Option::Some(VR::AE),
        0x4153 => Option::Some(VR::AS),
        0x4154 => Option::Some(VR::AT),
        0x4353 => Option::Some(VR::CS),
        0x4441 => Option::Some(VR::DA),
        0x4453 => Option::Some(VR::DS),
        0x4454 => Option::Some(VR::DT),
        0x4644 => Option::Some(VR::FD),
        0x464c => Option::Some(VR::FL),
        0x4953 => Option::Some(VR::IS),
        0x4c4f => Option::Some(VR::LO),
        0x4c54 => Option::Some(VR::LT),
        0x4f42 => Option::Some(VR::OB),
        0x4f46 => Option::Some(VR::OF),
        0x4f57 => Option::Some(VR::OW),
        0x504e => Option::Some(VR::PN),
        0x5348 => Option::Some(VR::SH),
        0x534c => Option::Some(VR::SL),
        0x5351 => Option::Some(VR::SQ),
        0x5353 => Option::Some(VR::SS),
        0x5354 => Option::Some(VR::ST),
        0x544d => Option::Some(VR::TM),
        0x5549 => Option::Some(VR::UI),
        0x554c => Option::Some(VR::UL),
        0x554E => Option::Some(VR::UN),
        0x5553 => Option::Some(VR::US),
        0x5554 => Option::Some(VR::UT),
        _ => Option::None,
    }
}