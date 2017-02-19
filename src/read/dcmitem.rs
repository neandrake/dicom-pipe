use byteorder::ByteOrder;

use core::vl::ValueLength;

use encoding::types::EncodingRef;

use read::dcmdataset::{DicomDataSet, DicomDataSetContainer};
use read::dcmelement::DicomElement;

use std::io::{Cursor, Error};


pub struct DicomItem {
    pub tag: u32,
    pub vl: ValueLength,
    value: Cursor<Vec<u8>>,

    dataset: DicomDataSet,
}

impl DicomItem {
    pub fn new(tag: u32, vl: ValueLength, value: Vec<u8>) -> DicomItem {
        DicomItem {
            tag: tag,
            vl: vl,
            value: Cursor::new(value),

            dataset: DicomDataSet::new(),
        }
    }

    pub fn get_value(&self) -> &Cursor<Vec<u8>> {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.value
    }

    pub fn is_empty(&self) -> bool {
        self.value.get_ref().len() == 0
    }

    pub fn get_dataset(&self) -> &DicomDataSet {
        &self.dataset
    }

    pub fn get_dataset_mut(&mut self) -> &mut DicomDataSet {
        &mut self.dataset
    }
}

impl DicomDataSetContainer for DicomItem {
    fn get_element(&self, tag: u32) -> Result<&DicomElement, Error> {
        self.dataset.get_element(tag)
    }

    fn get_element_mut(&mut self, tag: u32) -> Result<&mut DicomElement, Error> {
        self.dataset.get_element_mut(tag)
    }

    fn get_string(&mut self, tag: u32, cs: EncodingRef) -> Result<&String, Error> {
        self.dataset.get_string(tag, cs)
    }

    fn get_strings(&mut self, tag: u32, cs: EncodingRef) -> Result<&Vec<String>, Error> {
        self.dataset.get_strings(tag, cs)
    }

    fn get_f32<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&f32, Error> {
        self.dataset.get_f32::<Endian>(tag)
    }

    fn get_f32s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<f32>, Error> {
        self.dataset.get_f32s::<Endian>(tag)
    }

    fn get_f64<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&f64, Error> {
        self.dataset.get_f64::<Endian>(tag)
    }

    fn get_f64s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<f64>, Error> {
        self.dataset.get_f64s::<Endian>(tag)
    }

    fn get_i16<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&i16, Error> {
        self.dataset.get_i16::<Endian>(tag)
    }

    fn get_i16s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<i16>, Error> {
        self.dataset.get_i16s::<Endian>(tag)
    }

    fn get_i32<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&i32, Error> {
        self.dataset.get_i32::<Endian>(tag)
    }

    fn get_i32s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<i32>, Error> {
        self.dataset.get_i32s::<Endian>(tag)
    }

    fn get_u16<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&u16, Error> {
        self.dataset.get_u16::<Endian>(tag)
    }

    fn get_u32<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&u32, Error> {
        self.dataset.get_u32::<Endian>(tag)
    }


    fn put_element(&mut self, tag: u32, element: DicomElement) -> Option<DicomElement> {
        self.dataset.put_element(tag, element)
    }
}
