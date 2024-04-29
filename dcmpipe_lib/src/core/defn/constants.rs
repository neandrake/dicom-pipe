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

//! This module defines the minimal constant values necessary for parsing DICOM.

/// The minimal set of tags needed when parsing a DICOM dataset.
pub mod tags {
    pub const FILE_META_INFORMATION_GROUP_LENGTH: u32 = 0x0002_0000;
    pub const TRANSFER_SYNTAX_UID: u32 = 0x0002_0010;
    pub const FILE_META_GROUP_END: u32 = 0x0002_FFFF;

    pub const SPECIFIC_CHARACTER_SET: u32 = 0x0008_0005;
    pub const SOP_INSTANCE_UID: u32 = 0x0008_0018;

    pub const FLOAT_PIXEL_DATA: u32 = 0x7FE0_0008;
    pub const DOUBLE_PIXEL_DATA: u32 = 0x7FE0_0009;
    pub const PIXEL_DATA: u32 = 0x7FE0_0010;

    pub const DATASET_TRAILING_PADDING: u32 = 0xFFFC_FFFC;

    pub const ITEM: u32 = 0xFFFE_E000;
    pub const SEQUENCE_DELIMITATION_ITEM: u32 = 0xFFFE_E0DD;
    pub const ITEM_DELIMITATION_ITEM: u32 = 0xFFFE_E00D;
}

/// The minimal set of transfer syntaxes needed when parsing a DICOM dataset.
pub mod ts {
    #![allow(non_upper_case_globals)]

    use crate::core::defn::ts::TransferSyntax;

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
        uid: &super::uids::ExplicitVRLittleEndian,
        explicit_vr: true,
        big_endian: false,
        deflated: false,
        encapsulated: false,
    };

    /// Explicit VR Big Endian (Retired)
    ///
    /// - **UID:** 1.2.840.10008.1.2.2
    pub static ExplicitVRBigEndian: TransferSyntax = TransferSyntax {
        uid: &super::uids::ExplicitVRBigEndian,
        explicit_vr: true,
        big_endian: true,
        deflated: false,
        encapsulated: false,
    };

    /// Implicit VR Big Endian (Virtual)
    ///
    /// - **UID:**
    pub static ImplicitVRBigEndian: TransferSyntax = TransferSyntax {
        uid: &super::uids::ImplicitVRBigEndian,
        explicit_vr: false,
        big_endian: true,
        deflated: false,
        encapsulated: false,
    };

    /// Deflated Explicit VR Little Endian
    ///
    /// - **UID:** 1.2.840.10008.1.2.1.99
    pub static DeflatedExplicitVRLittleEndian: TransferSyntax = TransferSyntax {
        uid: &super::uids::DeflatedExplicitVRLittleEndian,
        explicit_vr: true,
        big_endian: false,
        deflated: true,
        encapsulated: false,
    };
}

/// The minimal set of UIDs needed when parsing a DICOM dataset.
pub mod uids {
    #![allow(non_upper_case_globals)]

    use crate::core::defn::uid::UID;

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

    /// Implicit VR Big Endian (Virtual)
    ///
    /// - **UID:**
    /// - **UID Type:** Transfer Syntax
    pub static ImplicitVRBigEndian: UID = UID {
        ident: "ImplicitVRBigEndian",
        uid: "",
        name: "Implicit VR Big Endian (Virtual)",
    };
}

/// The transfer syntax lookup for parsing a DICOM dataset.
pub mod lookup {
    use crate::core::defn::{dcmdict::DicomDictionary, tag::TagRef, ts::TSRef, uid::UIDRef};

    use super::{ts, uids};

    /// A minimal `DicomDictionary` necessary for parsing through a dicom dataset. Only implements a
    /// minimal set of `get_ts_by_uid`, all other functions return `None`.
    pub static MINIMAL_DICOM_DICTIONARY: MinimalDicomDictionary = MinimalDicomDictionary {};

    /// A minimal `DicomDictionary` necessary for parsing through a dicom dataset. Only implements a
    /// minimal set of `get_ts_by_uid`, all other functions return `None`.
    pub struct MinimalDicomDictionary {}
    impl DicomDictionary for MinimalDicomDictionary {
        fn get_ts_by_name(&self, _name: &str) -> Option<TSRef> {
            None
        }

        fn get_ts_by_uid(&self, uid: &str) -> Option<TSRef> {
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

        fn get_tag_by_name(&self, _name: &str) -> Option<TagRef> {
            None
        }

        fn get_tag_by_number(&self, _number: u32) -> Option<TagRef> {
            None
        }

        fn get_uid_by_name(&self, _name: &str) -> Option<UIDRef> {
            None
        }

        fn get_uid_by_uid(&self, _uid: &str) -> Option<UIDRef> {
            None
        }
    }
}
