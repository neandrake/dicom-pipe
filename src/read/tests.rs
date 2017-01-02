#![cfg(test)]

extern crate byteorder;
extern crate walkdir;

use byteorder::ReadBytesExt;

use read::dcmelement::DicomElement;
use read::dcmstream::{DicomStream, DICOM_PREFIX, DICOM_PREFIX_LENGTH, FILE_PREAMBLE_LENGTH};
use read::mock::MockDicomStream;

use std::fs::File;
use std::io::Seek;
use std::path::Path;

use walkdir::{DirEntry, WalkDir};

static FIXTURE_DATASET1_FOLDER: &'static str = "fixtures/dataset1";

#[test]
fn test_good_preamble() {
    let mut test_good_stream: DicomStream<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble();
    test_good_stream.read_file_preamble()
        .expect("Unable to read file preamble");
    test_good_stream.read_dicom_prefix()
        .expect("Unable to read DICOM prefix");
    let is_dcm: bool = is_standard_preamble(&test_good_stream);
    assert_eq!(is_dcm, true);
}

#[test]
fn test_nonzero_preamble() {
    let mut test_size_stream: DicomStream<MockDicomStream> =
        MockDicomStream::nonzero_preamble();
    test_size_stream.read_file_preamble()
        .expect("Unable to read file preamble");
    test_size_stream.read_dicom_prefix()
        .expect("Unable to read DICOM prefix");
    let is_dcm: bool = is_standard_preamble(&test_size_stream);
    assert_eq!(is_dcm, false);
}

#[test]
#[should_panic(expected = "Invalid DICOM Prefix")]
fn test_bad_dicom_prefix() {
    let mut test_bad_stream: DicomStream<MockDicomStream> =
        MockDicomStream::invalid_dicom_prefix();
    test_bad_stream.read_file_preamble()
        .expect("Unable to read file preamble");
    test_bad_stream.read_dicom_prefix()
        .expect("This should fail to read a valid prefix");
    let is_dcm: bool = is_standard_preamble(&test_bad_stream);
    assert_eq!(is_dcm, false);
}

#[test]
#[should_panic(expected = "failed to fill whole buffer")]
fn test_failure_to_read_preamble() {
    let mut test_stream: DicomStream<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble_diff_startpos_and_short_stream();
    test_stream.read_file_preamble()
        .expect("This should fail to read preamble due to not enough data");
    let start_pos: usize = test_stream.get_stream().pos;
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

        let mut dstream: DicomStream<File> = DicomStream::new_from_path(path, )
            .expect("Unable to read file");

        dstream.read_file_meta()
            .expect(&format!("Unable to read FileMetaInformation: {:?}", path));
        let is_dcm = is_standard_preamble(&dstream);
        assert!(is_dcm);

        // Ability to read dicom elements after FileMetaInformation
        // means that we interpret the transfer syntax properly
        let elem: DicomElement = dstream.read_dicom_element().expect("Unable to read element");
        println!("Read Element: {:?}", elem);
    }
}

/// Checks that the first 132 bytes are 128 0's followed by 'DICM'
fn is_standard_preamble<StreamType>(stream: &DicomStream<StreamType>) -> bool
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