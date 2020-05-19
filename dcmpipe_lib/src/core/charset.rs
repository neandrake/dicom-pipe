use encoding::all::{self, WINDOWS_1252};
use encoding::EncodingRef;

pub type CSRef = EncodingRef;

pub static DEFAULT_CHARACTER_SET: CSRef = WINDOWS_1252 as CSRef;

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
        .replace("-", "");

    match &label[..] {
        "unicode11utf8" | "utf8" | "isoir192" => Some(all::UTF_8 as CSRef),
        "866" | "cp866" | "csibm866" | "ibm866" => Some(all::IBM866 as CSRef),
        "csisolatin2" | "iso88592" | "isoir101" | "iso88592:1987" | "l2" | "latin2" => {
            Some(all::ISO_8859_2 as CSRef)
        }
        "csisolatin3" | "iso88593" | "isoir109" | "iso88593:1988" | "l3" | "latin3" => {
            Some(all::ISO_8859_3 as CSRef)
        }
        "csisolatin4" | "iso88594" | "isoir110" | "iso88594:1988" | "l4" | "latin4" => {
            Some(all::ISO_8859_4 as CSRef)
        }
        "csisolatincyrillic" | "cyrillic" | "iso88595" | "isoir144" | "iso88595:1988" => {
            Some(all::ISO_8859_5 as CSRef)
        }
        "arabic" | "asmo708" | "csiso88596e" | "csiso88596i" | "csisolatinarabic" | "ecma114"
        | "iso88596" | "iso88596e" | "iso88596i" | "isoir127" | "iso88596:1987" => {
            Some(all::ISO_8859_6 as CSRef)
        }
        "csisolatingreek" | "ecma118" | "elot928" | "greek" | "greek8" | "iso88597"
        | "isoir126" | "iso88597:1987" | "suneugreek" => Some(all::ISO_8859_7 as CSRef),
        "csiso88598e" | "csisolatinhebrew" | "hebrew" | "iso88598" | "iso88598e" | "isoir138"
        | "iso88598:1988" | "visual" => Some(all::ISO_8859_8 as CSRef),
        "csiso88598i" | "iso88598i" | "logical" => Some(all::whatwg::ISO_8859_8_I as CSRef),
        "csisolatin6" | "iso885910" | "isoir157" | "l6" | "latin6" => {
            Some(all::ISO_8859_10 as CSRef)
        }
        "iso885913" => Some(all::ISO_8859_13 as CSRef),
        "iso885914" => Some(all::ISO_8859_14 as CSRef),
        "csisolatin9" | "iso885915" | "l9" => Some(all::ISO_8859_15 as CSRef),
        "iso885916" => Some(all::ISO_8859_16 as CSRef),
        "cskoi8r" | "koi" | "koi8" | "koi8r" => Some(all::KOI8_R as CSRef),
        "koi8u" => Some(all::KOI8_U as CSRef),
        "csmacintosh" | "mac" | "macintosh" | "xmacroman" => Some(all::MAC_ROMAN as CSRef),
        "dos874" | "iso885911" | "tis620" | "windows874" => Some(all::WINDOWS_874 as CSRef),
        "cp1250" | "windows1250" | "xcp1250" => Some(all::WINDOWS_1250 as CSRef),
        "cp1251" | "windows1251" | "xcp1251" => Some(all::WINDOWS_1251 as CSRef),
        "ansix3.41968" | "ascii" | "cp1252" | "cp819" | "csisolatin1" | "ibm819" | "iso88591"
        | "isoir100" | "iso88591:1987" | "l1" | "latin1" | "usascii" | "windows1252"
        | "xcp1252" => Some(all::WINDOWS_1252 as CSRef),
        "cp1253" | "windows1253" | "xcp1253" => Some(all::WINDOWS_1253 as CSRef),
        "cp1254" | "csisolatin5" | "iso88599" | "isoir148" | "iso88599:1989" | "l5" | "latin5"
        | "windows1254" | "xcp1254" => Some(all::WINDOWS_1254 as CSRef),
        "cp1255" | "windows1255" | "xcp1255" => Some(all::WINDOWS_1255 as CSRef),
        "cp1256" | "windows1256" | "xcp1256" => Some(all::WINDOWS_1256 as CSRef),
        "cp1257" | "windows1257" | "xcp1257" => Some(all::WINDOWS_1257 as CSRef),
        "cp1258" | "windows1258" | "xcp1258" => Some(all::WINDOWS_1258 as CSRef),
        "xmaccyrillic" | "xmacukrainian" => Some(all::MAC_CYRILLIC as CSRef),
        "chinese" | "csgb2312" | "csiso58gb231280" | "gb2312" | "gb231280" | "gbk" | "isoir58"
        | "xgbk" => Some(all::GBK as CSRef),
        "gb18030" => Some(all::GB18030 as CSRef),
        "big5" | "big5hkscs" | "csbig5" | "xxbig5" => Some(all::BIG5_2003 as CSRef),
        "cseucpkdfmtjapanese" | "eucjp" | "xeucjp" => Some(all::EUC_JP as CSRef),
        "isoir13" | "isoir87" | "isoir159" | "iso2022ir13" | "iso2022ir87" | "iso2022ir159"
        | "csiso2022jp" | "iso2022jp" => Some(all::ISO_2022_JP as CSRef),
        "csshiftjis" | "mskanji" | "shiftjis" | "sjis" | "windows31j" | "xsjis" => {
            Some(all::WINDOWS_31J as CSRef)
        }
        "cseuckr" | "csksc56011987" | "euckr" | "isoir149" | "korean" | "ksc56011987"
        | "ksc56011989" | "ksc5601" | "windows949" => Some(all::WINDOWS_949 as CSRef),
        "csiso2022kr" | "hzgb2312" | "iso2022kr" | "iso2022cn" | "iso2022cnext" => {
            Some(all::whatwg::REPLACEMENT as CSRef)
        }
        "utf16be" => Some(all::UTF_16BE as CSRef),
        "utf16" | "utf16le" => Some(all::UTF_16LE as CSRef),
        "xuserdefined" => Some(all::whatwg::X_USER_DEFINED as CSRef),
        _ => None,
    }
}
