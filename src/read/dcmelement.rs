use core::dict::lookup::TAG_BY_VALUE;
use core::vl::ValueLength;
use core::vr;

use std::fmt;

pub struct DicomElement {
    pub tag: u32,
    pub vr: &'static vr::VR,
    pub vl: ValueLength,
    pub bytes: Vec<u8>,
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