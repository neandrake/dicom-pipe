#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::fmt;
use std::hash::{Hash, Hasher};

/// Value Representation
#[derive(Debug, Eq)]
pub struct VR {
	pub ident: &'static str,
	pub name: &'static str,
	pub code: u32,
	pub padding: u32,

	/// Part 5, 7.1.2 -- Total bytes for Tag, VR, and VL
	pub explicit_vr_header_bytes: u32,
}

impl PartialEq for VR {
	fn eq(&self, other: &VR) -> bool {
		self.code.eq(&other.code)
	}
}

impl Hash for VR {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.code.hash(state)
	}
}

/// Unique Identifiers
#[derive(Debug, Eq)]
pub struct UID {
    ident: &'static str,
    uid: &'static str,
    name: &'static str,
}

impl UID {
    pub fn new(uid: &'static str, ident: &'static str, name: &'static str) -> UID {
        UID {
            uid: uid,
            ident: ident,
            name: name,
        }
    }

    pub fn get_ident(&self) -> &'static str {
        self.ident
    }

    pub fn get_uid(&self) -> &'static str {
        self.uid
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }
}

impl PartialEq for UID {
    fn eq(&self, other: &UID) -> bool {
        self.uid.eq(other.uid)
    }
}

impl Hash for UID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}


/// Value Multiplicity
#[derive(Debug, PartialEq, Eq)]
pub enum VM {
    /// 1 or 1-n (not sure how this is different from 1-n)
    OneOrOneOrMore,
    /// 1, 2, 3, etc.
    N(u32),
    /// 1-2, etc.
    MToN(u32, u32),
    /// 1-n, 2-n, 3-n, NOrMore(6), etc.
    NOrMore(u32),
    /// 2-2n, 3-3n, etc.
    MultipleOfN(u32),
}


/// DICOM Element
#[derive(Debug, Eq)]
pub struct Tag {
    ident: &'static str,
    tag: u32,
    implicit_vr: Option<&'static VR>,
    vm: VM,
    desc: &'static str,
}

impl Tag {
    pub fn get_ident(&self) -> &'static str {
        self.ident
    }

    pub fn get_tag(&self) -> u32 {
        self.tag
    }

    pub fn get_implicit_vr(&self) -> Option<&'static VR> {
        self.implicit_vr
    }

    pub fn get_vm(&self) -> &VM {
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

/// TagType
pub enum TagType {
    Undefined(u32),
    Defined(&'static Tag),
}

impl fmt::UpperHex for TagType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagType::Undefined(num) => {
                write!(f, "0x{:08X}", num)
            },
            TagType::Defined(tag) => {
                write!(f, "0x{:08X}", tag.tag)
            }
        }
    }
}

impl fmt::LowerHex for TagType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagType::Undefined(num) => {
                write!(f, "0x{:08x}", num)
            },
            TagType::Defined(tag) => {
                write!(f, "0x{:08x}", tag.tag)
            }
        }
    }
}


/// Transfer Syntax UIDs
#[derive(Debug, Eq)]
pub struct TransferSyntax<'ts_lt> {
    uid: &'ts_lt UID,
    explicit_vr: bool,
    big_endian: bool,
    deflated: bool,
    encapsulated: bool,
}

impl<'ts_lt> PartialEq for TransferSyntax<'ts_lt> {
    fn eq(&self, other: &TransferSyntax) -> bool {
        self.uid.eq(other.uid)
    }
}

impl<'ts_lt> Hash for TransferSyntax<'ts_lt> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}

impl<'ts_lt> TransferSyntax<'ts_lt> {
    pub fn new<'new_ts_lt>(uid: &'new_ts_lt UID,
        explicit_vr: bool,
        big_endian: bool,
        deflated: bool,
        encapsulated: bool) -> TransferSyntax {
        TransferSyntax {
            uid: uid,
            explicit_vr: explicit_vr,
            big_endian: big_endian,
            deflated: deflated,
            encapsulated: encapsulated,
        }
    }

    pub fn get_uid(&self) -> &'ts_lt UID {
        self.uid
    }

    pub fn is_explicit_vr(&self) -> bool {
        self.explicit_vr
    }

    pub fn is_big_endian(&self) -> bool {
        self.big_endian
    }

    pub fn is_deflated(&self) -> bool {
        self.deflated
    }

    pub fn is_encapsulated(&self) -> bool {
        self.encapsulated
    }

    pub fn uncompressed(&self) -> bool {
        !self.deflated && !self.encapsulated
    }
}
