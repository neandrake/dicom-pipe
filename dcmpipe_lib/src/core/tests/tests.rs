extern crate phf;
extern crate walkdir;

use crate::core::dcmobject::DicomObject;
use crate::core::dcmparser::{
    Parser, ParserBuilder, DICOM_PREFIX, DICOM_PREFIX_LENGTH, FILE_PREAMBLE_LENGTH,
};
use crate::core::dcmreader::parse_stream;
use crate::core::tagstop::TagStop;
use crate::core::tests::mock::MockDicomStream;
use crate::defn::vr;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

static FIXTURE_DATASET1_FOLDER: &'static str = "../fixtures/dataset1";

const PIXEL_DATA: u32 = 0x7FE0_0010;
const SOP_CLASS_UID: u32 = 0x0008_0016;
static CT_IMAGE_STORAGE_UID: &'static str = "1.2.840.10008.5.1.4.1.1.2";

#[test]
fn test_good_preamble() {
    let mut test_good_iter: Parser<MockDicomStream> = MockDicomStream::standard_dicom_preamble();

    // reads the preamble, prefix, and first element (mock has no first element)
    let _ = test_good_iter
        .next()
        .expect("Should be able to iterate a valid dicom stream");

    let is_dcm: bool = is_standard_preamble(&test_good_iter);
    assert_eq!(is_dcm, true);
}

#[test]
fn test_nonzero_preamble() {
    let mut test_size_iter: Parser<MockDicomStream> = MockDicomStream::nonzero_preamble();

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
    let mut test_bad_iter: Parser<MockDicomStream> = MockDicomStream::invalid_dicom_prefix();

    // reads the preamble, prefix, and first element
    let _ = test_bad_iter
        .next()
        .expect("An invalid prefix should return Some(Err)")
        .expect("Invalid prefix should be Err, this expect should cause panic");
}

#[test]
#[should_panic(expected = "failed to fill whole buffer")]
fn test_failure_to_read_preamble() {
    let mut test_iter: Parser<MockDicomStream> =
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
    let tagstop: u32 = PIXEL_DATA;
    let mut dicom_iter: Parser<File> = get_first_file_stream(TagStop::BeforeTag(tagstop));

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
    let mut dicom_iter: Parser<File> = get_first_file_stream(TagStop::EndOfStream);

    let dcmobj: DicomObject = parse_stream(&mut dicom_iter).expect("Should be no probz");
    let sop_class_uid: &DicomObject = dcmobj
        .get_object(SOP_CLASS_UID)
        .expect("Should have SOP Class UID");
    if let Some(element) = sop_class_uid.as_element() {
        assert_eq!(
            element.parse_string_with_vr(&vr::UI).expect("get cs"),
            CT_IMAGE_STORAGE_UID
        )
    } else {
        panic!("Element should exist")
    }
}

fn get_first_file_stream(tagstop: TagStop) -> Parser<File> {
    let dirwalker: WalkDir = WalkDir::new(FIXTURE_DATASET1_FOLDER)
        .min_depth(1)
        .max_depth(1);

    for entry_res in dirwalker.into_iter() {
        let entry: DirEntry = entry_res.unwrap();
        let path: &Path = entry.path();

        let file: File = File::open(path).expect(&format!("Unable to open file: {:?}", path));

        let dstream: Parser<File> = ParserBuilder::new(file).tagstop(tagstop).build();

        return dstream;
    }

    panic!("No DICOM files found");
}

/// Checks that the first 132 bytes are 128 0's followed by 'DICM'
fn is_standard_preamble<StreamType>(stream: &Parser<StreamType>) -> bool
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
