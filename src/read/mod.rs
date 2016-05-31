#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;

pub struct DicomStream<StreamType> {
    stream: StreamType,
}

impl DicomStream<File> {
    pub fn new_from_path(path: &Path) -> Result<DicomStream<File>, Error> {
        if !path.is_file() {
            return Result::Err(Error::new(ErrorKind::InvalidData, format!("Invalid file: {:?}", path)));
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
        }
    }
    
    pub fn is_standard_dicom(&mut self) -> Result<bool, Error> {
        let standard_preamble: Vec<u8> = vec!['D' as u8, 'I' as u8, 'C' as u8, 'M' as u8];
        let filler_size: usize = 128;
        let preamble_size: usize = standard_preamble.len();
        let buf_size: usize = filler_size + preamble_size;

        // mark the current position, seek to beginning of stream to read preamble, seek back to current position
        let start_pos: u64 = try!(self.stream.seek(SeekFrom::Current(0)));
        try!(self.stream.seek(SeekFrom::Start(0)));
        let mut buffer: Vec<u8> = vec![0;buf_size];
        try!(self.stream.read_exact(buffer.as_mut_slice()));
        try!(self.stream.seek(SeekFrom::Start(start_pos)));

        // check that first 128 bytes are 0, followed by 'D', 'I', 'C', 'M'
        for n in 0..filler_size {
            if buffer[n] != 0 {
                return Result::Ok(false);
            }
        }

        let slice: &[u8] = &buffer[filler_size..filler_size + preamble_size];
        for n in 0..preamble_size {
            if slice[n] != standard_preamble[n] {
                return Result::Ok(false);
            }
        }

        Result::Ok(true)
    }
}


pub fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .map(|s| s.to_str().unwrap_or(""))
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
