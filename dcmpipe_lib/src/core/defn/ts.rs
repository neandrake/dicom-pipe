//! Transfer Syntax

use std::hash::{Hash, Hasher};

use super::uid::UIDRef;

pub type TSRef = &'static TransferSyntax;

/// Transfer Syntax Definition

// TransferSyntaxes are defined at compile-time and never instantiated at runtime. Since these
// boolean values are only ever read by the use of this API, the boolean fields are more ergonomic
// to use compared to converting to enums.
#[allow(clippy::struct_excessive_bools)]
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
    /// Get the transfer syntax's UID.
    #[must_use]
    pub fn uid(&self) -> UIDRef {
        self.uid
    }

    /// Indicates whether this transfer syntax uses Explicit or Implicit VR.
    #[must_use]
    pub fn explicit_vr(&self) -> bool {
        self.explicit_vr
    }

    /// Indicates whether this transfer syntax uses big or little endian.
    #[must_use]
    pub fn big_endian(&self) -> bool {
        self.big_endian
    }

    /// Indicates whether this transfer syntax uses compression/deflation.
    #[must_use]
    pub fn deflated(&self) -> bool {
        self.deflated
    }

    /// Indicates whether this transfer syntax uses encapsulation (?).
    #[must_use]
    pub fn encapsulated(&self) -> bool {
        self.encapsulated
    }

    /// Indicates whether this transfer syntax uses standard uncompressed data encoding.
    #[must_use]
    pub fn is_uncompressed(&self) -> bool {
        !self.deflated && !self.encapsulated
    }
}
