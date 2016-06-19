extern crate byteorder;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod mock;


use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

use std::fmt;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Seek};
use std::path::Path;

use util::vr::VR;


const FILE_PREAMBLE_LENGTH: usize = 128;
const DICOM_PREFIX_LENGTH: usize = 4;
static DICOM_PREFIX: [u8;DICOM_PREFIX_LENGTH] = ['D' as u8, 'I' as u8, 'C' as u8, 'M' as u8];


pub struct DicomStream<StreamType> {
    stream: StreamType,

    file_preamble: [u8;FILE_PREAMBLE_LENGTH],
    dicom_prefix: [u8;DICOM_PREFIX_LENGTH],
    
    dicom_header: Vec<DicomElement>,
}

pub struct DicomElement {
    pub tag: u32,
    pub vr: &'static VR,
    pub vl: u32,
    pub bytes: Vec<u8>,
}

impl fmt::Debug for DicomElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DicomElement {{ tag: 0x{:08X}  vr: {}  vl: {} }}", self.tag, self.vr.ident, self.vl)
    }
}

impl DicomStream<File> {
    pub fn new_from_path(path: &Path) -> Result<DicomStream<File>, Error> {
        if !path.is_file() {
            return Result::Err(Error::new(ErrorKind::InvalidData,
                                          format!("Invalid path: {:?}", path)));
        }

        let file: File = try!(File::open(path));
        Result::Ok::<DicomStream<File>, Error>(DicomStream::new(file))
    }
}

impl<StreamType: ReadBytesExt + Seek> DicomStream<StreamType> {
    pub fn new(stream: StreamType) -> DicomStream<StreamType> {
        DicomStream {
            stream: stream,
            file_preamble: [0u8;FILE_PREAMBLE_LENGTH],
            dicom_prefix: [0u8;DICOM_PREFIX_LENGTH],
            dicom_header: Vec::with_capacity(6),
        }
    }

    #[allow(dead_code)]
    pub fn get_stream(&self) -> &StreamType {
        &self.stream
    }

    #[allow(dead_code)]
    pub fn get_file_preamble(&self) -> &[u8;FILE_PREAMBLE_LENGTH] {
        &self.file_preamble
    }

    #[allow(dead_code)]
    pub fn get_dicom_prefix(&self) -> &[u8;DICOM_PREFIX_LENGTH] {
        &self.dicom_prefix
    }

    pub fn read_file_preamble(&mut self) -> Result<(), Error> {
        try!(self.stream.read_exact(&mut self.file_preamble));
        Result::Ok(())
    }

    pub fn read_dicom_prefix(&mut self) -> Result<(), Error> {
        try!(self.stream.read_exact(&mut self.dicom_prefix));

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
        Result::Ok(result)
    }

    pub fn read_vr(&mut self) -> Result<&'static VR, Error> {
        let first_char: u8 = try!(self.stream.read_u8());
        let second_char: u8 = try!(self.stream.read_u8());
        let code: u16 = ((first_char as u16) << 8) + second_char as u16;
        VR::code_to_vr(code).ok_or(Error::new(ErrorKind::InvalidData, format!("Unable to interpret VR: {:?}", code)))
    }

    pub fn read_value_length(&mut self, vr: &VR) -> Result<u32, Error> {
        match vr.explicit_vr_header_bytes {
            8 => self.stream.read_u16::<LittleEndian>().map(|n| n as u32),
            12 => {
                try!(self.stream.read_u16::<LittleEndian>());
                self.stream.read_u32::<LittleEndian>()
            },
            n => Err(Error::new(ErrorKind::InvalidData, format!("Invalid VR Header Length: {:?}", n))),
        }
    }

    pub fn read_value_field(&mut self, value_length: u32) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = vec![0;value_length as usize];
        try!(self.stream.read_exact(bytes.as_mut_slice()));
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
        println!("File Meta Information Group Length: {:?}", file_meta_info_group_length);

        let file_meta_info_version: DicomElement = try!(self.read_dicom_element());
        println!("File Meta Information Version: {:?}", file_meta_info_version);

        let media_storage_sop_class_uid: DicomElement = try!(self.read_dicom_element());
        println!("Media Storage SOP Class UID: {:?}", media_storage_sop_class_uid);

        let media_storage_sop_instance_uid: DicomElement = try!(self.read_dicom_element());
        println!("Media Storage SOP Instance UID: {:?}", media_storage_sop_instance_uid);

        let transfer_syntax_uid: DicomElement = try!(self.read_dicom_element());
        println!("Transfer Syntax UID: {:?}", transfer_syntax_uid);

        let implementation_class_uid: DicomElement = try!(self.read_dicom_element());
        println!("Implementation Class UID: {:?}", implementation_class_uid);

        self.dicom_header.push(file_meta_info_group_length);
        self.dicom_header.push(file_meta_info_version);
        self.dicom_header.push(media_storage_sop_class_uid);
        self.dicom_header.push(media_storage_sop_instance_uid);
        self.dicom_header.push(transfer_syntax_uid);
        self.dicom_header.push(implementation_class_uid);

        Result::Ok(())
    }
}
