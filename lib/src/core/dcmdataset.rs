use crate::core::dcmelement::DicomElement;
use crate::read::CSRef;
use byteorder::ByteOrder;
use std::io::Error;

pub trait DicomDataSet {
    /// Returns true if this data set contains an element with the given tag
    fn contains_element(&self, tag: u32) -> bool;

    /// Retrieve a reference to the immediate child element associated with the given tag
    fn get_element(&self, tag: u32) -> Result<&DicomElement, Error>;
    /// Retrieve a mutable reference to the immediate child element associated with the given tag
    fn get_element_mut(&mut self, tag: u32) -> Result<&mut DicomElement, Error>;

    /// Retrieve the value for the given element as a string
    fn get_string(&mut self, tag: u32, cs: CSRef) -> Result<&String, Error>;
    /// Retrieve the value for the given element as a list of strings
    fn get_strings(&mut self, tag: u32, cs: CSRef) -> Result<&Vec<String>, Error>;
    /// Retrieve the value for the given element as a 32bit floating point
    fn get_f32<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&f32, Error>;
    /// Retrieve the value for the given element as a list of 32bit floating point values
    fn get_f32s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<f32>, Error>;
    /// Retrieve the value for the given element as a 64bit floating point
    fn get_f64<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&f64, Error>;
    /// Retrieve the value for the given element as a list of 64bit floating point values
    fn get_f64s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<f64>, Error>;
    /// Retrieve the value for the given element as a 16bit signed integer
    fn get_i16<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&i16, Error>;
    /// Retrieve the value for the given element as a list of 16bit signed integer values
    fn get_i16s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<i16>, Error>;
    /// Retrieve the value for the given element as a 32bit signed integer
    fn get_i32<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&i32, Error>;
    /// Retrieve the value for the given element as a list of 32bit signed integer values
    fn get_i32s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<i32>, Error>;
    /// Retrieve the value for the given element as a 16bit unsigned integer
    fn get_u16<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&u16, Error>;
    /// Retrieve the value for the given element as a 32bit unsigned integer
    fn get_u32<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&u32, Error>;

    /// Sets the immediate child element
    fn put_element(&mut self, tag: u32, element: DicomElement) -> Option<DicomElement>;
}
