extern crate byteorder;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod mock;

use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

use std::collections::hash_map::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{Error, ErrorKind, Seek};
use std::path::Path;

use core::dict::dicom_elements as tags;
use core::{ts, vr};


const FILE_PREAMBLE_LENGTH: usize = 128;
const DICOM_PREFIX_LENGTH: usize = 4;

static DICOM_PREFIX: [u8;DICOM_PREFIX_LENGTH] = ['D' as u8, 'I' as u8, 'C' as u8, 'M' as u8];


pub struct DicomStream<StreamType> {
    stream: StreamType,

    file_preamble: [u8;FILE_PREAMBLE_LENGTH],
    dicom_prefix: [u8;DICOM_PREFIX_LENGTH],
    
    file_meta: HashMap<u32, DicomElement>,
    ts: &'static ts::TransferSyntax<'static>,
}

pub struct DicomElement {
    pub tag: u32,
    pub vr: &'static vr::VR,
    pub vl: u32,
    pub bytes: Vec<u8>,
}

impl fmt::Debug for DicomElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DicomElement {{ tag: {:08X}  vr: {}  vl: {} }}", self.tag, self.vr.ident, self.vl)
    }
}

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

impl<StreamType: ReadBytesExt + Seek> DicomStream<StreamType> {
    pub fn new(stream: StreamType) -> DicomStream<StreamType> {
        DicomStream {
            stream: stream,
            file_preamble: [0u8;FILE_PREAMBLE_LENGTH],
            dicom_prefix: [0u8;DICOM_PREFIX_LENGTH],
            file_meta: HashMap::with_capacity(12),
            ts: &ts::ExplicitVRLittleEndian,
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

    #[cfg(test)]
    pub fn is_standard_preamble(&self) -> bool {
        for i in 0..FILE_PREAMBLE_LENGTH {
            if self.file_preamble[i] != 0 {
                return false;
            }
        }
        for i in 0..DICOM_PREFIX_LENGTH {
            if self.dicom_prefix[i] != DICOM_PREFIX[i] {
                return false;
            }
        }
        true
    }

    pub fn read_file_preamble(&mut self) -> Result<(), Error> {
        self.stream.read_exact(&mut self.file_preamble)
    }

    pub fn read_dicom_prefix(&mut self) -> Result<(), Error> {
        self.stream.read_exact(&mut self.dicom_prefix)?;

        for n in 0..DICOM_PREFIX.len() {
            if self.dicom_prefix[n] != DICOM_PREFIX[n] {
                return Err(Error::new(ErrorKind::InvalidData,
                                    format!("Invalid DICOM Prefix: {:?}", self.dicom_prefix)));
            }
        }

        Ok(())
    }

    pub fn read_tag<Endian: ByteOrder>(&mut self) -> Result<u32, Error> {
        let first: u32 = (self.stream.read_u16::<Endian>()? as u32) << 16;
        let second: u32 = self.stream.read_u16::<Endian>()? as u32;
        let result: u32 = first + second;
        Ok(result)
    }

    pub fn read_vr(&mut self) -> Result<&'static vr::VR, Error> {
        let first_char: u8 = self.stream.read_u8()?;
        let second_char: u8 = self.stream.read_u8()?;
        let code: u16 = ((first_char as u16) << 8) + second_char as u16;
        match vr::VR::code_to_vr(code) {
            Some(vr) => Ok(vr),
            None => Err(Error::new(ErrorKind::InvalidData, format!("Unable to interpret VR: {:?}", code)))
        }
    }

    pub fn read_value_length<Endian: ByteOrder>(&mut self, vr: &vr::VR) -> Result<u32, Error> {
        match vr.explicit_vr_header_bytes {
            8 => self.stream.read_u16::<Endian>().map(|n| n as u32),
            12 => {
                self.stream.read_u16::<Endian>()?;
                self.stream.read_u32::<Endian>()
            },
            n => Err(Error::new(ErrorKind::InvalidData, format!("Invalid VR Header Length: {:?}", n))),
        }
    }

    pub fn read_value_field(&mut self, value_length: u32) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = vec![0;value_length as usize];
        self.stream.read_exact(bytes.as_mut_slice())?;
        Ok(bytes)
    }

    pub fn read_dicom_element(&mut self) -> Result<DicomElement, Error> {
        let tag: u32;
        let vl: u32;
        let mut vr: &vr::VR = &vr::UN;
        if self.ts.is_big_endian() {
            tag = self.read_tag::<BigEndian>()?;
            if self.ts.is_explicit_vr() {
                vr = self.read_vr()?;
            }
            vl = self.read_value_length::<BigEndian>(vr)?;
        } else {
            tag = self.read_tag::<LittleEndian>()?;
            if self.ts.is_explicit_vr() {
                vr = self.read_vr()?;
            }
            vl = self.read_value_length::<LittleEndian>(vr)?;
        }

        let bytes: Vec<u8> = self.read_value_field(vl)?;

        Ok(DicomElement {
            tag: tag,
            vr: vr,
            vl: vl,
            bytes: bytes,
        })
    }

    pub fn read_file_meta(&mut self) -> Result<(), Error> {
        self.read_file_preamble()?;
        self.read_dicom_prefix()?;

        let mut element: DicomElement = self.read_dicom_element()?;
        if element.tag != tags::FileMetaInformationGroupLength.get_tag() {
            return Err(Error::new(ErrorKind::InvalidData, format!("Expected FileMetaInformationGroupLength but read: {:?}", element)));
        }

        while element.tag <= tags::PrivateInformation.get_tag() {
            self.file_meta.insert(element.tag, element);
            element = self.read_dicom_element()?;
        }

        Ok(())
    }
}
