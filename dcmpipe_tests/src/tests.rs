use crate::mock::MockDicomStream;
use dcmpipe_dict::dict::dicom_elements as tags;
use dcmpipe_dict::dict::file_meta_elements as fme;
use dcmpipe_dict::dict::lookup::{TAG_BY_VALUE, TS_BY_UID};
use dcmpipe_dict::dict::transfer_syntaxes as ts;
use dcmpipe_dict::dict::uids;
use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dcmobject::DicomObject;
use dcmpipe_lib::core::dcmparser::{
    ParseState, Parser, ParserBuilder, DICOM_PREFIX, DICOM_PREFIX_LENGTH, FILE_PREAMBLE_LENGTH,
};
use dcmpipe_lib::core::dcmreader::parse_stream;
use dcmpipe_lib::core::tagstop::TagStop;
use dcmpipe_lib::defn::vl::ValueLength;
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
#[should_panic(expected = "This should fail: Invalid dicom prefix")]
fn test_bad_dicom_prefix() {
    let mut parser: Parser<MockDicomStream> = MockDicomStream::invalid_dicom_prefix();

    // reads the preamble, prefix, and first element
    let _ = parser
        .next()
        .expect("Should have returned Some(Err)")
        .expect("This should fail: Invalid dicom prefix");
}

#[test]
#[should_panic(expected = "This should fail: Failure to read preamble due to not enough data")]
fn test_failure_to_read_preamble() {
    let mut parser: Parser<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble_diff_startpos_and_short_stream();

    // reads the preamble, prefix, and first element
    let _first_elem = parser
        .next()
        .expect("Should have returned Some(Err)")
        .expect("This should fail: Failure to read preamble due to not enough data");

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

    let first_elem: DicomElement = parser.next().expect("First element should be Some")?;

    assert_eq!(first_elem.tag, fme::FileMetaInformationGroupLength.tag);

    assert_eq!(parser.get_parser_state(), ParseState::FileMeta);

    while let Some(_) = parser.next() {
        // read elements while iterator returns non-None
    }

    assert_eq!(parser.get_parser_state(), ParseState::Element);

    // Ability to read dicom elements after FileMetaInformation
    // means that we interpret the transfer syntax properly, as
    // this test file uses implicit VR (FMI is encoded as explicit)

    // subsequent item should not advance reading elements
    let next_elem: Option<Result<DicomElement, Error>> = parser.next();
    assert!(next_elem.is_none());

    // the iterator state should be just after having parsed the stop tag
    let stopped_at_tag: u32 = parser
        .get_partial_tag()
        .expect("Iteration should have stopped after reading the PixelData tag");
    assert_eq!(tagstop, stopped_at_tag);

    Ok(())
}

#[test]
pub fn test_dicom_object() -> Result<(), Error> {
    let file: File =
        File::open("./fixtures/gdcm/gdcmConformanceTests/D_CLUNIE_CT1_IVRLE_BigEndian.dcm")?;
    let mut parser: Parser<File> = ParserBuilder::new(file)
        .tagstop(TagStop::BeforeTag(tags::PixelData.tag))
        .build();

    let dcmobj: DicomObject = parse_stream(&mut parser)?;
    let sop_class_uid: &DicomObject = dcmobj
        .get_object(tags::SOPClassUID.tag)
        .expect("Should have SOP Class UID");

    let element: &DicomElement = sop_class_uid
        .as_element()
        .expect("Element should exist for SOP Class UID");

    assert_eq!(
        element.parse_string_with_vr(&vr::UI)?,
        uids::CTImageStorage.uid
    );

    Ok(())
}

/// In this file the `ReferencedStudySequence` and `ReferencedPatientSequence` tags are both `SQ`
/// elements defined with `UndefinedLength` and contain no data - the first element they have as
/// contents are `SequenceDelimitationItem` which ends the sequence.
#[test]
pub fn test_empty_seq_undefined_length() -> Result<(), Error> {
    let (_parser, dcmobj) =
        parse_file("./fixtures/gdcm/gdcmConformanceTests/DX_GE_FALCON_SNOWY-VOI.dcm")?;

    let rss_obj: &DicomObject = dcmobj
        .get_object(tags::ReferencedStudySequence.tag)
        .expect("Should be able to parse ReferencedStudySequence");

    let rss_elem: &DicomElement = rss_obj
        .as_element()
        .expect("Should be able to access rss as element");

    assert_eq!(rss_elem.vl, ValueLength::UndefinedLength);

    // does contain a child item which is the delimitation item
    assert_eq!(rss_obj.get_object_count(), 1);

    let sdi_pair: (&u32, &DicomObject) = rss_obj
        .iter()
        .next()
        .expect("Should be able to get single child item");

    assert_eq!(*sdi_pair.0, tags::SequenceDelimitationItem.tag);
    let sdi_elem: &DicomElement = sdi_pair
        .1
        .as_element()
        .expect("Should be able to access as element");
    assert_eq!(sdi_elem.tag, tags::SequenceDelimitationItem.tag);

    Ok(())
}

/// Private tags with UN VR and UndefinedLength should be parsed as sequences
#[test]
pub fn test_private_tag_un_sq() -> Result<(), Error> {
    let (_parser, dcmobj) = parse_file("./fixtures/gdcm/gdcmConformanceTests/Enhanced_MR_Image_Storage_Illegal_CP246_corrected.dcm")?;

    let private_un_seq_obj: &DicomObject = dcmobj
        .get_object(tags::SharedFunctionalGroupsSequence.tag)
        .expect("Fixture should have this this tag")
        .iter()
        .next()
        .expect("This sequence should have 1 sequence item")
        .1
        .get_object(0x2005_140E)
        .expect("This sequence should have private element as child");

    let private_un_seq_elem: &DicomElement = private_un_seq_obj
        .as_element()
        .expect("Should be able to get private un seq as element");

    assert_eq!(private_un_seq_elem.vr, &vr::UN);
    assert_eq!(private_un_seq_elem.vl, ValueLength::UndefinedLength);
    assert_eq!(private_un_seq_elem.is_seq_like(), true);
    assert_eq!(private_un_seq_elem.get_data().len(), 0);
    assert_eq!(private_un_seq_obj.get_object_count(), 1);

    let child_obj: &DicomObject = private_un_seq_obj
        .iter()
        .next()
        .expect("Private sequence should have one item")
        .1;

    // The first item has 28 elements
    assert_eq!(child_obj.get_object_count(), 28);

    let sopuid: &DicomElement = child_obj
        .get_object(tags::SOPClassUID.tag)
        .expect("Should have SOPClassUID child element")
        .as_element()
        .expect("Should be able to get child element as element");

    assert_eq!(sopuid.parse_string()?, uids::MRImageStorage.uid);

    Ok(())
}

/// `SequenceDelimitationItem`, `Item`, and `ItemDelimitationItem` are always encoded as IVRLE
/// despite what the transfer syntax is.
#[test]
pub fn test_seq_switch_to_ivrle() -> Result<(), Error> {
    let (parser, dcmobj) =
        parse_file("./fixtures/gdcm/gdcmConformanceTests/D_CLUNIE_CT1_IVRLE_BigEndian.dcm")?;

    assert_eq!(parser.get_ts(), &ts::ExplicitVRBigEndian);

    let sis_obj: &DicomObject = dcmobj
        .get_object(tags::SourceImageSequence.tag)
        .expect("Should have Source Image Sequence");

    assert_eq!(sis_obj.get_object_count(), 1);

    let sis_elem: &DicomElement = sis_obj
        .as_element()
        .expect("Should be able to get element for Source Image Sequence");

    assert_eq!(sis_elem.get_ts(), &ts::ImplicitVRLittleEndian);

    let item_obj: &DicomObject = sis_obj
        .iter()
        .next()
        .expect("Should be able to get child object").1;

    assert_eq!(item_obj.get_object_count(), 2);

    let item_elem: &DicomElement = item_obj
        .as_element()
        .expect("Should be able to get child element");

    assert_eq!(item_elem.tag, tags::Item.tag);
    assert_eq!(item_elem.get_ts(), &ts::ImplicitVRLittleEndian);

    for (_tag, inner_obj) in item_obj.iter() {
        let elem: &DicomElement = inner_obj.as_element().expect("Get inner object element");
        // This assertion seems wrong (should be EVRBE) based on Part 5, Section 7.5 --
        // However, the Data Set within the Value Field of the Data Element Item (FFFE,E000) shall
        // be encoded according to the rules conveyed by the Transfer Syntax.
        assert_eq!(elem.get_ts(), &ts::ImplicitVRLittleEndian);
    }

    Ok(())
}

/// This file has no preamble or file meta - should parse as the DICOM default IVRLE
#[test]
pub fn test_missing_preamble() -> Result<(), Error> {
    let file: File = File::open("./fixtures/gdcm/gdcmConformanceTests/OT-PAL-8-face.dcm")?;
    let mut parser: Parser<File> = ParserBuilder::new(file)
        .ts_by_uid(&TS_BY_UID)
        .tag_by_value(&TAG_BY_VALUE)
        .build();

    let first_elem: DicomElement = parser
        .next()
        .expect("First element should be parsable")?;

    // first tag is a group length tag
    assert_eq!(first_elem.tag, 0x0008_0000);
    // should immediately jump past preamble/prefix, group length, and file meta
    assert_eq!(parser.get_parser_state(), ParseState::Element);
    assert_eq!(parser.get_ts(), &ts::ImplicitVRLittleEndian);

    // parser doesn't differentiate between no preamble/prefix vs. what's read
    // so in this scenario both of these should be initialized to zeros
    for i in 0..FILE_PREAMBLE_LENGTH {
        assert_eq!(parser.get_file_preamble()[i], 0);
    }
    for i in 0..DICOM_PREFIX_LENGTH {
        assert_eq!(parser.get_dicom_prefix()[i], 0);
    }

    // parse the rest of the stream into an object
    let dcm_obj: DicomObject = parse_stream(&mut parser)?;
    assert_eq!(dcm_obj.get_object_count(), 32);
    Ok(())
}

#[test]
#[ignore]
pub fn test_parse_all_dicom_files_with_std_dict() -> Result<(), Error> {
    let errors: usize = parse_all_dicom_files(true)?;
    // currently 12 files fail to parse -- when testing for regressions flip the comment
    //assert_eq!(errors, 12);
    assert_eq!(errors, 0);
    Ok(())
}

#[test]
#[ignore]
pub fn test_parse_all_dicom_files_without_std_dict() -> Result<(), Error> {
    let errors: usize = parse_all_dicom_files(false)?;
    // currently 14 files fail to parse -- when testing for regressions flip the comment
    //assert_eq!(errors, 14);
    assert_eq!(errors, 0);
    Ok(())
}

/// Parses the given file into a `DicomObject`
fn parse_file(path: &str) -> Result<(Parser<File>, DicomObject), Error> {
    let file: File = File::open(path)?;
    let mut parser: Parser<File> = ParserBuilder::new(file)
        .tag_by_value(&TAG_BY_VALUE)
        .ts_by_uid(&TS_BY_UID)
        .build();
    let dcmobj: DicomObject = parse_stream(&mut parser)?;
    Ok((parser, dcmobj))
}

/// Parses through all dicom files in the `fixtures` folder. The `use_std_dict` argument specifies
/// whether the standard dicom dictionary should be reigstered with the parser.
fn parse_all_dicom_files(use_std_dict: bool) -> Result<usize, Error> {
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
    Ok(errors)
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
