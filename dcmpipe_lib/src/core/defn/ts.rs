//! Transfer Syntax

use std::hash::{Hash, Hasher};

use super::uid::UIDRef;

pub type TSRef = &'static TransferSyntax;

/// Transfer Syntax Definition
#[derive(Debug, Eq)]
pub struct TransferSyntax {
    /// Identifier or name which can be used with a `DicomDictionary`.
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

    /// Get the transfer syntax's UID.
    pub fn uid(&self) -> UIDRef {
        self.uid
    }

    /// Indicates whether this transfer syntax uses Explicit or Implicit VR.
    pub fn explicit_vr(&self) -> bool {
        self.explicit_vr
    }

    /// Indicates whether this transfer syntax uses big or little endian.
    pub fn big_endian(&self) -> bool {
        self.big_endian
    }

    /// Indicates whether this transfer syntax uses compression/deflation.
    pub fn deflated(&self) -> bool {
        self.deflated
    }

    /// Indicates whether this transfer syntax uses encapsulation (?).
    pub fn encapsulated(&self) -> bool {
        self.encapsulated
    }

    /// Indicates whether this transfer syntax uses standard uncompressed data encoding.
    pub fn is_uncompressed(&self) -> bool {
        !self.deflated && !self.encapsulated
    }
}
