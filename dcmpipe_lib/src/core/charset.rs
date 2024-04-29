//! Character Sets

use std::fmt::Debug;
use std::ops::Deref;

use encoding::all::{self, WINDOWS_1252};
use encoding::EncodingRef;

#[derive(Clone, Copy)]
pub struct CSRef {
    encoding: EncodingRef,
}

impl CSRef {
    pub const fn of(encoding: EncodingRef) -> Self {
        CSRef { encoding }
    }
}

impl Deref for CSRef {
    type Target = EncodingRef;

    fn deref(&self) -> &Self::Target {
        &self.encoding
    }
}

impl Debug for CSRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.encoding.name())
    }
}

pub static DEFAULT_CHARACTER_SET: CSRef = CSRef::of(WINDOWS_1252);

/// This is based off `encoding::label::encoding_from_whatwg_label` with a few minor changes
/// - All whitespace, hyphens, and underscores are stripped when doing a lookup
/// - Added `ISO-IR-192` mapping for `UTF-8`
/// See DICOM Part 2 Appendix D.6.2 Support of Character Sets - Character Sets
/// http://dicom.nema.org/medical/dicom/current/output/chtml/part02/sect_D.6.2.html
pub(crate) fn lookup_charset(label: &str) -> Option<CSRef> {
    let label: String = label
        .chars()
        .map(|c| match c {
            'A'..='Z' => (c as u8 + 32) as char,
            ' ' => '-',
            '_' => '-',
            '\n' => '-',
            '\r' => '-',
            '\t' => '-',
            '\x0C' => '-',
            _ => c,
        })
        .collect::<String>()
        .replace('-', "");

    match &label[..] {
        "unicode11utf8" | "utf8" | "isoir192" => Some(CSRef::of(all::UTF_8)),
        "866" | "cp866" | "csibm866" | "ibm866" => Some(CSRef::of(all::IBM866)),
        "csisolatin2" | "iso88592" | "isoir101" | "iso88592:1987" | "l2" | "latin2" => {
            Some(CSRef::of(all::ISO_8859_2))
        }
        "csisolatin3" | "iso88593" | "isoir109" | "iso88593:1988" | "l3" | "latin3" => {
            Some(CSRef::of(all::ISO_8859_3))
        }
        "csisolatin4" | "iso88594" | "isoir110" | "iso88594:1988" | "l4" | "latin4" => {
            Some(CSRef::of(all::ISO_8859_4))
        }
        "csisolatincyrillic" | "cyrillic" | "iso88595" | "isoir144" | "iso88595:1988" => {
            Some(CSRef::of(all::ISO_8859_5))
        }
        "arabic" | "asmo708" | "csiso88596e" | "csiso88596i" | "csisolatinarabic" | "ecma114"
        | "iso88596" | "iso88596e" | "iso88596i" | "isoir127" | "iso88596:1987" => {
            Some(CSRef::of(all::ISO_8859_6))
        }
        "csisolatingreek" | "ecma118" | "elot928" | "greek" | "greek8" | "iso88597"
        | "isoir126" | "iso88597:1987" | "suneugreek" => Some(CSRef::of(all::ISO_8859_7)),
        "csiso88598e" | "csisolatinhebrew" | "hebrew" | "iso88598" | "iso88598e" | "isoir138"
        | "iso88598:1988" | "visual" => Some(CSRef::of(all::ISO_8859_8)),
        "csiso88598i" | "iso88598i" | "logical" => Some(CSRef::of(all::whatwg::ISO_8859_8_I)),
        "csisolatin6" | "iso885910" | "isoir157" | "l6" | "latin6" => {
            Some(CSRef::of(all::ISO_8859_10))
        }
        "iso885913" => Some(CSRef::of(all::ISO_8859_13)),
        "iso885914" => Some(CSRef::of(all::ISO_8859_14)),
        "csisolatin9" | "iso885915" | "l9" => Some(CSRef::of(all::ISO_8859_15)),
        "iso885916" => Some(CSRef::of(all::ISO_8859_16)),
        "cskoi8r" | "koi" | "koi8" | "koi8r" => Some(CSRef::of(all::KOI8_R)),
        "koi8u" => Some(CSRef::of(all::KOI8_U)),
        "csmacintosh" | "mac" | "macintosh" | "xmacroman" => Some(CSRef::of(all::MAC_ROMAN)),
        "dos874" | "iso885911" | "tis620" | "windows874" => Some(CSRef::of(all::WINDOWS_874)),
        "cp1250" | "windows1250" | "xcp1250" => Some(CSRef::of(all::WINDOWS_1250)),
        "cp1251" | "windows1251" | "xcp1251" => Some(CSRef::of(all::WINDOWS_1251)),
        "ansix3.41968" | "ascii" | "cp1252" | "cp819" | "csisolatin1" | "ibm819" | "iso88591"
        | "isoir100" | "iso88591:1987" | "l1" | "latin1" | "usascii" | "windows1252"
        | "xcp1252" => Some(CSRef::of(all::WINDOWS_1252)),
        "cp1253" | "windows1253" | "xcp1253" => Some(CSRef::of(all::WINDOWS_1253)),
        "cp1254" | "csisolatin5" | "iso88599" | "isoir148" | "iso88599:1989" | "l5" | "latin5"
        | "windows1254" | "xcp1254" => Some(CSRef::of(all::WINDOWS_1254)),
        "cp1255" | "windows1255" | "xcp1255" => Some(CSRef::of(all::WINDOWS_1255)),
        "cp1256" | "windows1256" | "xcp1256" => Some(CSRef::of(all::WINDOWS_1256)),
        "cp1257" | "windows1257" | "xcp1257" => Some(CSRef::of(all::WINDOWS_1257)),
        "cp1258" | "windows1258" | "xcp1258" => Some(CSRef::of(all::WINDOWS_1258)),
        "xmaccyrillic" | "xmacukrainian" => Some(CSRef::of(all::MAC_CYRILLIC)),
        "chinese" | "csgb2312" | "csiso58gb231280" | "gb2312" | "gb231280" | "gbk" | "isoir58"
        | "xgbk" => Some(CSRef::of(all::GBK)),
        "gb18030" => Some(CSRef::of(all::GB18030)),
        "big5" | "big5hkscs" | "csbig5" | "xxbig5" => Some(CSRef::of(all::BIG5_2003)),
        "cseucpkdfmtjapanese" | "eucjp" | "xeucjp" => Some(CSRef::of(all::EUC_JP)),
        "isoir13" | "isoir87" | "isoir159" | "iso2022ir13" | "iso2022ir87" | "iso2022ir159"
        | "csiso2022jp" | "iso2022jp" => Some(CSRef::of(all::ISO_2022_JP)),
        "csshiftjis" | "mskanji" | "shiftjis" | "sjis" | "windows31j" | "xsjis" => {
            Some(CSRef::of(all::WINDOWS_31J))
        }
        "cseuckr" | "csksc56011987" | "euckr" | "isoir149" | "korean" | "ksc56011987"
        | "ksc56011989" | "ksc5601" | "windows949" => Some(CSRef::of(all::WINDOWS_949)),
        "csiso2022kr" | "hzgb2312" | "iso2022kr" | "iso2022cn" | "iso2022cnext" => {
            Some(CSRef::of(all::whatwg::REPLACEMENT))
        }
        "utf16be" => Some(CSRef::of(all::UTF_16BE)),
        "utf16" | "utf16le" => Some(CSRef::of(all::UTF_16LE)),
        "xuserdefined" => Some(CSRef::of(all::whatwg::X_USER_DEFINED)),
        _ => None,
    }
}
