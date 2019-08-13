//! This module is for defining the bare minimum constant values needed for parsing DICOM.
//! Note that without other tag definitions Implicit VR can't be parsed properly. It is the
//! responsibility of the user of `DicomStreamParser` to provide it with necessary tag and
//! transfer syntax lookup in order to fully parse a stream.

/// The tags necessary for parsing a DICOM stream
pub mod tags {
    pub const FILE_META_INFORMATION_GROUP_LENGTH: u32 = 0x0002_0000;
    pub const TRANSFER_SYNTAX_UID: u32 = 0x0002_0010;
    pub const SPECIFIC_CHARACTER_SET: u32 = 0x0008_0005;
    pub const ITEM: u32 = 0xFFFE_E000;
    pub const SEQUENCE_DELIMITATION_ITEM: u32 = 0xFFFE_E0DD;
}

/// The transfer syntaxes needed for parsing a DICOM stream
pub mod ts {
    use crate::defn::ts::TransferSyntax;

    /// Explicit VR Little Endian
    ///
    /// - **UID:** 1.2.840.10008.1.2.1
    pub static EXPLICIT_VR_LE: TransferSyntax = TransferSyntax {
        uid: &super::uids::EXPLICIT_VR_LE,
        explicit_vr: true,
        big_endian: false,
        deflated: false,
        encapsulated: false,
    };
}

/// The UIDs needed for parsing a DICOM stream
pub mod uids {
    use crate::defn::uid::UID;

    /// Explicit VR Little Endian
    ///
    /// - **UID:** 1.2.840.10008.1.2.1
    /// - **UID Type:** Transfer Syntax
    pub static EXPLICIT_VR_LE: UID = UID {
        ident: "ExplicitVRLittleEndian",
        uid: "1.2.840.10008.1.2.1",
        name: "Explicit VR Little Endian",
    };
}
