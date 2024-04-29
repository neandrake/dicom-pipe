//! Value Representation

use std::fmt;
use std::hash::{Hash, Hasher};

use crate::core::charset::{CSRef, DEFAULT_CHARACTER_SET};

/// The BACKSLASH used to delimit multi-value character string values, `\`
pub const CS_SEPARATOR_BYTE: u8 = 0x5C;

/// The BACKSLASH used to delimit multi-value character string values, `\`
pub const CS_SEPARATOR: char = CS_SEPARATOR_BYTE as char;

/// The SPACE character used for padding
pub const SPACE_PADDING: u8 = 0x20;

/// The NULL character used for padding
pub const NULL_PADDING: u8 = 0x00;

pub type VRRef = &'static VR;

/// Value Representation Definition
///
/// The Value Representation of a Data Element describes the data type and format of that Data
/// Element's Value(s).

// VRs are defined at compile-time and never instantiated at runtime. Since these boolean values
// are only ever read by the use of this API, the boolean fields are more ergonomic to use compared
// to converting to enums.
#[allow(clippy::struct_excessive_bools)]
#[derive(Eq)]
pub struct VR {
    /// The two-letter identifer, "AE", "IS", etc.
    pub ident: &'static str,

    /// A display name, e.g. `"Application Entity"` for `VR::AE`.
    pub name: &'static str,

    /// The 16-bit code for this AE. In most (all?) cases this is the ASCII representation of the
    /// ident.
    pub code: u32,

    /// Which value is used to pad the encoded value to achieve an even length.
    ///
    /// Part 5, Ch 6.2:
    /// - Values with VRs constructed of character strings, except
    /// in the case of the VR UI, shall be padded with SPACE characters
    /// (20H, in the Default Character Repertoire).
    /// - Values with a VR of UI shall be padded with a single trailing
    /// NULL (00H) character when necessary to achieve even length.
    /// - Values with a VR of OB shall be padded with a single trailing
    /// NULL byte value (00H) when necessary to achieve even length.
    pub padding: u8,

    /// If this VR is encoded explicitly, then depending on VR there might be a 2-byte padding after
    /// the VR encoding.
    ///
    /// Part 5 Ch 7.1.2:
    /// For VRs of OB, OD, OF, OL, OV, OW, SQ and UN the 16 bits following the two byte VR Field are
    /// reserved for use by later versions of the DICOM Standard. These reserved bytes shall be set
    /// to 0000H and shall not be used or decoded. The Value Length Field is a 32-bit unsigned
    /// integer. If the Value Field has an Explicit Length, then the Value Length Field shall
    /// contain a value equal to the length (in bytes) of the Value Field. Otherwise, the Value
    /// Field has an Undefined Length and a Sequence Delimitation Item marks the end of the Value
    /// Field.
    ///
    /// For VRs of SV, UC, UR, UV and UT the 16 bits following the two byte VR Field are reserved
    /// for use by later versions of the DICOM Standard. These reserved bytes shall be set to 0000H
    /// and shall not be used or decoded. The Value Length Field is a 32-bit unsigned integer. The
    /// Value Field is required to have an Explicit Length, that is the Value Length Field shall
    /// contain a value equal to the length (in bytes) of the Value Field.
    pub has_explicit_2byte_pad: bool,

    /// Whether the value is interpreted as a character string instead of binary value.
    pub is_character_string: bool,

    /// Part 5, Ch 6.1.2.3:
    /// Data Elements of the following VR can have their character repertoire
    /// replaced via the Specific Character Set element. Other textual VRs
    /// should use the default (ISO IR 100 / ISO 8859).
    ///
    /// - SH (Short String)
    /// - LO (Long String)
    /// - UC (Unlimited Characters)
    /// - ST (Short Text)
    /// - LT (Long Text)
    /// - UT (Unlimited Text)
    /// - PN (Person Name)
    ///
    /// This replacement character repertoire does not apply to other textual
    /// Value Representations (AE and CS).
    ///
    /// # Notes
    /// Use `VR::get_proper_cs(CSRef)` which will return the appropriate character set based on the
    /// value of this field.
    pub decode_text_with_replaced_cs: bool,

    /// Part 5, Ch 6.1.2.3:
    /// The Graphic Character represented by the bit combination 05/12, `\` (BACKSLASH) in the
    /// repertoire ISO-IR 6, shall only be used in character strings with Value Representations of
    /// UT, ST and LT. Otherwise the character code 05/12 is used as a separator for multiple valued
    /// Data Elements.
    pub allows_backslash_text_value: bool,

    /// Whether the VR specifies that the padding character may be used at the front of the value.
    pub can_pad_front: bool,

    /// Whether the VR specifies that the padding character may be used at the end of the value.
    pub can_pad_end: bool,
}

impl PartialEq for VR {
    fn eq(&self, other: &VR) -> bool {
        self.code.eq(&other.code)
    }
}

impl Hash for VR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

impl fmt::Debug for VR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ident)
    }
}

impl VR {
    /// Gets the VR based on the encoded VR value, or `None` if the VR is unknown.
    ///
    /// Note that the code is effectively the ASCII encoding of the two letters making
    /// up the value representation identifier.
    #[must_use]
    pub fn from_code(code: u16) -> Option<VRRef> {
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
            0x4F56 => Some(&OV),
            0x504E => Some(&PN),
            0x5348 => Some(&SH),
            0x534C => Some(&SL),
            0x5351 => Some(&SQ),
            0x5353 => Some(&SS),
            0x5354 => Some(&ST),
            0x5356 => Some(&SV),
            0x544D => Some(&TM),
            0x5543 => Some(&UC),
            0x5549 => Some(&UI),
            0x554C => Some(&UL),
            0x554E => Some(&UN),
            0x5552 => Some(&UR),
            0x5553 => Some(&US),
            0x5554 => Some(&UT),
            0x5556 => Some(&UV),
            _ => None,
        }
    }

    /// Determines the appropriate character set to decode the string value for this VR, with the
    /// given character set specified by the DICOM dataset.
    #[must_use]
    pub fn get_proper_cs(&self, cs: CSRef) -> CSRef {
        if self.decode_text_with_replaced_cs {
            return cs;
        }
        DEFAULT_CHARACTER_SET
    }
}

/// # Application Entity
///
/// ## Definition
/// A string of characters that identifies an Application Entity with
/// leading and trailing spaces (20H) being non-significant. A value
/// consisting solely of spaces shall not be used.
///
/// ## Character Repertoire
/// Default Character Repertoire excluding character code 5CH
/// (the BACKSLASH `\` in ISO-IR 6), and all control characters.
///
/// ## Length of Value
/// 16 bytes maximum
pub static AE: VR = VR {
    ident: "AE",
    name: "Application Entity",
    code: 0x4145,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: true,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: true,
    can_pad_end: true,
};

/// # Age String
///
/// ## Definition
/// A string of characters with one of the following formats --
/// nnnD, nnnW, nnnM, nnnY; where nnn shall contain the number of
///
/// - days for D,
/// - weeks for W,
/// - months for M, or
/// - years for Y.
///
/// ### Examples
/// "018M" would represent an age of 18 months.
///
/// ## Character Repertoire
/// `'0'`-`'9'`, `D`, `W`, `M`, `Y` of Default Character Repertoire
///
/// ## Length of Value
/// 4 bytes fixed
pub static AS: VR = VR {
    ident: "AS",
    name: "Age String",
    code: 0x4153,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: true,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: true,
    can_pad_end: true,
};

/// # Attribute Tag
///
/// ## Definition
/// Ordered pair of 16-bit unsigned integers that is the value of
/// a Data Element Tag.
///
/// ### Examples
/// A Data Element Tag of (0018,00FF) would be encoded as
/// a series of 4 bytes in a Little-Endian Transfer Syntax as 18H,00H,FFH,00H.
///
/// ### Notes
/// The encoding of an AT value is exactly the same as the
/// encoding of a Data Element Tag as defined in Part 5, Ch 7
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// 4 bytes fixed
pub static AT: VR = VR {
    ident: "AT",
    name: "Attribute Tag",
    code: 0x4154,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Code String
///
/// ## Definition
/// A string of characters identifying a controlled concept.
/// Leading or trailing spaces (20H) are not significant.
///
/// ## Character Repertoire
/// Uppercase characters, `'0'`-`'9'`, the SPACE character,
/// and underscore `_`, of the Default Character Repertoire
///
/// ## Length of Value
/// 16 bytes maximum
pub static CS: VR = VR {
    ident: "CS",
    name: "Code String",
    code: 0x4353,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: true,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: true,
    can_pad_end: true,
};

/// # Date
///
/// ## Definition
/// A string of characters of the format YYYYMMDD; where
///
/// - YYYY shall contain year,
/// - MM shall contain the month,
/// - and DD shall contain the day,
///
/// interpreted as a date of the Gregorian calendar system.
///
/// ### Examples
/// "19930822" would represent August 22, 1993.
///
/// ### Notes
/// 1. The ACR-NEMA Standard 300 (predecessor to DICOM) supported a
/// string of characters of the format YYYY.MM.DD for this VR.
/// Use of this format is not compliant.
/// 2. See also DT VR in this table.
/// 3. Dates before year 1582, e.g. used for dating historical or
/// archeological items, are interpreted as proleptic Gregorian calendar
/// dates, unless otherwise specified.
///
/// ## Character Repertoire
/// `'0'`-`'9'` of Default Character Repertoire
///
/// ### Notes
/// In the context of a Query with range matching (see PS3.4), the character
/// `-` is allowed, and a trailing SPACE character is allowed for padding.
///
/// ## Length of Value
/// 8 bytes fixed
///
/// ### Notes
/// In the context of a Query (PS3.4) with range matching the length
/// is 18 bytes maximum.
pub static DA: VR = VR {
    ident: "DA",
    name: "Date",
    code: 0x4441,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: true,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: true,
    can_pad_end: true,
};

/// # Decimal String
///
/// ## Definition
/// A string of characters representing either a fixed point number or
/// a floating point number. A fixed point number shall contain only the
/// characters 0-9 with an optional leading `+` or `-` and an optional
/// `.` to mark the decimal point. A floating point number shall be
/// conveyed as defined in ANSI X3.9, with an `E` or `e` to indicate the start
/// of the exponent. Decimal Strings may be padded with leading or trailing
/// spaces. Embedded spaces are not allowed.
///
/// ### Notes
/// Data Elements with multiple values using this VR may not be properly
/// encoded if Explicit-VR transfer syntax is used and the VL of this
/// attribute exceeds 65534 bytes.
///
/// ## Character Repertoire
/// `'0'`-`'9'`, `+`, `-`, `E`, `e`, `.` and the SPACE character
/// of Default Character Repertoire
///
/// ## Length of Value
/// 16 bytes maximum
pub static DS: VR = VR {
    ident: "DS",
    name: "Decimal String",
    code: 0x4453,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: true,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: true,
    can_pad_end: true,
};

/// # Date Time
///
/// ## Definition
/// A concatenated date-time character string in the format:
///
/// YYYYMMDDHHMMSS.FFFFFF&ZZXX
///
/// The components of this string, from left to right, are
///
/// - YYYY = Year,
/// - MM = Month,
/// - DD = Day,
/// - HH = Hour (range "00" - "23"),
/// - MM = Minute (range "00" - "59"),
/// - SS = Second (range "00" - "60").
///
/// FFFFFF = Fractional Second contains a fractional part of a second as
/// small as 1 millionth of a second (range "000000" - "999999").
///
/// &ZZXX is an optional suffix for offset from Coordinated Universal
/// Time (UTC), where
///
/// - & = `+` or `-`, and
/// - ZZ = Hours, and
/// - XX = Minutes of offset.
///
/// The year, month, and day shall be interpreted as a date of the
/// Gregorian calendar system.
///
/// A 24-hour clock is used. Midnight shall be represented by only
/// "0000" since "2400" would violate the hour range.
///
/// The Fractional Second component, if present, shall contain 1 to 6 digits.
/// If Fractional Second is unspecified the preceding `.` shall not be
/// included. The offset suffix, if present, shall contain 4 digits. The
/// string may be padded with trailing SPACE characters. Leading and embedded
/// spaces are not allowed.
///
/// A component that is omitted from the string is termed a null component.
/// Trailing null components of Date Time indicate that the value is not
/// precise to the precision of those components. The YYYY component shall
/// not be null. Non-trailing null components are prohibited. The optional
/// suffix is not considered as a component.
///
/// A Date Time value without the optional suffix is interpreted to be in
/// the local time zone of the application creating the Data Element, unless
/// explicitly specified by the Timezone Offset From UTC (0008,0201).
///
/// UTC offsets are calculated as "local time minus UTC". The offset for a
/// Date Time value in UTC shall be +0000.
///
/// ### Notes
/// 1. The range of the offset is -1200 to +1400. The offset for United States
/// Eastern Standard Time is -0500.
/// The offset for Japan Standard Time is +0900.
/// 2. The RFC 2822 use of -0000 as an offset to indicate local time is
/// not allowed.
/// 3. A Date Time value of 195308 means August 1953, not specific to
/// particular day. A Date Time value of 19530827111300.0 means
/// August 27, 1953, 11;13 a.m. accurate to 1/10th second.
/// 4. The Second component may have a value of 60 only for a leap second.
/// 5. The offset may be included regardless of null components; e.g.,
/// 2007-0500 is a legal value.
///
/// ## Character Repertoire
/// `'0'`-`'9'`, `+`, `-`, `.` and the SPACE character of Default
/// Character Repertoire
///
/// ## Length of Value
/// 26 bytes maximum
///
/// ### Notes
/// In the context of a Query with range matching (see PS3.4), the length is
/// 54 bytes maximum.
pub static DT: VR = VR {
    ident: "DT",
    name: "Date Time",
    code: 0x4454,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: true,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: true,
    can_pad_end: true,
};

/// # Floating Point Double
///
/// ## Definition
/// Double precision binary floating point number represented in
/// IEEE 754:1985 32-bit Floating Point Number Format.
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// 8 bytes fixed
pub static FD: VR = VR {
    ident: "FD",
    name: "Floating Point Double",
    code: 0x4644,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Floating Point Single
///
/// ## Definition
/// Single precision binary floating point number represented in
/// IEEE 754:1985 32-bit Floating Point Number Format.
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// 4 bytes fixed
pub static FL: VR = VR {
    ident: "FL",
    name: "Floating Point Single",
    code: 0x464C,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Integer String
///
/// ## Definition
/// A string of characters representing an Integer in base-10 (decimal),
/// shall contain only the characters 0 - 9, with an optional leading `+`
/// or `-`. It may be padded with leading and/or trailing spaces. Embedded
/// spaces are not allowed.
///
/// The integer, n, represented shall be in the range: -2^31 <= n <= 2^31 - 1.
///
/// ## Character Repertoire
/// `'0'`-`'9'`, `+`, `-` and the SPACE character of Default Character
/// Repertoire
///
/// ## Length of Value
/// 12 bytes maximum
pub static IS: VR = VR {
    ident: "IS",
    name: "Integer String",
    code: 0x4953,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: true,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: true,
    can_pad_end: true,
};

/// # Long String
///
/// ## Definition
/// A character string that may be padded with leading and/or trailing spaces.
/// The character code 5CH (the BACKSLASH `\` in ISO-IR 6) shall not be
/// present, as it is used as the delimiter between values in multiple valued
/// data elements. The string shall not have Control Characters except for ESC.
///
/// ## Character Repertoire
/// Default Character Repertoire and/or as defined by (0008,0005) excluding
/// character code 5CH (the BACKSLASH `\` in ISO-IR 6), and all Control
/// Characters except ESC when used for ISO 2022 escape sequences.
///
/// # #Length of Value
/// 64 chars maximum (see Note in Part 5, Ch 6.2)
pub static LO: VR = VR {
    ident: "LO",
    name: "Long String",
    code: 0x4C4F,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: true,
    decode_text_with_replaced_cs: true,
    allows_backslash_text_value: false,
    can_pad_front: true,
    can_pad_end: true,
};

/// # Long Text
///
/// ## Definition
/// A character string that may contain one or more paragraphs.
/// It may contain the Graphic Character set and the Control Characters,
/// CR, LF, FF, and ESC. It may be padded with trailing spaces, which may
/// be ignored, but leading spaces are considered to be significant. Data
/// Elements with this VR shall not be multi-valued and therefore character
/// code 5CH (the BACKSLASH `\` in ISO-IR 6) may be used.
///
/// ## Character Repertoire
/// Default Character Repertoire and/or as defined by (0008,0005) excluding
/// Control Characters except TAB, LF, FF, CR (and ESC when used for
/// ISO 2022 escape sequences).
///
/// ## Length of Value
/// 10240 chars maximum (see Note in Part 5, Ch 6.2)
pub static LT: VR = VR {
    ident: "LT",
    name: "Long Text",
    code: 0x4C54,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: true,
    decode_text_with_replaced_cs: true,
    allows_backslash_text_value: true,
    can_pad_front: false,
    can_pad_end: true,
};

/// # Other Byte
///
/// ## Definition
/// An octet-stream where the encoding of the contents is specified by the
/// negotiated Transfer Syntax. OB is a VR that is insensitive to byte
/// ordering (see Part 5, Ch 7.3). The octet-stream shall be padded with a
/// single trailing NULL byte value (00H) when necessary to achieve even
/// length.
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// see Transfer Syntax definition
pub static OB: VR = VR {
    ident: "OB",
    name: "Other Byte",
    code: 0x4F42,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: true,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: true,
};

/// # Other Double
///
/// ## Definition
/// A stream of 64-bit IEEE 754:1985 floating point words. OD is a VR that
/// requires byte swapping within each 64-bit word when changing byte ordering
/// (see Part 5, Ch 7.3).
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// (2^32) - 8 bytes maximum
pub static OD: VR = VR {
    ident: "OD",
    name: "Other Double",
    code: 0x4F44,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: true,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Other Float
///
/// ## Definition
/// A stream of 32-bit IEEE 754:1985 floating point words. OF is a VR that
/// requires byte swapping within each 32-bit word when changing byte ordering
/// (see Part 5, Ch 7.3).
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// (2^32) - 4 bytes maximum
pub static OF: VR = VR {
    ident: "OF",
    name: "Other Float",
    code: 0x4F46,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: true,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Other Long
///
/// ## Definition
/// A stream of 32-bit words where the encoding of the contents is specified
/// by the negotiated Transfer Syntax. OL is a VR that requires byte swapping
/// within each word when changing byte ordering (see Part 5, Ch 7.3).
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// see Transfer Syntax definiton
pub static OL: VR = VR {
    ident: "OL",
    name: "Other Long",
    code: 0x4F4C,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: true,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Other 64-bit Very Long
///
/// ## Definition
/// A stream of 64-bit words where the encoding of the contents is specified
/// by the negotiated Transfer Syntax. OV is a VR that requires byte swapping
/// within each word when changing byte ordering (see Part 5, Ch 7.3).
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// see Transfer Syntax definition
pub static OV: VR = VR {
    ident: "OV",
    name: "Other Very Long",
    code: 0x4F56,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: true,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Other Word
///
/// ## Definition
/// A stream of 16-bit words where the encoding of the contents is specified
/// by the negotiated Transfer Syntax. OW is a VR that requires byte swapping
/// within each word when changing byte ordering (see Part 5, Ch 7.3).
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// see Transfer Syntax definition
pub static OW: VR = VR {
    ident: "OW",
    name: "Other Word",
    code: 0x4F57,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: true,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Person Name
///
/// ## Definition
/// A character string encoded using a 5 component convention. The character
/// code 5CH (the BACKSLASH `\` in ISO-IR 6) shall not be present, as it is
/// used as the delimiter between values in multiple valued data elements.
/// The string may be padded with trailing spaces. For human use, the five
/// components in their order of occurrence are: family name complex, given
/// name complex, middle name, name prefix, name suffix.
///
/// Any of the five components may be an empty string. The component delimiter
/// shall be the caret `^` character (5EH). Delimiters are required for
/// interior null components. Trailing null components and their delimiters
/// may be omitted. Multiple entries are permitted in each component and are
/// encoded as natural text strings, in the format preferred by the
/// named person.
///
/// For veterinary use, the first two of the five components in their order of
/// occurrence are: responsible party family name or responsible organization
/// name, patient name. The remaining components are not used and shall not
/// be present.
///
/// This group of five components is referred to as a Person Name
/// component group.
///
/// For the purpose of writing names in ideographic characters and in phonetic
/// characters, up to 3 groups of components (see Part 5, Annexes H, I and J)
/// may be used. The delimiter for component groups shall be the equals
/// character `=` (3DH). The three component groups of components in their
/// order of occurrence are: an alphabetic representation, an ideographic
/// representation, and a phonetic representation.
///
/// Any component group may be absent, including the first component group.
/// In this case, the person name may start with one or more `=` delimiters.
/// Delimiters are required for interior null component groups. Trailing null
/// component groups and their delimiters may be omitted.
///
/// ### Notes
/// HL7 prohibits leading spaces within a component; DICOM allows leading and
/// trailing spaces and considers them insignificant.
///
/// ## Character Repertoire
/// Default Character Repertoire and/or as defined by (0008,0005) excluding
/// character code 5CH (the BACKSLASH `\` in ISO-IR 6) and all Control
/// Characters except ESC when used for ISO 2022 escape sequences.
///
/// ## Length of Value
/// 64 chars maximum per component group (see Note in Part 5, Ch 6.2)
pub static PN: VR = VR {
    ident: "PN",
    name: "Person Name",
    code: 0x504E,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: true,
    decode_text_with_replaced_cs: true,
    allows_backslash_text_value: false,
    can_pad_front: true,
    can_pad_end: true,
};

/// # Short String
///
/// ## Definition
/// A character string that may be padded with leading and/or trailing spaces.
/// The character code 05CH (the BACKSLASH `\` in ISO-IR 6) shall not be
/// present, as it is used as the delimiter between values for multiple data
/// elements. The string shall not have Control Characters except ESC.
///
/// ## Character Repertoire
/// Default Character Repertoire and/or as defined by (0008,0005) excluding
/// character code 5CH (the BACKSLASH `\` in ISO-IR 6) and all Control
/// Characters except ESC when used for ISO 2022 escape sequences.
///
/// ## Length of Value
/// 16 chars maximum (see Note in Part 5, Ch 6.2)
pub static SH: VR = VR {
    ident: "SH",
    name: "Short String",
    code: 0x5348,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: true,
    decode_text_with_replaced_cs: true,
    allows_backslash_text_value: false,
    can_pad_front: true,
    can_pad_end: true,
};

/// # Signed Long
///
/// ## Definition
/// Signed binary integer 32 bits long in 2's complement form.
///
/// Represents an integer, n, in the range:
/// (-2^31) <= n <= (2^31 - 1).
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// 4 bytes fixed
pub static SL: VR = VR {
    ident: "SL",
    name: "Signed Long",
    code: 0x534C,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Sequence of Items
///
/// ## Definition
/// Value is a Sequence of zero or more Items, as defined in Part 5, Ch 7.5.
///
/// ## Character Repertoire
/// N/A (see Part 5, Ch 7.5)
///
/// ## Length of Value
/// N/A (see Part 5, Ch 7.5)
pub static SQ: VR = VR {
    ident: "SQ",
    name: "Sequence of Items",
    code: 0x5351,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: true,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Signed Short
///
/// ## Definition
/// Signed binary integer 16 bits long in 2's complement form.
///
/// Represents an integer n in the range: (-2^15) <= n <= (2^15 - 1).
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// 4 bytes fixed
pub static SS: VR = VR {
    ident: "SS",
    name: "Signed Short",
    code: 0x5353,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Short Text
///
/// ## Definition
/// A character string that may contain one or more paragraphs. It may contain
/// the Graphic Character set and the Control Characters, CR, LF, FF, and ESC.
/// It may be padded with trailing spaces, which may be ignored, but leading
/// spaces are considered to be significant. Data Elements with this VR shall
/// not be multi-valued and therefore character code 5CH (the BACKSLASH `\`
/// in ISO-IR 6) may be used.
///
/// ## Character Repertoire
/// Default Character Repertoire and/or as defined by (0008,0005) excluding
/// Control Characters except TAB, LF, FF, CR (and ESC when used for ISO 2022
/// escape sequences).
///
/// ## Length of Value
/// 1024 chars maximum (see Note in Part 5, Ch 6.2)
pub static ST: VR = VR {
    ident: "ST",
    name: "Short Text",
    code: 0x5354,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: true,
    decode_text_with_replaced_cs: true,
    allows_backslash_text_value: true,
    can_pad_front: false,
    can_pad_end: true,
};

/// # Signed 64-bit Very Long
///
/// ## Description
/// Signed binary integer 64 bits long. Represents an integer n in the range:
/// -2^63 <= n <= 2^63-1.
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// 8 bytes fixed
pub static SV: VR = VR {
    ident: "SV",
    name: "Signed Very Long",
    code: 0x5356,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: true,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Time
///
/// ## Definition
/// A string of characters of the format HHMMSS.FFFFFF; where
///
/// - HH contains hours (range "00" - "23"),
/// - MM contains minutes (range "00" - "59"),
/// - SS contains seconds (range "00" - "60"), and
/// - FFFFFF contains a fractional part of a second as small
/// as 1 millionth of a second (range "000000" - "999999").
///
/// A 24-hour clock is used. Midnight shall be represented by only "0000"
/// since "2400" would violate the hour range. The string may be padded with
/// trailing spaces. Leading and embedded spaces are not allowed.
///
/// One or more of the components MM, SS, or FFFFFF may be unspecified as long
/// as every component to the right of an unspecified component is also
/// unspecified, which indicates that the value is not precise to the
/// precision of those unspecified components.
///
/// The FFFFFF component, if present, shall contain 1 to 6 digits. If FFFFFF
/// is unspecified the preceding "." shall not be included.
///
/// ### Examples
/// 1. "070907.0705 " represents a time of 7 hours, 9 minutes and 7.0705
/// seconds.
/// 2. "1010" represents a time of 10 hours, and 10 minutes.
/// 3. "021 " is an invalid value.
///
/// ### Notes
/// 1. The ACR-NEMA Standard 300 (predecessor to DICOM) supported a string of
/// characters of the format HH:MM:SS.frac for this VR. Use of this format is
/// not compliant.
/// 2. See also DT VR in this table.
/// 3. The SS component may have a value of 60 only for a leap second.
///
/// ## Character Repertoire
/// `'0'`-`'9'`, `.` and the SPACE character of Default Character Repertoire
///
/// In the context of a Query with range matching (see PS3.4), the character
/// `-` is allowed.
///
/// ## Length of Value
/// 14 bytes maximum. In the context of a Query with range matching
/// (see PS3.4), the length is 28 bytes maximum.
pub static TM: VR = VR {
    ident: "TM",
    name: "Time",
    code: 0x544D,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: true,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: true,
    can_pad_end: true,
};

/// # Unlimited Characters
///
/// ## Definition
/// A character string that may be of unlimited length that may be padded with
/// trailing spaces. The character code 5CH (the BACKSLASH `\` in ISO-IR 6)
/// shall not be present, as it is used as the delimiter between values in
/// multiple valued data elements. The string shall not have Control
/// Characters except for ESC.
///
/// ## Character Repertoire
/// Default Character Repertoire and/or as defined by (0008,0005) excluding
/// character code 5CH (the BACKSLASH `\` in ISO-IR 6), and all Control
/// Characters except ESC when used for ISO 2022 escape sequences.
///
/// ## Length of Value
/// 2^32 - 2 bytes maximum
pub static UC: VR = VR {
    ident: "UC",
    name: "Unlimited Characters",
    code: 0x5543,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: true,
    is_character_string: true,
    decode_text_with_replaced_cs: true,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: true,
};

/// # Unique Identifier
///
/// ## Description
/// A character string containing a UID that is used to uniquely identify a
/// wide variety of items. The UID is a series of numeric components separated
/// by the period `.` character. If a Value Field containing one or more UIDs
/// is an odd number of bytes in length, the Value Field shall be padded with
/// a single trailing NULL (00H) character to ensure that the Value Field is an
/// even number of bytes in length. See Part 5, Ch 9 and Part 5, Annex B for a
/// complete specification and examples.
///
/// ## Character Repertoire
/// `'0'`-`'9'`, `.` of Default Character Repertoire
///
/// ## Length of Value
/// 64 bytes maximum
pub static UI: VR = VR {
    ident: "UI",
    name: "Unique Identifier",
    code: 0x5549,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: true,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: true,
};

/// # Unsigned Long
///
/// ## Definition
/// Unsigned binary integer 32 bits long. Represents an integer n in
/// the range: 0 <= n < 2^32
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// 4 bytes fixed
pub static UL: VR = VR {
    ident: "UL",
    name: "Unsigned Long",
    code: 0x554C,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Unknown
///
/// ## Definition
/// An octet-stream where the encoding of the contents is Unknown
/// (see Part 5, Ch 6.2.2).
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// Any length valid for any of the other DICOM Value Representations
pub static UN: VR = VR {
    ident: "UN",
    name: "Unknown",
    code: 0x554E,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: true,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Universal Resource Identifier / Universal Resource Locator
///
/// ## Definition
/// A string of characters that identifies a URI or a URL as defined in
/// [RFC3986]. Leading spaces are not allowed. Trailing spaces shall be
/// ignored. Data Elements with this VR shall not be multi-valued.
///
/// ### Notes
/// Both absolute and relative URIs are permitted. If the URI is relative,
/// then it is relative to the base URI of the object within which it is
/// contained.
///
/// ## Character Repertoire
/// The subset of the Default Character Repertoire required for the URI as
/// defined in IETF RFC3986 Section 2, plus the space (20H) character
/// permitted only as trailing padding.
///
/// Characters outside the permitted character set must be "percent encoded".
///
/// ### Notes
/// The Backslash (5CH) character is among those disallowed in URIs.
///
/// ## Length of Value
/// 2^32 - 2 bytes maximum
///
/// [RFC3986]: https://datatracker.ietf.org/doc/html/rfc3986
pub static UR: VR = VR {
    ident: "UR",
    name: "Universal Resource Identifier / Universal Resource Locator",
    code: 0x5552,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: true,
    is_character_string: true,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: true,
    can_pad_end: true,
};

/// # Unsigned Short
///
/// ## Definition
/// Unsigned binary integer 16 bits long. Represents integer n in the range:
/// 0 <= n < 2^16
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// 2 bytes fixed
pub static US: VR = VR {
    ident: "US",
    name: "Unsigned Short",
    code: 0x5553,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Unlimited Text
///
/// ## Definition
/// A character string that may contain one or more paragraphs. It may contain
/// the Graphic Character set and the Control Characters, CR, LF, FF, and ESC.
/// It may be padded with trailing spaces, which may be ignored, but leading
/// spaces are considered to be significant. Data Elements with this VR shall
/// not be multi-valued and therefore character code 5CH (the BACKSLASH `\`
/// in ISO-IR 6) may be used.
///
/// ## Character Repertoire
/// Default Character Repertoire and/or as defined by (0008,0005) excluding
/// Control Characters except TAB, LF, FF, CR (and ESC when used for ISO 2022
/// escape sequences).
///
/// ## Length of Value
/// 2^32 - 2 bytes maximum
pub static UT: VR = VR {
    ident: "UT",
    name: "Unlimited Text",
    code: 0x5554,
    padding: SPACE_PADDING,
    has_explicit_2byte_pad: true,
    is_character_string: true,
    decode_text_with_replaced_cs: true,
    allows_backslash_text_value: true,
    can_pad_front: false,
    can_pad_end: true,
};

/// # Unsigned 64-bit Very Long
///
/// ## Definition
/// Unsigned binary integer 64 bits long. Represents an integer n in the range:
/// 0 <= n < 264.
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// 8 bytes fixed
pub static UV: VR = VR {
    ident: "UV",
    name: "Unsigned Very Long",
    code: 0x5556,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: true,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};

/// # Invalid
///
/// ## Definition
/// This is a virtual/sentinel value for an invalid/unknown VR parsed from
/// an explicit VR transfer syntax. The value should be interpreted as an
/// octet stream similar to `UN` however unlike `UN` this does not indicate
/// it has a 2-byte padding after the VR, as most non-byte-oriented VRs
/// (UN, OB, OW, OD, etc.) do not have 2-byte padding
///
/// ## Character Repertoire
/// N/A
///
/// ## Length of Value
/// Any length valid for any of the other DICOM Value Representations
pub static INVALID_VR: VR = VR {
    ident: "??",
    name: "Invalid",
    code: 0x0000,
    padding: NULL_PADDING,
    has_explicit_2byte_pad: false,
    is_character_string: false,
    decode_text_with_replaced_cs: false,
    allows_backslash_text_value: false,
    can_pad_front: false,
    can_pad_end: false,
};
