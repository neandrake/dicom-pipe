#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;

static PREAMBLE_HEADER: usize = 128;
static PREAMBLE_SIZE: usize = 4;
static PREAMBLE_STANDARD: [u8; 4] = ['D' as u8, 'I' as u8, 'C' as u8, 'M' as u8];

pub struct DicomStream<StreamType> {
    stream: StreamType,
}

impl DicomStream<File> {
    pub fn new_from_path(path: &Path) -> Result<DicomStream<File>, Error> {
        if !path.is_file() {
            return Result::Err(Error::new(ErrorKind::InvalidData,
                                          format!("Invalid path: {:?}", path)));
        }

        let mut fstream: DicomStream<File> = DicomStream::new(try!(File::open(path)));
        let is_dcm: bool = try!(fstream.is_standard_dicom());
        if is_dcm {
            return Result::Ok(fstream);
        }
        return Result::Err(Error::new(ErrorKind::InvalidData,
                                      format!("File is not DICOM: {:?}", path)));
    }
}

impl<StreamType: Read + Seek> DicomStream<StreamType> {
    pub fn new(stream: StreamType) -> DicomStream<StreamType> {
        DicomStream { stream: stream }
    }

    pub fn is_standard_dicom(&mut self) -> Result<bool, Error> {
        let buf_size: usize = PREAMBLE_HEADER + PREAMBLE_SIZE;

        // mark the current position, seek to beginning of stream to read preamble, seek back to current position
        let start_pos: u64 = try!(self.stream.seek(SeekFrom::Current(0)));
        try!(self.stream.seek(SeekFrom::Start(0)));
        let mut buffer: Vec<u8> = vec![0;buf_size];
        try!(self.stream.read_exact(buffer.as_mut_slice()));
        try!(self.stream.seek(SeekFrom::Start(start_pos)));

        // check that first 128 bytes are 0, followed by 'D', 'I', 'C', 'M'
        for n in 0..PREAMBLE_HEADER {
            if buffer[n] != 0 {
                return Result::Ok(false);
            }
        }

        let slice: &[u8] = &buffer[PREAMBLE_HEADER..PREAMBLE_HEADER + PREAMBLE_SIZE];
        for n in 0..PREAMBLE_SIZE {
            if slice[n] != PREAMBLE_STANDARD[n] {
                return Result::Ok(false);
            }
        }

        Result::Ok(true)
    }
}
