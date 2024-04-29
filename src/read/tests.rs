extern crate walkdir;

use read::DicomStream;
use read::mock::MockDicomStream;

use std::path::Path;

use walkdir::{DirEntry, WalkDir};

static FIXTURE_DATASET1_FOLDER: &'static str = "fixtures/dataset1";

#[test]
fn test_preambles() {
    let mut test_good_stream: DicomStream<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble();
    let is_dcm: bool = test_good_stream.is_standard_dicom().expect("unable to inspect stream");
    assert_eq!(is_dcm, true);

    let mut test_bad_stream: DicomStream<MockDicomStream> =
        MockDicomStream::invalid_dicom_preamble();
    let is_dcm: bool = test_bad_stream.is_standard_dicom().expect("unable to inspect stream");
    assert_eq!(is_dcm, false);

    let mut test_size_stream: DicomStream<MockDicomStream> =
        MockDicomStream::invalid_size_preamble();
    let is_dcm: bool = test_size_stream.is_standard_dicom().expect("unable to inspect stream");
    assert_eq!(is_dcm, false);

    let mut test_diffpos_stream: DicomStream<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble_diff_startpos();
    let is_dcm: bool = test_diffpos_stream.is_standard_dicom().expect("unable to inspect stream");
    assert_eq!(is_dcm, true);
}

#[test]
fn test_multiple_stddcm_checks_leave_stream_pos_in_place() {
    let mut test_stream: DicomStream<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble_diff_startpos();
    let start_pos: usize = test_stream.stream.pos;
    assert!(start_pos != 0);

    test_stream.is_standard_dicom().expect("unable to inspect stream");
    let end_pos: usize = test_stream.stream.pos;
    assert_eq!(start_pos, end_pos);

    test_stream.is_standard_dicom().expect("unable to inspect stream");
    let end_pos = test_stream.stream.pos;
    assert_eq!(start_pos, end_pos);
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
        let is_dcm: bool = DicomStream::new_from_path(path)
            .expect("unable to inspect file")
            .is_standard_dicom()
            .expect("unable to inspect file stream");
        assert_eq!(is_dcm, true);
    }
}
