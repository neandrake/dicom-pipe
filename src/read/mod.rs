extern crate byteorder;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod mock;


use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

use std::fs::File;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;

use util::vr::VR;


const FILE_PREAMBLE_SIZE: usize = 128;
static DICOM_PREFIX: [u8; 4] = ['D' as u8, 'I' as u8, 'C' as u8, 'M' as u8];


pub struct DicomStream<StreamType> {
    stream: StreamType,

    file_preamble: [u8;128],
    dicom_prefix: [u8;4],
    
    /*
    file_meta_info_group_length: DicomElement,
    file_meta_info_version: DicomElement,
    
    media_storage_sop_class_uid: DicomElement,
    media_storage_sop_instance_uid: DicomElement,
    transfer_syntax_uid: DicomElement,
    implementation_class_uid: DicomElement,
    */

    dicom_header: Vec<DicomElement>,
}

pub struct DicomElement {
    pub tag: u32,
    pub vr: &'static VR,
    pub vl: u32,
    pub bytes: Vec<u8>,
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

impl<StreamType: ReadBytesExt + Seek> DicomStream<StreamType> {
    pub fn new(stream: StreamType) -> DicomStream<StreamType> {
        DicomStream {
            stream: stream,
            file_preamble: [0u8;128],
            dicom_prefix: [0u8;4],
            dicom_header: Vec::with_capacity(6),
        }
    }

    #[allow(dead_code)]
    pub fn get_stream(&self) -> &StreamType {
        &self.stream
    }

    #[allow(dead_code)]
    pub fn get_file_preamble(&self) -> &[u8;128] {
        &self.file_preamble
    }

    #[allow(dead_code)]
    pub fn get_dicom_prefix(&self) -> &[u8;4] {
        &self.dicom_prefix
    }

    pub fn read_file_preamble(&mut self) -> Result<(), Error> {
        try!(self.stream.read_exact(&mut self.file_preamble));
        Result::Ok(())
    }

    pub fn read_dicom_prefix(&mut self) -> Result<(), Error> {
        try!(self.stream.read_exact(&mut self.dicom_prefix));

        // check that first 128 bytes are 0, followed by 'D', 'I', 'C', 'M'
        for n in 0..DICOM_PREFIX.len() {
            if self.dicom_prefix[n] != DICOM_PREFIX[n] {
                return Result::Err(Error::new(ErrorKind::InvalidData,
                                    format!("Invalid DICOM Prefix: {:?}", self.dicom_prefix)));
            }
        }

        return Result::Ok(())
    }

    pub fn read_tag(&mut self) -> Result<u32, Error> {
        let first: u32 = (try!(self.stream.read_u16::<LittleEndian>()) as u32) << 16;
        let second: u32 = try!(self.stream.read_u16::<LittleEndian>()) as u32;
        let result: u32 = first + second;
        println!("READ TAG: {:?}", result);
        Result::Ok(result)
    }

    pub fn read_vr(&mut self) -> Result<&'static VR, Error> {
        let first: u8 = try!(self.stream.read_u8());
        let second: u8 = try!(self.stream.read_u8());
        let code: u16 = ((first as u16) << 8) + second as u16;
        println!("READ VR: {:?}", code);
        VR::code_to_vr(code).ok_or(Error::new(ErrorKind::InvalidData, format!("Unable to interpret VR: {:?}", code)))
    }

    pub fn read_value_length(&mut self, vr: &VR) -> Result<u32, Error> {
        match vr.explicit_vr_header_length {
            8 => self.stream.read_u16::<LittleEndian>().map(|n| n as u32),
            12 => {
                let first: u32 = (try!(self.stream.read_u16::<LittleEndian>()) as u32) << 16;
                let second: u32 = try!(self.stream.read_u16::<LittleEndian>()) as u32;
                Result::Ok(first + second)
            },
            n => Err(Error::new(ErrorKind::InvalidData, format!("Invalid VR Header Length: {:?}", n))),
        }
    }

    pub fn read_value_field(&mut self, value_length: u32) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = vec![0;value_length as usize];
        try!(self.stream.read_exact(bytes.as_mut_slice()));
        println!("READ VALUE LENGTH: {:?}", value_length);
        Result::Ok(bytes)
    }

    pub fn read_dicom_element(&mut self) -> Result<DicomElement, Error> {
        let tag: u32 = try!(self.read_tag());
        let vr: &VR = try!(self.read_vr());
        let vl: u32 = try!(self.read_value_length(vr));
        let bytes: Vec<u8> = try!(self.read_value_field(vl));

        Result::Ok(DicomElement {
            tag: tag,
            vr: vr,
            vl: vl,
            bytes: bytes,
        })
    }

    pub fn read_file_meta(&mut self) -> Result<(), Error> {
        try!(self.read_file_preamble());
        try!(self.read_dicom_prefix());

        let file_meta_info_group_length: DicomElement = try!(self.read_dicom_element());
        let file_meta_info_version: DicomElement = try!(self.read_dicom_element());
        let media_storage_sop_class_uid: DicomElement = try!(self.read_dicom_element());
        let media_storage_sop_instance_uid: DicomElement = try!(self.read_dicom_element());
        let transfer_syntax_uid: DicomElement = try!(self.read_dicom_element());
        let implementation_class_uid: DicomElement = try!(self.read_dicom_element());

        self.dicom_header.push(file_meta_info_group_length);
        self.dicom_header.push(file_meta_info_version);
        self.dicom_header.push(media_storage_sop_class_uid);
        self.dicom_header.push(media_storage_sop_instance_uid);
        self.dicom_header.push(transfer_syntax_uid);
        self.dicom_header.push(implementation_class_uid);

        Result::Ok(())
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
