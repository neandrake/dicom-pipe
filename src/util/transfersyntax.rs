#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::hash::{Hash, Hasher};

use util::uids::UID;

pub struct TransferSyntax {
    uid: &'static UID,
    big_endian: bool,
    explicit_vr: bool,
    deflated: bool,
    encapsulated: bool,
}

impl TransferSyntax {
    pub fn new(uid: &'static UID,
        big_endian: bool,
        explicit_vr: bool,
        deflated: bool,
        encapsulated: bool) -> TransferSyntax {
        TransferSyntax {
            uid: uid,
            big_endian: big_endian,
            explicit_vr: explicit_vr,
            deflated: deflated,
            encapsulated: encapsulated,
        }
    }

    pub fn get_uid(&self) -> &'static UID {
        self.uid
    }

    pub fn is_big_endian(&self) -> bool {
        self.big_endian
    }

    pub fn is_explicit_vr(&self) -> bool {
        self.explicit_vr
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


pub static ImplicitVRLittleEndian: &'static TransferSyntax = &TransferSyntax {
    uid: & ::util::uids::ImplicitVRLittleEndian,
    big_endian: false,
    explicit_vr: false,
    deflated: false,
    encapsulated: false,
};
