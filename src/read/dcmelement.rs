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
        write!(f, "DicomElement {{ tag: {:08X}  vr: {}  vl: {} }}", self.tag, self.vr.ident, self.vl)
    }
}