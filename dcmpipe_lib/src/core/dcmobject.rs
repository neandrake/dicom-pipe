use crate::core::dcmdataset::DicomDataSet;
use crate::core::dcmelement::DicomElement;
use crate::core::tag::Tag;
use crate::read::CSRef;
use byteorder::ByteOrder;
use std::collections::hash_map::{Entry, HashMap};
use std::io::{Error, ErrorKind};

/// A container of DicomElements
/// Also provides a cache of the values parsed into native types -- the initial
/// request to get a child element as a specific type will cache the value
pub struct DicomObject {
    elements: HashMap<u32, DicomElement>,

    strings: HashMap<u32, String>,
    string_lists: HashMap<u32, Vec<String>>,

    floats: HashMap<u32, f32>,
    float_lists: HashMap<u32, Vec<f32>>,

    doubles: HashMap<u32, f64>,
    double_lists: HashMap<u32, Vec<f64>>,

    shorts: HashMap<u32, i16>,
    short_lists: HashMap<u32, Vec<i16>>,

    ints: HashMap<u32, i32>,
    int_lists: HashMap<u32, Vec<i32>>,

    ushorts: HashMap<u32, u16>,
    uints: HashMap<u32, u32>,
}

impl DicomObject {
    pub fn new() -> DicomObject {
        DicomObject {
            elements: HashMap::new(),

            strings: HashMap::new(),
            string_lists: HashMap::new(),

            floats: HashMap::new(),
            float_lists: HashMap::new(),

            doubles: HashMap::new(),
            double_lists: HashMap::new(),

            shorts: HashMap::new(),
            short_lists: HashMap::new(),

            ints: HashMap::new(),
            int_lists: HashMap::new(),

            ushorts: HashMap::new(),
            uints: HashMap::new(),
        }
    }
}

/// Implements the parsing and caching of DicomElements to different native types
impl DicomDataSet for DicomObject {
    fn contains_element(&self, tag: u32) -> bool {
        self.elements.contains_key(&tag)
    }

    fn get_element(&self, tag: u32) -> Result<&DicomElement, Error> {
        self.elements.get(&tag).ok_or(Error::new(
            ErrorKind::InvalidData,
            format!("No element for tag: {}", Tag::format_tag_to_display(tag)),
        ))
    }

    fn get_element_mut(&mut self, tag: u32) -> Result<&mut DicomElement, Error> {
        self.elements.get_mut(&tag).ok_or(Error::new(
            ErrorKind::InvalidData,
            format!("No element for tag: {}", Tag::format_tag_to_display(tag)),
        ))
    }

    fn get_string(&mut self, tag: u32, cs: CSRef) -> Result<&String, Error> {
        let entry: Entry<u32, String> = self.strings.entry(tag);
        match entry {
            Entry::Occupied(occ_entry) => return Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                if let Some(ref mut elem) = self.elements.get_mut(&tag) {
                    let value: String = elem.parse_string(cs)?;
                    return Ok(vac_entry.insert(value));
                }
            }
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Element not found: {}", Tag::format_tag_to_display(tag)),
        ))
    }

    fn get_strings(&mut self, tag: u32, cs: CSRef) -> Result<&Vec<String>, Error> {
        let entry: Entry<u32, Vec<String>> = self.string_lists.entry(tag);
        match entry {
            Entry::Occupied(occ_entry) => return Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                if let Some(ref mut elem) = self.elements.get_mut(&tag) {
                    let value: Vec<String> = elem.parse_strings(cs)?;
                    return Ok(vac_entry.insert(value));
                }
            }
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Element not found: {}", Tag::format_tag_to_display(tag)),
        ))
    }

    fn get_f32<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&f32, Error> {
        let entry: Entry<u32, f32> = self.floats.entry(tag);
        match entry {
            Entry::Occupied(occ_entry) => return Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                if let Some(elem) = self.elements.get_mut(&tag) {
                    let value: f32 = elem.parse_f32::<Endian>()?;
                    return Ok(vac_entry.insert(value));
                }
            }
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Element not found: {}", Tag::format_tag_to_display(tag)),
        ))
    }

    fn get_f32s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<f32>, Error> {
        let entry: Entry<u32, Vec<f32>> = self.float_lists.entry(tag);
        match entry {
            Entry::Occupied(occ_entry) => return Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                if let Some(elem) = self.elements.get_mut(&tag) {
                    let value: Vec<f32> = elem.parse_f32s::<Endian>()?;
                    return Ok(vac_entry.insert(value));
                }
            }
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Element not found: {}", Tag::format_tag_to_display(tag)),
        ))
    }

    fn get_f64<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&f64, Error> {
        let entry: Entry<u32, f64> = self.doubles.entry(tag);
        match entry {
            Entry::Occupied(occ_entry) => return Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                if let Some(elem) = self.elements.get_mut(&tag) {
                    let value: f64 = elem.parse_f64::<Endian>()?;
                    return Ok(vac_entry.insert(value));
                }
            }
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Element not found: {}", Tag::format_tag_to_display(tag)),
        ))
    }

    fn get_f64s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<f64>, Error> {
        let entry: Entry<u32, Vec<f64>> = self.double_lists.entry(tag);
        match entry {
            Entry::Occupied(occ_entry) => return Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                if let Some(elem) = self.elements.get_mut(&tag) {
                    let value: Vec<f64> = elem.parse_f64s::<Endian>()?;
                    return Ok(vac_entry.insert(value));
                }
            }
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Element not found: {}", Tag::format_tag_to_display(tag)),
        ))
    }

    fn get_i16<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&i16, Error> {
        let entry: Entry<u32, i16> = self.shorts.entry(tag);
        match entry {
            Entry::Occupied(occ_entry) => return Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                if let Some(elem) = self.elements.get_mut(&tag) {
                    let value: i16 = elem.parse_i16::<Endian>()?;
                    return Ok(vac_entry.insert(value));
                }
            }
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Element not found: {}", Tag::format_tag_to_display(tag)),
        ))
    }

    fn get_i16s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<i16>, Error> {
        let entry: Entry<u32, Vec<i16>> = self.short_lists.entry(tag);
        match entry {
            Entry::Occupied(occ_entry) => return Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                if let Some(elem) = self.elements.get_mut(&tag) {
                    let value: Vec<i16> = elem.parse_i16s::<Endian>()?;
                    return Ok(vac_entry.insert(value));
                }
            }
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Element not found: {}", Tag::format_tag_to_display(tag)),
        ))
    }

    fn get_i32<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&i32, Error> {
        let entry: Entry<u32, i32> = self.ints.entry(tag);
        match entry {
            Entry::Occupied(occ_entry) => return Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                if let Some(elem) = self.elements.get_mut(&tag) {
                    let value: i32 = elem.parse_i32::<Endian>()?;
                    return Ok(vac_entry.insert(value));
                }
            }
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Element not found: {}", Tag::format_tag_to_display(tag)),
        ))
    }

    fn get_i32s<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&Vec<i32>, Error> {
        let entry: Entry<u32, Vec<i32>> = self.int_lists.entry(tag);
        match entry {
            Entry::Occupied(occ_entry) => return Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                if let Some(elem) = self.elements.get_mut(&tag) {
                    let value: Vec<i32> = elem.parse_i32s::<Endian>()?;
                    return Ok(vac_entry.insert(value));
                }
            }
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Element not found: {}", Tag::format_tag_to_display(tag)),
        ))
    }

    fn get_u16<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&u16, Error> {
        let entry: Entry<u32, u16> = self.ushorts.entry(tag);
        match entry {
            Entry::Occupied(occ_entry) => return Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                if let Some(elem) = self.elements.get_mut(&tag) {
                    let value: u16 = elem.parse_u16::<Endian>()?;
                    return Ok(vac_entry.insert(value));
                }
            }
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Element not found: {}", Tag::format_tag_to_display(tag)),
        ))
    }

    fn get_u32<Endian: ByteOrder>(&mut self, tag: u32) -> Result<&u32, Error> {
        let entry: Entry<u32, u32> = self.uints.entry(tag);
        match entry {
            Entry::Occupied(occ_entry) => return Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                if let Some(elem) = self.elements.get_mut(&tag) {
                    let value: u32 = elem.parse_u32::<Endian>()?;
                    return Ok(vac_entry.insert(value));
                }
            }
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Element not found: {}", Tag::format_tag_to_display(tag)),
        ))
    }

    fn put_element(&mut self, tag: u32, element: DicomElement) -> Option<DicomElement> {
        self.elements.insert(tag, element)
    }
}
