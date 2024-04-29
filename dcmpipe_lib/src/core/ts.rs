//! Transfer Syntax UIDs

use crate::core::uid::UIDRef;
use std::hash::{Hash, Hasher};

pub type TSRef = &'static TransferSyntax;

/// Transfer Syntax
#[derive(Debug, Eq)]
pub struct TransferSyntax {
    /// The UID of the Transfer Syntax
    pub uid: UIDRef,
    /// If Native Encoding, whether this encodes with ExplicitVR or ImplicitVR
    pub explicit_vr: bool,
    /// If Native Encoding, whether this encodes using BigEndian or LittleEndian
    pub big_endian: bool,
    /// -- unsure
    pub deflated: bool,
    /// Whether this is Native Encoding or Encapsulated/Compressed
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
    pub fn new(
        uid: UIDRef,
        explicit_vr: bool,
        big_endian: bool,
        deflated: bool,
        encapsulated: bool,
    ) -> TransferSyntax {
        TransferSyntax {
            uid,
            explicit_vr,
            big_endian,
            deflated,
            encapsulated,
        }
    }

    pub fn get_uid(&self) -> UIDRef {
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