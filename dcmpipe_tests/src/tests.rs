use crate::mock::MockDicomStream;
use dcmpipe_dict::dict::dicom_elements as tags;
use dcmpipe_dict::dict::lookup::{TAG_BY_VALUE, TS_BY_UID};
use dcmpipe_dict::dict::uids;
use dcmpipe_lib::core::dcmobject::DicomObject;
use dcmpipe_lib::core::dcmparser::{
    ParseState, Parser, ParserBuilder, DICOM_PREFIX, DICOM_PREFIX_LENGTH, FILE_PREAMBLE_LENGTH,
};
use dcmpipe_lib::core::dcmreader::parse_stream;
use dcmpipe_lib::core::tagstop::TagStop;
use dcmpipe_lib::defn::vr;
use std::fs::File;
use std::io::{Error, Read};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

#[test]
fn test_good_preamble() {
    let mut parser: Parser<MockDicomStream> = MockDicomStream::standard_dicom_preamble();

    // reads the preamble, prefix, and first element (mock has no first element)
    let _ = parser
        .next()
        .expect("Should be able to iterate a valid dicom stream");

    let is_dcm: bool = is_standard_dcm_file(&parser);
    assert_eq!(is_dcm, true);
}

#[test]
fn test_nonzero_preamble() {
    let mut parser: Parser<MockDicomStream> = MockDicomStream::nonzero_preamble();

    // reads the preamble, prefix, and first element (mock has no first element)
    let _ = parser
        .next()
        .expect("Should be able to iterate a valid dicom stream with non-standard preamble");

    let is_dcm: bool = is_standard_dcm_file(&parser);
    assert_eq!(is_dcm, false);
}

#[test]
#[should_panic(expected = "Invalid DICOM Prefix")]
fn test_bad_dicom_prefix() {
    let mut parser: Parser<MockDicomStream> = MockDicomStream::invalid_dicom_prefix();

    // reads the preamble, prefix, and first element
    let _ = parser
        .next()
        .expect("An invalid prefix should return Some(Err)")
        .expect("Invalid prefix should be Err, this expect should cause panic");
}

#[test]
#[should_panic(expected = "failed to fill whole buffer")]
fn test_failure_to_read_preamble() {
    let mut parser: Parser<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble_diff_startpos_and_short_stream();

    // reads the preamble, prefix, and first element
    let _first_elem = parser
        .next()
        .expect("Value should be Some Error")
        .expect("This should fail to read preamble due to not enough data");

    // should record zero bytes read since the first attempt to read into buffer should fail to fill
    let start_pos: u64 = parser.get_bytes_read();
    assert_eq!(start_pos, 0);
}

#[test]
fn test_parser_state() -> Result<(), Error> {
    let tagstop: u32 = tags::PixelData.tag;
    let file: File =
        File::open("./fixtures/gdcm/gdcmConformanceTests/D_CLUNIE_CT1_IVRLE_BigEndian.dcm")?;
    let mut parser: Parser<File> = ParserBuilder::new(file)
        .tagstop(TagStop::BeforeTag(tagstop))
        .build();

    assert_eq!(parser.get_parser_state(), ParseState::DetectState);

    let _first_elem = parser.next().expect("First element should be Some")?;

    assert_eq!(parser.get_parser_state(), ParseState::FileMeta);

    while let Some(_) = parser.next() {
        // read elements while iterator returns non-None
    }

    assert_eq!(parser.get_parser_state(), ParseState::Element);

    // Ability to read dicom elements after FileMetaInformation
    // means that we interpret the transfer syntax properly, as
    // this test file uses implicit VR (FMI is encoded as explicit)

    // subsequent item should not advance reading elements
    let next_elem = parser.next();
    assert!(next_elem.is_none());

    // the iterator state should be just after having parsed the stop tag
    let stopped_at_tag = parser
        .get_partial_tag()
        .expect("Iteration should have stopped after reading the PixelData tag");
    assert_eq!(tagstop, stopped_at_tag);

    Ok(())
}

#[test]
pub fn test_dicom_object() -> Result<(), Error> {
    let file: File =
        File::open("./fixtures/gdcm/gdcmConformanceTests/D_CLUNIE_CT1_IVRLE_BigEndian.dcm")?;
    let mut dicom_iter: Parser<File> = ParserBuilder::new(file).build();

    let dcmobj: DicomObject = parse_stream(&mut dicom_iter)?;
    let sop_class_uid: &DicomObject = dcmobj
        .get_object(tags::SOPClassUID.tag)
        .expect("Should have SOP Class UID");

    let element = sop_class_uid
        .as_element()
        .expect("Element should exist for SOP Class UID");

    assert_eq!(
        element.parse_string_with_vr(&vr::UI)?,
        uids::CTImageStorage.uid
    );

    Ok(())
}

#[test]
pub fn test_parse_all_dicom_files_with_std_dict() -> Result<(), Error> {
    parse_all_dicom_files(true)
}

#[test]
pub fn test_parse_all_dicom_files_without_std_dict() -> Result<(), Error> {
    parse_all_dicom_files(false)
}

/// Parses through all dicom files in the `fixtures` folder. The `use_std_dict` argument specifies
/// whether the standard dicom dictionary should be reigstered with the parser.
fn parse_all_dicom_files(use_std_dict: bool) -> Result<(), Error> {
    let mut errors: usize = 0;
    for mut pair in get_all_dicom_file_parsers(use_std_dict)? {
        while let Some(element) = pair.1.next() {
            if let Err(e) = element {
                errors += 1;
                eprintln!(
                    "Error parsing DICOM:\n\t{}\n\t{}",
                    pair.0.to_str().expect("Should get path"),
                    e
                );
            }
        }
    }

    assert_eq!(errors, 0);

    Ok(())
}

/// Creates parsers for every dicom file in the `fixutres` folder. The `use_std_dict` argument
/// specifies whether the standard dicom dictionary should be registered with the parser.
/// See the `readme.md` in this project for information on obtaining test fixtures.
fn get_all_dicom_file_parsers(use_std_dict: bool) -> Result<Vec<(PathBuf, Parser<File>)>, Error> {
    let fixtures_path: &Path = Path::new("./fixtures");
    assert!(
        fixtures_path.is_dir(),
        "The fixtures are missing and need downloaded"
    );

    let dirwalker: WalkDir = WalkDir::new(fixtures_path);
    let mut parsers: Vec<(PathBuf, Parser<File>)> = Vec::new();
    for entry_res in dirwalker.into_iter() {
        let entry: DirEntry = entry_res?;
        let path: &Path = entry.path();

        let filename = path
            .file_name()
            .expect("Should be able to get filename")
            .to_str()
            .expect("Should be able to stringify filename");

        // Only attempt to parse .DCM files or files without any extension
        if (!filename.ends_with(".dcm") && filename.contains(".")) || filename.eq("README") {
            continue;
        }

        let file: File = File::open(path)?;
        if file.metadata()?.is_file() {
            let mut parser: ParserBuilder<File> = ParserBuilder::new(file);
            if use_std_dict {
                parser = parser.tag_by_value(&TAG_BY_VALUE).ts_by_uid(&TS_BY_UID);
            }

            let parser: Parser<File> = parser.build();
            parsers.push((path.to_path_buf(), parser));
        }
    }

    Ok(parsers)
}

/// Checks that the first 132 bytes are 128 0's followed by 'DICM'.
/// DICOM files do not need to abide by this format to be valid, but it's standard.
fn is_standard_dcm_file<StreamType>(stream: &Parser<StreamType>) -> bool
where
    StreamType: Read,
{
    for i in 0..FILE_PREAMBLE_LENGTH {
        if stream.get_file_preamble()[i] != 0 {
            return false;
        }
    }
    for i in 0..DICOM_PREFIX_LENGTH {
        if stream.get_dicom_prefix()[i] != DICOM_PREFIX[i] {
            return false;
        }
    }
    true
}
