//! This module is for defining the bare minimum constant values needed for parsing DICOM.

/// The tags necessary for parsing a DICOM stream
pub mod tags {
    pub const FILE_META_INFORMATION_GROUP_LENGTH: u32 = 0x0002_0000;
    pub const TRANSFER_SYNTAX_UID: u32 = 0x0002_0010;
    pub const SPECIFIC_CHARACTER_SET: u32 = 0x0008_0005;
    pub const ITEM: u32 = 0xFFFE_E000;
    pub const SEQUENCE_DELIMITATION_ITEM: u32 = 0xFFFE_E0DD;
    pub const ITEM_DELIMITATION_ITEM: u32 = 0xFFFE_E00D;
}

/// The transfer syntaxes needed for parsing a DICOM stream
pub mod ts {
    #![allow(non_upper_case_globals)]

    use crate::defn::ts::TransferSyntax;

    /// Implicit VR Little Endian: Default Transfer Syntax for DICOM
    ///
    /// - **UID:** 1.2.840.10008.1.2
    pub static ImplicitVRLittleEndian: TransferSyntax = TransferSyntax {
        uid: &super::uids::ImplicitVRLittleEndian,
        explicit_vr: false,
        big_endian: false,
        deflated: false,
        encapsulated: false,
    };

    /// Explicit VR Little Endian
    ///
    /// - **UID:** 1.2.840.10008.1.2.1
    pub static ExplicitVRLittleEndian: TransferSyntax = TransferSyntax {
        uid: &&super::uids::ExplicitVRLittleEndian,
        explicit_vr: true,
        big_endian: false,
        deflated: false,
        encapsulated: false,
    };

    /// Deflated Explicit VR Little Endian
    ///
    /// - **UID:** 1.2.840.10008.1.2.1.99
    pub static DeflatedExplicitVRLittleEndian: TransferSyntax = TransferSyntax {
        uid: &&super::uids::DeflatedExplicitVRLittleEndian,
        explicit_vr: true,
        big_endian: false,
        deflated: true,
        encapsulated: false,
    };

    /// Explicit VR Big Endian (Retired)
    ///
    /// - **UID:** 1.2.840.10008.1.2.2
    pub static ExplicitVRBigEndian: TransferSyntax = TransferSyntax {
        uid: &&super::uids::ExplicitVRBigEndian,
        explicit_vr: true,
        big_endian: true,
        deflated: false,
        encapsulated: false,
    };
}

/// The UIDs needed for parsing a DICOM stream
pub mod uids {
    #![allow(non_upper_case_globals)]

    use crate::defn::uid::UID;

    /// Implicit VR Little Endian: Default Transfer Syntax for DICOM
    ///
    /// - **UID:** 1.2.840.10008.1.2
    /// - **UID Type:** Transfer Syntax
    pub static ImplicitVRLittleEndian: UID = UID {
        ident: "ImplicitVRLittleEndian",
        uid: "1.2.840.10008.1.2",
        name: "Implicit VR Little Endian: Default Transfer Syntax for DICOM",
    };

    /// Explicit VR Little Endian
    ///
    /// - **UID:** 1.2.840.10008.1.2.1
    /// - **UID Type:** Transfer Syntax
    pub static ExplicitVRLittleEndian: UID = UID {
        ident: "ExplicitVRLittleEndian",
        uid: "1.2.840.10008.1.2.1",
        name: "Explicit VR Little Endian",
    };

    /// Deflated Explicit VR Little Endian
    ///
    /// - **UID:** 1.2.840.10008.1.2.1.99
    /// - **UID Type:** Transfer Syntax
    pub static DeflatedExplicitVRLittleEndian: UID = UID {
        ident: "DeflatedExplicitVRLittleEndian",
        uid: "1.2.840.10008.1.2.1.99",
        name: "Deflated Explicit VR Little Endian",
    };

    /// Explicit VR Big Endian (Retired)
    ///
    /// - **UID:** 1.2.840.10008.1.2.2
    /// - **UID Type:** Transfer Syntax
    pub static ExplicitVRBigEndian: UID = UID {
        ident: "ExplicitVRBigEndian",
        uid: "1.2.840.10008.1.2.2",
        name: "Explicit VR Big Endian (Retired)",
    };
}

pub mod lookup {
    #![allow(clippy::unreadable_literal)]

    use super::ts;
    use super::uids;
    use crate::defn::ts::TSRef;

    pub fn get_ts_by_uid(uid: &str) -> Option<TSRef> {
        if uid == uids::ImplicitVRLittleEndian.uid {
            Some(&ts::ImplicitVRLittleEndian)
        } else if uid == uids::ExplicitVRLittleEndian.uid {
            Some(&ts::ExplicitVRLittleEndian)
        } else if uid == uids::DeflatedExplicitVRLittleEndian.uid {
            Some(&ts::DeflatedExplicitVRLittleEndian)
        } else if uid == uids::ExplicitVRBigEndian.uid {
            Some(&ts::ExplicitVRBigEndian)
        } else {
            None
        }
    }
}
