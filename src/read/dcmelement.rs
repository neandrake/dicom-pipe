use core::dict::lookup::TAG_BY_VALUE;
use core::vl::ValueLength;
use core::vr::VRRef;

use std::fmt;
use std::io::Cursor;

pub struct DicomElement {
    pub tag: u32,
    pub vr: VRRef,
    pub vl: ValueLength,
    value: Cursor<Vec<u8>>,
}

impl DicomElement {
    pub fn new(tag: u32, vr: VRRef, vl: ValueLength, value: Vec<u8>) -> DicomElement {
        DicomElement {
            tag: tag,
            vr: vr,
            vl: vl,
            value: Cursor::new(value),
        }
    }

    pub fn get_value(&self) -> &Cursor<Vec<u8>> {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.value
    }
}

impl fmt::Debug for DicomElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tag_display: String = if let Some(tag) = TAG_BY_VALUE.get(&self.tag) {
            format!("{}", tag.ident)
        } else {
            format!("{:08X}", self.tag)
        };

        write!(f, "DicomElement {{ tag: {}  vr: {}  vl: {:?} }}", tag_display, self.vr.ident, self.vl)
    }
}