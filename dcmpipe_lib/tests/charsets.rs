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

mod common;

#[cfg(feature = "stddicom")]
mod charsets_tests {
    use std::convert::{TryFrom, TryInto};

    use dcmpipe_lib::{
        core::{
            charset::CSRef,
            dcmobject::DicomRoot,
            read::{ParseResult, ParserBuilder},
            values::ElementWithVr,
        },
        dict::{
            stdlookup::STANDARD_DICOM_DICTIONARY,
            tags::{DirectoryRecordSequence, PatientsName, SpecificCharacterSet},
        },
    };

    use crate::common::common_stddicom::{fixture, parse_file};

    /// This DICOMDIR has sequences with nested elements that change charsets
    #[test]
    fn test_parse_nested_charset_values() -> ParseResult<()> {
        let fixture = fixture("dclunie/charsettests/DICOMDIR")?;
        let mut parser = ParserBuilder::default().build(fixture, &STANDARD_DICOM_DICTIONARY);

        let dcmroot: DicomRoot =
            DicomRoot::parse(&mut parser)?.expect("Failed to parse DICOM elements");

        test_nested_charset(
            &dcmroot,
            1,
            CSRef::of(encoding_rs::ISO_8859_8),
            "ISO_IR 138",
            "שרון^דבורה",
        )?;
        test_nested_charset(
            &dcmroot,
            5,
            CSRef::of(encoding_rs::ISO_8859_5),
            "ISO_IR 144",
            "Люкceмбypг",
        )?;
        test_nested_charset(
            &dcmroot,
            9,
            CSRef::of(encoding_rs::ISO_8859_6),
            "ISO_IR 127",
            "قباني^لنزار",
        )?;
        test_nested_charset(
            &dcmroot,
            13,
            CSRef::of(encoding_rs::WINDOWS_1252),
            "ISO_IR 100",
            "Äneas^Rüdiger",
        )?;
        test_nested_charset(
            &dcmroot,
            17,
            CSRef::of(encoding_rs::WINDOWS_1252),
            "ISO_IR 100",
            "Buc^Jérôme",
        )?;
        test_nested_charset(
            &dcmroot,
            21,
            CSRef::of(encoding_rs::ISO_8859_7),
            "ISO_IR 126",
            "Διονυσιος",
        )?;
        test_nested_charset(
            &dcmroot,
            25,
            CSRef::of(encoding_rs::GB18030),
            "GB18030",
            "Wang^XiaoDong=王^小东=",
        )?;
        test_nested_charset(
            &dcmroot,
            29,
            CSRef::of(encoding_rs::UTF_8),
            "ISO_IR 192",
            "Wang^XiaoDong=王^小東=",
        )?;
        test_nested_charset(
            &dcmroot,
            33,
            CSRef::of(encoding_rs::WINDOWS_1252),
            "ISO 2022 IR 149",
            "Hong^Gildong=\u{1b}$)Cûó^\u{1b}$)CÑÎÔ×=\u{1b}$)CÈ«^\u{1b}$)C±æµ¿",
        )?;

        Ok(())
    }

    fn test_nested_charset(
        dcmroot: &DicomRoot,
        item_num: usize,
        cs: CSRef,
        scs: &str,
        pn: &str,
    ) -> ParseResult<()> {
        let item = dcmroot
            .get_child_by_tag(&DirectoryRecordSequence)
            .expect("Should have DirectoryRecordSequence")
            .get_item_by_index(item_num)
            .expect("Should have item");

        let item_scs_values: Vec<String> = item
            .get_child_by_tag(&SpecificCharacterSet)
            .expect("Item should have SCS")
            .element()
            .try_into()?;

        let item_scs = item_scs_values
            .into_iter()
            .find(|cs_entry| !cs_entry.is_empty())
            .expect("Should have at least one value for SCS");

        assert_eq!(scs, item_scs);

        let item_pn = item
            .get_child_by_tag(&PatientsName)
            .expect("Item should have PN")
            .element();

        assert_eq!(cs.name(), item_pn.cs().name());

        let item_pn_value = String::try_from(&ElementWithVr::of(item_pn))?;

        assert_eq!(pn, item_pn_value);

        Ok(())
    }

    #[test]
    fn test_scs_arab() -> ParseResult<()> {
        test_scs_file(
            true,
            "dclunie/charsettests/SCSARAB",
            CSRef::of(encoding_rs::ISO_8859_6),
            "ISO_IR 127",
            "قباني^لنزار",
        )
    }

    #[test]
    fn test_scs_french() -> ParseResult<()> {
        test_scs_file(
            true,
            "dclunie/charsettests/SCSFREN",
            CSRef::of(encoding_rs::WINDOWS_1252),
            "ISO_IR 100",
            "Buc^Jérôme",
        )
    }

    #[test]
    fn test_scs_german() -> ParseResult<()> {
        test_scs_file(
            true,
            "dclunie/charsettests/SCSGERM",
            CSRef::of(encoding_rs::WINDOWS_1252),
            "ISO_IR 100",
            "Äneas^Rüdiger",
        )
    }

    #[test]
    fn test_scs_greek() -> ParseResult<()> {
        test_scs_file(
            true,
            "dclunie/charsettests/SCSGREEK",
            CSRef::of(encoding_rs::ISO_8859_7),
            "ISO_IR 126",
            "Διονυσιος",
        )
    }

    #[test]
    fn test_scs_h31() -> ParseResult<()> {
        test_scs_file(
            true,
            "dclunie/charsettests/SCSH31",
            CSRef::of(encoding_rs::ISO_2022_JP),
            "ISO 2022 IR 87",
            "Yamada^Tarou=山田^太郎=やまだ^たろう",
        )
    }

    /// This uses multiple charsets, ISO-IR-13 and ISO-IR-87.
    #[test]
    #[ignore]
    fn test_scs_h32() -> ParseResult<()> {
        test_scs_file(
            true,
            "dclunie/charsettests/SCSH32",
            CSRef::of(encoding_rs::SHIFT_JIS),
            "ISO 2022 IR 13",
            "",
        )
    }

    #[test]
    fn test_scs_hebrew() -> ParseResult<()> {
        test_scs_file(
            true,
            "dclunie/charsettests/SCSHBRW",
            CSRef::of(encoding_rs::ISO_8859_8),
            "ISO_IR 138",
            "שרון^דבורה",
        )
    }

    #[test]
    fn test_scs_i12() -> ParseResult<()> {
        test_scs_file(
            true,
            "dclunie/charsettests/SCSI2",
            CSRef::of(encoding_rs::WINDOWS_1252),
            "ISO 2022 IR 149",
            "Hong^Gildong=\u{1b}$)Cûó^\u{1b}$)CÑÎÔ×=\u{1b}$)CÈ«^\u{1b}$)C±æµ¿",
        )
    }

    #[test]
    fn test_scs_russian() -> ParseResult<()> {
        test_scs_file(
            true,
            "dclunie/charsettests/SCSRUSS",
            CSRef::of(encoding_rs::ISO_8859_5),
            "ISO_IR 144",
            "Люкceмбypг",
        )
    }

    #[test]
    fn test_scs_x1() -> ParseResult<()> {
        test_scs_file(
            true,
            "dclunie/charsettests/SCSX1",
            CSRef::of(encoding_rs::UTF_8),
            "ISO_IR 192",
            "Wang^XiaoDong=王^小東=",
        )
    }

    #[test]
    fn test_scs_x2() -> ParseResult<()> {
        test_scs_file(
            true,
            "dclunie/charsettests/SCSX2",
            CSRef::of(encoding_rs::GB18030),
            "GB18030",
            "Wang^XiaoDong=王^小东=",
        )
    }

    fn test_scs_file(
        with_std: bool,
        path: &str,
        cs: CSRef,
        scs: &str,
        pn: &str,
    ) -> ParseResult<()> {
        let dcmroot: DicomRoot = parse_file(path, with_std)?;

        assert_eq!(cs.name(), dcmroot.cs().name());

        let scs_elem = dcmroot
            .get_child_by_tag(&SpecificCharacterSet)
            .expect("Should have SCS")
            .element();

        // value can be multiple and sometimes contains empties -- match logic from the parser
        let scs_val: String = Vec::<String>::try_from(scs_elem)?
            .into_iter()
            .find(|cs_entry| !cs_entry.is_empty())
            .expect("Should have at least one value for SCS");
        assert_eq!(scs, scs_val);

        let pn_elem = dcmroot
            .get_child_by_tag(&PatientsName)
            .expect("Should have PN")
            .element();

        let pn_val = String::try_from(&ElementWithVr::of(pn_elem))?;
        assert_eq!(pn, pn_val);

        Ok(())
    }
}
