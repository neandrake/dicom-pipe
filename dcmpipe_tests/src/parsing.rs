use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::ErrorKind;

use dcmpipe_dict::dict::stdlookup::STANDARD_DICOM_DICTIONARY;
use dcmpipe_dict::dict::tags;
use dcmpipe_dict::dict::transfer_syntaxes as ts;
use dcmpipe_dict::dict::uids;
use dcmpipe_lib::core::dcmelement::{DicomElement, ElementWithVr};
use dcmpipe_lib::core::dcmobject::{DicomNode, DicomObject, DicomRoot};
use dcmpipe_lib::core::parser::{error::ParseError, parser::ParseState, parser::Parser, builder::ParserBuilder, parser::Result};
use dcmpipe_lib::core::parser::util::parse_into_object;
use dcmpipe_lib::core::tagstop::TagStop;
use dcmpipe_lib::defn::tag::{Tag, TagNode, TagPath};
use dcmpipe_lib::defn::vl::ValueLength;
use dcmpipe_lib::defn::vr;

use crate::mock::MockDicomDataset;
use crate::{is_standard_dcm_file, parse_all_dicom_files, parse_file, parse_file_with_tagstop};

#[test]
fn test_good_preamble() {
    let mut parser: Parser<'_, MockDicomDataset> = MockDicomDataset::standard_dicom_preamble();

    // reads the preamble, prefix, and first element (mock has no first element)
    let _ = parser
        .next()
        .expect("Should be able to iterate a valid dicom dataset");

    let is_dcm: bool = is_standard_dcm_file(&parser);
    assert_eq!(is_dcm, true);
}

#[test]
fn test_nonzero_preamble() {
    let mut parser: Parser<'_, MockDicomDataset> = MockDicomDataset::nonzero_preamble();

    // reads the preamble, prefix, and first element (mock has no first element)
    let _ = parser
        .next()
        .expect("Should be able to iterate a valid dicom dataset with non-standard preamble");

    let is_dcm: bool = is_standard_dcm_file(&parser);
    assert_eq!(is_dcm, false);
}

#[test]
fn test_bad_dicom_prefix_parser() {
    let mut parser: Parser<'_, MockDicomDataset> = MockDicomDataset::invalid_dicom_prefix();

    // reads the preamble, prefix, and first element
    let result = parser.next();
    assert!(result.is_some());

    let result = result.unwrap();
    assert!(result.is_err());

    let parse_error: ParseError = result.err().unwrap();
    match parse_error {
        ParseError::BadDICOMPrefix([68, 79, 67, 77]) => {}
        other => assert!(false, format!("{:?}", other)),
    };
}

/// Test that failure to read a single element into a `DicomObject` results in a `None` result
/// rather than an error.
#[test]
fn test_invalid_dicom_prefix_is_none() {
    let mut parser: Parser<'_, MockDicomDataset> = MockDicomDataset::invalid_dicom_prefix();
    let parse_result: Option<DicomRoot<'_>> =
        parse_into_object(&mut parser).expect("Failed to parse DICOM");
    assert!(parse_result.is_none());
}

#[test]
#[should_panic(expected = "failed to fill whole buffer")]
fn test_failure_to_read_preamble() {
    let mut parser: Parser<'_, MockDicomDataset> =
        MockDicomDataset::standard_dicom_preamble_diff_startpos_and_short_dataset();

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
fn test_unknown_explicit_vr_is_error() {
    let parser: Parser<'_, MockDicomDataset> =
        MockDicomDataset::standard_dicom_header_bad_explicit_vr();

    // a test dataset that has a regular file-meta section (defines explicit vr) and the first
    // non-file-meta is the  specific character set, followed by zeros. this tests that parsing
    // explicit element of all zeroes results in an UnknownExplicitVR. all other zeroes in an
    // explicit element are valid (though we don't validate that tag numbers are ordered, an all
    // zero tag is not technically valid but itself should't cause a parse error). for an implicit
    // vr transfer syntax the VR will be selected as UN and should parse
    let first_elem: Result<DicomElement> = parser
        .skip_while(|x| x.is_ok() && x.as_ref().unwrap().tag <= tags::SpecificCharacterSet.tag)
        .next()
        .expect("Should have returned Some(Err)");

    match first_elem {
        Ok(_) => {
            panic!("first element after SpecificCharacterSet should not parse");
        }
        Err(ParseError::UnknownExplicitVR(code)) => {
            assert_eq!(code, 0);
        }
        Err(e) => panic!("unexpected error: {:?}", e),
    }
}

#[test]
fn test_parser_state_with_std() -> Result<()> {
    test_parser_state(true)
}

#[test]
fn test_parser_state_without_std() -> Result<()> {
    test_parser_state(false)
}

fn test_parser_state(with_std: bool) -> Result<()> {
    let tagstop: u32 = tags::PixelData.tag;
    let file: File =
        File::open("./fixtures/gdcm/gdcmConformanceTests/D_CLUNIE_CT1_IVRLE_BigEndian.dcm")?;
    let mut parser: ParserBuilder<'_> =
        ParserBuilder::default().tagstop(TagStop::BeforeTag(tagstop.into()));
    if with_std {
        parser = parser.dictionary(&STANDARD_DICOM_DICTIONARY);
    }
    let mut parser: Parser<'_, File> = parser.build(file);

    assert_eq!(parser.get_parser_state(), ParseState::DetectTransferSyntax);

    let first_elem: DicomElement = parser.next().expect("First element should be Some")?;

    assert_eq!(first_elem.tag, tags::FileMetaInformationGroupLength.tag);

    assert_eq!(parser.get_parser_state(), ParseState::FileMeta);

    while let Some(_) = parser.next() {
        // read through the entire dataset
    }

    assert_eq!(parser.get_parser_state(), ParseState::Element);

    // Ability to read dicom elements after FileMetaInformation
    // means that we interpret the transfer syntax properly, as
    // this test file uses implicit VR (FMI is encoded as explicit)

    // subsequent item should not advance reading elements
    let next_elem: Option<Result<DicomElement>> = parser.next();
    assert!(next_elem.is_none());

    // the iterator state should be just after having parsed the stop tag
    let stopped_at_tag: u32 = parser.get_tag_last_read();
    assert_eq!(
        Tag::format_tag_to_display(stopped_at_tag),
        Tag::format_tag_to_display(tagstop)
    );

    Ok(())
}

#[test]
fn test_dicom_object_with_std() -> Result<()> {
    test_dicom_object(true)
}

#[test]
fn test_dicom_object_without_std() -> Result<()> {
    test_dicom_object(false)
}

fn test_dicom_object(with_std: bool) -> Result<()> {
    let file: File =
        File::open("./fixtures/gdcm/gdcmConformanceTests/D_CLUNIE_CT1_IVRLE_BigEndian.dcm")?;
    let mut parser: ParserBuilder<'_> =
        ParserBuilder::default().tagstop(TagStop::BeforeTag(tags::PixelData.tag.into()));
    if with_std {
        parser = parser.dictionary(&STANDARD_DICOM_DICTIONARY);
    }
    let mut parser: Parser<'_, File> = parser.build(file);

    let dcmroot: DicomRoot<'_> =
        parse_into_object(&mut parser)?.expect("Failed to parse DICOM elements");
    let sop_class_uid: &DicomObject = dcmroot
        .get_child_by_tag(tags::SOPClassUID.tag)
        .expect("Should have SOP Class UID");

    let element: &DicomElement = sop_class_uid.get_element();

    assert_eq!(
        String::try_from(ElementWithVr(element, &vr::UI))?,
        uids::CTImageStorage.uid
    );

    Ok(())
}

#[test]
fn test_dicom_object_sequences_with_std() -> Result<()> {
    test_dicom_object_sequences(true)
}

#[test]
fn test_dicom_object_sequences_without_std() -> Result<()> {
    test_dicom_object_sequences(false)
}

fn test_dicom_object_sequences(with_std: bool) -> Result<()> {
    let file: File = File::open("./fixtures/gdcm/gdcmConformanceTests/RTStruct_VRDSAsVRUN.dcm")?;
    let mut parser: ParserBuilder<'_> =
        ParserBuilder::default().tagstop(TagStop::BeforeTag(tags::PixelData.tag.into()));
    if with_std {
        parser = parser.dictionary(&STANDARD_DICOM_DICTIONARY);
    }
    let mut parser: Parser<'_, File> = parser.build(file);

    let dcmroot: DicomRoot<'_> =
        parse_into_object(&mut parser)?.expect("Failed to parse DICOM elements");

    // StructureSetTime is the last element before a sequence item
    let ss_time: &DicomElement = dcmroot
        .get_child_by_tag(tags::StructureSetTime.tag)
        .expect("Should have StructureSetTime")
        .get_element();
    // pull value into local var so it can be typed properly, otherwise it defaults type to &Vec<u8>
    let ss_time_bytes: &[u8] = ss_time.get_data().as_ref();
    assert_eq!(ss_time_bytes, "092108.000".as_bytes());

    // walk the depths of the first sequence to make sure the structure is setup as we expect
    {
        /* Output form dcmdump
        (3006,0010) SQ (Sequence with undefined length #=1)     # u/l, 1 ReferencedFrameOfReferenceSequence
          (fffe,e000) na (Item with explicit length #=2)          # 1286, 1 Item
            (0020,0052) UI [1.2.246.352.91.0000217.20050503182534.1.1] #  42, 1 FrameOfReferenceUID
            (3006,0012) SQ (Sequence with undefined length #=1)     # u/l, 1 RTReferencedStudySequence
              (fffe,e000) na (Item with explicit length #=3)          # 1208, 1 Item
                (0008,1150) UI =RETIRED_DetachedStudyManagementSOPClass #  24, 1 ReferencedSOPClassUID
                (0008,1155) UI [1.2.246.352.91.0000217.20050503182534]  #  38, 1 ReferencedSOPInstanceUID
                (3006,0014) SQ (Sequence with undefined length #=1)     # u/l, 1 RTReferencedSeriesSequence
                  (fffe,e000) na (Item with explicit length #=2)          # 1102, 1 Item
                    (0020,000e) UI [1.2.246.352.91.0000217.20050503182534.1] #  40, 1 SeriesInstanceUID
                    (3006,0016) SQ (Sequence with undefined length #=11)    # u/l, 1 ContourImageSequence
                    [11 items, pairs of sop-class and sop-uid references]
                */

        let tagpath: TagPath = vec![
            TagNode::new(tags::ReferencedFrameofReferenceSequence.tag, Some(1)),
            TagNode::new(tags::RTReferencedStudySequence.tag, Some(1)),
            TagNode::new(tags::RTReferencedSeriesSequence.tag, Some(1)),
            TagNode::new(tags::ContourImageSequence.tag, Some(11)),
            tags::ReferencedSOPInstanceUID.tag.into(),
        ]
        .into();

        let last_ref_sop_uid_elem: &DicomElement = dcmroot
            .get_child_by_tagpath(&tagpath)
            .expect("Should get element by tagpath")
            .get_element();

        let last_ref_sop_uid: String = last_ref_sop_uid_elem.try_into()?;

        let retrieved_tagpath: TagPath = last_ref_sop_uid_elem.get_tagpath();
        assert_eq!(retrieved_tagpath, tagpath);

        assert_eq!(
            last_ref_sop_uid,
            "1.2.246.352.91.0000217.20050503182534671465"
        );

        // the first sequence item in this object
        let rfor_sq: &DicomObject = dcmroot
            .get_child_by_tag(tags::ReferencedFrameofReferenceSequence.tag)
            .expect("Should have ReferencedFrameOfReferenceSequence");

        assert_eq!(rfor_sq.get_item_count(), 1);
        let item_obj: &DicomObject = rfor_sq.get_item_by_index(1).expect("Have first item");
        assert_eq!(item_obj.get_child_count(), 2);
        let item_foruid: &DicomObject = item_obj
            .get_child_by_tag(tags::FrameofReferenceUID.tag)
            .expect("Have FORUID");
        let item_foruid_bytes: &[u8] = item_foruid.get_element().get_data().as_ref();
        assert_eq!(
            item_foruid_bytes,
            "1.2.246.352.91.0000217.20050503182534.1.1\0".as_bytes()
        );

        assert_eq!(rfor_sq.get_child_count(), 1);
        let child_obj: &DicomObject = rfor_sq
            .iter_child_nodes()
            .next()
            .expect("Have first child")
            .1;
        assert_eq!(
            child_obj.get_element().tag,
            tags::SequenceDelimitationItem.tag
        );

        let rtrss_sq: &DicomObject = item_obj
            .get_child_by_tag(tags::RTReferencedStudySequence.tag)
            .expect("Have RTReferencedStudySequence");
        assert_eq!(rtrss_sq.get_item_count(), 1);
        assert_eq!(rtrss_sq.get_child_count(), 1);

        let rtrss_sq_item: &DicomObject = rtrss_sq.get_item_by_index(1).expect("Have first item");
        assert_eq!(rtrss_sq_item.get_child_count(), 3);
        let ref_sopclass: &DicomElement = rtrss_sq_item
            .get_child_by_tag(tags::ReferencedSOPClassUID.tag)
            .expect("Have ref sop class")
            .get_element();
        let ref_sopclass_bytes: &[u8] = ref_sopclass.get_data().as_ref();
        assert_eq!(ref_sopclass_bytes, "1.2.840.10008.3.1.2.3.1\0".as_bytes());

        let ref_sopuid: &DicomElement = rtrss_sq_item
            .get_child_by_tag(tags::ReferencedSOPInstanceUID.tag)
            .expect("Have ref sop instance uid")
            .get_element();
        let ref_sopuid_bytes: &[u8] = ref_sopuid.get_data().as_ref();
        assert_eq!(
            ref_sopuid_bytes,
            "1.2.246.352.91.0000217.20050503182534\0".as_bytes()
        );

        let rtref_ser_sq: &DicomObject = rtrss_sq_item
            .get_child_by_tag(tags::RTReferencedSeriesSequence.tag)
            .expect("Have ref series seq");
        assert_eq!(rtref_ser_sq.get_item_count(), 1);
        assert_eq!(rtref_ser_sq.get_child_count(), 1);

        let rtref_ser_item: &DicomObject =
            rtref_ser_sq.get_item_by_index(1).expect("Have first item");
        assert_eq!(rtref_ser_item.get_child_count(), 2);
        let rtref_ser_uid: &DicomElement = rtref_ser_item
            .get_child_by_tag(tags::SeriesInstanceUID.tag)
            .expect("Have series uid")
            .get_element();
        let rtref_ser_uid_bytes: &[u8] = rtref_ser_uid.get_data().as_ref();
        assert_eq!(
            rtref_ser_uid_bytes,
            "1.2.246.352.91.0000217.20050503182534.1\0".as_bytes()
        );

        let cont_img_sq: &DicomObject = rtref_ser_item
            .get_child_by_tag(tags::ContourImageSequence.tag)
            .expect("Have contour image seq");
        assert_eq!(cont_img_sq.get_item_count(), 11);
        assert_eq!(cont_img_sq.get_child_count(), 1);

        let cont_img_sq_child: &DicomObject = cont_img_sq
            .iter_child_nodes()
            .next()
            .expect("Get only child of contour image seq")
            .1;
        assert_eq!(
            cont_img_sq_child.get_element().tag,
            tags::SequenceDelimitationItem.tag
        );
        assert_eq!(cont_img_sq_child.get_child_count(), 0);
        assert_eq!(cont_img_sq_child.get_item_count(), 0);

        let last_sop_uid: &DicomElement = cont_img_sq
            .get_item_by_index(11)
            .expect("Get last item")
            .get_child_by_tag(tags::ReferencedSOPInstanceUID.tag)
            .expect("Get last item's ref sop uid")
            .get_element();
        let last_sop_uid_bytes: &[u8] = last_sop_uid.get_data().as_ref();
        assert_eq!(
            last_sop_uid_bytes,
            "1.2.246.352.91.0000217.20050503182534671465\0".as_bytes()
        );
    }

    // test next tag after the first sequence
    let ssroi_sq: &DicomObject = dcmroot
        .get_child_by_tag(tags::StructureSetROISequence.tag)
        .expect("Should have StructureSetROISequence");
    assert_eq!(ssroi_sq.get_item_count(), 4);

    // ContourData's implicit VR is DS, however the first contour in this dataset is encoded
    // explicitly with UN. Verify that it still parses as UN and not DS.
    let contour_data: &DicomElement = dcmroot
        .get_child_by_tag(tags::ROIContourSequence.tag)
        .expect("Have roi contour seq")
        .get_item_by_index(1)
        .expect("Have first item")
        .get_child_by_tag(tags::ContourSequence.tag)
        .expect("Have contour sequence")
        .get_item_by_index(1)
        .expect("Have first item")
        .get_child_by_tag(tags::ContourData.tag)
        .expect("Have contour data")
        .get_element();

    assert_eq!(contour_data.vr, &vr::UN);
    assert_eq!(contour_data.vl, ValueLength::Explicit(107074));
    assert_eq!(contour_data.get_data().len(), 107074);

    Ok(())
}

#[test]
fn test_empty_seq_undefined_length_with_std() -> Result<()> {
    test_empty_seq_undefined_length(true)
}

#[test]
fn test_empty_seq_undefined_length_without_std() -> Result<()> {
    test_empty_seq_undefined_length(false)
}

/// In this file the `ReferencedStudySequence` and `ReferencedPatientSequence` tags are both `SQ`
/// elements defined with `UndefinedLength` and contain no data - the first element they have as
/// contents are `SequenceDelimitationItem` which ends the sequence.
fn test_empty_seq_undefined_length(with_std: bool) -> Result<()> {
    let dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmConformanceTests/DX_GE_FALCON_SNOWY-VOI.dcm",
        with_std,
    )?;

    let rss_obj: &DicomObject = dcmroot
        .get_child_by_tag(tags::ReferencedStudySequence.tag)
        .expect("Should be able to parse ReferencedStudySequence");
    // does contain a child item which is the delimitation item
    assert_eq!(rss_obj.get_child_count(), 1);

    let rss_elem: &DicomElement = rss_obj.get_element();
    assert_eq!(rss_elem.vl, ValueLength::UndefinedLength);

    let sdi_elem: &DicomElement = rss_obj
        .iter_child_nodes()
        .next()
        .expect("Should be able to get single child item")
        .1
        .get_element();
    assert_eq!(sdi_elem.tag, tags::SequenceDelimitationItem.tag);

    Ok(())
}

#[test]
fn test_private_tag_un_sq_with_std() -> Result<()> {
    test_private_tag_un_sq(true)
}

#[test]
fn test_private_tag_un_sq_without_std() -> Result<()> {
    test_private_tag_un_sq(false)
}

/// Private tags with UN VR and UndefinedLength should be parsed as sequences. This file uses tags
/// which are not known to the dictionaries we're parsing with.
fn test_private_tag_un_sq(with_std: bool) -> Result<()> {
    let dcmroot: DicomRoot<'_> =
        parse_file("./fixtures/gdcm/gdcmConformanceTests/Enhanced_MR_Image_Storage_Illegal_CP246_corrected.dcm", with_std)?;

    let private_un_seq_obj: &DicomObject = dcmroot
        .get_child_by_tag(tags::SharedFunctionalGroupsSequence.tag)
        .expect("Fixture should have this this tag")
        .get_item_by_index(1)
        .expect("This sequence should have 1 sequence item")
        .get_child_by_tag(0x2005_140E)
        .expect("This sequence should have private element as child");
    assert_eq!(private_un_seq_obj.get_item_count(), 1);
    assert_eq!(private_un_seq_obj.get_child_count(), 1);

    let private_un_seq_elem: &DicomElement = private_un_seq_obj.get_element();
    assert_eq!(private_un_seq_elem.vr, &vr::UN);
    assert_eq!(private_un_seq_elem.vl, ValueLength::UndefinedLength);
    assert_eq!(private_un_seq_elem.is_seq_like(), true);
    assert_eq!(private_un_seq_elem.get_data().len(), 0);

    let child_obj: &DicomObject = private_un_seq_obj
        .get_item_by_index(1)
        .expect("Private sequence should have one item");
    // The item has 26 elements, plus item delimiter
    assert_eq!(child_obj.get_child_count(), 27);
    assert_eq!(
        *child_obj.iter_child_nodes().last().unwrap().0,
        tags::ItemDelimitationItem.tag
    );

    let sopuid: &DicomElement = child_obj
        .get_child_by_tag(tags::SOPClassUID.tag)
        .expect("Should have SOPClassUID child element")
        .get_element();
    // The MR Image Storage UID is odd-length which means the value is padded with a null byte.
    // Only if we detect the VR as UI (when using standard dictionary) then the value should
    // match exactly when parsed as a string otherwise we have to check it with the null byte.
    if with_std {
        assert_eq!(String::try_from(sopuid)?, uids::MRImageStorage.uid);
    } else {
        assert_eq!(
            String::try_from(sopuid)?,
            format!("{}\u{0}", uids::MRImageStorage.uid)
        );
        // force parsing as UI should match exactly
        assert_eq!(
            String::try_from(ElementWithVr(sopuid, &vr::UI))?,
            uids::MRImageStorage.uid
        );
    }

    Ok(())
}

#[test]
fn test_seq_switch_to_ivrle_with_std() -> Result<()> {
    test_seq_switch_to_ivrle(true)
}

#[test]
fn test_seq_switch_to_ivrle_without_std() -> Result<()> {
    test_seq_switch_to_ivrle(false)
}

/// `SequenceDelimitationItem`, `Item`, and `ItemDelimitationItem` are always encoded as IVRLE
/// despite what the transfer syntax is.
fn test_seq_switch_to_ivrle(with_std: bool) -> Result<()> {
    let dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmConformanceTests/D_CLUNIE_CT1_IVRLE_BigEndian.dcm",
        with_std,
    )?;
    assert_eq!(dcmroot.get_ts(), &ts::ExplicitVRBigEndian);

    let sis_obj: &DicomObject = dcmroot
        .get_child_by_tag(tags::SourceImageSequence.tag)
        .expect("Should have Source Image Sequence");

    if with_std {
        assert_eq!(sis_obj.get_item_count(), 1);
    } else {
        // Without standard lookup we won't know the implicit VR for this element and won't know
        // that it should be parsed as a sequence, so it won't be a parent element.
        assert_eq!(sis_obj.get_item_count(), 0);
    }

    let sis_elem: &DicomElement = sis_obj.get_element();
    if with_std {
        // Should be switched to IVRLE during parse just for this element
        assert_eq!(sis_elem.get_ts(), &ts::ImplicitVRLittleEndian);
    } else {
        // If not parsed as sequence then it should have the same TS as the parsed file
        assert_eq!(sis_elem.get_ts(), dcmroot.get_ts());
        // Nothing else to test in this case
        return Ok(());
    }

    let item_obj: &DicomObject = sis_obj
        .get_item_by_index(1)
        .expect("Should be able to get child object");

    assert_eq!(item_obj.get_child_count(), 2);

    let item_elem: &DicomElement = item_obj.get_element();
    assert_eq!(item_elem.tag, tags::Item.tag);
    assert_eq!(item_elem.get_ts(), &ts::ImplicitVRLittleEndian);

    for (_tag, inner_obj) in item_obj.iter_child_nodes() {
        let elem: &DicomElement = inner_obj.get_element();
        // This assertion seems wrong (should be EVRBE) based on Part 5, Section 7.5 --
        // However, the Data Set within the Value Field of the Data Element Item (FFFE,E000) shall
        // be encoded according to the rules conveyed by the Transfer Syntax.
        assert_eq!(elem.get_ts(), &ts::ImplicitVRLittleEndian);
    }

    Ok(())
}

#[test]
fn test_missing_preamble_with_std() -> Result<()> {
    test_missing_preamble(true)
}

#[test]
fn test_missing_preamble_without_std() -> Result<()> {
    test_missing_preamble(false)
}

/// This file has no preamble or file meta - should parse as the DICOM default IVRLE
fn test_missing_preamble(with_std: bool) -> Result<()> {
    let file: File = File::open("./fixtures/gdcm/gdcmConformanceTests/OT-PAL-8-face.dcm")?;
    let mut parser: ParserBuilder<'_> = ParserBuilder::default();
    if with_std {
        parser = parser.dictionary(&STANDARD_DICOM_DICTIONARY);
    }
    let mut parser: Parser<'_, File> = parser.build(file);

    let first_elem: DicomElement = parser.next().expect("First element should be parsable")?;

    // first tag is a group length tag
    assert_eq!(first_elem.tag, 0x0008_0000);
    // should immediately jump past preamble/prefix, group length, and file meta
    assert_eq!(parser.get_parser_state(), ParseState::Element);
    assert_eq!(parser.get_ts(), &ts::ImplicitVRLittleEndian);

    assert!(parser.get_file_preamble().is_none());
    assert!(parser.get_dicom_prefix().is_none());

    // parse the rest of the dataset into an object
    let dcmroot: DicomRoot<'_> =
        parse_into_object(&mut parser)?.expect("Failed to parse DICOM elements");
    assert_eq!(dcmroot.get_child_count(), 32);
    Ok(())
}

#[test]
fn test_undefined_charset_with_std() -> Result<()> {
    test_undefined_charset(true)
}

#[test]
fn test_undefined_charset_without_std() -> Result<()> {
    test_undefined_charset(false)
}

/// This file has no Specific Character Set defined and tests the behavior of parsing string values.
fn test_undefined_charset(with_std: bool) -> Result<()> {
    let dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmConformanceTests/UndefinedValueLengthIllegalNonEncapsulatedTS.dcm",
        with_std,
    )?;

    let scs_elem: &DicomElement = dcmroot
        .get_child_by_tag(tags::SpecificCharacterSet.tag)
        .expect("Should have Specific Character Set")
        .get_element();
    assert!(scs_elem.is_empty());

    let pat_name: &DicomElement = dcmroot
        .get_child_by_tag(tags::PatientsName.tag)
        .expect("Should have Patient Name")
        .get_element();

    let pn: String = String::try_from(pat_name)?;
    if with_std {
        assert_eq!(pn, "6063^Anon17216");
    } else {
        // The patient name won't be trimmed because without knowing it's a PN element the padding
        // isn't considered in parsing.
        assert_eq!(pn, "6063^Anon17216      ");
        // Forcing the parse using a specific VR should trim the value though
        let pn: String = String::try_from(ElementWithVr(pat_name, &vr::PN))?;
        assert_eq!(pn, "6063^Anon17216");
    }

    let pat_com: &DicomElement = dcmroot
        .get_child_by_tag(tags::PatientComments.tag)
        .expect("Should have Patient Comments")
        .get_element();

    let pc: String = String::try_from(pat_com)?;
    // this value is a bunch of null bytes. with the standard dictionary this will attempt to parse
    // as a string based on the known VR and be stripped of all null bytes.
    let pc_expected: String = if with_std {
        String::new()
    } else {
        String::from_utf8(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).map_err(|e| {
            ParseError::IOError {
                source: std::io::Error::new(ErrorKind::InvalidData, format!("{:?}", e)),
            }
        })?
    };

    assert_eq!(pc, pc_expected);

    Ok(())
}

#[test]
fn test_rle_with_std() -> Result<()> {
    test_rle(true)
}

#[test]
fn test_rle_without_std() -> Result<()> {
    test_rle(false)
}

/// This file is RLE encoded. Eventually test the data can be decompressed properly.
fn test_rle(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmConformanceTests/D_CLUNIE_CT1_RLE_FRAGS.dcm",
        with_std,
    )?;

    Ok(())
}

#[test]
fn test_deflated_evrle_with_std() -> Result<()> {
    test_deflated_evrle(true)
}

#[test]
fn test_deflated_evrle_without_std() -> Result<()> {
    test_deflated_evrle(false)
}

fn test_deflated_evrle(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> =
        parse_file("./fixtures/gdcm/gdcmConformanceTests/SequenceWithUndefinedLengthNotConvertibleToDefinedLength.dcm", with_std)?;

    Ok(())
}

#[test]
fn test_illegal_cp246_with_std() -> Result<()> {
    test_illegal_cp246(true)
}

#[test]
fn test_illegal_cp246_without_std() -> Result<()> {
    test_illegal_cp246(false)
}

/// Something funky going on in tag after (5200,9229)[1].(2005,140E)[1] - also can't be parsed by
/// dcmtk for the same reason.
fn test_illegal_cp246(with_std: bool) -> Result<()> {
    let dcmroot: DicomRoot<'_> = parse_file_with_tagstop(
        "./fixtures/gdcm/gdcmConformanceTests/Enhanced_MR_Image_Storage_Illegal_CP246.dcm",
        with_std,
        // this file has invalid tag after this position, sequence-contained tag (0700,0300) which
        // has a massive value length which goes past the file contents.
        TagStop::AfterBytePos(6484),
    )?;

    let ref_sop_class_uid_elem: &DicomElement = dcmroot
        .get_child_by_tag(tags::SharedFunctionalGroupsSequence.tag)
        .expect("Should have SharedFunctionalGroupsSequence")
        .get_item_by_index(1)
        .expect("Should have item")
        .get_child_by_tag(tags::ReferencedImageSequence.tag)
        .expect("Should have ReferencedImageSequence")
        .get_item_by_index(1)
        .expect("Should have item")
        .get_child_by_tag(tags::ReferencedSOPClassUID.tag)
        .expect("Should have ReferencedSOPClassUID")
        .get_element();

    let ref_sop_class_uid: String = ref_sop_class_uid_elem.try_into()?;

    assert_eq!(ref_sop_class_uid, uids::EnhancedMRImageStorage.uid);

    let ref_sop_class_uid: String = dcmroot
        .get_child_by_tagpath(
            &vec![
                TagNode::new(tags::SharedFunctionalGroupsSequence.tag, Some(1)),
                TagNode::new(tags::ReferencedImageSequence.tag, Some(1)),
                TagNode::new(tags::ReferencedSOPClassUID.tag, None),
            ]
            .into(),
        )
        .expect("Should get by tagpath")
        .get_element()
        .try_into()?;

    assert_eq!(ref_sop_class_uid, uids::EnhancedMRImageStorage.uid);

    let elem_tagpath: TagPath = ref_sop_class_uid_elem.get_tagpath();

    let ref_sop_class_uid: String = dcmroot
        .get_child_by_tagpath(&elem_tagpath)
        .expect("Should get by element tagpath")
        .get_element()
        .try_into()?;

    assert_eq!(ref_sop_class_uid, uids::EnhancedMRImageStorage.uid);

    Ok(())
}

#[test]
fn test_no_preamble_start_with_0005_with_std() -> Result<()> {
    test_no_preamble_start_with_0005(true)
}

#[test]
fn test_no_preamble_start_with_0005_without_std() -> Result<()> {
    test_no_preamble_start_with_0005(false)
}

/// File has no preamble/prefix and also no File Meta Info header, should default to IVRLE
fn test_no_preamble_start_with_0005(with_std: bool) -> Result<()> {
    let dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/US-IRAD-NoPreambleStartWith0005.dcm",
        with_std,
    )?;

    assert_eq!(dcmroot.get_ts(), &ts::ImplicitVRLittleEndian);

    let study_desc_elem: &DicomElement = dcmroot
        .get_child_by_tag(tags::StudyDescription.tag)
        .expect("Should have Study Description tag")
        .get_element();

    let study_desc: String = study_desc_elem.try_into()?;

    if with_std {
        assert_eq!(study_desc, "ABDOMEN");
    } else {
        // parsing without dictionary doesn't know VR since parsed as implicit, so it won't know to
        // remove padding value.
        assert_eq!(study_desc, "ABDOMEN ");
        // force parsing using the actual VR should trim the padding
        let study_desc: String = String::try_from(ElementWithVr(study_desc_elem, &vr::LO))?;
        assert_eq!(study_desc, "ABDOMEN");
    }

    Ok(())
}

#[test]
fn test_no_dicomv3_preamble_with_std() -> Result<()> {
    test_no_dicomv3_preamble(true)
}

#[test]
fn test_no_dicomv3_preamble_without_std() -> Result<()> {
    test_no_dicomv3_preamble(false)
}

/// File has no preamble/prefix
fn test_no_dicomv3_preamble(with_std: bool) -> Result<()> {
    let dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/PICKER-16-MONO2-No_DicomV3_Preamble.dcm",
        with_std,
    )?;

    // check we can read the first element just fine
    let fme_length: u32 = dcmroot
        .get_child_by_tag(tags::FileMetaInformationGroupLength.tag)
        .expect("Should have FileMetaInfo GroupLength tag")
        .get_element()
        .try_into()?;

    assert_eq!(fme_length, 84);

    Ok(())
}

#[test]
fn test_private_ge_ts_with_std() -> Result<()> {
    test_private_ge_ts(true)
}

#[test]
fn test_private_ge_ts_without_std() -> Result<()> {
    test_private_ge_ts(false)
}

fn test_private_ge_ts(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/PrivateGEImplicitVRBigEndianTransferSyntax16Bits.dcm",
        with_std,
    )?;

    Ok(())
}

#[test]
#[ignore]
fn test_secured_dicomdir_with_std() -> Result<()> {
    test_secured_dicomdir(true)
}

#[test]
#[ignore]
fn test_secured_dicomdir_without_std() -> Result<()> {
    test_secured_dicomdir(false)
}

/// See Part 10, Section 7.4 on Secure DICOM File Format. File is encrypted with RFC3369
fn test_secured_dicomdir(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/securedicomfileset/DICOMDIR",
        with_std,
    )?;

    Ok(())
}

#[test]
#[ignore]
fn test_secured_image_with_std() -> Result<()> {
    test_secured_image(true)
}

#[test]
#[ignore]
fn test_secured_image_without_std() -> Result<()> {
    test_secured_image(false)
}

/// See Part 10, Section 7.4 on Secure DICOM File Format. File is encrypted with RFC3369
fn test_secured_image(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/securedicomfileset/IMAGES/IMAGE1",
        with_std,
    )?;

    Ok(())
}

#[test]
fn test_private_ge_dlx_ts_with_std() -> Result<()> {
    test_private_ge_dlx_ts(true)
}

#[test]
fn test_private_ge_dlx_ts_without_std() -> Result<()> {
    test_private_ge_dlx_ts(false)
}

fn test_private_ge_dlx_ts(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/GE_DLX-8-MONO2-PrivateSyntax.dcm",
        with_std,
    )?;

    Ok(())
}

#[test]
fn test_undefined_item_wrong_vl_with_std() -> Result<()> {
    test_undefined_item_wrong_vl(true)
}

#[test]
fn test_undefined_item_wrong_vl_without_std() -> Result<()> {
    test_undefined_item_wrong_vl(false)
}

fn test_undefined_item_wrong_vl(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/BugGDCM2_UndefItemWrongVL.dcm",
        with_std,
    )?;

    Ok(())
}

#[test]
fn test_uncompressed_even_length_tag_with_std() -> Result<()> {
    test_uncompressed_even_length_tag(true)
}

#[test]
fn test_uncompressed_even_length_tag_without_std() -> Result<()> {
    test_uncompressed_even_length_tag(false)
}

fn test_uncompressed_even_length_tag(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/THERALYS-12-MONO2-Uncompressed-Even_Length_Tag.dcm",
        with_std,
    )?;

    Ok(())
}

#[test]
fn test_dicomdir_with_embedded_icons_with_std() -> Result<()> {
    test_dicomdir_with_embedded_icons(true)
}

#[test]
fn test_dicomdir_with_embedded_icons_without_std() -> Result<()> {
    test_dicomdir_with_embedded_icons(false)
}

/// High number of items in a sequence
fn test_dicomdir_with_embedded_icons(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/dicomdir_With_embedded_icons",
        with_std,
    )?;

    Ok(())
}

#[test]
fn test_radbw_lossless_with_std() -> Result<()> {
    test_radbw_lossless(true)
}

#[test]
fn test_radbw_lossless_without_std() -> Result<()> {
    test_radbw_lossless(false)
}

fn test_radbw_lossless(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> =
        parse_file("./fixtures/gdcm/gdcmData/RadBWLossLess.dcm", with_std)?;

    Ok(())
}

#[test]
fn test_derma_color_lossless_with_std() -> Result<()> {
    test_derma_color_lossless(true)
}

#[test]
fn test_derma_color_lossless_without_std() -> Result<()> {
    test_derma_color_lossless(false)
}

fn test_derma_color_lossless(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> =
        parse_file("./fixtures/gdcm/gdcmData/DermaColorLossLess.dcm", with_std)?;

    Ok(())
}

#[test]
fn test_libido_16_acr_nema_volume_with_std() -> Result<()> {
    test_libido_16_acr_nema_volume(true)
}

#[test]
fn test_libido_16_acr_nema_volume_without_std() -> Result<()> {
    test_libido_16_acr_nema_volume(false)
}

fn test_libido_16_acr_nema_volume(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/LIBIDO-16-ACR_NEMA-Volume.dcm",
        with_std,
    )?;

    Ok(())
}

#[test]
fn test_jpeg_lossless_zerolength_sq_with_std() -> Result<()> {
    test_jpeg_lossless_zerolength_sq(true)
}

#[test]
fn test_jpeg_lossless_zerolength_sq_without_std() -> Result<()> {
    test_jpeg_lossless_zerolength_sq(false)
}

fn test_jpeg_lossless_zerolength_sq(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/MARCONI_MxTWin-12-MONO2-JpegLossless-ZeroLengthSQ.dcm",
        with_std,
    )?;

    Ok(())
}

#[test]
fn test_illegal_group2_implicit_ts_with_std() -> Result<()> {
    test_illegal_group2_implicit_ts(true)
}

#[test]
fn test_illegal_group2_implicit_ts_without_std() -> Result<()> {
    test_illegal_group2_implicit_ts(false)
}

fn test_illegal_group2_implicit_ts(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/IllegalGroup2ImplicitTS.dcm",
        with_std,
    )?;

    Ok(())
}

#[test]
#[should_panic(expected = "num bytes not multiple of size of u32")]
fn test_ul_is_2bytes_with_std() {
    test_ul_is_2bytes(true).unwrap();
}

#[test]
#[should_panic(expected = "num bytes not multiple of size of u32")]
fn test_ul_is_2bytes_without_std() {
    test_ul_is_2bytes(false).unwrap();
}

/// Contains tags (0009,1130), (0009,1131), (0009,1140) with explicit VR of UL but value length is
/// actually only 2 bytes instead of 4.
fn test_ul_is_2bytes(with_std: bool) -> Result<()> {
    let dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/SIEMENS_GBS_III-16-ACR_NEMA_1-ULis2Bytes.dcm",
        with_std,
    )?;

    let element1: &DicomElement = dcmroot
        .get_child_by_tag(0x0009_1130)
        .expect("Element should exist")
        .get_element();
    assert_eq!(element1.vr, &vr::UL);
    assert_eq!(element1.vl, ValueLength::Explicit(2));
    // should be able to parse the value as u16 since it has 2 bytes
    let element1_val: u16 = element1.try_into()?;
    assert_eq!(element1_val, 0x0800);

    let element2: &DicomElement = dcmroot
        .get_child_by_tag(0x0009_1131)
        .expect("Element should exist")
        .get_element();
    assert_eq!(element2.vr, &vr::UL);
    assert_eq!(element2.vl, ValueLength::Explicit(2));
    // should be able to parse the value as u16 since it has 2 bytes
    let element2_val: u16 = element1.try_into()?;
    assert_eq!(element2_val, 0x0800);

    let element3: &DicomElement = dcmroot
        .get_child_by_tag(0x0009_1140)
        .expect("Element should exist")
        .get_element();
    assert_eq!(element3.vr, &vr::UL);
    assert_eq!(element3.vl, ValueLength::Explicit(2));
    // should be able to parse the value as u16 since it has 2 bytes
    let element3_val: u16 = element1.try_into()?;
    assert_eq!(element3_val, 0x0800);

    // check that we can properly parse the element after the ones with incorrect value length
    let element4: &DicomElement = dcmroot
        .get_child_by_tag(0x0009_1141)
        .expect("Element should exist")
        .get_element();
    assert_eq!(element4.vr, &vr::UL);
    assert_eq!(element4.vl, ValueLength::Explicit(4));

    let element4_val: u32 = u32::try_from(element4)?;
    assert_eq!(element4_val, 0x2_0000);

    // this will return an error
    TryInto::<u32>::try_into(element1)?;

    Ok(())
}

#[test]
fn test_dicomdir_with_std() -> Result<()> {
    test_dicomdir(true)
}

#[test]
fn test_dicomdir_without_std() -> Result<()> {
    test_dicomdir(false)
}

fn test_dicomdir(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file("./fixtures/dclunie/charsettests/DICOMDIR", with_std)?;

    Ok(())
}

#[test]
fn test_sq_with_undefined_length_converted_to_defined_length_with_std() -> Result<()> {
    test_sq_with_undefined_length_converted_to_defined_length(true)
}

#[test]
fn test_sq_with_undefined_length_converted_to_defined_length_without_std() -> Result<()> {
    test_sq_with_undefined_length_converted_to_defined_length(false)
}

fn test_sq_with_undefined_length_converted_to_defined_length(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file("./fixtures/gdcm/gdcmConformanceTests/SequenceWithUndefinedLengthConvertedToDefinedLength.dcm", with_std)?;

    Ok(())
}

#[test]
fn test_sq_with_undefined_length_unconvertable_to_defined_length_with_std() -> Result<()> {
    test_sq_with_undefined_length_unconvertable_to_defined_length(true)
}

#[test]
fn test_sq_with_undefined_length_unconvertable_to_defined_length_without_std() -> Result<()> {
    test_sq_with_undefined_length_unconvertable_to_defined_length(false)
}

/// This is a deflated dataset
fn test_sq_with_undefined_length_unconvertable_to_defined_length(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file("./fixtures/gdcm/gdcmConformanceTests/SequenceWithUndefinedLengthNotConvertibleToDefinedLength.dcm", with_std)?;

    Ok(())
}

#[test]
#[ignore]
fn test_explicit_vr_for_pub_element_implicit_vr_for_shadow_elements_with_std() -> Result<()> {
    test_explicit_vr_for_pub_element_implicit_vr_for_shadow_elements(true)
}

#[test]
#[ignore]
fn test_explicit_vr_for_pub_element_implicit_vr_for_shadow_elements_without_std() -> Result<()> {
    test_explicit_vr_for_pub_element_implicit_vr_for_shadow_elements(false)
}

/// This dataset seems partially malformed. It looks like after the SourceImageSequence the transfer
/// syntax of elements at the root switch to being ImplicitVR rather than the defined ExplicitVR.
/// It's not clear if this is something that we can handle -- dcmtk also is unable to parse this
/// though it's not clear if for the same reason.
/// See: http://compgroups.net/comp.protocols.dicom/mixing-explicit-and-implicit-transfer-sy/2221446
/// >Pre-1996 versions of DCMTK (then still called the European CTN software) had some
/// >code that would "guess" the transfer syntax for every sequence item and would even
/// >handle implicit VR big endian encoding. However, the heuristics created more problems
/// >then they solved.
fn test_explicit_vr_for_pub_element_implicit_vr_for_shadow_elements(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/ExplicitVRforPublicElementsImplicitVRforShadowElements.dcm",
        with_std,
    )?;

    Ok(())
}

#[test]
#[ignore]
fn test_explicit_implicit_bogus_iop_with_std() -> Result<()> {
    test_explicit_implicit_bogus_iop(true)
}

#[test]
#[ignore]
fn test_explicit_implicit_bogus_iop_without_std() -> Result<()> {
    test_explicit_implicit_bogus_iop(false)
}

/// This dataset also randomly switches between implicit and explicit transfer syntax. This one
/// switches to implicit after Modality, and seems to switch back shortly after SeriesDescription.
fn test_explicit_implicit_bogus_iop(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/DMCPACS_ExplicitImplicit_BogusIOP.dcm",
        with_std,
    )?;

    Ok(())
}

#[test]
#[ignore]
fn test_jpeg_lossless3a_with_std() -> Result<()> {
    test_jpeg_lossless3a(true)
}

#[test]
#[ignore]
fn test_jpeg_lossless3a_without_std() -> Result<()> {
    test_jpeg_lossless3a(false)
}

/// This dataset has a non-standard pixel-data structure. The second item/frame seems to be encoded
/// in a different transfer syntax. This also fails parsing in dcmtk for the same reason.
fn test_jpeg_lossless3a(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/gdcm-JPEG-LossLess3a.dcm",
        with_std,
    )?;

    Ok(())
}

#[test]
fn test_kodak_compressed_icon_with_std() -> Result<()> {
    test_kodak_compressed_icon(true)
}

#[test]
fn test_kodak_compressed_icon_without_std() -> Result<()> {
    test_kodak_compressed_icon(false)
}

/// This dataset has a pixel data inside IconImageSqeuence - the pixel data itself having 2 items.
/// This pixel data is undefined length and its items are defined so do not have item delimiters,
/// but does have a sequence delimiter which should pop off the inner item.
fn test_kodak_compressed_icon(with_std: bool) -> Result<()> {
    let _dcmroot: DicomRoot<'_> = parse_file(
        "./fixtures/gdcm/gdcmData/KODAK_CompressedIcon.dcm",
        with_std,
    )?;

    Ok(())
}

#[test]
#[ignore]
fn test_parse_all_dicom_files_with_std() -> Result<()> {
    let num_failed: usize = parse_all_dicom_files(true)?;
    if num_failed > 0 {
        Err(ParseError::IOError {
            source: std::io::Error::new(
                ErrorKind::InvalidData,
                format!("Failed to parse DICOM files: {}", num_failed),
            ),
        })
    } else {
        Ok(())
    }
}

#[test]
#[ignore]
fn test_parse_all_dicom_files_without_std() -> Result<()> {
    let num_failed: usize = parse_all_dicom_files(false)?;
    if num_failed > 0 {
        Err(ParseError::IOError {
            source: std::io::Error::new(
                ErrorKind::InvalidData,
                format!("Failed to parse DICOM files: {}", num_failed),
            ),
        })
    } else {
        Ok(())
    }
}
