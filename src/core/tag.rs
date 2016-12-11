#![allow(dead_code)]
#![allow(non_camel_case_types)]

//! DICOM Data Elements

use core::{vm, vr};

use std::hash::{Hash, Hasher};
use std::fmt::{Formatter, LowerHex, Result as FmtResult, UpperHex};


/// DICOM Data Element (Tag)
#[derive(Debug, Eq)]
pub struct Tag {
    /// Some identifier or name, useful for lookup (no spaces - matches definition/name in code)
    pub ident: &'static str,
    /// The tag value
    pub tag: u32,
    /// The default VR which should be used to read this tag -- used for ImplicitVR
    /// This should maybe be a Vec<&VR> instead as a handful of tags can use several
    pub implicit_vr: Option<&'static vr::VR>,
    /// The value multiplicity -- how many values this tag can have
    pub vm: &'static vm::VM,
    /// Long name or description
    pub desc: &'static str,
}

impl Tag {
    pub fn get_ident(&self) -> &'static str {
        self.ident
    }

    pub fn get_tag(&self) -> u32 {
        self.tag
    }

    pub fn get_implicit_vr(&self) -> Option<&'static vr::VR> {
        self.implicit_vr
    }

    pub fn get_vm(&self) -> &'static vm::VM {
        &self.vm
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Tag) -> bool {
        self.tag.eq(&other.tag)
    }
}

impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
    }
}


pub enum ElementTag<'et> {
    Undefined(u32),
    Defined(&'et Tag),
}

impl<'et> UpperHex for ElementTag<'et> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            ElementTag::Undefined(num) => {
                write!(f, "0x{:08X}", num)
            },
            ElementTag::Defined(tag) => {
                write!(f, "0x{:08X}", tag.tag)
            }
        }
    }
}

impl<'et> LowerHex for ElementTag<'et> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            ElementTag::Undefined(num) => {
                write!(f, "0x{:08x}", num)
            },
            ElementTag::Defined(tag) => {
                write!(f, "0x{:08x}", tag.tag)
            }
        }
    }
}