use crate::core::dcmelement::DicomElement;
use std::collections::hash_map::{Entry, HashMap};
use std::io::Error;

/// A cache of the values parsed into native types -- the initial request
/// for a value will result in parsing the raw value and caching it
pub struct DicomValueCache {
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

impl DicomValueCache {
    pub fn new() -> DicomValueCache {
        DicomValueCache {
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
impl DicomValueCache {
    pub fn get_string(&mut self, element: &mut DicomElement) -> Result<&String, Error> {
        let entry: Entry<u32, String> = self.strings.entry(element.tag);
        match entry {
            Entry::Occupied(occ_entry) => Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                let value: String = element.parse_string()?;
                Ok(vac_entry.insert(value))
            }
        }
    }

    pub fn get_strings(&mut self, element: &mut DicomElement) -> Result<&Vec<String>, Error> {
        let entry: Entry<u32, Vec<String>> = self.string_lists.entry(element.tag);
        match entry {
            Entry::Occupied(occ_entry) => Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                let value: Vec<String> = element.parse_strings()?;
                Ok(vac_entry.insert(value))
            }
        }
    }

    pub fn get_f32(&mut self, element: &mut DicomElement) -> Result<&f32, Error> {
        let entry: Entry<u32, f32> = self.floats.entry(element.tag);
        match entry {
            Entry::Occupied(occ_entry) => Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                let value: f32 = element.parse_f32()?;
                Ok(vac_entry.insert(value))
            }
        }
    }

    pub fn get_f32s(&mut self, element: &mut DicomElement) -> Result<&Vec<f32>, Error> {
        let entry: Entry<u32, Vec<f32>> = self.float_lists.entry(element.tag);
        match entry {
            Entry::Occupied(occ_entry) => Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                let value: Vec<f32> = element.parse_f32s()?;
                Ok(vac_entry.insert(value))
            }
        }
    }

    pub fn get_f64(&mut self, element: &mut DicomElement) -> Result<&f64, Error> {
        let entry: Entry<u32, f64> = self.doubles.entry(element.tag);
        match entry {
            Entry::Occupied(occ_entry) => Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                let value: f64 = element.parse_f64()?;
                Ok(vac_entry.insert(value))
            }
        }
    }

    pub fn get_f64s(&mut self, element: &mut DicomElement) -> Result<&Vec<f64>, Error> {
        let entry: Entry<u32, Vec<f64>> = self.double_lists.entry(element.tag);
        match entry {
            Entry::Occupied(occ_entry) => Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                let value: Vec<f64> = element.parse_f64s()?;
                Ok(vac_entry.insert(value))
            }
        }
    }

    pub fn get_i16(&mut self, element: &mut DicomElement) -> Result<&i16, Error> {
        let entry: Entry<u32, i16> = self.shorts.entry(element.tag);
        match entry {
            Entry::Occupied(occ_entry) => Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                let value: i16 = element.parse_i16()?;
                Ok(vac_entry.insert(value))
            }
        }
    }

    pub fn get_i16s(&mut self, element: &mut DicomElement) -> Result<&Vec<i16>, Error> {
        let entry: Entry<u32, Vec<i16>> = self.short_lists.entry(element.tag);
        match entry {
            Entry::Occupied(occ_entry) => Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                let value: Vec<i16> = element.parse_i16s()?;
                Ok(vac_entry.insert(value))
            }
        }
    }

    pub fn get_i32(&mut self, element: &mut DicomElement) -> Result<&i32, Error> {
        let entry: Entry<u32, i32> = self.ints.entry(element.tag);
        match entry {
            Entry::Occupied(occ_entry) => Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                let value: i32 = element.parse_i32()?;
                Ok(vac_entry.insert(value))
            }
        }
    }

    pub fn get_i32s(&mut self, element: &mut DicomElement) -> Result<&Vec<i32>, Error> {
        let entry: Entry<u32, Vec<i32>> = self.int_lists.entry(element.tag);
        match entry {
            Entry::Occupied(occ_entry) => Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                let value: Vec<i32> = element.parse_i32s()?;
                Ok(vac_entry.insert(value))
            }
        }
    }

    pub fn get_u16(&mut self, element: &mut DicomElement) -> Result<&u16, Error> {
        let entry: Entry<u32, u16> = self.ushorts.entry(element.tag);
        match entry {
            Entry::Occupied(occ_entry) => Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                let value: u16 = element.parse_u16()?;
                return Ok(vac_entry.insert(value));
            }
        }
    }

    pub fn get_u32(&mut self, element: &mut DicomElement) -> Result<&u32, Error> {
        let entry: Entry<u32, u32> = self.uints.entry(element.tag);
        match entry {
            Entry::Occupied(occ_entry) => Ok(occ_entry.into_mut()),
            Entry::Vacant(vac_entry) => {
                let value: u32 = element.parse_u32()?;
                return Ok(vac_entry.insert(value))
            }
        }
    }
}
