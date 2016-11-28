#![cfg(test)]

extern crate walkdir;

use read::DicomStream;
use read::mock::MockDicomStream;

use std::fs::File;
use std::path::Path;

use walkdir::{DirEntry, WalkDir};

static FIXTURE_DATASET1_FOLDER: &'static str = "fixtures/dataset1";

#[test]
fn test_good_preamble() {
    let mut test_good_stream: DicomStream<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble();
    test_good_stream.read_file_preamble().expect("Unable to read file preamble");
    test_good_stream.read_dicom_prefix().expect("Unable to read DICOM prefix");
    let is_dcm: bool = test_good_stream.is_standard_preamble();
    assert_eq!(is_dcm, true);
}

#[test]
fn test_nonzero_preamble() {
    let mut test_size_stream: DicomStream<MockDicomStream> =
        MockDicomStream::nonzero_preamble();
    test_size_stream.read_file_preamble().expect("Unable to read file preamble");
    test_size_stream.read_dicom_prefix().expect("Unable to read DICOM prefix");
    let is_dcm: bool = test_size_stream.is_standard_preamble();
    assert_eq!(is_dcm, false);
}

#[test]
#[should_panic(expected = "Invalid DICOM Prefix")]
fn test_bad_dicom_prefix() {
    let mut test_bad_stream: DicomStream<MockDicomStream> =
        MockDicomStream::invalid_dicom_prefix();
    test_bad_stream.read_file_preamble().expect("Unable to read file preamble");
    test_bad_stream.read_dicom_prefix().expect("This should fail to read a valid prefix");
    let is_dcm: bool = test_bad_stream.is_standard_preamble();
    assert_eq!(is_dcm, false);
}

#[test]
#[should_panic(expected = "failed to fill whole buffer")]
fn test_failure_to_read_preamble() {
    let mut test_stream: DicomStream<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble_diff_startpos_and_short_stream();
    test_stream.read_file_preamble().expect("This should fail to read preamble due to not enough data");
    let start_pos: usize = test_stream.stream.pos;
    assert!(start_pos != 0);
}

#[test]	// slow
fn test_parse_known_dicom_files() {
    let dirwalker: WalkDir = WalkDir::new(FIXTURE_DATASET1_FOLDER)
        .min_depth(1)
        .max_depth(1);
    let dirwalker_iter: walkdir::Iter = dirwalker.into_iter();
    for entry_res in dirwalker_iter {
        let entry: DirEntry = entry_res.unwrap();
        let path: &Path = entry.path();
        let mut dstream: DicomStream<File> = DicomStream::new_from_path(path)
            .expect("Unable to read file");
        dstream.read_file_preamble().expect("Unable to read file preamble");
        dstream.read_dicom_prefix().expect("Unable to read DICOM prefix");
        let is_dcm = dstream.is_standard_preamble();
        assert_eq!(is_dcm, true);
    }
}
