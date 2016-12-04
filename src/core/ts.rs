#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

//! Transfer Syntax UIDs

use core::dict::uids;
use core::uid::UID;

use std::hash::{Hash, Hasher};


#[derive(Debug, Eq)]
pub struct TransferSyntax<'ts> {
    uid: &'ts UID,
    explicit_vr: bool,
    big_endian: bool,
    deflated: bool,
    encapsulated: bool,
}

impl<'ts> PartialEq for TransferSyntax<'ts> {
    fn eq(&self, other: &TransferSyntax) -> bool {
        self.uid.eq(other.uid)
    }
}

impl<'ts> Hash for TransferSyntax<'ts> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}

impl<'ts> TransferSyntax<'ts> {
    pub fn new(uid: &UID,
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

    pub fn get_uid(&self) -> &'ts UID {
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


pub static ImplicitVRLittleEndian: TransferSyntax<'static> = TransferSyntax {
    uid: &uids::ImplicitVRLittleEndian,
    explicit_vr: false,
    big_endian: false,
    deflated: false,
    encapsulated: false,
};

pub static ExplicitVRLittleEndian: TransferSyntax<'static> = TransferSyntax {
    uid: &uids::ExplicitVRLittleEndian,
    explicit_vr: true,
    big_endian: false,
    deflated: false,
    encapsulated: false,
};

pub static ExplicitVRBigEndian: TransferSyntax<'static> = TransferSyntax {
    uid: &uids::ExplicitVRBigEndian,
    explicit_vr: true,
    big_endian: true,
    deflated: false,
    encapsulated: false,
};

pub static DeflatedExplicitVRLittleEndian: TransferSyntax<'static> = TransferSyntax {
    uid: &uids::DeflatedExplicitVRLittleEndian,
    explicit_vr: true,
    big_endian: false,
    deflated: true,
    encapsulated: false,
};

pub static JPEGBaselineProcess1: TransferSyntax<'static> = TransferSyntax {
    uid: &uids::JPEGBaselineProcess1,
    explicit_vr: true,
    big_endian: false,
    deflated: false,
    encapsulated: false,
};

pub static JPEGExtendedProcess2_and_4: TransferSyntax<'static> = TransferSyntax {
    uid: &uids::JPEGExtendedProcess2_and_4,
    explicit_vr: true,
    big_endian: false,
    deflated: false,
    encapsulated: false,
};

pub static JPEGLosslessNonHierarchicalProcess14: TransferSyntax<'static> = TransferSyntax {
    uid: &uids::JPEGLosslessNonHierarchicalProcess14,
    explicit_vr: true,
    big_endian: false,
    deflated: false,
    encapsulated: false,
};

pub static JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1: TransferSyntax<'static> = TransferSyntax {
    uid: &uids::JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1,
    explicit_vr: true,
    big_endian: false,
    deflated: false,
    encapsulated: false,
};