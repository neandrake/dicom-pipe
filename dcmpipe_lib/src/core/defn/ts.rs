/*
   Copyright 2024 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! Transfer Syntax

use std::hash::{Hash, Hasher};

use crate::core::defn::uid::UIDRef;

pub type TSRef = &'static TransferSyntax;

// TransferSyntaxes are defined at compile-time and never instantiated at runtime. Since these
// boolean values are only ever read by the use of this API, the boolean fields are more ergonomic
// to use compared to converting to enums.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Eq)]
/// Transfer Syntax Definition
pub struct TransferSyntax {
    /// Identifier or name which can be used with a `DicomDictionary`.
    uid: UIDRef,

    /// If Native Encoding, whether this encodes with `ExplicitVR` or `ImplicitVR`. The majority of
    /// transfer syntaxes are explicit VR.
    explicit_vr: bool,

    /// If Native Encoding, whether this encodes using `BigEndian` or `LittleEndian`. The majority of
    /// transfer syntaxes are little endian.
    big_endian: bool,

    /// A few transfer syntaxes are deflated which means all contents after the file meta segment
    /// are compressed using RFC 1951, including the dicom encodings.
    deflated: bool,

    /// Encapsulated transfer syntaxes (basically anything that isn't the handful of
    /// implicit/explicit big/little endian), including all jpeg - the content of the `PixelData`
    /// segment is encoded in a different format from the rest of the dicom elements.
    encapsulated: bool,
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
    /// Create a new TransferSyntax.
    #[must_use]
    pub const fn new(
        uid: UIDRef,
        explicit_vr: bool,
        big_endian: bool,
        deflated: bool,
        encapsulated: bool,
    ) -> Self {
        Self {
            uid,
            explicit_vr,
            big_endian,
            deflated,
            encapsulated,
        }
    }

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
