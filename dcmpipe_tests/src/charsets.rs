use std::convert::{TryFrom, TryInto};
use std::fs::File;

use encoding::all;

use dcmpipe_dict::dict::stdlookup::STANDARD_DICOM_DICTIONARY;
use dcmpipe_dict::dict::tags;
use dcmpipe_lib::core::charset::CSRef;
use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dcmobject::{DicomNode, DicomObject, DicomRoot};
use dcmpipe_lib::core::read::util::parse_into_object;
use dcmpipe_lib::core::read::{Parser, ParserBuilder, Result};

use crate::parse_file;

/// This DICOMDIR has sequences with nested elements that change charsets
#[test]
fn test_parse_nested_charset_values() -> Result<()> {
    let path_str: &str = "./fixtures/dclunie/charsettests/DICOMDIR";
    let file: File = File::open(path_str)?;
    let mut parser: Parser<'_, File> = ParserBuilder::default()
        .dictionary(&STANDARD_DICOM_DICTIONARY)
        .build(file);

    let dcmroot: DicomRoot<'_> =
        parse_into_object(&mut parser)?.expect("Failed to parse DICOM elements");

    test_nested_charset(
        &dcmroot,
        1,
        CSRef::of(all::ISO_8859_8),
        "ISO_IR 138",
        "שרון^דבורה",
    )?;
    test_nested_charset(
        &dcmroot,
        5,
        CSRef::of(all::ISO_8859_5),
        "ISO_IR 144",
        "Люкceмбypг",
    )?;
    test_nested_charset(
        &dcmroot,
        9,
        CSRef::of(all::ISO_8859_6),
        "ISO_IR 127",
        "قباني^لنزار",
    )?;
    test_nested_charset(
        &dcmroot,
        13,
        CSRef::of(all::WINDOWS_1252),
        "ISO_IR 100",
        "Äneas^Rüdiger",
    )?;
    test_nested_charset(
        &dcmroot,
        17,
        CSRef::of(all::WINDOWS_1252),
        "ISO_IR 100",
        "Buc^Jérôme",
    )?;
    test_nested_charset(
        &dcmroot,
        21,
        CSRef::of(all::ISO_8859_7),
        "ISO_IR 126",
        "Διονυσιος",
    )?;
    test_nested_charset(
        &dcmroot,
        25,
        CSRef::of(all::GB18030),
        "GB18030",
        "Wang^XiaoDong=王^小东=",
    )?;
    test_nested_charset(
        &dcmroot,
        29,
        CSRef::of(all::UTF_8),
        "ISO_IR 192",
        "Wang^XiaoDong=王^小東=",
    )?;
    test_nested_charset(
        &dcmroot,
        33,
        CSRef::of(all::WINDOWS_1252),
        "ISO 2022 IR 149",
        "Hong^Gildong=\u{1b}$)Cûó^\u{1b}$)CÑÎÔ×=\u{1b}$)CÈ«^\u{1b}$)C±æµ¿",
    )?;

    Ok(())
}

fn test_nested_charset(
    dcmroot: &DicomRoot<'_>,
    item_num: usize,
    cs: CSRef,
    scs: &str,
    pn: &str,
) -> Result<()> {
    let item: &DicomObject = dcmroot
        .get_child_by_tag(tags::DirectoryRecordSequence.tag)
        .expect("Should have DirectoryRecordSequence")
        .get_item_by_index(item_num)
        .expect("Should have item");

    let item_scs_values: Vec<String> = item
        .get_child_by_tag(tags::SpecificCharacterSet.tag)
        .expect("Item should have SCS")
        .get_element()
        .try_into()?;

    let item_scs: String = item_scs_values
        .into_iter()
        .filter(|cs_entry: &String| !cs_entry.is_empty())
        .nth(0)
        .expect("Should have at least one value for SCS");

    assert_eq!(item_scs, scs);

    let item_pn: &DicomElement = item
        .get_child_by_tag(tags::PatientsName.tag)
        .expect("Item should have PN")
        .get_element();

    assert_eq!(item_pn.get_cs().name(), cs.name());

    let item_pn_value: String = String::try_from(item_pn)?;

    assert_eq!(item_pn_value, pn);

    Ok(())
}

#[test]
fn test_scs_arab() -> Result<()> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSARAB",
        CSRef::of(all::ISO_8859_6),
        "ISO_IR 127",
        "قباني^لنزار",
    )
}

#[test]
fn test_scs_french() -> Result<()> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSFREN",
        CSRef::of(all::WINDOWS_1252),
        "ISO_IR 100",
        "Buc^Jérôme",
    )
}

#[test]
fn test_scs_german() -> Result<()> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSGERM",
        CSRef::of(all::WINDOWS_1252),
        "ISO_IR 100",
        "Äneas^Rüdiger",
    )
}

#[test]
fn test_scs_greek() -> Result<()> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSGREEK",
        CSRef::of(all::ISO_8859_7),
        "ISO_IR 126",
        "Διονυσιος",
    )
}

#[test]
fn test_scs_h31() -> Result<()> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSH31",
        CSRef::of(all::ISO_2022_JP),
        "ISO 2022 IR 87",
        "Yamada^Tarou=山田^太郎=やまだ^たろう",
    )
}

#[test]
#[ignore]
fn test_scs_h32() -> Result<()> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSH32",
        CSRef::of(all::ISO_2022_JP),
        "ISO 2022 IR 13",
        "",
    )
}

#[test]
fn test_scs_hebrew() -> Result<()> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSHBRW",
        CSRef::of(all::ISO_8859_8),
        "ISO_IR 138",
        "שרון^דבורה",
    )
}

#[test]
fn test_scs_i12() -> Result<()> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSI2",
        CSRef::of(all::WINDOWS_1252),
        "ISO 2022 IR 149",
        "Hong^Gildong=\u{1b}$)Cûó^\u{1b}$)CÑÎÔ×=\u{1b}$)CÈ«^\u{1b}$)C±æµ¿",
    )
}

#[test]
fn test_scs_russian() -> Result<()> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSRUSS",
        CSRef::of(all::ISO_8859_5),
        "ISO_IR 144",
        "Люкceмбypг",
    )
}

#[test]
fn test_scs_x1() -> Result<()> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSX1",
        CSRef::of(all::UTF_8),
        "ISO_IR 192",
        "Wang^XiaoDong=王^小東=",
    )
}

#[test]
fn test_scs_x2() -> Result<()> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSX2",
        CSRef::of(all::GB18030),
        "GB18030",
        "Wang^XiaoDong=王^小东=",
    )
}

fn test_scs_file(with_std: bool, path: &str, cs: CSRef, scs: &str, pn: &str) -> Result<()> {
    let dcmroot: DicomRoot<'_> = parse_file(path, with_std)?;

    assert_eq!(dcmroot.get_cs().name(), cs.name());

    let scs_elem: &DicomElement = dcmroot
        .get_child_by_tag(tags::SpecificCharacterSet.tag)
        .expect("Should have SCS")
        .get_element();

    // value can be multiple and sometimes contains empties -- match logic from the parser
    let scs_val: String = Vec::<String>::try_from(scs_elem)?
        .into_iter()
        .filter(|cs_entry: &String| !cs_entry.is_empty())
        .nth(0)
        .expect("Should have at least one value for SCS");
    assert_eq!(scs_val, scs);

    let pn_elem: &DicomElement = dcmroot
        .get_child_by_tag(tags::PatientsName.tag)
        .expect("Should have PN")
        .get_element();

    let pn_val: String = String::try_from(pn_elem)?;
    assert_eq!(pn_val, pn);

    Ok(())
}
