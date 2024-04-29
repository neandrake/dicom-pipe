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

use encoding::types::EncodingRef;
use encoding::label::encoding_from_whatwg_label;

use read::dcmdataset::{DicomDataSet, DicomDataSetContainer};
use read::dcmelement::DicomElement;
use read::tagstop::TagStop;

use std::ascii::AsciiExt;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::Path;


pub const FILE_PREAMBLE_LENGTH: usize = 128;
pub const DICOM_PREFIX_LENGTH: usize = 4;

pub static DICOM_PREFIX: [u8;DICOM_PREFIX_LENGTH] = ['D' as u8, 'I' as u8, 'C' as u8, 'M' as u8];

pub struct DicomStream<StreamType: ReadBytesExt> {
    stream: StreamType,
    bytes_read: usize,

    file_preamble: [u8;FILE_PREAMBLE_LENGTH],
    dicom_prefix: [u8;DICOM_PREFIX_LENGTH],
    
    root_dataset: DicomDataSet,
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
            root_dataset: DicomDataSet::new(),
            ts: &ts::ExplicitVRLittleEndian,
            cs: vr::DEFAULT_CHARACTER_SET,
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

    pub fn get_ts(&self) -> TSRef {
        self.ts
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
        if let Some(last_tag) = self.tag_peek {
            return Ok(last_tag);
        }
        let first: u32 = (self.stream.read_u16::<Endian>()? as u32) << 16;
        self.bytes_read += 2;
        let second: u32 = self.stream.read_u16::<Endian>()? as u32;
        self.bytes_read += 2;
        let result: u32 = first + second;
        self.tag_peek = Some(result);
        Ok(result)
    }

    // Reads the next tag. If `self.tag_peek` is Some, that will be returned
    // instead of trying to read the next tag from the stream.
    pub fn read_next_tag<Endian: ByteOrder>(&mut self) -> Result<u32, Error> {
        self.tag_peek.map_or_else(
            || self.read_tag::<Endian>(),
            |read_tag: u32| Ok(read_tag)
        )
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
                unimplemented!();
            },
        }
    }

    pub fn read_dicom_element<Endian: ByteOrder>(&mut self) -> Result<u32, Error> {
        let tag: u32 = self.read_next_tag::<Endian>()?;
        let vr: VRRef = self.read_vr(tag)?;
        let vl: ValueLength = self.read_value_length::<Endian>(vr)?;
        let bytes: Vec<u8> = self.read_value_field(&vl)?;

        // clear `self.tag_peek` as we've now read the entire element and the next
        // read should advance to the next tag
        self.tag_peek = None;

        let element: DicomElement = DicomElement::new(tag, vr, vl, bytes);

        self.root_dataset.put_element(tag, element);

        if tag == tags::SpecificCharacterSet.tag {
            self.cs = self.parse_specific_character_set()?;
        }

        Ok(tag)
    }

    pub fn read_file_meta<F>(&mut self, each: F) -> Result<(), Error>
        where F: Fn(&mut Self, u32) {
        // This is required for "well-formed" DICOM files however it's not 100% required
        // so somehow detect reading of FileMetaInformationGroupLength maybe?
        self.read_file_preamble()?;
        self.read_dicom_prefix()?;

        let bytes_read_before_fme: usize = self.bytes_read;

        // All FileMetaInformation fields are encoded as LittleEndian

        // The FileMetaInformationGroupLength is required first element and
        // tells us how many bytes to reach end of FileMetaInformation
        let fme_bytes: usize;
        let fmi_grouplength_tag: u32 = self.read_dicom_element::<LittleEndian>()?;
        if fmi_grouplength_tag != fme::FileMetaInformationGroupLength.tag {
            return Err(Error::new(ErrorKind::InvalidData, format!("Expected FileMetaInformationGroupLength but read: {:?}", fmi_grouplength_tag)))
        } else {
            let fmi_grouplength: &mut DicomElement = self.get_element_mut(fmi_grouplength_tag)?;
            // TODO: this reading of bytes as u32 should be part of VR (and remove padding)
            fme_bytes = fmi_grouplength.get_value_mut().read_u32::<LittleEndian>()? as usize;
        }

        let mut transfer_syntax: TSRef = self.ts;
        while self.bytes_read - bytes_read_before_fme < fme_bytes {    
            let element_tag: u32 = self.read_dicom_element::<LittleEndian>()?;
            if element_tag == fme::TransferSyntaxUID.tag {
                transfer_syntax = self.parse_transfer_syntax()?;
            }

            each(self, element_tag);
        }

        // don't set the transfer syntax until after reading all FileMeta, otherwise it 
        // will attempt to read remaining FME tags as different syntax than ExplicitVRLittleEndian (which is required)
        self.ts = transfer_syntax;

        Ok(())
    }

    pub fn read_until<F>(&mut self, tagstop: TagStop, each: F) -> Result<(), Error>
        where F: Fn(&mut Self, u32) {
        while !self.is_at_tag_stop(&tagstop)? {
            let element_tag: u32 = if self.ts.big_endian {
                self.read_dicom_element::<BigEndian>()?
            } else {
                self.read_dicom_element::<LittleEndian>()?
            };

            if element_tag == tags::SpecificCharacterSet.tag {
                self.cs = self.parse_specific_character_set()?;
            }

            each(self, element_tag);
        }
        Ok(())
    }

    // TODO: This should have a test.
    // Current `EndOfStream` doesn't work right as we will continue to try reading
    // elements past the end of the stream
    pub fn is_at_tag_stop(&mut self, tagstop: &TagStop) -> Result<bool, Error> {
        let element_tag: u32 = if self.ts.big_endian {
            self.read_next_tag::<BigEndian>()?
        } else {
            self.read_next_tag::<LittleEndian>()?
        };

        let is_at_tag_stop: bool = match *tagstop {
            TagStop::EndOfStream => false,
            TagStop::BeforeTag(before_tag) => element_tag >= before_tag,
            TagStop::AfterTag(after_tag) => element_tag > after_tag,
            TagStop::AfterBytePos(byte_pos) => self.bytes_read >= byte_pos,
        };

        Ok(is_at_tag_stop)
    }

    fn parse_transfer_syntax(&mut self) -> Result<TSRef, Error> {
        let mut ts_uid: &str = self.get_string(fme::TransferSyntaxUID.tag, vr::DEFAULT_CHARACTER_SET)?
            .as_ref();

        if ts_uid.ends_with(char::from(vr::UI.padding)) {
            ts_uid = unsafe { ts_uid.slice_unchecked(0, ts_uid.len() - 1) };
        }

        TS_BY_ID
            .get(ts_uid)
            .map(|tsref: &TSRef| *tsref)
            .ok_or(Error::new(ErrorKind::InvalidData, format!("Unknown TransferSyntax: {}", ts_uid)))
    }

    fn parse_specific_character_set(&mut self) -> Result<EncodingRef, Error> {
        if let Ok(ref mut element) = self.get_element_mut(tags::SpecificCharacterSet.tag) {
            let decoder: EncodingRef = element.vr.get_proper_cs(self.cs);
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

    pub fn print_element(&mut self, element_tag: u32) -> Result<String, Error> {
        let is_big_endian: bool = self.ts.big_endian;

        let elem: &mut DicomElement = self.get_element_mut(element_tag)?;
        let cs: EncodingRef = elem.vr.get_proper_cs(self.cs);
        let tag_num: String = Tag::format_tag_to_display(elem.tag);

        let tag_name: String = if let Some(tag) = TAG_BY_VALUE.get(&elem.tag) {
            format!("{}", tag.ident)
        } else {
            format!("{{Unknown Tag}}")
        };

        if elem.is_empty() {
            return Ok(format!("{} {} {} [EMPTY]", tag_num, elem.vr.ident, tag_name));
        }
        
        let tag_value: String = if is_big_endian {
            elem.fmt_string_value::<BigEndian>(cs)?
        } else {
            elem.fmt_string_value::<LittleEndian>(cs)?
        };

        Ok(format!("{} {} {} => {}", tag_num, elem.vr.ident, tag_name, tag_value))
    }
}

impl<StreamType: ReadBytesExt> DicomDataSetContainer for DicomStream<StreamType> {
    fn get_element(&self, tag: u32) -> Result<&DicomElement, Error> {
        self.root_dataset.get_element(tag)
    }

    fn get_element_mut(&mut self, tag: u32) -> Result<&mut DicomElement, Error> {
        self.root_dataset.get_element_mut(tag)
    }

    fn get_string(&mut self, tag: u32, cs: EncodingRef) -> Result<&String, Error> {
        self.root_dataset.get_string(tag, cs)
    }

    fn get_strings(&mut self, tag: u32, cs: EncodingRef) -> Result<&Vec<String>, Error> {
        self.root_dataset.get_strings(tag, cs)
    }

    fn get_f32<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&f32, Error> {
        self.root_dataset.get_f32::<Endian>(tag)
    }

    fn get_f32s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<f32>, Error> {
        self.root_dataset.get_f32s::<Endian>(tag)
    }

    fn get_f64<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&f64, Error> {
        self.root_dataset.get_f64::<Endian>(tag)
    }

    fn get_f64s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<f64>, Error> {
        self.root_dataset.get_f64s::<Endian>(tag)
    }

    fn get_i16<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&i16, Error> {
        self.root_dataset.get_i16::<Endian>(tag)
    }

    fn get_i16s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<i16>, Error> {
        self.root_dataset.get_i16s::<Endian>(tag)
    }

    fn get_i32<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&i32, Error> {
        self.root_dataset.get_i32::<Endian>(tag)
    }

    fn get_i32s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<i32>, Error> {
        self.root_dataset.get_i32s::<Endian>(tag)
    }

    fn get_u16<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&u16, Error> {
        self.root_dataset.get_u16::<Endian>(tag)
    }

    fn get_u32<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&u32, Error> {
        self.root_dataset.get_u32::<Endian>(tag)
    }


    fn put_element(&mut self, tag: u32, element: DicomElement) -> Option<DicomElement> {
        self.root_dataset.put_element(tag, element)
    }
}
