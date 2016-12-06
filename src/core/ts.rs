#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

//! Transfer Syntax UIDs

use core::uid::UID;

use std::hash::{Hash, Hasher};


#[derive(Debug, Eq)]
pub struct TransferSyntax {
    pub uid: &'static UID,
    pub explicit_vr: bool,
    pub big_endian: bool,
    pub deflated: bool,
    pub encapsulated: bool,
}

impl PartialEq for TransferSyntax {
    fn eq(&self, other: &TransferSyntax) -> bool {
        self.uid.eq(other.uid)
    }
}

impl Hash for TransferSyntax {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}

impl TransferSyntax {
    pub fn new(uid: &'static UID,
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

    pub fn get_uid(&self) -> &UID {
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
