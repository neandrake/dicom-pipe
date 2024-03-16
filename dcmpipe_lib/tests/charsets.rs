use dcmpipe_lib;

use std::convert::{TryFrom, TryInto};
use std::fs::File;

use dcmpipe_lib::core::charset::CSRef;
use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dcmobject::{DicomObject, DicomRoot};
use dcmpipe_lib::core::read::{ParseResult, Parser, ParserBuilder};
use dcmpipe_lib::dict::stdlookup::STANDARD_DICOM_DICTIONARY;
use dcmpipe_lib::dict::tags;

mod common;

use common::{fixture, parse_file};

/// This DICOMDIR has sequences with nested elements that change charsets
#[test]
fn test_parse_nested_charset_values() -> ParseResult<()> {
    let file = fixture("dclunie/charsettests/DICOMDIR")?;
    let mut parser: Parser<'_, File> = ParserBuilder::default()
        .dictionary(&STANDARD_DICOM_DICTIONARY)
        .build(file);

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
    let item: &DicomObject = dcmroot
        .get_child_by_tag(tags::DirectoryRecordSequence.tag)
        .expect("Should have DirectoryRecordSequence")
        .get_item_by_index(item_num)
        .expect("Should have item");

    let item_scs_values: Vec<String> = item
        .get_child_by_tag(tags::SpecificCharacterSet.tag)
        .expect("Item should have SCS")
        .element()
        .try_into()?;

    let item_scs: String = item_scs_values
        .into_iter()
        .filter(|cs_entry: &String| !cs_entry.is_empty())
        .nth(0)
        .expect("Should have at least one value for SCS");

    assert_eq!(scs, item_scs);

    let item_pn: &DicomElement = item
        .get_child_by_tag(tags::PatientsName.tag)
        .expect("Item should have PN")
        .element();

    assert_eq!(cs.name(), item_pn.cs().name());

    let item_pn_value: String = String::try_from(item_pn)?;

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

fn test_scs_file(with_std: bool, path: &str, cs: CSRef, scs: &str, pn: &str) -> ParseResult<()> {
    let dcmroot: DicomRoot = parse_file(path, with_std)?;

    assert_eq!(cs.name(), dcmroot.cs().name());

    let scs_elem: &DicomElement = dcmroot
        .get_child_by_tag(tags::SpecificCharacterSet.tag)
        .expect("Should have SCS")
        .element();

    // value can be multiple and sometimes contains empties -- match logic from the parser
    let scs_val: String = Vec::<String>::try_from(scs_elem)?
        .into_iter()
        .filter(|cs_entry: &String| !cs_entry.is_empty())
        .nth(0)
        .expect("Should have at least one value for SCS");
    assert_eq!(scs, scs_val);

    let pn_elem: &DicomElement = dcmroot
        .get_child_by_tag(tags::PatientsName.tag)
        .expect("Should have PN")
        .element();

    let pn_val: String = String::try_from(pn_elem)?;
    assert_eq!(pn, pn_val);

    Ok(())
}
