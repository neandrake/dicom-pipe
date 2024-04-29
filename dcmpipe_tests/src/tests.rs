use crate::{parse_file, is_standard_dcm_file, parse_all_dicom_files};
use crate::mock::MockDicomStream;
use dcmpipe_dict::dict::dicom_elements as tags;
use dcmpipe_dict::dict::file_meta_elements as fme;
use dcmpipe_dict::dict::lookup::{TAG_BY_VALUE, TS_BY_UID};
use dcmpipe_dict::dict::transfer_syntaxes as ts;
use dcmpipe_dict::dict::uids;
use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dcmobject::{DicomNode, DicomObject, DicomRoot};
use dcmpipe_lib::core::dcmparser::{ParseState, Parser, ParserBuilder};
use dcmpipe_lib::core::dcmreader::parse_stream;
use dcmpipe_lib::core::tagstop::TagStop;
use dcmpipe_lib::defn::vl::ValueLength;
use dcmpipe_lib::defn::vr;
use std::fs::File;
use std::io::{Error, ErrorKind};

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
fn test_bad_dicom_prefix_parser() {
    let mut parser: Parser<MockDicomStream> = MockDicomStream::invalid_dicom_prefix();

    // reads the preamble, prefix, and first element
    let _ = parser
        .next()
        .expect("Should have returned Some(Err)")
        // This should fail: Invalid dicom prefix
        .unwrap();
}

/// Test that reading the parser into a DICOM object properly propagates errors
#[test]
#[should_panic(expected = "Invalid DICOM Prefix")]
fn test_bad_dicom_prefix_reader() {
    let mut parser: Parser<MockDicomStream> = MockDicomStream::invalid_dicom_prefix();
    parse_stream(&mut parser).unwrap();
}

#[test]
#[should_panic(expected = "failed to fill whole buffer")]
fn test_failure_to_read_preamble() {
    let mut parser: Parser<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble_diff_startpos_and_short_stream();

    // reads the preamble, prefix, and first element
    let _first_elem = parser
        .next()
        .expect("Should have returned Some(Err)")
        // This should fail: Failure to read preamble due to not enough data
        .unwrap();

    // should record zero bytes read since the first attempt to read into buffer should fail to fill
    let start_pos: u64 = parser.get_bytes_read();
    assert_eq!(start_pos, 0);
}

#[test]
fn test_parser_state_with_std() -> Result<(), Error> {
    test_parser_state(true)
}

#[test]
fn test_parser_state_without_std() -> Result<(), Error> {
    test_parser_state(false)
}

fn test_parser_state(with_std: bool) -> Result<(), Error> {
    let tagstop: u32 = tags::PixelData.tag;
    let file: File =
        File::open("./fixtures/gdcm/gdcmConformanceTests/D_CLUNIE_CT1_IVRLE_BigEndian.dcm")?;
    let mut parser: ParserBuilder<File> =
        ParserBuilder::new(file).tagstop(TagStop::BeforeTag(tagstop));
    if with_std {
        parser = parser.ts_by_uid(&TS_BY_UID).tag_by_value(&TAG_BY_VALUE);
    }
    let mut parser: Parser<File> = parser.build();

    assert_eq!(parser.get_parser_state(), ParseState::DetectState);

    let first_elem: DicomElement = parser.next().expect("First element should be Some")?;

    assert_eq!(first_elem.tag, fme::FileMetaInformationGroupLength.tag);

    assert_eq!(parser.get_parser_state(), ParseState::FileMeta);

    while let Some(_) = parser.next() {
        // read through the entire stream
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
fn test_dicom_object_with_std() -> Result<(), Error> {
    test_dicom_object(true)
}

#[test]
fn test_dicom_object_without_std() -> Result<(), Error> {
    test_dicom_object(false)
}

fn test_dicom_object(with_std: bool) -> Result<(), Error> {
    let file: File =
        File::open("./fixtures/gdcm/gdcmConformanceTests/D_CLUNIE_CT1_IVRLE_BigEndian.dcm")?;
    let mut parser: ParserBuilder<File> =
        ParserBuilder::new(file).tagstop(TagStop::BeforeTag(tags::PixelData.tag));
    if with_std {
        parser = parser.ts_by_uid(&TS_BY_UID).tag_by_value(&TAG_BY_VALUE);
    }
    let mut parser: Parser<File> = parser.build();

    let dcmroot: DicomRoot = parse_stream(&mut parser)?;
    let sop_class_uid: &DicomObject = dcmroot
        .get_child(tags::SOPClassUID.tag)
        .expect("Should have SOP Class UID");

    let element: &DicomElement = sop_class_uid.as_element();

    assert_eq!(
        element.parse_string_with_vr(&vr::UI)?,
        uids::CTImageStorage.uid
    );

    Ok(())
}

#[test]
fn test_empty_seq_undefined_length_with_std() -> Result<(), Error> {
    test_empty_seq_undefined_length(true)
}

#[test]
fn test_empty_seq_undefined_length_without_std() -> Result<(), Error> {
    test_empty_seq_undefined_length(false)
}

/// In this file the `ReferencedStudySequence` and `ReferencedPatientSequence` tags are both `SQ`
/// elements defined with `UndefinedLength` and contain no data - the first element they have as
/// contents are `SequenceDelimitationItem` which ends the sequence.
fn test_empty_seq_undefined_length(with_std: bool) -> Result<(), Error> {
    let (_parser, dcmroot) = parse_file(
        "./fixtures/gdcm/gdcmConformanceTests/DX_GE_FALCON_SNOWY-VOI.dcm",
        with_std,
    )?;

    let rss_obj: &DicomObject = dcmroot
        .get_child(tags::ReferencedStudySequence.tag)
        .expect("Should be able to parse ReferencedStudySequence");
    // does contain a child item which is the delimitation item
    assert_eq!(rss_obj.get_child_count(), 1);

    let rss_elem: &DicomElement = rss_obj.as_element();
    assert_eq!(rss_elem.vl, ValueLength::UndefinedLength);

    let sdi_pair: (&u32, &DicomObject) = rss_obj
        .iter()
        .next()
        .expect("Should be able to get single child item");
    assert_eq!(*sdi_pair.0, tags::SequenceDelimitationItem.tag);

    let sdi_elem: &DicomElement = sdi_pair.1.as_element();
    assert_eq!(sdi_elem.tag, tags::SequenceDelimitationItem.tag);

    Ok(())
}

#[test]
fn test_private_tag_un_sq_with_std() -> Result<(), Error> {
    test_private_tag_un_sq(true)
}

#[test]
fn test_private_tag_un_sq_without_std() -> Result<(), Error> {
    test_private_tag_un_sq(false)
}

/// Private tags with UN VR and UndefinedLength should be parsed as sequences. This file uses tags
/// which are not known to the dictionaries we're parsing with.
fn test_private_tag_un_sq(with_std: bool) -> Result<(), Error> {
    let (_parser, dcmroot) =
        parse_file("./fixtures/gdcm/gdcmConformanceTests/Enhanced_MR_Image_Storage_Illegal_CP246_corrected.dcm", with_std)?;

    let private_un_seq_obj: &DicomObject = dcmroot
        .get_child(tags::SharedFunctionalGroupsSequence.tag)
        .expect("Fixture should have this this tag")
        .iter()
        .next()
        .expect("This sequence should have 1 sequence item")
        .1
        .get_child(0x2005_140E)
        .expect("This sequence should have private element as child");
    assert_eq!(private_un_seq_obj.get_child_count(), 1);

    let private_un_seq_elem: &DicomElement = private_un_seq_obj.as_element();
    assert_eq!(private_un_seq_elem.vr, &vr::UN);
    assert_eq!(private_un_seq_elem.vl, ValueLength::UndefinedLength);
    assert_eq!(private_un_seq_elem.is_seq_like(), true);
    assert_eq!(private_un_seq_elem.get_data().len(), 0);

    let child_obj: &DicomObject = private_un_seq_obj
        .iter()
        .next()
        .expect("Private sequence should have one item")
        .1;
    // The first item has 28 elements
    assert_eq!(child_obj.get_child_count(), 28);

    let sopuid: &DicomElement = child_obj
        .get_child(tags::SOPClassUID.tag)
        .expect("Should have SOPClassUID child element")
        .as_element();
    // The MR Image Storage UID is odd-length which means the value is padded with a null byte.
    // Only if we detect the VR as UI (when using standard dictionary) then the value should
    // match exactly when parsed as a string otherwise we have to check it with the null byte.
    if with_std {
        assert_eq!(sopuid.parse_string()?, uids::MRImageStorage.uid);
    } else {
        assert_eq!(sopuid.parse_string()?, format!("{}\u{0}", uids::MRImageStorage.uid));
        // force parsing as UI should match exactly
        assert_eq!(sopuid.parse_string_with_vr(&vr::UI)?, uids::MRImageStorage.uid);
    }

    Ok(())
}

#[test]
fn test_seq_switch_to_ivrle_with_std() -> Result<(), Error> {
    test_seq_switch_to_ivrle(true)
}

#[test]
fn test_seq_switch_to_ivrle_without_std() -> Result<(), Error> {
    test_seq_switch_to_ivrle(false)
}

/// `SequenceDelimitationItem`, `Item`, and `ItemDelimitationItem` are always encoded as IVRLE
/// despite what the transfer syntax is.
fn test_seq_switch_to_ivrle(with_std: bool) -> Result<(), Error> {
    let (parser, dcmroot) = parse_file(
        "./fixtures/gdcm/gdcmConformanceTests/D_CLUNIE_CT1_IVRLE_BigEndian.dcm",
        with_std,
    )?;
    assert_eq!(parser.get_ts(), &ts::ExplicitVRBigEndian);

    let sis_obj: &DicomObject = dcmroot
        .get_child(tags::SourceImageSequence.tag)
        .expect("Should have Source Image Sequence");

    if with_std {
        assert_eq!(sis_obj.get_child_count(), 1);
    } else {
        // Without standard lookup we won't know the implicit VR for this element and won't know
        // that it should be parsed as a sequence, so it won't be a parent element.
        assert_eq!(sis_obj.get_child_count(), 0);
    }

    let sis_elem: &DicomElement = sis_obj.as_element();
    if with_std {
        // Should be switched to IVRLE during parse just for this element
        assert_eq!(sis_elem.get_ts(), &ts::ImplicitVRLittleEndian);
    } else {
        // If not parsed as sequence then it should have the same TS as the parsed file
        assert_eq!(sis_elem.get_ts(), parser.get_ts());
        // Nothing else to test in this case
        return Ok(());
    }

    let item_obj: &DicomObject = sis_obj
        .iter()
        .next()
        .expect("Should be able to get child object")
        .1;
    assert_eq!(item_obj.get_child_count(), 2);

    let item_elem: &DicomElement = item_obj.as_element();
    assert_eq!(item_elem.tag, tags::Item.tag);
    assert_eq!(item_elem.get_ts(), &ts::ImplicitVRLittleEndian);

    for (_tag, inner_obj) in item_obj.iter() {
        let elem: &DicomElement = inner_obj.as_element();
        // This assertion seems wrong (should be EVRBE) based on Part 5, Section 7.5 --
        // However, the Data Set within the Value Field of the Data Element Item (FFFE,E000) shall
        // be encoded according to the rules conveyed by the Transfer Syntax.
        assert_eq!(elem.get_ts(), &ts::ImplicitVRLittleEndian);
    }

    Ok(())
}

#[test]
fn test_missing_preamble_with_std() -> Result<(), Error> {
    test_missing_preamble(true)
}

#[test]
fn test_missing_preamble_without_std() -> Result<(), Error> {
    test_missing_preamble(false)
}

/// This file has no preamble or file meta - should parse as the DICOM default IVRLE
fn test_missing_preamble(with_std: bool) -> Result<(), Error> {
    let file: File = File::open("./fixtures/gdcm/gdcmConformanceTests/OT-PAL-8-face.dcm")?;
    let mut parser: ParserBuilder<File> = ParserBuilder::new(file);
    if with_std {
        parser = parser.ts_by_uid(&TS_BY_UID).tag_by_value(&TAG_BY_VALUE);
    }
    let mut parser: Parser<File> = parser.build();

    let first_elem: DicomElement = parser.next().expect("First element should be parsable")?;

    // first tag is a group length tag
    assert_eq!(first_elem.tag, 0x0008_0000);
    // should immediately jump past preamble/prefix, group length, and file meta
    assert_eq!(parser.get_parser_state(), ParseState::Element);
    assert_eq!(parser.get_ts(), &ts::ImplicitVRLittleEndian);

    assert!(parser.get_file_preamble().is_none());
    assert!(parser.get_dicom_prefix().is_none());

    // parse the rest of the stream into an object
    let dcmroot: DicomRoot = parse_stream(&mut parser)?;
    assert_eq!(dcmroot.get_child_count(), 32);
    Ok(())
}

#[test]
fn test_undefined_charset_with_std() -> Result<(), Error> {
    test_undefined_charset(true)
}

#[test]
fn test_undefined_charset_without_std() -> Result<(), Error> {
    test_undefined_charset(false)
}

/// This file has no Specific Character Set defined and tests the behavior of parsing string values.
fn test_undefined_charset(with_std: bool) -> Result<(), Error> {
    let (_parser, dcmroot) = parse_file(
        "./fixtures/gdcm/gdcmConformanceTests/UndefinedValueLengthIllegalNonEncapsulatedTS.dcm",
        with_std,
    )?;

    let scs_elem: &DicomElement = dcmroot
        .get_child(tags::SpecificCharacterSet.tag)
        .expect("Should have Specific Character Set")
        .as_element();
    assert!(scs_elem.is_empty());

    let pat_name: &DicomElement = dcmroot
        .get_child(tags::PatientsName.tag)
        .expect("Should have Patient Name")
        .as_element();

    let pn: String = pat_name.parse_string()?;
    if with_std {
        assert_eq!(pn, "6063^Anon17216");
    } else {
        // The patient name won't be trimmed because without knowing it's a PN element the padding
        // isn't considered in parsing.
        assert_eq!(pn, "6063^Anon17216      ");
        // Forcing the parse using a specific VR should trim the value though
        let pn: String = pat_name.parse_string_with_vr(&vr::PN)?;
        assert_eq!(pn, "6063^Anon17216");
    }

    let pat_com: &DicomElement = dcmroot
        .get_child(tags::PatientComments.tag)
        .expect("Should have Patient Comments")
        .as_element();

    let pc: String = pat_com.parse_string()?;
    let pc_expected: String = String::from_utf8(vec![0,0,0,0,0,0,0,0,0,0,0,0])
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    assert_eq!(pc, pc_expected);

    Ok(())
}

#[test]
fn test_rle_with_std() -> Result<(), Error> {
    test_rle(true)
}

#[test]
fn test_rle_without_std() -> Result<(), Error> {
    test_rle(false)
}

/// This file is RLE encoded. Eventually test the data can be decompressed properly.
fn test_rle(with_std: bool) -> Result<(), Error> {
    let (_parser, _dcmroot) = parse_file(
        "./fixtures/gdcm/gdcmConformanceTests/D_CLUNIE_CT1_RLE_FRAGS.dcm",
        with_std,
    )?;

    Ok(())
}

#[test]
fn test_deflated_evrle_with_std() -> Result<(), Error> {
    test_deflated_evrle(true)
}

#[test]
fn test_deflated_evrle_without_std() -> Result<(), Error> {
    test_deflated_evrle(false)
}

fn test_deflated_evrle(with_std: bool) -> Result<(), Error> {
    let (_parser, _dcmroot) =
        parse_file("./fixtures/gdcm/gdcmConformanceTests/SequenceWithUndefinedLengthNotConvertibleToDefinedLength.dcm", with_std)?;

    Ok(())
}

#[test]
fn test_illegal_cp246_with_std() -> Result<(), Error> {
    test_illegal_cp246(true)
}

#[test]
fn test_illegal_cp246_without_std() -> Result<(), Error> {
    test_illegal_cp246(false)
}

/// Something funky going on in tag after (5200,9229)[1].(2005,140E)[1], doesn't cause parsing error though
fn test_illegal_cp246(with_std: bool) -> Result<(), Error> {
    let (_parser, _dcmroot) =
        parse_file("./fixtures/gdcm/gdcmConformanceTests/Enhanced_MR_Image_Storage_Illegal_CP246.dcm", with_std)?;

    Ok(())
}

#[test]
fn test_no_preamble_start_with_0005_with_std() -> Result<(), Error> {
    test_no_preamble_start_with_0005(true)
}

#[test]
fn test_no_preamble_start_with_0005_without_std() -> Result<(), Error> {
    test_no_preamble_start_with_0005(false)
}

fn test_no_preamble_start_with_0005(with_std: bool) -> Result<(), Error> {
    let (_parser, _dcmroot) =
        parse_file("./fixtures/gdcm/gdcmData/US-IRAD-NoPreambleStartWith0005.dcm", with_std)?;

    Ok(())
}

#[test]
#[ignore]
fn test_parse_all_dicom_files_with_std() -> Result<(), Error> {
    let errors: usize = parse_all_dicom_files(true)?;
    // currently 12 files fail to parse -- when testing for regressions flip the comment
    //assert_eq!(errors, 12);
    assert_eq!(errors, 0);
    Ok(())
}

#[test]
#[ignore]
fn test_parse_all_dicom_files_without_std() -> Result<(), Error> {
    let errors: usize = parse_all_dicom_files(false)?;
    // currently 14 files fail to parse -- when testing for regressions flip the comment
    //assert_eq!(errors, 14);
    assert_eq!(errors, 0);
    Ok(())
}
