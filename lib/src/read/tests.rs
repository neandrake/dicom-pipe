extern crate byteorder;
extern crate walkdir;

use byteorder::ReadBytesExt;
use core::dict::dicom_elements as tags;
use read::dcmparser::{DicomStreamParser, DICOM_PREFIX, DICOM_PREFIX_LENGTH, FILE_PREAMBLE_LENGTH};
use read::mock::MockDicomStream;
use read::tagstop::TagStop;
use std::fs::File;
use std::io::Seek;
use std::path::Path;
use self::walkdir::{DirEntry, WalkDir};

static FIXTURE_DATASET1_FOLDER: &'static str = "../fixtures/dataset1";

#[test]
fn test_good_preamble() {
    let mut test_good_iter: DicomStreamParser<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble();

    // reads the preamble, prefix, and first element
    let _ = test_good_iter.next()
        .expect("Value should be Some Error");

    let is_dcm: bool = is_standard_preamble(&test_good_iter);
    assert_eq!(is_dcm, true);
}

#[test]
fn test_nonzero_preamble() {
    let mut test_size_iter: DicomStreamParser<MockDicomStream> =
        MockDicomStream::nonzero_preamble();

    // reads the preamble, prefix, and first element
    let _ = test_size_iter.next()
        .expect("Value should be Some Error");

    let is_dcm: bool = is_standard_preamble(&test_size_iter);
    assert_eq!(is_dcm, false);
}

#[test]
#[should_panic(expected = "Invalid DICOM Prefix")]
fn test_bad_dicom_prefix() {
    let mut test_bad_iter: DicomStreamParser<MockDicomStream> =
        MockDicomStream::invalid_dicom_prefix();

    // reads the preamble, prefix, and first element
    let _ = test_bad_iter.next()
        .expect("Value should be Some Error")
        .expect("This should fail to read a valid prefix");

    let is_dcm: bool = is_standard_preamble(&test_bad_iter);
    assert_eq!(is_dcm, false);
}

#[test]
#[should_panic(expected = "failed to fill whole buffer")]
fn test_failure_to_read_preamble() {
    let mut test_iter: DicomStreamParser<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble_diff_startpos_and_short_stream();

    // reads the preamble, prefix, and first element
    let _first_elem = test_iter.next()
        .expect("Value should be Some Error")
        .expect("This should fail to read preamble due to not enough data");

    // should record zero bytes read since the first attempt to read into buffer should fail to fill
    let start_pos: u64 = test_iter.bytes_read();
    assert_eq!(start_pos, 0);
}

#[test]	// slow
fn test_parse_known_dicom_files() {
    let tagstop: u32 = tags::PixelData.tag;
    let mut dicom_iter: DicomStreamParser<File> = get_first_file_stream(TagStop::BeforeTag(tagstop));

    let _first_elem = dicom_iter.next()
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
    let stopped_at_tag = dicom_iter.partial_tag()
        .expect("Iteration should have stopped after reading the PixelData tag");
    assert_eq!(tagstop, stopped_at_tag);
}

fn get_first_file_stream(tagstop: TagStop) -> DicomStreamParser<File> {
    let dirwalker: WalkDir = WalkDir::new(FIXTURE_DATASET1_FOLDER)
        .min_depth(1)
        .max_depth(1);

    for entry_res in dirwalker.into_iter() {
        let entry: DirEntry = entry_res.unwrap();
        let path: &Path = entry.path();

        let file: File = File::open(path)
            .expect(&format!("Unable to open file: {:?}", path));

        let dstream: DicomStreamParser<File> = DicomStreamParser::new(file, tagstop);

        return dstream;
    }

    panic!("No DICOM files found");
}

/// Checks that the first 132 bytes are 128 0's followed by 'DICM'
fn is_standard_preamble<StreamType>(stream: &DicomStreamParser<StreamType>) -> bool
    where StreamType: ReadBytesExt + Seek {
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