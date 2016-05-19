extern crate walkdir;

use std::fs::File;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::Path;

use walkdir::{DirEntry, WalkDir, WalkDirIterator};

static FIXTURE_DATASET1_FOLDER: &'static str = "fixtures/dataset1";

trait DicomStream : Read + Seek {}
impl DicomStream for File {}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn main() {
	let dirwalker: walkdir::Iter = WalkDir::new(FIXTURE_DATASET1_FOLDER)
										.min_depth(1)
										.max_depth(1)
										.into_iter();
	let dir_entries = dirwalker.filter_entry(|e| !is_hidden(e));
	for entry_res in dir_entries {
		if entry_res.is_ok() {
			let entry: DirEntry = entry_res.unwrap();
			if entry.file_type().is_file() {
				let file_path: &Path = entry.path();
				let file_stream_result: Result<File, Error> = File::open(file_path);
				if file_stream_result.is_err() {
					writeln!(
						&mut std::io::stderr(),
						"[ERROR] Error reading file: {:?} - {}",
						file_path,
						file_stream_result.err().unwrap()
					).unwrap();
					continue;
				}
				
				let fstream : File = file_stream_result.unwrap();
				let is_dcm_result : Result<bool, Error> = is_standard_dicom(fstream);
				if is_dcm_result.is_ok() {
					let is_dcm : bool = is_dcm_result.ok().expect("wut");
					if is_dcm {
						println!("[INFO] File is DICOM: {:?}", file_path);
					} else {
						println!("[INFO] File is not DICOM: {:?}", file_path);
					}
				} else {
					writeln!(
						&mut std::io::stderr(),
						"[ERROR] Error parsing file as DICOM: {:?} - {:?}",
						file_path,
						is_dcm_result.err()
					).unwrap();
				}
			}
		} else {
			writeln!(
				&mut std::io::stderr(),
				"[ERROR] Error reading path: {}",
				entry_res.err().unwrap()
			).unwrap();
		}
	}
}

fn do_thing<'streamlife>(entry: Result<DirEntry, Error>) -> Result<&'streamlife DicomStream, Error> {
	let entry: DirEntry = try!(entry);
	if !entry.file_type().is_file() {
		return Result::Err(Error::new(ErrorKind::InvalidData, "File is a directory"));
	}

	let file_path: &Path = entry.path();
	let fstream: File = try!(File::open(file_path));
	let is_dcm: bool = try!(is_standard_dicom(fstream));
	if is_dcm {
		return Result::Ok(&fstream as &DicomStream);
		//println!("[INFO] File is DICOM: {:?}", file_path);
	}
	//println!("[INFO] File is not DICOM: {:?}", file_path);
	return Result::Err(Error::new(ErrorKind::InvalidData, "File is not DICOM"));
}

fn is_standard_dicom<StreamType: Read + Seek>(mut stream : StreamType) -> Result<bool, Error> {
	let filler_size : usize = 128;
	let preamble_size : usize = 4;
	let buf_size : usize = filler_size + preamble_size;
	
	let start_pos : u64 = try!(stream.seek(SeekFrom::Current(0)));
	
	let mut buffer : Vec<u8> = vec![0;buf_size];
	try!(stream.read_exact(buffer.as_mut_slice()));
	
	// check that first 128 bytes are 0, followed by 'D', 'I', 'C', 'M'
	
	for n in 0..(filler_size) {
		if buffer[n] != 0 {
			try!(stream.seek(SeekFrom::Start(start_pos)));
			return Result::Ok(false);
		}
	}
	
	let preamble : Vec<u8> = vec!['D' as u8, 'I' as u8, 'C' as u8, 'M' as u8];
	let slice : &[u8] = &buffer[filler_size..filler_size + preamble_size];
	for n in 0..preamble_size {
		if slice[n] != preamble[n] {
			try!(stream.seek(SeekFrom::Start(start_pos)));
			return Result::Ok(false);
		}
	}
	
	Result::Ok(true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
