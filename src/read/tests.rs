extern crate walkdir;

use read::DicomStream;

use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;

use walkdir::{DirEntry, WalkDir, WalkDirIterator};

static FIXTURE_DATASET1_FOLDER: &'static str = "fixtures/dataset1";

struct TestDicomStream {
    data: Vec<u8>,
    pos: usize,
}

impl TestDicomStream {
    pub fn standard_dicom_preamble() -> DicomStream<TestDicomStream> {
        DicomStream::new(TestDicomStream {
            data: {
                let mut data: Vec<u8> = vec![0u8;132];
                data[128] = 'D' as u8;
                data[129] = 'I' as u8;
                data[130] = 'C' as u8;
                data[131] = 'M' as u8;
                data
            },
            pos: 0,
        })
    }

    pub fn invalid_dicom_preamble() -> DicomStream<TestDicomStream> {
        DicomStream::new(TestDicomStream {
            data: {
                let mut data: Vec<u8> = vec![0u8;132];
                data[128] = 'D' as u8;
                data[129] = 'O' as u8;
                data[130] = 'C' as u8;
                data[131] = 'M' as u8;
                data
            },
            pos: 0,
        })
    }

    pub fn invalid_size_preamble() -> DicomStream<TestDicomStream> {
        DicomStream::new(TestDicomStream {
            data: {
                let mut data: Vec<u8> = vec![0u8;132];
                data[127] = 'D' as u8;
                data[128] = 'I' as u8;
                data[129] = 'C' as u8;
                data[130] = 'M' as u8;
                data
            },
            pos: 0,
        })
    }

    pub fn standard_dicom_preamble_diff_startpos() -> DicomStream<TestDicomStream> {
        DicomStream::new(TestDicomStream {
            data: {
                let mut data: Vec<u8> = vec![0u8;132];
                data[128] = 'D' as u8;
                data[129] = 'I' as u8;
                data[130] = 'C' as u8;
                data[131] = 'M' as u8;
                data
            },
            pos: 131,
        })
    }
}

impl Read for TestDicomStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if self.pos >= self.data.len() {
            return Result::Ok(0);
        }

        let mut count: usize = 0;
        for i in self.pos..self.data.len() {
            if count >= buf.len() {
                break;
            }
            buf[count] = self.data[i];
            count += 1;
        }
        self.pos = self.pos + count;
        Result::Ok(count)
    }
}

impl Seek for TestDicomStream {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
        let newpos: usize = match pos {
            SeekFrom::Start(n) => 0usize.saturating_add(n as usize),
            SeekFrom::Current(n) => self.pos.saturating_add(n as usize),
            SeekFrom::End(n) => self.data.len().saturating_sub(n as usize),
        };

        if newpos < self.data.len() {
            self.pos = newpos;
            return Result::Ok(newpos as u64);
        }

        return Result::Err(Error::new(ErrorKind::UnexpectedEof,
                                      format!("seek to invalid position: {:?}", newpos)));
    }
}

#[test]
fn test_preambles() {
    let mut test_good_stream: DicomStream<TestDicomStream> = TestDicomStream::standard_dicom_preamble();
    let is_dcm: bool = test_good_stream.is_standard_dicom().expect("unable to inspect stream");
    assert_eq!(is_dcm, true);

    let mut test_bad_stream: DicomStream<TestDicomStream> = TestDicomStream::invalid_dicom_preamble();
    let is_dcm: bool = test_bad_stream.is_standard_dicom().expect("unable to inspect stream");
    assert_eq!(is_dcm, false);

    let mut test_size_stream: DicomStream<TestDicomStream> = TestDicomStream::invalid_size_preamble();
    let is_dcm: bool = test_size_stream.is_standard_dicom().expect("unable to inspect stream");
    assert_eq!(is_dcm, false);

    let mut test_diffpos_stream: DicomStream<TestDicomStream> = TestDicomStream::standard_dicom_preamble_diff_startpos();
    let is_dcm: bool = test_diffpos_stream.is_standard_dicom().expect("unable to inspect stream");
    assert_eq!(is_dcm, true);
}

#[test]
fn test_multiple_stddcm_checks_leave_stream_pos_in_place() {
    let mut test_stream: DicomStream<TestDicomStream> = TestDicomStream::standard_dicom_preamble_diff_startpos();
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
    let dir_entries = dirwalker_iter.filter_entry(|e| !::read::is_hidden(e.path()));
    for entry_res in dir_entries {
        let entry: DirEntry = entry_res.unwrap();
        let path: &Path = entry.path();
        let is_dcm: bool = DicomStream::new_from_path(path)
            .expect("unable to inspect file")
            .is_standard_dicom()
            .expect("unable to inspect file stream");
        assert_eq!(is_dcm, true);
    }
}
