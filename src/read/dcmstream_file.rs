use read::dcmstream::DicomStream;

use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::Path;

impl DicomStream<File> {
    pub fn new_from_path(path: &Path) -> Result<DicomStream<File>, Error> {
        if !path.is_file() {
            return Err(Error::new(ErrorKind::InvalidData,
                                          format!("Invalid path: {:?}", path)));
        }

        let file: File = File::open(path)?;
        Ok::<DicomStream<File>, Error>(DicomStream::new(file))
    }
}