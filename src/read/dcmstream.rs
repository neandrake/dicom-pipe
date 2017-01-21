use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

use core::dict::dicom_elements as tags;
use core::dict::file_meta_elements as fme;
use core::dict::lookup::{TAG_BY_VALUE, TS_BY_ID};
use core::dict::transfer_syntaxes as ts;
use core::tag::Tag;
use core::ts::TSRef;
use core::vl;
use core::vl::ValueLength;
use core::vr;
use core::vr::{VR, VRRef};

use encoding::all::WINDOWS_1252;
use encoding::types::EncodingRef;
use encoding::label::encoding_from_whatwg_label;

use read::dcmelement::DicomElement;
use read::tagstop::TagStop;

use std::ascii::AsciiExt;
use std::collections::hash_map::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::Path;


pub const FILE_PREAMBLE_LENGTH: usize = 128;
pub const DICOM_PREFIX_LENGTH: usize = 4;

pub static DICOM_PREFIX: [u8;DICOM_PREFIX_LENGTH] = ['D' as u8, 'I' as u8, 'C' as u8, 'M' as u8];

pub static DEFAULT_CHARACTER_SET: EncodingRef = WINDOWS_1252 as EncodingRef;

pub struct DicomStream<StreamType: ReadBytesExt> {
    stream: StreamType,
    bytes_read: usize,

    file_preamble: [u8;FILE_PREAMBLE_LENGTH],
    dicom_prefix: [u8;DICOM_PREFIX_LENGTH],
    
    elements: HashMap<u32, DicomElement>,
    ts: TSRef,
    cs: EncodingRef,

    // To allow peeking the next tag without fully reading the next element 
    tag_peek: Option<u32>,
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

impl<StreamType: ReadBytesExt> DicomStream<StreamType> {
    pub fn new(stream: StreamType) -> DicomStream<StreamType> {
        DicomStream {
            stream: stream,
            bytes_read: 0usize,
            file_preamble: [0u8;FILE_PREAMBLE_LENGTH],
            dicom_prefix: [0u8;DICOM_PREFIX_LENGTH],
            elements: HashMap::with_capacity(64),
            ts: &ts::ExplicitVRLittleEndian,
            cs: WINDOWS_1252 as EncodingRef,
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

    pub fn get_element(&self, tag: u32) -> Result<&DicomElement, Error> {
        self.elements.get(&tag)
            .ok_or(Error::new(ErrorKind::InvalidData, format!("No element for tag: {}", tag)))
    }

    pub fn get_element_mut(&mut self, tag: u32) -> Result<&mut DicomElement, Error> {
        self.elements.get_mut(&tag)
            .ok_or(Error::new(ErrorKind::InvalidData, format!("No element for tag: {}", tag)))
    }

    pub fn get_text_codec(&self, element: &DicomElement) -> EncodingRef {
        match element.vr.decode_text_with_replaced_cs {
            true => self.cs,
            false => DEFAULT_CHARACTER_SET,
        }
    }

    pub fn read_file_preamble(&mut self) -> Result<(), Error> {
        self.stream.read_exact(&mut self.file_preamble)?;
        self.bytes_read += self.file_preamble.len();
        Ok(())
    }

    pub fn read_dicom_prefix(&mut self) -> Result<(), Error> {
        self.stream.read_exact(&mut self.dicom_prefix)?;
        self.bytes_read += self.dicom_prefix.len();

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
            return self.tag_peek
                .ok_or(Error::new(ErrorKind::InvalidData, format!("Unable to read next tag")));
        }
        let first: u32 = (self.stream.read_u16::<Endian>()? as u32) << 16;
        self.bytes_read += 2;
        let second: u32 = self.stream.read_u16::<Endian>()? as u32;
        self.bytes_read += 2;
        let result: u32 = first + second;
        self.tag_peek = Some(result);
        Ok(result)
    }

    pub fn read_vr(&mut self, tag: u32) -> Result<VRRef, Error> {
        if self.ts.explicit_vr {
            let first_char: u8 = self.stream.read_u8()?;
            self.bytes_read += 1;
            let second_char: u8 = self.stream.read_u8()?;
            self.bytes_read += 1;

            let code: u16 = ((first_char as u16) << 8) + second_char as u16;
            match VR::from_code(code) {
                Some(vr) => Ok(vr),
                None => Err(Error::new(ErrorKind::InvalidData, format!("Unable to interpret VR: {:?}", code)))
            }
        } else {
            TAG_BY_VALUE.get(&tag)
                .and_then(|read_tag: &&Tag| read_tag.implicit_vr)
                .ok_or(Error::new(ErrorKind::InvalidData, format!("ImplicitVR TS but VR is unknown for tag: {}", tag)))
        }
    }

    pub fn read_value_length<Endian: ByteOrder>(&mut self, vr: VRRef) -> Result<ValueLength, Error> {
        let value_length: u32;
        if self.ts.explicit_vr {
            if vr.has_explicit_2byte_pad {
                self.stream.read_u16::<Endian>()?;
                self.bytes_read += 2;
                value_length = self.stream.read_u32::<Endian>()?;
                self.bytes_read += 4;
            } else {
                value_length = self.stream.read_u16::<Endian>()? as u32;
                self.bytes_read += 2;
            }
        } else {
            value_length = self.stream.read_u32::<Endian>()?;
            self.bytes_read += 4;
        }
        Ok(vl::from_value_length(value_length))
    }

    pub fn read_value_field(&mut self, vl: &ValueLength) -> Result<Vec<u8>, Error> {
        match *vl {
            ValueLength::Explicit(value_length) => {
                let mut bytes: Vec<u8> = vec![0;value_length as usize];
                self.stream.read_exact(bytes.as_mut_slice())?;
                self.bytes_read += value_length as usize;
                Ok(bytes)
            },
            ValueLength::UndefinedLength => {
                // TODO: Read until Sequence Delimitation Item
                // Part 5 Ch. 7.1.3
                // The Value Field has an Undefined Length and a Sequence Delimitation Item marks the end of the Value Field.
                Err(Error::new(ErrorKind::Other, format!("Reading values of undefined length not yet supported")))
            },
        }
    }

    pub fn read_dicom_element(&mut self) -> Result<u32, Error> {
        if self.ts.big_endian {
            self._read_dicom_element::<BigEndian>()
        } else {
            self._read_dicom_element::<LittleEndian>()
        }
    }

    fn _read_dicom_element<Endian: ByteOrder>(&mut self) -> Result<u32, Error> {
        let tag: u32 = match self.tag_peek {
            Some(read_tag) => read_tag,
            None => self.read_tag::<Endian>()?,
        };
        // Clear `self.tag_peek` so subsequent calls will read the next tag value
        self.tag_peek = None;
        
        //println!("Read Tag: {}", tag);
        let vr: VRRef = self.read_vr(tag)?;
        //println!("Read VR: {:?}", vr);
        let vl: ValueLength = self.read_value_length::<Endian>(vr)?;
        //println!("Read VL: {:?}", vl);
        let bytes: Vec<u8> = self.read_value_field(&vl)?;

        let element: DicomElement = DicomElement::new(tag, vr, vl, bytes);

        self.elements.insert(tag, element);
        Ok(tag)
    }

    pub fn parse_transfer_syntax(&self) -> Result<TSRef, Error> {
        let element: &DicomElement = self.get_element(fme::TransferSyntaxUID.tag)?;
        
        let mut ts_uid: String = element.parse_string(DEFAULT_CHARACTER_SET)?;

        if let Some(last_char) = ts_uid.chars().last() {
            if last_char == char::from(vr::UI.padding) {
                ts_uid.pop();
            }
        }

        let ts_uid_str: &str = ts_uid.as_ref();
        TS_BY_ID
            .get(ts_uid_str)
            .map(|tsref: &TSRef| *tsref)
            .ok_or(Error::new(ErrorKind::InvalidData, format!("Unknown TransferSyntax: {}", ts_uid_str)))
    }

    pub fn parse_specific_character_set(&self) -> Result<EncodingRef, Error> {
        if let Ok(element) = self.get_element(tags::SpecificCharacterSet.tag) {
            let decoder: EncodingRef = self.get_text_codec(element);
            // Change the lookup key into format that the encoding package and recognize
            let new_cs: String = element.parse_string(decoder)
                .iter()
                .flat_map(|s| s.chars())
                .map(|c: char| {
                    match c {
                        '_' => '-',
                        ' ' => '-',
                        a => a.to_ascii_lowercase(),
                    }
                })
                .collect::<String>();

            // TODO: There are options for what to do if we can't support the character repertoire
            // See note on Ch 5 Part 6.1.2.3 under "Considerations on the Handling of Unsupported Character Sets"
            return encoding_from_whatwg_label(&new_cs)
                .ok_or(Error::new(ErrorKind::InvalidData, format!("Unable to determine Specific Character Set: {:?}", &new_cs)));
        }

        Err(Error::new(ErrorKind::InvalidData, format!("DicomStream does not have SpecificCharacterSet")))
    }

    pub fn read_file_meta(&mut self) -> Result<(), Error> {
        // This is required for "well-formed" DICOM files however it's not 100% required
        // so somehow detect reading of FileMetaInformationGroupLength maybe?
        self.read_file_preamble()?;
        self.read_dicom_prefix()?;

        let bytes_read_before_fme: usize = self.bytes_read;

        // All FileMetaInformation fields are encoded as LittleEndian

        // The FileMetaInformationGroupLength is required first element and
        // tells us how many bytes to reach end of FileMetaInformation
        let fme_bytes: usize;
        let fmi_grouplength_tag: u32 = self.read_dicom_element()?;
        if fmi_grouplength_tag != fme::FileMetaInformationGroupLength.tag {
            return Err(Error::new(ErrorKind::InvalidData, format!("Expected FileMetaInformationGroupLength but read: {:?}", fmi_grouplength_tag)))
        } else {
            let fmi_grouplength: &mut DicomElement = self.get_element_mut(fmi_grouplength_tag)?;
            // TODO: this reading of bytes as u32 should be part of VR (and remove padding)
            fme_bytes = fmi_grouplength.get_value_mut().read_u32::<LittleEndian>()? as usize;
        }

        let mut transfer_syntax: TSRef = self.ts;
        while self.bytes_read - bytes_read_before_fme < fme_bytes {    
            let element_tag: u32 = self.read_dicom_element()?;
            if element_tag == fme::TransferSyntaxUID.tag {
                transfer_syntax = self.parse_transfer_syntax()?;
            }
        }

        // don't set the transfer syntax until after reading all FileMeta, otherwise it 
        // will attempt to read remaining FME tags as different syntax than ExplicitVRLittleEndian (which is required)
        self.ts = transfer_syntax;

        Ok(())
    }

    pub fn read_until(&mut self, tagstop: TagStop) -> Result<(), Error> {
        let mut still_loop: bool = true;

        while still_loop {
            let element_tag: u32 = self.read_dicom_element()?;

            if element_tag == tags::SpecificCharacterSet.tag {
                self.cs = self.parse_specific_character_set()?;
            }

            // TODO: This should have a test.
            // Current `EndOfStream` doesn't work right as we will continue to try reading
            // elements past the end of the stream
            still_loop = match tagstop {
                TagStop::EndOfStream => true,
                TagStop::BeforeTag(before_tag) => element_tag < before_tag,
                TagStop::AfterTag(after_tag) => element_tag <= after_tag,
                TagStop::AfterBytePos(byte_pos) => self.bytes_read < byte_pos,
            }
        }
        Ok(())
    }
}
