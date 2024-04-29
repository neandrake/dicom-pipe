#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::hash::{Hash, Hasher};

use util::uids::UID;

#[derive(Debug)]
pub struct TransferSyntax<'uid_lifetime> {
    uid: &'uid_lifetime UID,
    explicit_vr: bool,
    big_endian: bool,
    deflated: bool,
    encapsulated: bool,
}

impl<'uid_lifetime> TransferSyntax<'uid_lifetime> {
    pub fn new<'new_uid_lifetime>(uid: &'new_uid_lifetime UID,
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

    pub fn get_uid(&self) -> &'uid_lifetime UID {
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


pub static ImplicitVRLittleEndian: TransferSyntax<'static> = TransferSyntax {
    uid: &::util::uids::ImplicitVRLittleEndian,
    explicit_vr: false,
    big_endian: false,
    deflated: false,
    encapsulated: false,
};

pub static ImplicitVRBigEndian: TransferSyntax<'static> = TransferSyntax {
    uid: &::util::uids::ImplicitVRBigEndian,
    explicit_vr: false,
    big_endian: true,
    deflated: false,
    encapsulated: false,
};

pub static ExplicitVRLittleEndian: TransferSyntax<'static> = TransferSyntax {
    uid: &::util::uids::ExplicitVRLittleEndian,
    explicit_vr: true,
    big_endian: false,
    deflated: false,
    encapsulated: false,
};

pub static ExplicitVRBigEndian: TransferSyntax<'static> = TransferSyntax {
    uid: &::util::uids::ExplicitVRBigEndian,
    explicit_vr: true,
    big_endian: true,
    deflated: false,
    encapsulated: false,
};

pub static DeflatedExplicitVRLittleEndian: TransferSyntax<'static> = TransferSyntax {
    uid: &::util::uids::DeflatedExplicitVRLittleEndian,
    explicit_vr: true,
    big_endian: false,
    deflated: true,
    encapsulated: false,
};

pub static NoPixelData: TransferSyntax<'static> = TransferSyntax {
    uid: &::util::uids::NoPixelData,
    explicit_vr: true,
    big_endian: false,
    deflated: false,
    encapsulated: false,
};

pub static NoPixelDataDeflate: TransferSyntax<'static> = TransferSyntax {
    uid: &::util::uids::NoPixelDataDeflate,
    explicit_vr: true,
    big_endian: false,
    deflated: true,
    encapsulated: false,
};

/// Checks that PartialEq is sanely implemented and relies only on UID
#[test]
fn test_diff_instances_eq() {
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
    assert_eq!(&ImplicitVRLittleEndian, &implicit_vr_le);
}

/// Sanity-check of the pre-defined TransferSyntax's to ensure
/// that their defined properties reflect the UID's name.
/// May catch issues with improperly copying over values from definitions.
#[test]
fn test_name_vs_properties() {
    let known_ts: Vec<&TransferSyntax> = vec![
        &ImplicitVRLittleEndian,
        &ImplicitVRBigEndian,
        &ExplicitVRLittleEndian,
        &ExplicitVRBigEndian,
        &DeflatedExplicitVRLittleEndian,
        &NoPixelData,
        &NoPixelDataDeflate,
    ];

    for ts in known_ts {
        let contains_little: bool = ts.uid.get_ident().contains("Little");
        let contains_big: bool = ts.uid.get_ident().contains("Big");
        let contains_explicit: bool = ts.uid.get_ident().contains("Explicit");
        let contains_implicit: bool = ts.uid.get_ident().contains("Implicit");
        let contains_deflate: bool = ts.uid.get_ident().contains("Deflate");
        let contains_encapsulated: bool = ts.uid.get_ident().contains("Encapsulated");

        if contains_little {
            assert!(!ts.big_endian);
        } else if contains_big {
            assert!(ts.big_endian);
        } else { 
            // Currently the defined/known TS's which don't have Big/Little in the name are LittleEndian
            assert!(!ts.big_endian);
        }

        if contains_explicit {
            assert!(ts.explicit_vr);
        } else if contains_implicit {
            assert!(!ts.explicit_vr);
        } else {
            // Currently the defined/known TS's which don't have Implicit/Explicit in the name are Explicit
            assert!(ts.explicit_vr);
        }

        assert_eq!(contains_deflate, ts.deflated);
        assert_eq!(contains_encapsulated, ts.encapsulated);
    }
}