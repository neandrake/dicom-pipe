use core::dict::lookup::TAG_BY_VALUE;
use core::vr;

use std::fmt;

pub struct DicomElement {
    pub tag: u32,
    pub vr: &'static vr::VR,
    pub vl: u32,
    pub bytes: Vec<u8>,
}

impl fmt::Debug for DicomElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(tag) = TAG_BY_VALUE.get(&self.tag) {
            write!(f, "DicomElement {{ tag: {}  vr: {}  vl: {} }}", tag.ident, self.vr.ident, self.vl)
        } else {
            write!(f, "DicomElement {{ tag: {:08X}  vr: {}  vl: {} }}", self.tag, self.vr.ident, self.vl)
        }
    }
}