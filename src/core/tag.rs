#![allow(non_camel_case_types)]

//! DICOM Element

use core::{vm, vr};

use std::hash::{Hash, Hasher};
use std::fmt::{Formatter, LowerHex, Result as FmtResult, UpperHex};


#[derive(Debug, Eq)]
pub struct Tag {
    ident: &'static str,
    tag: u32,
    implicit_vr: Option<&'static vr::VR>,
    vm: &'static vm::VM,
    desc: &'static str,
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
