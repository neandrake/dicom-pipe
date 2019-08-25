//! Transfer Syntax UIDs

use crate::defn::uid::UIDRef;
use std::hash::{Hash, Hasher};

pub type TSRef = &'static TransferSyntax;

/// Transfer Syntax
#[derive(Debug, Eq)]
pub struct TransferSyntax {
    /// The UID of the Transfer Syntax
    pub uid: UIDRef,
    /// If Native Encoding, whether this encodes with ExplicitVR or ImplicitVR. The majority of
    /// transfer syntaxes are explicit VR.
    pub explicit_vr: bool,
    /// If Native Encoding, whether this encodes using BigEndian or LittleEndian. The majority of
    /// transfer syntaxes are little endian.
    pub big_endian: bool,
    /// A few transfer syntaxes are deflated which means all contents after the file meta segment
    /// are compressed using RFC 1951, including the dicom encodings.
    pub deflated: bool,
    /// Encapsulated transfer syntaxes (basically anything that isn't the handful of
    /// implicit/explicit big/little endian), including all jpeg - the content of the PixelData
    /// segment is encoded in a different format from the rest of the dicom elements.
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
