extern crate byteorder;
extern crate encoding;
extern crate walkdir;

mod read;
mod util;

use read::{DicomStream};

use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;

use walkdir::{DirEntry, WalkDir, WalkDirIterator};

// duplicated from read::tests::FIXTURE_DATASET1_FOLDER
static FIXTURE_DATASET1_FOLDER: &'static str = "fixtures/dataset1";

fn main() {
    let dirwalker: WalkDir = WalkDir::new(FIXTURE_DATASET1_FOLDER)
        .min_depth(1)
        .max_depth(1);
    match parse_directory(dirwalker) {
        Ok(()) => {}
        Err(err) => {
            writeln!(&mut std::io::stderr(),
                     "[ERROR] Error parsing directory: {:?}",
                     err)
                .unwrap();
        }
    }
}

pub fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .map(|s| s.to_str().unwrap_or(""))
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn parse_directory(dirwalker: WalkDir) -> Result<(), Error> {
    let dirwalker_iter: walkdir::Iter = dirwalker.into_iter();
    let dir_entries = dirwalker_iter.filter_entry(|e| !is_hidden(e.path()));
    for entry_res in dir_entries {
        let entry: DirEntry = try!(entry_res);
        let path: &Path = entry.path();
        let dstream_res: Result<DicomStream<File>, Error> = DicomStream::new_from_path(&path);
        match dstream_res {
            Ok(mut dstream) => {
                try!(dstream.read_file_meta());
                println!("[DEBUG] File is DICOM: {:?}", path);
            }
            Err(err) => {
                writeln!(&mut std::io::stderr(),
                         "[ERROR] Error parsing file as DICOM: {:?}",
                         err)
                    .unwrap();
            }
        }
    }
    return Result::Ok(());
}
