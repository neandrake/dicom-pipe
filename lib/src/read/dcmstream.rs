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

use std::fs::File;
use std::io::{Error, ErrorKind, Seek, SeekFrom};
use std::path::Path;


pub const FILE_PREAMBLE_LENGTH: usize = 128;
pub const DICOM_PREFIX_LENGTH: usize = 4;

pub static DICOM_PREFIX: [u8;DICOM_PREFIX_LENGTH] = ['D' as u8, 'I' as u8, 'C' as u8, 'M' as u8];

pub struct DicomStream<StreamType: Seek + ReadBytesExt> {
    stream: StreamType,

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
        Ok(DicomStream::new(file))
    }
}

impl<StreamType: Seek + ReadBytesExt> DicomStream<StreamType> {
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

    pub fn position(&mut self) -> Result<u64, Error> {
        self.stream.seek(SeekFrom::Current(0))
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
        Ok(())
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
    fn read_tag<Endian: ByteOrder>(&mut self) -> Result<u32, Error> {
        if let Some(last_tag) = self.tag_peek {
            return Ok(last_tag);
        }
        let first: u32 = (self.stream.read_u16::<Endian>()? as u32) << 16;
        let second: u32 = self.stream.read_u16::<Endian>()? as u32;
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

    fn read_vr(&mut self, tag: u32) -> Result<VRRef, Error> {
        if self.ts.explicit_vr {
            let first_char: u8 = self.stream.read_u8()?;
            let second_char: u8 = self.stream.read_u8()?;

            let code: u16 = ((first_char as u16) << 8) + second_char as u16;
            match VR::from_code(code) {
                Some(vr) => Ok(vr),
                None => {
                    Ok(&::core::vr::UN)
                    // TODO: Log an error but still use UN?
                    //Err(Error::new(ErrorKind::InvalidData, format!("Unable to interpret VR: {:?}", code)))
                }
            }
        } else {
            TAG_BY_VALUE.get(&tag)
                .and_then(|read_tag: &&Tag| read_tag.implicit_vr)
                .or(Some(&::core::vr::UN))
                // TODO: Log an error but still use UN?
                .ok_or(Error::new(ErrorKind::InvalidData, format!("ImplicitVR TS but VR is unknown for tag: {}", tag)))
        }
    }

    fn read_value_length<Endian: ByteOrder>(&mut self, vr: VRRef) -> Result<ValueLength, Error> {
        let value_length: u32;
        if self.ts.explicit_vr {
            if vr.has_explicit_2byte_pad {
                self.stream.read_u16::<Endian>()?;
                value_length = self.stream.read_u32::<Endian>()?;
            } else {
                value_length = self.stream.read_u16::<Endian>()? as u32;
            }
        } else {
            value_length = self.stream.read_u32::<Endian>()?;
        }
        Ok(vl::from_value_length(value_length))
    }

    fn read_value_field(&mut self, vl: &ValueLength) -> Result<Vec<u8>, Error> {
        match *vl {
            ValueLength::Explicit(value_length) => {
                let mut bytes: Vec<u8> = vec![0;value_length as usize];
                self.stream.read_exact(bytes.as_mut_slice())?;
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

    fn read_dicom_element<Endian: ByteOrder>(&mut self) -> Result<DicomElement, Error> {
        let tag: u32 = self.read_next_tag::<Endian>()?;
        let vr: VRRef = self.read_vr(tag)?;
        let vl: ValueLength = self.read_value_length::<Endian>(vr)?;
        
        // don't read the element value if it's a sequence or an item since
        // the value will be another dicom element
        let bytes: Vec<u8> = if vr == &::core::vr::SQ || tag == tags::Item.tag {
            Vec::new()
        } else {
            self.read_value_field(&vl)?
        };

        // clear `self.tag_peek` as we've now read the entire element and the next
        // read should advance to the next tag
        self.tag_peek = None;

        Ok(DicomElement::new(tag, vr, vl, bytes))
    }

    pub fn read_file_meta(&mut self) -> Result<(), Error> {
        self.read_file_meta_on_each(|_,_|{})
    }

    pub fn read_file_meta_on_each<F>(&mut self, each: F) -> Result<(), Error>
        where F: Fn(&mut Self, u32) {
        // This is required for "well-formed" DICOM files however it's not 100% required
        // so somehow detect reading of FileMetaInformationGroupLength maybe?
        self.read_file_preamble()?;
        self.read_dicom_prefix()?;

        let pos_before_fme: u64 = self.position()?;

        // All FileMetaInformation fields are encoded as LittleEndian

        // The FileMetaInformationGroupLength is required first element and
        // tells us how many bytes to reach end of FileMetaInformation
        let fme_bytes: u64;
        let fmi_grouplength: DicomElement = self.read_dicom_element::<LittleEndian>()?;
        let fmi_grouplength_tag: u32 = fmi_grouplength.tag;
        if fmi_grouplength_tag != fme::FileMetaInformationGroupLength.tag {
            return Err(Error::new(ErrorKind::InvalidData,
                format!("Expected FileMetaInformationGroupLength but read: {:?}", Tag::format_tag_to_display(fmi_grouplength_tag))));
        }

        self.put_element(fmi_grouplength_tag, fmi_grouplength);
        fme_bytes = *self.get_u32::<LittleEndian>(fmi_grouplength_tag)? as u64;
        each(self, fmi_grouplength_tag);

        let mut pos: u64 = self.position()?;
        while pos - pos_before_fme < fme_bytes {
            let element: DicomElement = self.read_dicom_element::<LittleEndian>()?;
            pos = self.position()?;
            let element_tag: u32 = element.tag;
            self.put_element(element_tag, element);
            each(self, element_tag);
        }

        // don't set the transfer syntax until after reading all FileMeta, otherwise it 
        // will attempt to read remaining FME tags as different syntax than ExplicitVRLittleEndian (which is required)
        self.ts = self.parse_transfer_syntax()?;

        Ok(())
    }

    pub fn read_until(&mut self, tagstop: TagStop) -> Result<(), Error> {
        self.read_until_on_each(tagstop, |_,_|{})
    }

    pub fn read_until_on_each<F>(&mut self, tagstop: TagStop, each: F) -> Result<(), Error>
        where F: Fn(&mut Self, u32) {
        
        // TODO: convert to Tag Path for nested sequences
        let mut last_sequence_tag: Option<u32> = None;
        let mut last_seq_item_num: u32 = 1;
        while !self.is_at_tag_stop(&tagstop)? {
            let element: DicomElement = if self.ts.big_endian {
                self.read_dicom_element::<BigEndian>()?
            } else {
                self.read_dicom_element::<LittleEndian>()?
            };

            let element_tag: u32 = element.tag;

            if element_tag == tags::Item.tag {
                if let Some(last_seq_tag) = last_sequence_tag {
                    let last_seq_elem: &mut DicomElement = self.get_element_mut(last_seq_tag)?;
                    last_seq_elem.add_item(last_seq_item_num, element);
                }
                last_seq_item_num += 1;
            } else {
                if element.vr == &vr::SQ {
                    last_sequence_tag = Some(element_tag);
                    last_seq_item_num = 1;
                }
                self.put_element(element_tag, element);
            }

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
    fn is_at_tag_stop(&mut self, tagstop: &TagStop) -> Result<bool, Error> {
        let element_tag: u32 = if self.ts.big_endian {
            self.read_next_tag::<BigEndian>()?
        } else {
            self.read_next_tag::<LittleEndian>()?
        };

        let is_at_tag_stop: bool = match *tagstop {
            TagStop::EndOfStream => false,
            TagStop::BeforeTag(before_tag) => element_tag >= before_tag,
            TagStop::AfterTag(after_tag) => element_tag > after_tag,
            TagStop::AfterBytePos(byte_pos) => {
                let pos: u64 = self.position()?;
                pos >= byte_pos
            },
        };

        Ok(is_at_tag_stop)
    }

    fn parse_transfer_syntax(&mut self) -> Result<TSRef, Error> {
        let ts_uid: &str = self.get_string(fme::TransferSyntaxUID.tag, vr::DEFAULT_CHARACTER_SET)?
            .as_ref();

        TS_BY_ID
            .get(ts_uid)
            .map(|tsref: &TSRef| *tsref)
            .ok_or(Error::new(ErrorKind::InvalidData, format!("Unknown TransferSyntax: {:?}", ts_uid)))
    }

    /// Character sets not yet supported (not exhaustive, found by testing dclunie's test set)
    /// \\\\iso-2022-ir-87
    /// iso-2022-ir-13\\\\iso-2022-ir-87
    /// \\\\iso-2022-ir-149
    /// iso-ir-192
    fn parse_specific_character_set(&mut self) -> Result<EncodingRef, Error> {
        if let Some(ref vr) = tags::SpecificCharacterSet.implicit_vr {
            let decoder: EncodingRef = vr.get_proper_cs(self.cs);
            // Change the lookup key into format that the encoding package and recognize
            let new_cs: String = self.get_string(tags::SpecificCharacterSet.tag, decoder)
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

        Err(Error::new(ErrorKind::InvalidData, format!("No VR associated with SpecificCharacterSet")))
    }

    pub fn print_element(&mut self, element_tag: u32) -> Result<String, Error> {
        let is_big_endian: bool = self.ts.big_endian;

        let elem: &mut DicomElement = self.get_element_mut(element_tag)?;
        let cs: EncodingRef = elem.vr.get_proper_cs(self.cs);
        let tag_num: String = Tag::format_tag_to_display(elem.tag);

        let tag_name: String = if let Some(tag) = TAG_BY_VALUE.get(&elem.tag) {
            tag.ident.to_owned()
        } else {
            "{{Unknown Tag}}".to_owned()
        };
        
        let tag_value: String = if is_big_endian {
            elem.fmt_string_value::<BigEndian>(cs)?
        } else {
            elem.fmt_string_value::<LittleEndian>(cs)?
        };

        Ok(format!("{} {} {} => {}", tag_num, elem.vr.ident, tag_name, tag_value))
    }
}

/// Allows treating this stream as a dataset by delegating to the root dataset
impl<StreamType: Seek + ReadBytesExt> DicomDataSetContainer for DicomStream<StreamType> {
    fn contains_element(&self, tag: u32) -> bool {
        self.root_dataset.contains_element(tag)
    }

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
