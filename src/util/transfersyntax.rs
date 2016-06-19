#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::collections::hash_map::{HashMap, Iter};
use std::hash::{Hash, Hasher};

use util::uids::UID;


pub struct TransferSyntaxLookup<'ts_lt> {
    uid_to_ts: HashMap<&'ts_lt UID, &'ts_lt TransferSyntax<'ts_lt>>,
    ident_to_ts: HashMap<&'ts_lt str, &'ts_lt TransferSyntax<'ts_lt>>,
}

impl<'ts_lt> TransferSyntaxLookup<'ts_lt> {
    pub fn new<'new_ts_lt>() -> TransferSyntaxLookup<'new_ts_lt> {
        let mut uid_to_ts: HashMap<&UID, &TransferSyntax> = HashMap::new();
        let mut ident_to_ts: HashMap<&str, &TransferSyntax> = HashMap::new();
        
        uid_to_ts.insert(ImplicitVRLittleEndian.uid, &ImplicitVRLittleEndian);
        ident_to_ts.insert(ImplicitVRLittleEndian.uid.get_ident(), &ImplicitVRLittleEndian);
        
        uid_to_ts.insert(ImplicitVRBigEndian.uid, &ImplicitVRBigEndian);
        ident_to_ts.insert(ImplicitVRBigEndian.uid.get_ident(), &ImplicitVRBigEndian);

        uid_to_ts.insert(ExplicitVRLittleEndian.uid, &ExplicitVRLittleEndian);
        ident_to_ts.insert(ExplicitVRLittleEndian.uid.get_ident(), &ExplicitVRLittleEndian);

        uid_to_ts.insert(ExplicitVRBigEndian.uid, &ExplicitVRBigEndian);
        ident_to_ts.insert(ExplicitVRBigEndian.uid.get_ident(), &ExplicitVRBigEndian);

        uid_to_ts.insert(DeflatedExplicitVRLittleEndian.uid, &DeflatedExplicitVRLittleEndian);
        ident_to_ts.insert(DeflatedExplicitVRLittleEndian.uid.get_ident(), &DeflatedExplicitVRLittleEndian);

        uid_to_ts.insert(NoPixelData.uid, &NoPixelData);
        ident_to_ts.insert(NoPixelData.uid.get_ident(), &NoPixelData);

        uid_to_ts.insert(NoPixelDataDeflate.uid, &NoPixelDataDeflate);
        ident_to_ts.insert(NoPixelDataDeflate.uid.get_ident(), &NoPixelDataDeflate);

        uid_to_ts.insert(JPEGBaseline1.uid, &JPEGBaseline1);
        ident_to_ts.insert(JPEGBaseline1.uid.get_ident(), &JPEGBaseline1);

        uid_to_ts.insert(JPEGExtended24.uid, &JPEGExtended24);
        ident_to_ts.insert(JPEGExtended24.uid.get_ident(), &JPEGExtended24);

        uid_to_ts.insert(JPEGLosslessNonHierarchical14.uid, &JPEGLosslessNonHierarchical14);
        ident_to_ts.insert(JPEGLosslessNonHierarchical14.uid.get_ident(), &JPEGLosslessNonHierarchical14);

        uid_to_ts.insert(JPEGLossless.uid, &JPEGLossless);
        ident_to_ts.insert(JPEGLossless.uid.get_ident(), &JPEGLossless);

        TransferSyntaxLookup {
            uid_to_ts: uid_to_ts,
            ident_to_ts: ident_to_ts,
        }
    }

    pub fn get_by_uid(&self, uid: &UID) -> Option<&TransferSyntax<'ts_lt>> {
        self.uid_to_ts.get(uid).map(|ts| *ts)
    }

    pub fn get_by_ident(&self, ident: &str) -> Option<&TransferSyntax<'ts_lt>> {
        self.ident_to_ts.get(ident).map(|ts| *ts)
    }

    pub fn iter(&self) -> Iter<&UID, &TransferSyntax> {
        self.uid_to_ts.iter()
    }
}

#[derive(Debug, Eq)]
pub struct TransferSyntax<'ts_lt> {
    uid: &'ts_lt UID,
    explicit_vr: bool,
    big_endian: bool,
    deflated: bool,
    encapsulated: bool,
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

pub static JPEGBaseline1: TransferSyntax<'static> = TransferSyntax {
    uid: &::util::uids::JPEGBaseline1,
    explicit_vr: true,
    big_endian: false,
    deflated: false,
    encapsulated: false,
};

pub static JPEGExtended24: TransferSyntax<'static> = TransferSyntax {
    uid: &::util::uids::JPEGExtended24,
    explicit_vr: true,
    big_endian: false,
    deflated: false,
    encapsulated: false,
};

pub static JPEGLosslessNonHierarchical14: TransferSyntax<'static> = TransferSyntax {
    uid: &::util::uids::JPEGLosslessNonHierarchical14,
    explicit_vr: true,
    big_endian: false,
    deflated: false,
    encapsulated: false,
};

pub static JPEGLossless: TransferSyntax<'static> = TransferSyntax {
    uid: &::util::uids::JPEGLossless,
    explicit_vr: true,
    big_endian: false,
    deflated: false,
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

#[test]
fn test_lookup() {
    let lookup: TransferSyntaxLookup = TransferSyntaxLookup::new();
    let ts: &TransferSyntax = lookup.get_by_uid(&::util::uids::ImplicitVRLittleEndian).expect("Unable to lookup TransferSyntax by UID");
    assert_eq!(ts, &ImplicitVRLittleEndian);

    let ts_opt: Option<&TransferSyntax> = lookup.get_by_uid(&::util::uids::CTImageStorage);
    assert_eq!(ts_opt.is_none(), true);

    let ts: &TransferSyntax = lookup.get_by_ident("ImplicitVRLittleEndian").expect("Unable to lookup TransferSyntax by ident");
    assert_eq!(ts, &ImplicitVRLittleEndian);

    let ts_opt: Option<&TransferSyntax> = lookup.get_by_ident("Implicit VR Little Endian");
    assert!(ts_opt.is_none(), true);
}

/// Sanity-check of the pre-defined TransferSyntax's to ensure
/// that their defined properties reflect the UID's name.
/// May catch issues with improperly copying over values from definitions.
#[test]
fn test_name_vs_properties() {
    let lookup: TransferSyntaxLookup = TransferSyntaxLookup::new();
    for (_, ts) in lookup.iter() {
        let contains_little: bool = ts.uid.get_ident().contains("LittleEndian");
        let contains_big: bool = ts.uid.get_ident().contains("BigEndian");
        let contains_explicit: bool = ts.uid.get_ident().contains("ExplicitVR");
        let contains_implicit: bool = ts.uid.get_ident().contains("ImplicitVR");
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