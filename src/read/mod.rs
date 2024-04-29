extern crate byteorder;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod mock;

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

use std::collections::hash_map::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{Error, ErrorKind, Seek};
use std::path::Path;

use core::dict::file_meta_elements as fme;
use core::dict::transfer_syntaxes as ts;
use core::ts::TransferSyntax;
use core::vr;


pub const FILE_PREAMBLE_LENGTH: usize = 128;
pub const DICOM_PREFIX_LENGTH: usize = 4;

pub static DICOM_PREFIX: [u8;DICOM_PREFIX_LENGTH] = ['D' as u8, 'I' as u8, 'C' as u8, 'M' as u8];


pub struct DicomStream<StreamType> {
    stream: StreamType,

    file_preamble: [u8;FILE_PREAMBLE_LENGTH],
    dicom_prefix: [u8;DICOM_PREFIX_LENGTH],
    
    file_meta: HashMap<u32, DicomElement>,
    ts: &'static TransferSyntax,

    // To allow peeking the next tag without fully reading the next element 
    tag_peek: Option<u32>,
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
            tag_peek: None,
        }
    }

    pub fn get_stream(&self) -> &StreamType {
        &self.stream
    }

    pub fn get_file_preamble(&self) -> &[u8;FILE_PREAMBLE_LENGTH] {
        &self.file_preamble
    }

    pub fn get_dicom_prefix(&self) -> &[u8;DICOM_PREFIX_LENGTH] {
        &self.dicom_prefix
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

    /// Reads the next tag using the selected Endian. To allow for peeking
    /// what the next tag is without fully parsing it, the read tag value
    /// is stored in `self.tag_peek`. Calls to this method will repeatedly
    /// return the previously peek'd value until `self.tag_peek` is cleared.
    pub fn read_tag<Endian: ByteOrder>(&mut self) -> Result<u32, Error> {
        if self.tag_peek != None {
            return self.tag_peek.ok_or(Error::new(ErrorKind::InvalidData, format!("Unable to read next tag")));
        }
        let first: u32 = (self.stream.read_u16::<Endian>()? as u32) << 16;
        let second: u32 = self.stream.read_u16::<Endian>()? as u32;
        let result: u32 = first + second;
        self.tag_peek = Some(result);
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

    pub fn read_dicom_element<Endian: ByteOrder>(&mut self) -> Result<DicomElement, Error> {
        let tag: u32;
        match self.tag_peek {
            Some(read_tag) => tag = read_tag,
            None => tag = self.read_tag::<Endian>()?,
        };

        // Clear `self.tag_peek` so subsequent calls will read the next tag value
        self.tag_peek = None;

        // TODO: lookup Tag from tag value and use the default implicit VR
        let mut vr: &vr::VR = &vr::UN;
        if self.ts.is_explicit_vr() {
            vr = self.read_vr()?;
        }

        let vl: u32 = self.read_value_length::<Endian>(vr)?;
        let bytes: Vec<u8> = self.read_value_field(vl)?;

        Ok(DicomElement {
            tag: tag,
            vr: vr,
            vl: vl,
            bytes: bytes,
        })
    }

    pub fn read_file_meta(&mut self) -> Result<(), Error> {
        // This is required for "well-formed" DICOM files however it's not 100% required
        // so somehow detect reading of FileMetaInformationGroupLength maybe?
        self.read_file_preamble()?;
        self.read_dicom_prefix()?;

        // All FileMetaInformation fields are encoded as LittleEndian
        // But reading the TransferSyntax should swap the 

        // In theory the value of the FileMetaInformationGroupLength tells us
        // how many more bytes to read to reach the end of the File Meta tags
        self.read_tag::<LittleEndian>()?;
        match self.tag_peek {
            Some(read_tag) => {
                if read_tag != fme::FileMetaInformationGroupLength.tag {
                    return Err(Error::new(ErrorKind::InvalidData, format!("Expected FileMetaInformationGroupLength but read: {:08X}", read_tag)))
                }
            },
            None => return Err(Error::new(ErrorKind::InvalidData, "No tag to read"))
        };

        loop {    
            let element = self.read_dicom_element::<LittleEndian>()?;
            if element.tag == fme::TransferSyntaxUID.tag {
                // TODO: lookup the TransferSyntax by UID and switch it out for this element's ts
            }
            self.file_meta.insert(element.tag, element);

            // Peek the next tag, all tags in 0002,xxxx are reserved for FileMetaInformation
            self.read_tag::<LittleEndian>()?;
            if let Some(read_tag) = self.tag_peek {
                if read_tag < 0x0003_0000 {
                    continue;
                }
            }
            break;
        }
        Ok(())
    }
}
