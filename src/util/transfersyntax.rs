#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::hash::{Hash, Hasher};

use util::uids::UID;

#[derive(Debug)]
pub struct TransferSyntax<'uid_lifetime> {
    uid: &'uid_lifetime UID,
    big_endian: bool,
    explicit_vr: bool,
    deflated: bool,
    encapsulated: bool,
}

impl<'uid_lifetime> TransferSyntax<'uid_lifetime> {
    pub fn new<'new_uid_lifetime>(uid: &'new_uid_lifetime UID,
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

    pub fn get_uid(&self) -> &'uid_lifetime UID {
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

impl<'uid_lifetime> PartialEq for TransferSyntax<'uid_lifetime> {
    fn eq(&self, other: &TransferSyntax) -> bool {
        self.uid.eq(other.uid)
    }
}

impl<'uid_lifetime> Hash for TransferSyntax<'uid_lifetime> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}


pub const ImplicitVRLittleEndian: TransferSyntax<'static> = TransferSyntax {
    uid: & ::util::uids::ImplicitVRLittleEndian,
    big_endian: false,
    explicit_vr: false,
    deflated: false,
    encapsulated: false,
};

#[test]
fn test_diff_ts_instances_eq() {
    let ts_uid: UID = UID::new(
        "1.2.840.10008.1.2",
        "Blarf ImplicitVRLittleEndian",
        "Blarf Implicit VR Little Endian",
    );
    let implicit_vr_le: TransferSyntax = TransferSyntax::new(
        &ts_uid,
        false,
        false,
        false,
        false,
    );
    assert_eq!(ImplicitVRLittleEndian, implicit_vr_le);
}