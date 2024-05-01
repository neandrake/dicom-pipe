/*
   Copyright 2024 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! Character Sets

use std::fmt::Debug;

use encoding_rs::{Encoding, WINDOWS_1252};
use thiserror::Error;

#[derive(Error, Debug)]
/// Errors that can occur during writing of a DICOM dataset.
pub enum CSError {
    #[error("failed encoding data to string using: {encoder}")]
    EncodingError { encoder: &'static str },

    #[error("failed decoding string to data using: {encoder}")]
    DecodingError { encoder: &'static str },
}

#[derive(Clone, Copy)]
pub struct CSRef {
    encoding: &'static Encoding,
}

impl CSRef {
    #[must_use]
    pub const fn of(encoding: &'static Encoding) -> Self {
        CSRef { encoding }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.encoding.name()
    }

    /// Encodes the given text using the encoding.
    ///
    /// # Errors
    /// Errors during encoding.
    pub fn encode(&self, text: &str) -> Result<Vec<u8>, CSError> {
        Ok(self.encoding.encode(text).0.into_owned())
    }

    /// Decodes the given text using the encoding.
    ///
    /// # Errors
    /// Errors during decoding.
    pub fn decode(&self, data: &[u8]) -> Result<String, CSError> {
        self.encoding
            .decode_without_bom_handling_and_without_replacement(data)
            .map(|s| s.to_string())
            .ok_or_else(|| CSError::DecodingError {
                encoder: self.encoding.name(),
            })
    }

    /// This is based off `encoding::label::encoding_from_whatwg_label` with a few minor changes
    /// - All whitespace, hyphens, and underscores are stripped when doing a lookup
    /// - Added `ISO-IR-192` mapping for `UTF-8`
    /// See DICOM Part 2 Appendix D.6.2 Support of Character Sets - Character Sets
    /// <http://dicom.nema.org/medical/dicom/current/output/chtml/part02/sect_D.6.2.html>
    #[must_use]
    pub fn lookup_charset(label: &str) -> Option<CSRef> {
        let label: String = label
            .chars()
            .map(|c| match c {
                'A'..='Z' => char::from(u8::try_from(c).unwrap_or_default() + 32),
                ' ' | '_' | '\n' | '\r' | '\t' | '\x0C' => '-',
                _ => c,
            })
            .collect::<String>()
            .replace('-', "");

        match &label[..] {
            "unicode11utf8" | "utf8" | "isoir192" => Some(CSRef::of(encoding_rs::UTF_8)),
            "866" | "cp866" | "csibm866" | "ibm866" => Some(CSRef::of(encoding_rs::IBM866)),
            "csisolatin2" | "iso88592" | "isoir101" | "iso88592:1987" | "l2" | "latin2" => {
                Some(CSRef::of(encoding_rs::ISO_8859_2))
            }
            "csisolatin3" | "iso88593" | "isoir109" | "iso88593:1988" | "l3" | "latin3" => {
                Some(CSRef::of(encoding_rs::ISO_8859_3))
            }
            "csisolatin4" | "iso88594" | "isoir110" | "iso88594:1988" | "l4" | "latin4" => {
                Some(CSRef::of(encoding_rs::ISO_8859_4))
            }
            "csisolatincyrillic" | "cyrillic" | "iso88595" | "isoir144" | "iso88595:1988" => {
                Some(CSRef::of(encoding_rs::ISO_8859_5))
            }
            "arabic" | "asmo708" | "csiso88596e" | "csiso88596i" | "csisolatinarabic"
            | "ecma114" | "iso88596" | "iso88596e" | "iso88596i" | "isoir127" | "iso88596:1987" => {
                Some(CSRef::of(encoding_rs::ISO_8859_6))
            }
            "csisolatingreek" | "ecma118" | "elot928" | "greek" | "greek8" | "iso88597"
            | "isoir126" | "iso88597:1987" | "suneugreek" => {
                Some(CSRef::of(encoding_rs::ISO_8859_7))
            }
            "csiso88598e" | "csisolatinhebrew" | "hebrew" | "iso88598" | "iso88598e"
            | "isoir138" | "iso88598:1988" | "visual" => Some(CSRef::of(encoding_rs::ISO_8859_8)),
            "csiso88598i" | "iso88598i" | "logical" => Some(CSRef::of(encoding_rs::ISO_8859_8_I)),
            "csisolatin6" | "iso885910" | "isoir157" | "l6" | "latin6" => {
                Some(CSRef::of(encoding_rs::ISO_8859_10))
            }
            "iso885913" => Some(CSRef::of(encoding_rs::ISO_8859_13)),
            "iso885914" => Some(CSRef::of(encoding_rs::ISO_8859_14)),
            "csisolatin9" | "iso885915" | "l9" => Some(CSRef::of(encoding_rs::ISO_8859_15)),
            "iso885916" => Some(CSRef::of(encoding_rs::ISO_8859_16)),
            "cskoi8r" | "koi" | "koi8" | "koi8r" => Some(CSRef::of(encoding_rs::KOI8_R)),
            "koi8u" => Some(CSRef::of(encoding_rs::KOI8_U)),
            "csmacintosh" | "mac" | "macintosh" | "xmacroman" => {
                Some(CSRef::of(encoding_rs::MACINTOSH))
            }
            "dos874" | "iso885911" | "tis620" | "windows874" => {
                Some(CSRef::of(encoding_rs::WINDOWS_874))
            }
            "cp1250" | "windows1250" | "xcp1250" => Some(CSRef::of(encoding_rs::WINDOWS_1250)),
            "cp1251" | "windows1251" | "xcp1251" => Some(CSRef::of(encoding_rs::WINDOWS_1251)),
            "ansix3.41968" | "ascii" | "cp1252" | "cp819" | "csisolatin1" | "ibm819"
            | "iso88591" | "isoir100" | "iso88591:1987" | "l1" | "latin1" | "usascii"
            | "windows1252" | "xcp1252" => Some(CSRef::of(encoding_rs::WINDOWS_1252)),
            "cp1253" | "windows1253" | "xcp1253" => Some(CSRef::of(encoding_rs::WINDOWS_1253)),
            "cp1254" | "csisolatin5" | "iso88599" | "isoir148" | "iso88599:1989" | "l5"
            | "latin5" | "windows1254" | "xcp1254" => Some(CSRef::of(encoding_rs::WINDOWS_1254)),
            "cp1255" | "windows1255" | "xcp1255" => Some(CSRef::of(encoding_rs::WINDOWS_1255)),
            "cp1256" | "windows1256" | "xcp1256" => Some(CSRef::of(encoding_rs::WINDOWS_1256)),
            "cp1257" | "windows1257" | "xcp1257" => Some(CSRef::of(encoding_rs::WINDOWS_1257)),
            "cp1258" | "windows1258" | "xcp1258" => Some(CSRef::of(encoding_rs::WINDOWS_1258)),
            "xmaccyrillic" | "xmacukrainian" => Some(CSRef::of(encoding_rs::X_MAC_CYRILLIC)),
            "chinese" | "csgb2312" | "csiso58gb231280" | "gb2312" | "gb231280" | "gbk"
            | "isoir58" | "xgbk" => Some(CSRef::of(encoding_rs::GBK)),
            "gb18030" => Some(CSRef::of(encoding_rs::GB18030)),
            "big5" | "big5hkscs" | "csbig5" | "xxbig5" => Some(CSRef::of(encoding_rs::BIG5)),
            "cseucpkdfmtjapanese" | "eucjp" | "xeucjp" => Some(CSRef::of(encoding_rs::EUC_JP)),
            "isoir13" | "iso2022ir13" => Some(CSRef::of(encoding_rs::SHIFT_JIS)),
            "isoir87" | "iso2022ir87" | "isoir159" | "iso2022ir159" | "csiso2022jp"
            | "iso2022jp" => Some(CSRef::of(encoding_rs::ISO_2022_JP)),
            "csshiftjis" | "mskanji" | "shiftjis" | "sjis" | "windows31j" | "xsjis" => {
                Some(CSRef::of(encoding_rs::SHIFT_JIS))
            }
            "cseuckr" | "csksc56011987" | "euckr" | "isoir149" | "korean" | "ksc56011987"
            | "ksc56011989" | "ksc5601" | "windows949" => Some(CSRef::of(encoding_rs::EUC_KR)),
            "csiso2022kr" | "hzgb2312" | "iso2022kr" | "iso2022cn" | "iso2022cnext" => {
                Some(CSRef::of(encoding_rs::REPLACEMENT))
            }
            "utf16be" => Some(CSRef::of(encoding_rs::UTF_16BE)),
            "utf16" | "utf16le" => Some(CSRef::of(encoding_rs::UTF_16LE)),
            "xuserdefined" => Some(CSRef::of(encoding_rs::X_USER_DEFINED)),
            _ => None,
        }
    }
}

impl Default for CSRef {
    /// The default character set for DICOM.
    fn default() -> Self {
        CSRef::of(WINDOWS_1252)
    }
}

impl Debug for CSRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.encoding.name())
    }
}
