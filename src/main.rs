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
		
		let start_pos : u64 = try!(self.seek(SeekFrom::Current(0)));
		
		let mut buffer : Vec<u8> = vec![0;buf_size];
		try!(self.read_exact(buffer.as_mut_slice()));
		
		// check that first 128 bytes are 0, followed by 'D', 'I', 'C', 'M'
		
		for n in 0..(filler_size) {
			if buffer[n] != 0 {
				try!(self.seek(SeekFrom::Start(start_pos)));
				return Result::Ok(false);
			}
		}
		
		let preamble : Vec<u8> = vec!['D' as u8, 'I' as u8, 'C' as u8, 'M' as u8];
		let slice : &[u8] = &buffer[filler_size..filler_size + preamble_size];
		for n in 0..preamble_size {
			if slice[n] != preamble[n] {
				try!(self.seek(SeekFrom::Start(start_pos)));
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
			Ok(val) => {
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

struct TestDicomHeader {
	data: Vec<u8>,
	pos: usize,
}

impl TestDicomHeader {
	pub fn valid_dicom_preamble() -> TestDicomHeader {
		TestDicomHeader {
			data : {
				let mut data = vec![0u8;132];
				data[128] = 'D' as u8;
				data[129] = 'I' as u8;
				data[130] = 'C' as u8;
				data[131] = 'M' as u8;
				data
			},
			pos: 0,
		}
	}
}

impl Read for TestDicomHeader {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
		
		Result::Ok(0)
	}
}

impl Seek for TestDicomHeader {
	fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
		Result::Ok(0)
	}
}

impl DicomStream for TestDicomHeader {
	
}

#[test]
fn test_preamble() {
	
}

#[test]
fn test_parse_known_dicom() {
	let dirwalker: WalkDir = WalkDir::new(FIXTURE_DATASET1_FOLDER)
										.min_depth(1)
										.max_depth(1);
	let dirwalker_iter: walkdir::Iter = dirwalker.into_iter();
	let dir_entries = dirwalker_iter.filter_entry(|e| !is_hidden(e));
	for entry_res in dir_entries {
		let entry: DirEntry = entry_res.unwrap();
		open_file_as_dicom_stream(&entry).expect("File is not DICOM");
	}
}
