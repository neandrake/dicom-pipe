extern crate walkdir;

use std::fs::File;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::Path;

use walkdir::{DirEntry, WalkDir, WalkDirIterator};

static FIXTURE_DATASET1_FOLDER: &'static str = "fixtures/dataset1";

trait DicomStream : Read + Seek {
	fn is_standard_dicom(&mut self) -> Result<bool, Error> {
		let filler_size : usize = 128;
		let preamble_size : usize = 4;
		let buf_size : usize = filler_size + preamble_size;
		
		// mark the current position, seek to beginning of stream to read preamble, seek back to current position
		let start_pos : u64 = try!(self.seek(SeekFrom::Current(0)));
		try!(self.seek(SeekFrom::Start(0)));
		let mut buffer : Vec<u8> = vec![0;buf_size];
		try!(self.read_exact(buffer.as_mut_slice()));
		try!(self.seek(SeekFrom::Start(start_pos)));
		
		// check that first 128 bytes are 0, followed by 'D', 'I', 'C', 'M'
		
		for n in 0..(filler_size) {
			if buffer[n] != 0 {
				return Result::Ok(false);
			}
		}
		
		let preamble : Vec<u8> = vec!['D' as u8, 'I' as u8, 'C' as u8, 'M' as u8];
		let slice : &[u8] = &buffer[filler_size..filler_size + preamble_size];
		for n in 0..preamble_size {
			if slice[n] != preamble[n] {
				return Result::Ok(false);
			}
		}
		
		Result::Ok(true)
	}
}

impl DicomStream for File {}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn main() {
	let dirwalker: WalkDir = WalkDir::new(FIXTURE_DATASET1_FOLDER)
										.min_depth(1)
										.max_depth(1);
	match parse_directory(dirwalker) {
		Ok(()) => {
			
		},
		Err(err) => {
			writeln!(
				&mut std::io::stderr(),
				"[ERROR] Error accessing directory: {:?}",
				err
			).unwrap();
		}
	}
}

fn parse_directory(dirwalker: WalkDir) -> Result<(), Error> {
	let dirwalker_iter: walkdir::Iter = dirwalker.into_iter();
	let dir_entries = dirwalker_iter.filter_entry(|e| !is_hidden(e));
	for entry_res in dir_entries {
		let entry: DirEntry = try!(entry_res);
		let dstream_res: Result<Box<DicomStream>, Error> = open_file_as_dicom_stream(&entry);
		match dstream_res {
			Ok(_) => {
				println!("[DEBUG] File is DICOM: {:?}", entry.path());
			},
			Err(err) => {
				writeln!(
					&mut std::io::stderr(),
					"[ERROR] Error parsing file as DICOM: {:?}",
					err
				).unwrap();
			}
		}
	}
	return Result::Ok(());
}

fn open_file_as_dicom_stream(entry: &DirEntry) -> Result<Box<DicomStream>, Error> {
	if !entry.file_type().is_file() {
		return Result::Err(Error::new(ErrorKind::InvalidData, "File is a directory"));
	}

	let file_path: &Path = entry.path();
	let mut fstream: File = try!(File::open(file_path));
	let is_dcm: bool = try!(fstream.is_standard_dicom());
	if is_dcm {
		return Result::Ok(Box::new(fstream));
	}
	return Result::Err(Error::new(ErrorKind::InvalidData, format!("File is not DICOM: {:?}", file_path)));
}

// --- Test

#[cfg(test)]
struct TestDicomStream {
	data: Vec<u8>,
	pos: usize,
}

#[cfg(test)]
impl TestDicomStream {
	pub fn standard_dicom_preamble() -> TestDicomStream {
		TestDicomStream {
			data : {
				let mut data: Vec<u8> = vec![0u8;132];
				data[128] = 'D' as u8;
				data[129] = 'I' as u8;
				data[130] = 'C' as u8;
				data[131] = 'M' as u8;
				data
			},
			pos: 0,
		}
	}
	
	pub fn invalid_dicom_preamble() -> TestDicomStream {
		TestDicomStream {
			data : {
				let mut data: Vec<u8> = vec![0u8;132];
				data[128] = 'D' as u8;
				data[129] = 'O' as u8;
				data[130] = 'C' as u8;
				data[131] = 'M' as u8;
				data
			},
			pos: 0,
		}
	}
	
	pub fn invalid_size_preamble() -> TestDicomStream {
		TestDicomStream {
			data : {
				let mut data: Vec<u8> = vec![0u8;132];
				data[127] = 'D' as u8;
				data[128] = 'I' as u8;
				data[129] = 'C' as u8;
				data[130] = 'M' as u8;
				data
			},
			pos: 0,
		}
	}
	
	pub fn standard_dicom_preamble_diff_startpos() -> TestDicomStream {
		TestDicomStream {
			data : {
				let mut data: Vec<u8> = vec![0u8;132];
				data[128] = 'D' as u8;
				data[129] = 'I' as u8;
				data[130] = 'C' as u8;
				data[131] = 'M' as u8;
				data
			},
			pos: 131,
		}
	}
}

#[cfg(test)]
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

#[cfg(test)]
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
		
		return Result::Err(Error::new(
			ErrorKind::UnexpectedEof,
			format!("seek to invalid position: {:?}", newpos)));
	}
}

#[cfg(test)]
impl DicomStream for TestDicomStream {}

#[test]
fn test_preambles() {
	let mut test_good_stream: TestDicomStream = TestDicomStream::standard_dicom_preamble();
	let is_dcm: bool = test_good_stream.is_standard_dicom().expect("unable to inspect stream");
	assert_eq!(is_dcm, true);
	
	let mut test_bad_stream: TestDicomStream = TestDicomStream::invalid_dicom_preamble();
	let is_dcm: bool = test_bad_stream.is_standard_dicom().expect("unable to inspect stream");
	assert_eq!(is_dcm, false);
	
	let mut test_size_stream: TestDicomStream = TestDicomStream::invalid_size_preamble();
	let is_dcm: bool = test_size_stream.is_standard_dicom().expect("unable to inspect stream");
	assert_eq!(is_dcm, false);
	
	let mut test_diffpos_stream: TestDicomStream = TestDicomStream::standard_dicom_preamble_diff_startpos();
	let is_dcm: bool = test_diffpos_stream.is_standard_dicom().expect("unable to inspect stream");
	assert_eq!(is_dcm, true);
}

#[test]
fn test_multiple_stddcm_checks_leave_stream_pos_in_place() {
	let mut test_stream: TestDicomStream = TestDicomStream::standard_dicom_preamble_diff_startpos();
	let start_pos: usize = test_stream.pos;
	assert!(start_pos != 0);
	
	test_stream.is_standard_dicom().expect("unable to inspect stream");
	let end_pos: usize = test_stream.pos;
	assert_eq!(start_pos, end_pos);
	
	test_stream.is_standard_dicom().expect("unable to inspect stream");
	let end_pos = test_stream.pos;
	assert_eq!(start_pos, end_pos);
}

#[test]	// slow
fn test_parse_known_dicom_files() {
	let dirwalker: WalkDir = WalkDir::new(FIXTURE_DATASET1_FOLDER)
										.min_depth(1)
										.max_depth(1);
	let dirwalker_iter: walkdir::Iter = dirwalker.into_iter();
	let dir_entries = dirwalker_iter.filter_entry(|e| !is_hidden(e));
	for entry_res in dir_entries {
		let entry: DirEntry = entry_res.unwrap();
		let is_dcm: bool = open_file_as_dicom_stream(&entry)
			.expect("unable to inspect file")
			.is_standard_dicom()
			.expect("unable to inspect file stream");
		assert_eq!(is_dcm, true);
	}
}
