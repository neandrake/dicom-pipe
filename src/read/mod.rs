#[cfg(test)]
mod tests;
#[cfg(test)]
mod mock;

use std::fs::File;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;

const FILE_PREAMBLE_SIZE: usize = 128;
static DICOM_PREFIX: [u8; 4] = ['D' as u8, 'I' as u8, 'C' as u8, 'M' as u8];

pub struct DicomStream<StreamType> {
    stream: StreamType,

    file_preamble: [u8;128],
    dicom_prefix: [u8;4],
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
        DicomStream {
            stream: stream,
            file_preamble: [0u8;128],
            dicom_prefix: [0u8;4],
        }
    }

    pub fn read_file_preamble(&mut self) -> Result<(), Error> {
        let start_pos: u64 = try!(self.stream.seek(SeekFrom::Current(0)));
        try!(self.stream.seek(SeekFrom::Start(0)));
        try!(self.stream.read_exact(&mut self.file_preamble));
        try!(self.stream.seek(SeekFrom::Start(start_pos)));
        Result::Ok(())
    }

    pub fn read_dicom_prefix(&mut self) -> Result<(), Error> {
        let start_pos: u64 = try!(self.stream.seek(SeekFrom::Current(0)));
        try!(self.stream.seek(SeekFrom::Start(128)));
        try!(self.stream.read_exact(&mut self.dicom_prefix));
        try!(self.stream.seek(SeekFrom::Start(start_pos)));

        // check that first 128 bytes are 0, followed by 'D', 'I', 'C', 'M'
        for n in 0..DICOM_PREFIX.len() {
            if self.dicom_prefix[n] != DICOM_PREFIX[n] {
                return Result::Err(Error::new(ErrorKind::InvalidData,
                                    format!("Invalid DICOM Prefix: {:?}", self.dicom_prefix)));
            }
        }

        return Result::Ok(())
    }

    pub fn read_file_meta(&self) {

    }

    pub fn is_standard_dicom(&mut self) -> Result<bool, Error> {
        let buf_size: usize = FILE_PREAMBLE_SIZE + DICOM_PREFIX.len();

        // mark the current position, seek to beginning of stream to read preamble, seek back to current position
        let start_pos: u64 = try!(self.stream.seek(SeekFrom::Current(0)));
        try!(self.stream.seek(SeekFrom::Start(0)));
        let mut buffer: Vec<u8> = vec![0;buf_size];
        try!(self.stream.read_exact(buffer.as_mut_slice()));
        try!(self.stream.seek(SeekFrom::Start(start_pos)));

        // check that first 128 bytes are 0, followed by 'D', 'I', 'C', 'M'
        for n in 0..FILE_PREAMBLE_SIZE {
            if buffer[n] != 0 {
                return Result::Ok(false);
            }
        }

        let slice: &[u8] = &buffer[FILE_PREAMBLE_SIZE..FILE_PREAMBLE_SIZE + DICOM_PREFIX.len()];
        for n in 0..DICOM_PREFIX.len() {
            if slice[n] != DICOM_PREFIX[n] {
                return Result::Ok(false);
            }
        }

        Result::Ok(true)
    }
}
