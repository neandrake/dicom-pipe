extern crate walkdir;

use crate::core::dcmobject::DicomObject;
use crate::core::dcmparser::{
    DicomStreamParser, DICOM_PREFIX, DICOM_PREFIX_LENGTH, FILE_PREAMBLE_LENGTH,
};
use crate::core::dcmreader::parse_stream;
use crate::core::tests::mock::MockDicomStream;
use crate::core::tagstop::TagStop;
use crate::defn::tag::Tag;
use crate::defn::ts::TransferSyntax;
use crate::defn::uid::UIDRef;
use crate::dict::dicom_elements as tags;
use crate::dict::dir_structure_elements as dse;
use crate::dict::file_meta_elements as fme;
use crate::dict::lookup::{
    TAG_BY_IDENT, TAG_BY_VALUE, TS_BY_ID, TS_BY_IDENT, UID_BY_ID, UID_BY_IDENT,
};
use crate::dict::transfer_syntaxes as ts;
use crate::dict::uids;

use byteorder::ReadBytesExt;
use std::fs::File;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

static FIXTURE_DATASET1_FOLDER: &'static str = "../fixtures/dataset1";

#[test]
fn test_good_preamble() {
    let mut test_good_iter: DicomStreamParser<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble();

    // reads the preamble, prefix, and first element (mock has no first element)
    let _ = test_good_iter
        .next()
        .expect("Should be able to iterate a valid dicom stream");

    let is_dcm: bool = is_standard_preamble(&test_good_iter);
    assert_eq!(is_dcm, true);
}

#[test]
fn test_nonzero_preamble() {
    let mut test_size_iter: DicomStreamParser<MockDicomStream> =
        MockDicomStream::nonzero_preamble();

    // reads the preamble, prefix, and first element (mock has no first element)
    let _ = test_size_iter
        .next()
        .expect("Should be able to iterate a valid dicom stream with non-standard preamble");

    let is_dcm: bool = is_standard_preamble(&test_size_iter);
    assert_eq!(is_dcm, false);
}

#[test]
#[should_panic(expected = "Invalid DICOM Prefix")]
fn test_bad_dicom_prefix() {
    let mut test_bad_iter: DicomStreamParser<MockDicomStream> =
        MockDicomStream::invalid_dicom_prefix();

    // reads the preamble, prefix, and first element
    let _ = test_bad_iter
        .next()
        .expect("An invalid prefix should return Some(Err)")
        .expect("Invalid prefix should be Err, this expect should cause panic");
}

#[test]
#[should_panic(expected = "failed to fill whole buffer")]
fn test_failure_to_read_preamble() {
    let mut test_iter: DicomStreamParser<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble_diff_startpos_and_short_stream();

    // reads the preamble, prefix, and first element
    let _first_elem = test_iter
        .next()
        .expect("Value should be Some Error")
        .expect("This should fail to read preamble due to not enough data");

    // should record zero bytes read since the first attempt to read into buffer should fail to fill
    let start_pos: u64 = test_iter.bytes_read();
    assert_eq!(start_pos, 0);
}

#[test] // slow
fn test_parse_known_dicom_files() {
    let tagstop: u32 = tags::PixelData.tag;
    let mut dicom_iter: DicomStreamParser<File> =
        get_first_file_stream(TagStop::BeforeTag(tagstop));

    let _first_elem = dicom_iter
        .next()
        .expect("Unable to read first element")
        .expect("Unable to read first element");

    let is_dcm = is_standard_preamble(&dicom_iter);
    assert!(is_dcm);

    while let Some(_) = dicom_iter.next() {
        // read elements while iterator returns non-None
    }

    // Ability to read dicom elements after FileMetaInformation
    // means that we interpret the transfer syntax properly, as
    // the fixtures are implicit VR (FMI is encoded as explicit)

    // subsequent item should not advance reading elements
    let next_elem = dicom_iter.next();
    assert!(next_elem.is_none());

    // the iterator
    let stopped_at_tag = dicom_iter
        .partial_tag()
        .expect("Iteration should have stopped after reading the PixelData tag");
    assert_eq!(tagstop, stopped_at_tag);
}

#[test]
pub fn test_dicom_object() {
    let mut dicom_iter: DicomStreamParser<File> = get_first_file_stream(TagStop::EndOfStream);

    let mut dcmobj: DicomObject = parse_stream(&mut dicom_iter).expect("Should be no probz");
    let sop_class_uid: &mut DicomObject = dcmobj
        .get_object(tags::SOPClassUID.tag)
        .expect("Should have SOP Class UID");
    if let Some(ref mut element) = sop_class_uid.as_element() {
        assert_eq!(
            element.parse_string().expect("get cs"),
            uids::CTImageStorage.uid
        )
    } else {
        panic!("Element should exist")
    }
}

fn get_first_file_stream(tagstop: TagStop) -> DicomStreamParser<File> {
    let dirwalker: WalkDir = WalkDir::new(FIXTURE_DATASET1_FOLDER)
        .min_depth(1)
        .max_depth(1);

    for entry_res in dirwalker.into_iter() {
        let entry: DirEntry = entry_res.unwrap();
        let path: &Path = entry.path();

        let file: File = File::open(path).expect(&format!("Unable to open file: {:?}", path));

        let dstream: DicomStreamParser<File> = DicomStreamParser::new(file, tagstop);

        return dstream;
    }

    panic!("No DICOM files found");
}

/// Checks that the first 132 bytes are 128 0's followed by 'DICM'
fn is_standard_preamble<StreamType>(stream: &DicomStreamParser<StreamType>) -> bool
where
    StreamType: ReadBytesExt,
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

#[test]
pub fn test_tags_lookup() {
    // Lookup a DICOM Element by identity/value from the TAG maps
    let pd_by_ident: &Tag = TAG_BY_IDENT.get("PixelData").expect("Tag not found");
    assert_eq!(pd_by_ident, &tags::PixelData);

    let pd_by_tag: &Tag = TAG_BY_VALUE.get(&0x7FE00010).expect("Tag not found");
    assert_eq!(pd_by_tag, &tags::PixelData);

    // Lookup a Directory Structure Element by identity/value from the TAG maps
    let fsid_by_ident: &Tag = TAG_BY_IDENT.get("FilesetID").expect("Tag not found");
    assert_eq!(fsid_by_ident, &dse::FilesetID);

    let fsid_by_tag: &Tag = TAG_BY_VALUE.get(&0x00041130).expect("Tag not found");
    assert_eq!(fsid_by_tag, &dse::FilesetID);

    // Lookup a File Meta Element by identity/value from the TAG maps
    let tsuid_by_ident: &Tag = TAG_BY_IDENT
        .get("TransferSyntaxUID")
        .expect("Tag not found");
    assert_eq!(tsuid_by_ident, &fme::TransferSyntaxUID);

    let tsuid_by_tag: &Tag = TAG_BY_VALUE.get(&0x00020010).expect("Tag not found");
    assert_eq!(tsuid_by_tag, &fme::TransferSyntaxUID);
}

#[test]
pub fn test_transfer_syntaxes_lookup() {
    let ivrle_by_ident: &TransferSyntax = TS_BY_IDENT
        .get("ImplicitVRLittleEndian")
        .expect("TransferSyntax not found");
    assert_eq!(ivrle_by_ident, &ts::ImplicitVRLittleEndian);

    let ivrle_by_id: &TransferSyntax = TS_BY_ID
        .get("1.2.840.10008.1.2")
        .expect("TransferSyntax not found");
    assert_eq!(ivrle_by_id, &ts::ImplicitVRLittleEndian);

    let ivrle_by_uid: &TransferSyntax = TS_BY_ID
        .get(&uids::ImplicitVRLittleEndian.uid)
        .expect("TransferSyntax not found");
    assert_eq!(ivrle_by_uid, &ts::ImplicitVRLittleEndian);

    let evrle_by_ident: &TransferSyntax = TS_BY_IDENT
        .get("ExplicitVRLittleEndian")
        .expect("TransferSyntax not found");
    assert_eq!(evrle_by_ident, &ts::ExplicitVRLittleEndian);

    let evrle_by_id: &TransferSyntax = TS_BY_ID
        .get("1.2.840.10008.1.2.1")
        .expect("TransferSyntax not found");
    assert_eq!(evrle_by_id, &ts::ExplicitVRLittleEndian);

    let evrle_by_uid: &TransferSyntax = TS_BY_ID
        .get(&uids::ExplicitVRLittleEndian.uid)
        .expect("TransferSyntax not found");
    assert_eq!(evrle_by_uid, &ts::ExplicitVRLittleEndian);
}

#[test]
pub fn test_uids_lookup() {
    let ctis_by_ident: UIDRef = UID_BY_IDENT.get("CTImageStorage").expect("UID not found");
    assert_eq!(ctis_by_ident, &uids::CTImageStorage);

    let ctis_by_id: UIDRef = UID_BY_ID
        .get("1.2.840.10008.5.1.4.1.1.2")
        .expect("UID not found");
    assert_eq!(ctis_by_id, &uids::CTImageStorage);
}

/// Sanity-check of the pre-defined TransferSyntax's to ensure
/// that their defined properties reflect the UID's name.
/// May catch issues with improperly copying over values from definitions.
#[test]
fn test_ts_name_vs_properties() {
    for (_, ts) in TS_BY_IDENT.entries() {
        let contains_little: bool = ts.uid.get_ident().contains("LittleEndian");
        let contains_big: bool = ts.uid.get_ident().contains("BigEndian");
        let contains_explicit: bool = ts.uid.get_ident().contains("ExplicitVR");
        let contains_implicit: bool = ts.uid.get_ident().contains("ImplicitVR");
        let contains_deflate: bool = ts.uid.get_ident().contains("Deflate");
        let contains_encapsulated: bool = ts.uid.get_ident().contains("Encapsulated");

        if contains_little {
            assert!(
                !ts.big_endian,
                "Name contains \"LittleEndian\" but is big_endian: {:?}",
                ts.uid
            );
        } else if contains_big {
            assert!(
                ts.big_endian,
                "Name contains \"BigEndian\" but is not big_endian: {:?}",
                ts.uid
            );
        } else {
            // Currently the defined/known TS's which don't have Big/Little in the name are LittleEndian
            assert!(
                !ts.big_endian,
                "Name contains no endian but is not big_endian: {:?}",
                ts.uid
            );
        }

        if contains_explicit {
            assert!(
                ts.explicit_vr,
                "Name contains \"ExplicitVR\" but is not explicit_vr: {:?}",
                ts.uid
            );
        } else if contains_implicit {
            assert!(
                !ts.explicit_vr,
                "Name contains \"ImplicitVR\" but is explicit_vr: {:?}",
                ts.uid
            );
        } else {
            // Currently the defined/known TS's which don't have Implicit/Explicit in the name are Implicit
            assert!(
                !ts.explicit_vr,
                "Name contains no vr but is not explicit_vr: {:?}",
                ts.uid
            );
        }

        assert_eq!(
            contains_deflate, ts.deflated,
            "Name contains \"Deflate\" but is not deflated: {:?}",
            ts.uid
        );
        assert_eq!(
            contains_encapsulated, ts.encapsulated,
            "Name contains \"Encapsulated\" but is not encapsulated: {:?}",
            ts.uid
        );
    }
}
