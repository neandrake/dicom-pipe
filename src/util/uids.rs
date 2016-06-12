#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct UID {
    uid: &'static str,
    ident: &'static str,
    name: &'static str,
}

impl UID {
    pub fn new(uid: &'static str, ident: &'static str, name: &'static str) -> UID {
        UID {
            uid: uid,
            ident: ident,
            name: name,
        }
    }
}

impl PartialEq for UID {
    fn eq(&self, other: &UID) -> bool {
        self.uid.eq(other.uid)
    }
}

impl Hash for UID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}


/// Verification SOP Class - SOP Class, "1.2.840.10008.1.1"
pub const VerificationSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.1.1",
    ident: "VerificationSOPClass",
    name: "Verification SOP Class",
};

/// Implicit VR Little Endian - Transfer Syntax, "1.2.840.10008.1.2"
pub const ImplicitVRLittleEndian: &'static UID = &UID {
    uid: "1.2.840.10008.1.2",
    ident: "ImplicitVRLittleEndian",
    name: "Implicit VR Little Endian",
};

/// Explicit VR Little Endian - Transfer Syntax, "1.2.840.10008.1.2.1"
pub const ExplicitVRLittleEndian: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.1",
    ident: "ExplicitVRLittleEndian",
    name: "Explicit VR Little Endian",
};

/// Deflated Explicit VR Little Endian - Transfer Syntax, "1.2.840.10008.1.2.1.99"
pub const DeflatedExplicitVRLittleEndian: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.1.99",
    ident: "DeflatedExplicitVRLittleEndian",
    name: "Deflated Explicit VR Little Endian",
};

/// Explicit VR Big Endian - Transfer Syntax, "1.2.840.10008.1.2.2"
pub const ExplicitVRBigEndian: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.2",
    ident: "ExplicitVRBigEndian",
    name: "Explicit VR Big Endian",
};

/// JPEG Baseline (Process 1) - Transfer Syntax, "1.2.840.10008.1.2.4.50"
pub const JPEGBaseline1: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.50",
    ident: "JPEGBaseline1",
    name: "JPEG Baseline (Process 1)",
};

/// JPEG Extended (Process 2 & 4) - Transfer Syntax, "1.2.840.10008.1.2.4.51"
pub const JPEGExtended24: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.51",
    ident: "JPEGExtended24",
    name: "JPEG Extended (Process 2 & 4)",
};

/// JPEG Extended (Process 3 & 5) (Retired) - Transfer Syntax, "1.2.840.10008.1.2.4.52"
pub const JPEGExtended35Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.52",
    ident: "JPEGExtended35Retired",
    name: "JPEG Extended (Process 3 & 5) (Retired)",
};

/// JPEG Spectral Selection, Non-Hierarchical (Process 6 & 8) (Retired) - Transfer Syntax, "1.2.840.10008.1.2.4.53"
pub const JPEGSpectralSelectionNonHierarchical68Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.53",
    ident: "JPEGSpectralSelectionNonHierarchical68Retired",
    name: "JPEG Spectral Selection, Non-Hierarchical (Process 6 & 8) (Retired)",
};

/// JPEG Spectral Selection, Non-Hierarchical (Process 7 & 9) (Retired) - Transfer Syntax, "1.2.840.10008.1.2.4.54"
pub const JPEGSpectralSelectionNonHierarchical79Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.54",
    ident: "JPEGSpectralSelectionNonHierarchical79Retired",
    name: "JPEG Spectral Selection, Non-Hierarchical (Process 7 & 9) (Retired)",
};

/// JPEG Full Progression, Non-Hierarchical (Process 10 & 12) (Retired) - Transfer Syntax, "1.2.840.10008.1.2.4.55"
pub const JPEGFullProgressionNonHierarchical1012Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.55",
    ident: "JPEGFullProgressionNonHierarchical1012Retired",
    name: "JPEG Full Progression, Non-Hierarchical (Process 10 & 12) (Retired)",
};

/// JPEG Full Progression, Non-Hierarchical (Process 11 & 13) (Retired) - Transfer Syntax, "1.2.840.10008.1.2.4.56"
pub const JPEGFullProgressionNonHierarchical1113Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.56",
    ident: "JPEGFullProgressionNonHierarchical1113Retired",
    name: "JPEG Full Progression, Non-Hierarchical (Process 11 & 13) (Retired)",
};

/// JPEG Lossless, Non-Hierarchical (Process 14) - Transfer Syntax, "1.2.840.10008.1.2.4.57"
pub const JPEGLosslessNonHierarchical14: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.57",
    ident: "JPEGLosslessNonHierarchical14",
    name: "JPEG Lossless, Non-Hierarchical (Process 14)",
};

/// JPEG Lossless, Non-Hierarchical (Process 15) (Retired) - Transfer Syntax, "1.2.840.10008.1.2.4.58"
pub const JPEGLosslessNonHierarchical15Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.58",
    ident: "JPEGLosslessNonHierarchical15Retired",
    name: "JPEG Lossless, Non-Hierarchical (Process 15) (Retired)",
};

/// JPEG Extended, Hierarchical (Process 16 & 18) (Retired) - Transfer Syntax, "1.2.840.10008.1.2.4.59"
pub const JPEGExtendedHierarchical1618Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.59",
    ident: "JPEGExtendedHierarchical1618Retired",
    name: "JPEG Extended, Hierarchical (Process 16 & 18) (Retired)",
};

/// JPEG Extended, Hierarchical (Process 17 & 19) (Retired) - Transfer Syntax, "1.2.840.10008.1.2.4.60"
pub const JPEGExtendedHierarchical1719Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.60",
    ident: "JPEGExtendedHierarchical1719Retired",
    name: "JPEG Extended, Hierarchical (Process 17 & 19) (Retired)",
};

/// JPEG Spectral Selection, Hierarchical (Process 20 & 22) (Retired) - Transfer Syntax, "1.2.840.10008.1.2.4.61"
pub const JPEGSpectralSelectionHierarchical2022Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.61",
    ident: "JPEGSpectralSelectionHierarchical2022Retired",
    name: "JPEG Spectral Selection, Hierarchical (Process 20 & 22) (Retired)",
};

/// JPEG Spectral Selection, Hierarchical (Process 21 & 23) (Retired) - Transfer Syntax, "1.2.840.10008.1.2.4.62"
pub const JPEGSpectralSelectionHierarchical2123Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.62",
    ident: "JPEGSpectralSelectionHierarchical2123Retired",
    name: "JPEG Spectral Selection, Hierarchical (Process 21 & 23) (Retired)",
};

/// JPEG Full Progression, Hierarchical (Process 24 & 26) (Retired) - Transfer Syntax, "1.2.840.10008.1.2.4.63"
pub const JPEGFullProgressionHierarchical2426Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.63",
    ident: "JPEGFullProgressionHierarchical2426Retired",
    name: "JPEG Full Progression, Hierarchical (Process 24 & 26) (Retired)",
};

/// JPEG Full Progression, Hierarchical (Process 25 & 27) (Retired) - Transfer Syntax, "1.2.840.10008.1.2.4.64"
pub const JPEGFullProgressionHierarchical2527Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.64",
    ident: "JPEGFullProgressionHierarchical2527Retired",
    name: "JPEG Full Progression, Hierarchical (Process 25 & 27) (Retired)",
};

/// JPEG Lossless, Hierarchical (Process 28) (Retired) - Transfer Syntax, "1.2.840.10008.1.2.4.65"
pub const JPEGLosslessHierarchical28Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.65",
    ident: "JPEGLosslessHierarchical28Retired",
    name: "JPEG Lossless, Hierarchical (Process 28) (Retired)",
};

/// JPEG Lossless, Hierarchical (Process 29) (Retired) - Transfer Syntax, "1.2.840.10008.1.2.4.66"
pub const JPEGLosslessHierarchical29Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.66",
    ident: "JPEGLosslessHierarchical29Retired",
    name: "JPEG Lossless, Hierarchical (Process 29) (Retired)",
};

/// JPEG Lossless, Non-Hierarchical, First-Order Prediction (Process 14 [Selection Value 1]) - Transfer Syntax, "1.2.840.10008.1.2.4.70"
pub const JPEGLossless: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.70",
    ident: "JPEGLossless",
    name: "JPEG Lossless, Non-Hierarchical, First-Order Prediction (Process 14 [Selection Value 1])",
};

/// JPEG-LS Lossless Image Compression - Transfer Syntax, "1.2.840.10008.1.2.4.80"
pub const JPEGLSLossless: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.80",
    ident: "JPEGLSLossless",
    name: "JPEG-LS Lossless Image Compression",
};

/// JPEG-LS Lossy (Near-Lossless) Image Compression - Transfer Syntax, "1.2.840.10008.1.2.4.81"
pub const JPEGLSLossyNearLossless: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.81",
    ident: "JPEGLSLossyNearLossless",
    name: "JPEG-LS Lossy (Near-Lossless) Image Compression",
};

/// JPEG 2000 Image Compression (Lossless Only) - Transfer Syntax, "1.2.840.10008.1.2.4.90"
pub const JPEG2000LosslessOnly: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.90",
    ident: "JPEG2000LosslessOnly",
    name: "JPEG 2000 Image Compression (Lossless Only)",
};

/// JPEG 2000 Image Compression - Transfer Syntax, "1.2.840.10008.1.2.4.91"
pub const JPEG2000: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.91",
    ident: "JPEG2000",
    name: "JPEG 2000 Image Compression",
};

/// JPEG 2000 Part 2 Multi-component Image Compression (Lossless Only) - Transfer Syntax, "1.2.840.10008.1.2.4.92"
pub const JPEG2000Part2MultiComponentLosslessOnly: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.92",
    ident: "JPEG2000Part2MultiComponentLosslessOnly",
    name: "JPEG 2000 Part 2 Multi-component Image Compression (Lossless Only)",
};

/// JPEG 2000 Part 2 Multi-component Image Compression - Transfer Syntax, "1.2.840.10008.1.2.4.93"
pub const JPEG2000Part2MultiComponent: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.93",
    ident: "JPEG2000Part2MultiComponent",
    name: "JPEG 2000 Part 2 Multi-component Image Compression",
};

/// JPIP Referenced - Transfer Syntax, "1.2.840.10008.1.2.4.94"
pub const JPIPReferenced: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.94",
    ident: "JPIPReferenced",
    name: "JPIP Referenced",
};

/// JPIP Referenced Deflate - Transfer Syntax, "1.2.840.10008.1.2.4.95"
pub const JPIPReferencedDeflate: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.95",
    ident: "JPIPReferencedDeflate",
    name: "JPIP Referenced Deflate",
};

/// MPEG2 Main Profile @ Main Level - Transfer Syntax, "1.2.840.10008.1.2.4.100"
pub const MPEG2: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.100",
    ident: "MPEG2",
    name: "MPEG2 Main Profile @ Main Level",
};

/// MPEG2 Main Profile @ High Level - Transfer Syntax, "1.2.840.10008.1.2.4.101"
pub const MPEG2MainProfileHighLevel: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.101",
    ident: "MPEG2MainProfileHighLevel",
    name: "MPEG2 Main Profile @ High Level",
};

/// MPEG-4 AVC/H.264 High Profile / Level 4.1 - Transfer Syntax, "1.2.840.10008.1.2.4.102"
pub const MPEG4AVCH264HighProfileLevel41: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.102",
    ident: "MPEG4AVCH264HighProfileLevel41",
    name: "MPEG-4 AVC/H.264 High Profile / Level 4.1",
};

/// MPEG-4 AVC/H.264 BD-compatible High Profile / Level 4.1 - Transfer Syntax, "1.2.840.10008.1.2.4.103"
pub const MPEG4AVCH264BDCompatibleHighProfileLevel41: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.103",
    ident: "MPEG4AVCH264BDCompatibleHighProfileLevel41",
    name: "MPEG-4 AVC/H.264 BD-compatible High Profile / Level 4.1",
};

/// RLE Lossless - Transfer Syntax, "1.2.840.10008.1.2.5"
pub const RLELossless: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.5",
    ident: "RLELossless",
    name: "RLE Lossless",
};

/// RFC 2557 MIME encapsulation - Transfer Syntax, "1.2.840.10008.1.2.6.1"
pub const RFC2557MIMEEncapsulation: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.6.1",
    ident: "RFC2557MIMEEncapsulation",
    name: "RFC 2557 MIME encapsulation",
};

/// XML Encoding - Transfer Syntax, "1.2.840.10008.1.2.6.2"
pub const XMLEncoding: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.6.2",
    ident: "XMLEncoding",
    name: "XML Encoding",
};

/// Media Storage Directory Storage - SOP Class, "1.2.840.10008.1.3.10"
pub const MediaStorageDirectoryStorage: &'static UID = &UID {
    uid: "1.2.840.10008.1.3.10",
    ident: "MediaStorageDirectoryStorage",
    name: "Media Storage Directory Storage",
};

/// Talairach Brain Atlas Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.1"
pub const TalairachBrainAtlasFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.1",
    ident: "TalairachBrainAtlasFrameOfReference",
    name: "Talairach Brain Atlas Frame of Reference",
};

/// SPM2 T1 Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.2"
pub const SPM2T1FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.2",
    ident: "SPM2T1FrameOfReference",
    name: "SPM2 T1 Frame of Reference",
};

/// SPM2 T2 Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.3"
pub const SPM2T2FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.3",
    ident: "SPM2T2FrameOfReference",
    name: "SPM2 T2 Frame of Reference",
};

/// SPM2 PD Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.4"
pub const SPM2PDFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.4",
    ident: "SPM2PDFrameOfReference",
    name: "SPM2 PD Frame of Reference",
};

/// SPM2 EPI Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.5"
pub const SPM2EPIFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.5",
    ident: "SPM2EPIFrameOfReference",
    name: "SPM2 EPI Frame of Reference",
};

/// SPM2 FIL T1 Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.6"
pub const SPM2FILT1FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.6",
    ident: "SPM2FILT1FrameOfReference",
    name: "SPM2 FIL T1 Frame of Reference",
};

/// SPM2 PET Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.7"
pub const SPM2PETFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.7",
    ident: "SPM2PETFrameOfReference",
    name: "SPM2 PET Frame of Reference",
};

/// SPM2 TRANSM Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.8"
pub const SPM2TRANSMFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.8",
    ident: "SPM2TRANSMFrameOfReference",
    name: "SPM2 TRANSM Frame of Reference",
};

/// SPM2 SPECT Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.9"
pub const SPM2SPECTFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.9",
    ident: "SPM2SPECTFrameOfReference",
    name: "SPM2 SPECT Frame of Reference",
};

/// SPM2 GRAY Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.10"
pub const SPM2GRAYFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.10",
    ident: "SPM2GRAYFrameOfReference",
    name: "SPM2 GRAY Frame of Reference",
};

/// SPM2 WHITE Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.11"
pub const SPM2WHITEFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.11",
    ident: "SPM2WHITEFrameOfReference",
    name: "SPM2 WHITE Frame of Reference",
};

/// SPM2 CSF Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.12"
pub const SPM2CSFFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.12",
    ident: "SPM2CSFFrameOfReference",
    name: "SPM2 CSF Frame of Reference",
};

/// SPM2 BRAINMASK Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.13"
pub const SPM2BRAINMASKFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.13",
    ident: "SPM2BRAINMASKFrameOfReference",
    name: "SPM2 BRAINMASK Frame of Reference",
};

/// SPM2 AVG305T1 Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.14"
pub const SPM2AVG305T1FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.14",
    ident: "SPM2AVG305T1FrameOfReference",
    name: "SPM2 AVG305T1 Frame of Reference",
};

/// SPM2 AVG152T1 Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.15"
pub const SPM2AVG152T1FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.15",
    ident: "SPM2AVG152T1FrameOfReference",
    name: "SPM2 AVG152T1 Frame of Reference",
};

/// SPM2 AVG152T2 Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.16"
pub const SPM2AVG152T2FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.16",
    ident: "SPM2AVG152T2FrameOfReference",
    name: "SPM2 AVG152T2 Frame of Reference",
};

/// SPM2 AVG152PD Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.17"
pub const SPM2AVG152PDFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.17",
    ident: "SPM2AVG152PDFrameOfReference",
    name: "SPM2 AVG152PD Frame of Reference",
};

/// SPM2 SINGLESUBJT1 Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.1.18"
pub const SPM2SINGLESUBJT1FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.18",
    ident: "SPM2SINGLESUBJT1FrameOfReference",
    name: "SPM2 SINGLESUBJT1 Frame of Reference",
};

/// ICBM 452 T1 Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.2.1"
pub const ICBM452T1FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.2.1",
    ident: "ICBM452T1FrameOfReference",
    name: "ICBM 452 T1 Frame of Reference",
};

/// ICBM Single Subject MRI Frame of Reference - Well-known frame of reference, "1.2.840.10008.1.4.2.2"
pub const ICBMSingleSubjectMRIFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.2.2",
    ident: "ICBMSingleSubjectMRIFrameOfReference",
    name: "ICBM Single Subject MRI Frame of Reference",
};

/// Hot Iron Color Palette SOP Instance - Well-known SOP Instance, "1.2.840.10008.1.5.1"
pub const HotIronColorPaletteSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.1.5.1",
    ident: "HotIronColorPaletteSOPInstance",
    name: "Hot Iron Color Palette SOP Instance",
};

/// PET Color Palette SOP Instance - Well-known SOP Instance, "1.2.840.10008.1.5.2"
pub const PETColorPaletteSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.1.5.2",
    ident: "PETColorPaletteSOPInstance",
    name: "PET Color Palette SOP Instance",
};

/// Hot Metal Blue Color Palette SOP Instance - Well-known SOP Instance, "1.2.840.10008.1.5.3"
pub const HotMetalBlueColorPaletteSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.1.5.3",
    ident: "HotMetalBlueColorPaletteSOPInstance",
    name: "Hot Metal Blue Color Palette SOP Instance",
};

/// PET 20 Step Color Palette SOP Instance - Well-known SOP Instance, "1.2.840.10008.1.5.4"
pub const PET20StepColorPaletteSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.1.5.4",
    ident: "PET20StepColorPaletteSOPInstance",
    name: "PET 20 Step Color Palette SOP Instance",
};

/// Basic Study Content Notification SOP Class (Retired) - SOP Class, "1.2.840.10008.1.9"
pub const BasicStudyContentNotificationSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.1.9",
    ident: "BasicStudyContentNotificationSOPClassRetired",
    name: "Basic Study Content Notification SOP Class (Retired)",
};

/// Storage Commitment Push Model SOP Class - SOP Class, "1.2.840.10008.1.20.1"
pub const StorageCommitmentPushModelSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.1.20.1",
    ident: "StorageCommitmentPushModelSOPClass",
    name: "Storage Commitment Push Model SOP Class",
};

/// Storage Commitment Push Model SOP Instance - Well-known SOP Instance, "1.2.840.10008.1.20.1.1"
pub const StorageCommitmentPushModelSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.1.20.1.1",
    ident: "StorageCommitmentPushModelSOPInstance",
    name: "Storage Commitment Push Model SOP Instance",
};

/// Storage Commitment Pull Model SOP Class (Retired) - SOP Class, "1.2.840.10008.1.20.2"
pub const StorageCommitmentPullModelSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.1.20.2",
    ident: "StorageCommitmentPullModelSOPClassRetired",
    name: "Storage Commitment Pull Model SOP Class (Retired)",
};

/// Storage Commitment Pull Model SOP Instance (Retired) - Well-known SOP Instance, "1.2.840.10008.1.20.2.1"
pub const StorageCommitmentPullModelSOPInstanceRetired: &'static UID = &UID {
    uid: "1.2.840.10008.1.20.2.1",
    ident: "StorageCommitmentPullModelSOPInstanceRetired",
    name: "Storage Commitment Pull Model SOP Instance (Retired)",
};

/// Procedural Event Logging SOP Class - SOP Class, "1.2.840.10008.1.40"
pub const ProceduralEventLoggingSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.1.40",
    ident: "ProceduralEventLoggingSOPClass",
    name: "Procedural Event Logging SOP Class",
};

/// Procedural Event Logging SOP Instance - Well-known SOP Instance, "1.2.840.10008.1.40.1"
pub const ProceduralEventLoggingSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.1.40.1",
    ident: "ProceduralEventLoggingSOPInstance",
    name: "Procedural Event Logging SOP Instance",
};

/// Substance Administration Logging SOP Class - SOP Class, "1.2.840.10008.1.42"
pub const SubstanceAdministrationLoggingSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.1.42",
    ident: "SubstanceAdministrationLoggingSOPClass",
    name: "Substance Administration Logging SOP Class",
};

/// Substance Administration Logging SOP Instance - Well-known SOP Instance, "1.2.840.10008.1.42.1"
pub const SubstanceAdministrationLoggingSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.1.42.1",
    ident: "SubstanceAdministrationLoggingSOPInstance",
    name: "Substance Administration Logging SOP Instance",
};

/// DICOM UID Registry - DICOM UIDs as a Coding Scheme, "1.2.840.10008.2.6.1"
pub const DICOMUIDRegistry: &'static UID = &UID {
    uid: "1.2.840.10008.2.6.1",
    ident: "DICOMUIDRegistry",
    name: "DICOM UID Registry",
};

/// DICOM Controlled Terminology - Coding Scheme, "1.2.840.10008.2.16.4"
pub const DICOMControlledTerminology: &'static UID = &UID {
    uid: "1.2.840.10008.2.16.4",
    ident: "DICOMControlledTerminology",
    name: "DICOM Controlled Terminology",
};

/// DICOM Application Context Name - Application Context Name, "1.2.840.10008.3.1.1.1"
pub const DICOMApplicationContextName: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.1.1",
    ident: "DICOMApplicationContextName",
    name: "DICOM Application Context Name",
};

/// Detached Patient Management SOP Class (Retired) - SOP Class, "1.2.840.10008.3.1.2.1.1"
pub const DetachedPatientManagementSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.1.1",
    ident: "DetachedPatientManagementSOPClassRetired",
    name: "Detached Patient Management SOP Class (Retired)",
};

/// Detached Patient Management Meta SOP Class (Retired) - Meta SOP Class, "1.2.840.10008.3.1.2.1.4"
pub const DetachedPatientManagementMetaSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.1.4",
    ident: "DetachedPatientManagementMetaSOPClassRetired",
    name: "Detached Patient Management Meta SOP Class (Retired)",
};

/// Detached Visit Management SOP Class (Retired) - SOP Class, "1.2.840.10008.3.1.2.2.1"
pub const DetachedVisitManagementSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.2.1",
    ident: "DetachedVisitManagementSOPClassRetired",
    name: "Detached Visit Management SOP Class (Retired)",
};

/// Detached Study Management SOP Class (Retired) - SOP Class, "1.2.840.10008.3.1.2.3.1"
pub const DetachedStudyManagementSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.3.1",
    ident: "DetachedStudyManagementSOPClassRetired",
    name: "Detached Study Management SOP Class (Retired)",
};

/// Study Component Management SOP Class (Retired) - SOP Class, "1.2.840.10008.3.1.2.3.2"
pub const StudyComponentManagementSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.3.2",
    ident: "StudyComponentManagementSOPClassRetired",
    name: "Study Component Management SOP Class (Retired)",
};

/// Modality Performed Procedure Step SOP Class - SOP Class, "1.2.840.10008.3.1.2.3.3"
pub const ModalityPerformedProcedureStepSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.3.3",
    ident: "ModalityPerformedProcedureStepSOPClass",
    name: "Modality Performed Procedure Step SOP Class",
};

/// Modality Performed Procedure Step Retrieve SOP Class - SOP Class, "1.2.840.10008.3.1.2.3.4"
pub const ModalityPerformedProcedureStepRetrieveSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.3.4",
    ident: "ModalityPerformedProcedureStepRetrieveSOPClass",
    name: "Modality Performed Procedure Step Retrieve SOP Class",
};

/// Modality Performed Procedure Step Notification SOP Class - SOP Class, "1.2.840.10008.3.1.2.3.5"
pub const ModalityPerformedProcedureStepNotificationSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.3.5",
    ident: "ModalityPerformedProcedureStepNotificationSOPClass",
    name: "Modality Performed Procedure Step Notification SOP Class",
};

/// Detached Results Management SOP Class (Retired) - SOP Class, "1.2.840.10008.3.1.2.5.1"
pub const DetachedResultsManagementSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.5.1",
    ident: "DetachedResultsManagementSOPClassRetired",
    name: "Detached Results Management SOP Class (Retired)",
};

/// Detached Results Management Meta SOP Class (Retired) - Meta SOP Class, "1.2.840.10008.3.1.2.5.4"
pub const DetachedResultsManagementMetaSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.5.4",
    ident: "DetachedResultsManagementMetaSOPClassRetired",
    name: "Detached Results Management Meta SOP Class (Retired)",
};

/// Detached Study Management Meta SOP Class (Retired) - Meta SOP Class, "1.2.840.10008.3.1.2.5.5"
pub const DetachedStudyManagementMetaSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.5.5",
    ident: "DetachedStudyManagementMetaSOPClassRetired",
    name: "Detached Study Management Meta SOP Class (Retired)",
};

/// Detached Interpretation Management SOP Class (Retired) - SOP Class, "1.2.840.10008.3.1.2.6.1"
pub const DetachedInterpretationManagementSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.6.1",
    ident: "DetachedInterpretationManagementSOPClassRetired",
    name: "Detached Interpretation Management SOP Class (Retired)",
};

/// Storage Service Class - Service Class, "1.2.840.10008.4.2"
pub const StorageServiceClass: &'static UID = &UID {
    uid: "1.2.840.10008.4.2",
    ident: "StorageServiceClass",
    name: "Storage Service Class",
};

/// Basic Film Session SOP Class - SOP Class, "1.2.840.10008.5.1.1.1"
pub const BasicFilmSessionSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.1",
    ident: "BasicFilmSessionSOPClass",
    name: "Basic Film Session SOP Class",
};

/// Basic Film Box SOP Class - SOP Class, "1.2.840.10008.5.1.1.2"
pub const BasicFilmBoxSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.2",
    ident: "BasicFilmBoxSOPClass",
    name: "Basic Film Box SOP Class",
};

/// Basic Grayscale Image Box SOP Class - SOP Class, "1.2.840.10008.5.1.1.4"
pub const BasicGrayscaleImageBoxSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.4",
    ident: "BasicGrayscaleImageBoxSOPClass",
    name: "Basic Grayscale Image Box SOP Class",
};

/// Basic Color Image Box SOP Class - SOP Class, "1.2.840.10008.5.1.1.4.1"
pub const BasicColorImageBoxSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.4.1",
    ident: "BasicColorImageBoxSOPClass",
    name: "Basic Color Image Box SOP Class",
};

/// Referenced Image Box SOP Class (Retired) - SOP Class, "1.2.840.10008.5.1.1.4.2"
pub const ReferencedImageBoxSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.4.2",
    ident: "ReferencedImageBoxSOPClassRetired",
    name: "Referenced Image Box SOP Class (Retired)",
};

/// Basic Grayscale Print Management Meta SOP Class - Meta SOP Class, "1.2.840.10008.5.1.1.9"
pub const BasicGrayscalePrintManagementMetaSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.9",
    ident: "BasicGrayscalePrintManagementMetaSOPClass",
    name: "Basic Grayscale Print Management Meta SOP Class",
};

/// Referenced Grayscale Print Management Meta SOP Class (Retired) - Meta SOP Class, "1.2.840.10008.5.1.1.9.1"
pub const ReferencedGrayscalePrintManagementMetaSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.9.1",
    ident: "ReferencedGrayscalePrintManagementMetaSOPClassRetired",
    name: "Referenced Grayscale Print Management Meta SOP Class (Retired)",
};

/// Print Job SOP Class - SOP Class, "1.2.840.10008.5.1.1.14"
pub const PrintJobSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.14",
    ident: "PrintJobSOPClass",
    name: "Print Job SOP Class",
};

/// Basic Annotation Box SOP Class - SOP Class, "1.2.840.10008.5.1.1.15"
pub const BasicAnnotationBoxSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.15",
    ident: "BasicAnnotationBoxSOPClass",
    name: "Basic Annotation Box SOP Class",
};

/// Printer SOP Class - SOP Class, "1.2.840.10008.5.1.1.16"
pub const PrinterSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.16",
    ident: "PrinterSOPClass",
    name: "Printer SOP Class",
};

/// Printer Configuration Retrieval SOP Class - SOP Class, "1.2.840.10008.5.1.1.16.376"
pub const PrinterConfigurationRetrievalSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.16.376",
    ident: "PrinterConfigurationRetrievalSOPClass",
    name: "Printer Configuration Retrieval SOP Class",
};

/// Printer SOP Instance - Well-known Printer SOP Instance, "1.2.840.10008.5.1.1.17"
pub const PrinterSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.17",
    ident: "PrinterSOPInstance",
    name: "Printer SOP Instance",
};

/// Printer Configuration Retrieval SOP Instance - Well-known Printer SOP Instance, "1.2.840.10008.5.1.1.17.376"
pub const PrinterConfigurationRetrievalSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.17.376",
    ident: "PrinterConfigurationRetrievalSOPInstance",
    name: "Printer Configuration Retrieval SOP Instance",
};

/// Basic Color Print Management Meta SOP Class - Meta SOP Class, "1.2.840.10008.5.1.1.18"
pub const BasicColorPrintManagementMetaSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.18",
    ident: "BasicColorPrintManagementMetaSOPClass",
    name: "Basic Color Print Management Meta SOP Class",
};

/// Referenced Color Print Management Meta SOP Class (Retired) - Meta SOP Class, "1.2.840.10008.5.1.1.18.1"
pub const ReferencedColorPrintManagementMetaSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.18.1",
    ident: "ReferencedColorPrintManagementMetaSOPClassRetired",
    name: "Referenced Color Print Management Meta SOP Class (Retired)",
};

/// VOI LUT Box SOP Class - SOP Class, "1.2.840.10008.5.1.1.22"
pub const VOILUTBoxSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.22",
    ident: "VOILUTBoxSOPClass",
    name: "VOI LUT Box SOP Class",
};

/// Presentation LUT SOP Class - SOP Class, "1.2.840.10008.5.1.1.23"
pub const PresentationLUTSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.23",
    ident: "PresentationLUTSOPClass",
    name: "Presentation LUT SOP Class",
};

/// Image Overlay Box SOP Class (Retired) - SOP Class, "1.2.840.10008.5.1.1.24"
pub const ImageOverlayBoxSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.24",
    ident: "ImageOverlayBoxSOPClassRetired",
    name: "Image Overlay Box SOP Class (Retired)",
};

/// Basic Print Image Overlay Box SOP Class (Retired) - SOP Class, "1.2.840.10008.5.1.1.24.1"
pub const BasicPrintImageOverlayBoxSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.24.1",
    ident: "BasicPrintImageOverlayBoxSOPClassRetired",
    name: "Basic Print Image Overlay Box SOP Class (Retired)",
};

/// Print Queue SOP Instance (Retired) - Well-known Print Queue SOP Instance, "1.2.840.10008.5.1.1.25"
pub const PrintQueueSOPInstanceRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.25",
    ident: "PrintQueueSOPInstanceRetired",
    name: "Print Queue SOP Instance (Retired)",
};

/// Print Queue Management SOP Class (Retired) - SOP Class, "1.2.840.10008.5.1.1.26"
pub const PrintQueueManagementSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.26",
    ident: "PrintQueueManagementSOPClassRetired",
    name: "Print Queue Management SOP Class (Retired)",
};

/// Stored Print Storage SOP Class (Retired) - SOP Class, "1.2.840.10008.5.1.1.27"
pub const StoredPrintStorageSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.27",
    ident: "StoredPrintStorageSOPClassRetired",
    name: "Stored Print Storage SOP Class (Retired)",
};

/// Hardcopy Grayscale Image Storage SOP Class (Retired) - SOP Class, "1.2.840.10008.5.1.1.29"
pub const HardcopyGrayscaleImageStorageSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.29",
    ident: "HardcopyGrayscaleImageStorageSOPClassRetired",
    name: "Hardcopy Grayscale Image Storage SOP Class (Retired)",
};

/// Hardcopy Color Image Storage SOP Class (Retired) - SOP Class, "1.2.840.10008.5.1.1.30"
pub const HardcopyColorImageStorageSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.30",
    ident: "HardcopyColorImageStorageSOPClassRetired",
    name: "Hardcopy Color Image Storage SOP Class (Retired)",
};

/// Pull Print Request SOP Class (Retired) - SOP Class, "1.2.840.10008.5.1.1.31"
pub const PullPrintRequestSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.31",
    ident: "PullPrintRequestSOPClassRetired",
    name: "Pull Print Request SOP Class (Retired)",
};

/// Pull Stored Print Management Meta SOP Class (Retired) - Meta SOP Class, "1.2.840.10008.5.1.1.32"
pub const PullStoredPrintManagementMetaSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.32",
    ident: "PullStoredPrintManagementMetaSOPClassRetired",
    name: "Pull Stored Print Management Meta SOP Class (Retired)",
};

/// Media Creation Management SOP Class UID - SOP Class, "1.2.840.10008.5.1.1.33"
pub const MediaCreationManagementSOPClassUID: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.33",
    ident: "MediaCreationManagementSOPClassUID",
    name: "Media Creation Management SOP Class UID",
};

/// Computed Radiography Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.1"
pub const ComputedRadiographyImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.1",
    ident: "ComputedRadiographyImageStorage",
    name: "Computed Radiography Image Storage",
};

/// Digital X-Ray Image Storage : For Presentation - SOP Class, "1.2.840.10008.5.1.4.1.1.1.1"
pub const DigitalXRayImageStorageForPresentation: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.1.1",
    ident: "DigitalXRayImageStorageForPresentation",
    name: "Digital X-Ray Image Storage - For Presentation",
};

/// Digital X-Ray Image Storage : For Processing - SOP Class, "1.2.840.10008.5.1.4.1.1.1.1.1"
pub const DigitalXRayImageStorageForProcessing: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.1.1.1",
    ident: "DigitalXRayImageStorageForProcessing",
    name: "Digital X-Ray Image Storage - For Processing",
};

/// Digital Mammography X-Ray Image Storage : For Presentation - SOP Class, "1.2.840.10008.5.1.4.1.1.1.2"
pub const DigitalMammographyXRayImageStorageForPresentation: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.1.2",
    ident: "DigitalMammographyXRayImageStorageForPresentation",
    name: "Digital Mammography X-Ray Image Storage - For Presentation",
};

/// Digital Mammography X-Ray Image Storage : For Processing - SOP Class, "1.2.840.10008.5.1.4.1.1.1.2.1"
pub const DigitalMammographyXRayImageStorageForProcessing: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.1.2.1",
    ident: "DigitalMammographyXRayImageStorageForProcessing",
    name: "Digital Mammography X-Ray Image Storage - For Processing",
};

/// Digital Intra-oral X-Ray Image Storage : For Presentation - SOP Class, "1.2.840.10008.5.1.4.1.1.1.3"
pub const DigitalIntraOralXRayImageStorageForPresentation: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.1.3",
    ident: "DigitalIntraOralXRayImageStorageForPresentation",
    name: "Digital Intra-oral X-Ray Image Storage - For Presentation",
};

/// Digital Intra-oral X-Ray Image Storage : For Processing - SOP Class, "1.2.840.10008.5.1.4.1.1.1.3.1"
pub const DigitalIntraOralXRayImageStorageForProcessing: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.1.3.1",
    ident: "DigitalIntraOralXRayImageStorageForProcessing",
    name: "Digital Intra-oral X-Ray Image Storage - For Processing",
};

/// CT Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.2"
pub const CTImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.2",
    ident: "CTImageStorage",
    name: "CT Image Storage",
};

/// Enhanced CT Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.2.1"
pub const EnhancedCTImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.2.1",
    ident: "EnhancedCTImageStorage",
    name: "Enhanced CT Image Storage",
};

/// Ultrasound Multi-frame Image Storage (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.1.3"
pub const UltrasoundMultiFrameImageStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.3",
    ident: "UltrasoundMultiFrameImageStorageRetired",
    name: "Ultrasound Multi-frame Image Storage (Retired)",
};

/// Ultrasound Multi-frame Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.3.1"
pub const UltrasoundMultiFrameImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.3.1",
    ident: "UltrasoundMultiFrameImageStorage",
    name: "Ultrasound Multi-frame Image Storage",
};

/// MR Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.4"
pub const MRImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.4",
    ident: "MRImageStorage",
    name: "MR Image Storage",
};

/// Enhanced MR Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.4.1"
pub const EnhancedMRImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.4.1",
    ident: "EnhancedMRImageStorage",
    name: "Enhanced MR Image Storage",
};

/// MR Spectroscopy Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.4.2"
pub const MRSpectroscopyStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.4.2",
    ident: "MRSpectroscopyStorage",
    name: "MR Spectroscopy Storage",
};

/// Enhanced MR Color Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.4.3"
pub const EnhancedMRColorImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.4.3",
    ident: "EnhancedMRColorImageStorage",
    name: "Enhanced MR Color Image Storage",
};

/// Nuclear Medicine Image Storage (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.1.5"
pub const NuclearMedicineImageStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.5",
    ident: "NuclearMedicineImageStorageRetired",
    name: "Nuclear Medicine Image Storage (Retired)",
};

/// Ultrasound Image Storage (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.1.6"
pub const UltrasoundImageStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.6",
    ident: "UltrasoundImageStorageRetired",
    name: "Ultrasound Image Storage (Retired)",
};

/// Ultrasound Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.6.1"
pub const UltrasoundImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.6.1",
    ident: "UltrasoundImageStorage",
    name: "Ultrasound Image Storage",
};

/// Enhanced US Volume Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.6.2"
pub const EnhancedUSVolumeStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.6.2",
    ident: "EnhancedUSVolumeStorage",
    name: "Enhanced US Volume Storage",
};

/// Secondary Capture Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.7"
pub const SecondaryCaptureImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.7",
    ident: "SecondaryCaptureImageStorage",
    name: "Secondary Capture Image Storage",
};

/// Multi-frame Single Bit Secondary Capture Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.7.1"
pub const MultiFrameSingleBitSecondaryCaptureImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.7.1",
    ident: "MultiFrameSingleBitSecondaryCaptureImageStorage",
    name: "Multi-frame Single Bit Secondary Capture Image Storage",
};

/// Multi-frame Grayscale Byte Secondary Capture Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.7.2"
pub const MultiFrameGrayscaleByteSecondaryCaptureImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.7.2",
    ident: "MultiFrameGrayscaleByteSecondaryCaptureImageStorage",
    name: "Multi-frame Grayscale Byte Secondary Capture Image Storage",
};

/// Multi-frame Grayscale Word Secondary Capture Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.7.3"
pub const MultiFrameGrayscaleWordSecondaryCaptureImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.7.3",
    ident: "MultiFrameGrayscaleWordSecondaryCaptureImageStorage",
    name: "Multi-frame Grayscale Word Secondary Capture Image Storage",
};

/// Multi-frame True Color Secondary Capture Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.7.4"
pub const MultiFrameTrueColorSecondaryCaptureImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.7.4",
    ident: "MultiFrameTrueColorSecondaryCaptureImageStorage",
    name: "Multi-frame True Color Secondary Capture Image Storage",
};

/// Standalone Overlay Storage (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.1.8"
pub const StandaloneOverlayStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.8",
    ident: "StandaloneOverlayStorageRetired",
    name: "Standalone Overlay Storage (Retired)",
};

/// Standalone Curve Storage (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.1.9"
pub const StandaloneCurveStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9",
    ident: "StandaloneCurveStorageRetired",
    name: "Standalone Curve Storage (Retired)",
};

/// Waveform Storage : Trial (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.1.9.1"
pub const WaveformStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.1",
    ident: "WaveformStorageTrialRetired",
    name: "Waveform Storage - Trial (Retired)",
};

/// 12-lead ECG Waveform Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.9.1.1"
pub const TwelveLeadECGWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.1.1",
    ident: "TwelveLeadECGWaveformStorage",
    name: "12-lead ECG Waveform Storage",
};

/// General ECG Waveform Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.9.1.2"
pub const GeneralECGWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.1.2",
    ident: "GeneralECGWaveformStorage",
    name: "General ECG Waveform Storage",
};

/// Ambulatory ECG Waveform Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.9.1.3"
pub const AmbulatoryECGWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.1.3",
    ident: "AmbulatoryECGWaveformStorage",
    name: "Ambulatory ECG Waveform Storage",
};

/// Hemodynamic Waveform Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.9.2.1"
pub const HemodynamicWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.2.1",
    ident: "HemodynamicWaveformStorage",
    name: "Hemodynamic Waveform Storage",
};

/// Cardiac Electrophysiology Waveform Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.9.3.1"
pub const CardiacElectrophysiologyWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.3.1",
    ident: "CardiacElectrophysiologyWaveformStorage",
    name: "Cardiac Electrophysiology Waveform Storage",
};

/// Basic Voice Audio Waveform Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.9.4.1"
pub const BasicVoiceAudioWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.4.1",
    ident: "BasicVoiceAudioWaveformStorage",
    name: "Basic Voice Audio Waveform Storage",
};

/// General Audio Waveform Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.9.4.2"
pub const GeneralAudioWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.4.2",
    ident: "GeneralAudioWaveformStorage",
    name: "General Audio Waveform Storage",
};

/// Arterial Pulse Waveform Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.9.5.1"
pub const ArterialPulseWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.5.1",
    ident: "ArterialPulseWaveformStorage",
    name: "Arterial Pulse Waveform Storage",
};

/// Respiratory Waveform Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.9.6.1"
pub const RespiratoryWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.6.1",
    ident: "RespiratoryWaveformStorage",
    name: "Respiratory Waveform Storage",
};

/// Standalone Modality LUT Storage (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.1.10"
pub const StandaloneModalityLUTStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.10",
    ident: "StandaloneModalityLUTStorageRetired",
    name: "Standalone Modality LUT Storage (Retired)",
};

/// Standalone VOI LUT Storage (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.1.11"
pub const StandaloneVOILUTStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.11",
    ident: "StandaloneVOILUTStorageRetired",
    name: "Standalone VOI LUT Storage (Retired)",
};

/// Grayscale Softcopy Presentation State Storage SOP Class - SOP Class, "1.2.840.10008.5.1.4.1.1.11.1"
pub const GrayscaleSoftcopyPresentationStateStorageSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.11.1",
    ident: "GrayscaleSoftcopyPresentationStateStorageSOPClass",
    name: "Grayscale Softcopy Presentation State Storage SOP Class",
};

/// Color Softcopy Presentation State Storage SOP Class - SOP Class, "1.2.840.10008.5.1.4.1.1.11.2"
pub const ColorSoftcopyPresentationStateStorageSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.11.2",
    ident: "ColorSoftcopyPresentationStateStorageSOPClass",
    name: "Color Softcopy Presentation State Storage SOP Class",
};

/// Pseudo-Color Softcopy Presentation State Storage SOP Class - SOP Class, "1.2.840.10008.5.1.4.1.1.11.3"
pub const PseudoColorSoftcopyPresentationStateStorageSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.11.3",
    ident: "PseudoColorSoftcopyPresentationStateStorageSOPClass",
    name: "Pseudo-Color Softcopy Presentation State Storage SOP Class",
};

/// Blending Softcopy Presentation State Storage SOP Class - SOP Class, "1.2.840.10008.5.1.4.1.1.11.4"
pub const BlendingSoftcopyPresentationStateStorageSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.11.4",
    ident: "BlendingSoftcopyPresentationStateStorageSOPClass",
    name: "Blending Softcopy Presentation State Storage SOP Class",
};

/// XA/XRF Grayscale Softcopy Presentation State Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.11.5"
pub const XAXRFGrayscaleSoftcopyPresentationStateStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.11.5",
    ident: "XAXRFGrayscaleSoftcopyPresentationStateStorage",
    name: "XA/XRF Grayscale Softcopy Presentation State Storage",
};

/// X-Ray Angiographic Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.12.1"
pub const XRayAngiographicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.12.1",
    ident: "XRayAngiographicImageStorage",
    name: "X-Ray Angiographic Image Storage",
};

/// Enhanced XA Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.12.1.1"
pub const EnhancedXAImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.12.1.1",
    ident: "EnhancedXAImageStorage",
    name: "Enhanced XA Image Storage",
};

/// X-Ray Radiofluoroscopic Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.12.2"
pub const XRayRadiofluoroscopicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.12.2",
    ident: "XRayRadiofluoroscopicImageStorage",
    name: "X-Ray Radiofluoroscopic Image Storage",
};

/// Enhanced XRF Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.12.2.1"
pub const EnhancedXRFImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.12.2.1",
    ident: "EnhancedXRFImageStorage",
    name: "Enhanced XRF Image Storage",
};

/// X-Ray Angiographic Bi-Plane Image Storage (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.1.12.3"
pub const XRayAngiographicBiPlaneImageStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.12.3",
    ident: "XRayAngiographicBiPlaneImageStorageRetired",
    name: "X-Ray Angiographic Bi-Plane Image Storage (Retired)",
};

/// X-Ray 3D Angiographic Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.13.1.1"
pub const XRay3DAngiographicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.13.1.1",
    ident: "XRay3DAngiographicImageStorage",
    name: "X-Ray 3D Angiographic Image Storage",
};

/// X-Ray 3D Craniofacial Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.13.1.2"
pub const XRay3DCraniofacialImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.13.1.2",
    ident: "XRay3DCraniofacialImageStorage",
    name: "X-Ray 3D Craniofacial Image Storage",
};

/// Breast Tomosynthesis Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.13.1.3"
pub const BreastTomosynthesisImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.13.1.3",
    ident: "BreastTomosynthesisImageStorage",
    name: "Breast Tomosynthesis Image Storage",
};

/// Intravascular Optical Coherence Tomography Image Storage : For Presentation - SOP Class, {
pub const IntravascularOpticalCoherenceTomographyImageStorageForPresentation: &'static UID =
    &UID {
        uid: "1.2.840.10008.5.1.4.1.1.14.1",
        name: "Intravascular Optical Coherence Tomography Image Storage - For Presentation",
        ident: "IntravascularOpticalCoherenceTomographyImageStorageForPresentation",
    };

/// Intravascular Optical Coherence Tomography Image Storage : For Processing - SOP Class, "1.2.840.10008.5.1.4.1.1.14.2"
pub const IntravascularOpticalCoherenceTomographyImageStorageForProcessing: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.14.2",
    ident: "IntravascularOpticalCoherenceTomographyImageStorageForProcessing",
    name: "Intravascular Optical Coherence Tomography Image Storage - For Processing",
};

/// Nuclear Medicine Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.20"
pub const NuclearMedicineImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.20",
    ident: "NuclearMedicineImageStorage",
    name: "Nuclear Medicine Image Storage",
};

/// Raw Data Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.66"
pub const RawDataStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.66",
    ident: "RawDataStorage",
    name: "Raw Data Storage",
};

/// Spatial Registration Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.66.1"
pub const SpatialRegistrationStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.66.1",
    ident: "SpatialRegistrationStorage",
    name: "Spatial Registration Storage",
};

/// Spatial Fiducials Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.66.2"
pub const SpatialFiducialsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.66.2",
    ident: "SpatialFiducialsStorage",
    name: "Spatial Fiducials Storage",
};

/// Deformable Spatial Registration Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.66.3"
pub const DeformableSpatialRegistrationStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.66.3",
    ident: "DeformableSpatialRegistrationStorage",
    name: "Deformable Spatial Registration Storage",
};

/// Segmentation Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.66.4"
pub const SegmentationStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.66.4",
    ident: "SegmentationStorage",
    name: "Segmentation Storage",
};

/// Surface Segmentation Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.66.5"
pub const SurfaceSegmentationStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.66.5",
    ident: "SurfaceSegmentationStorage",
    name: "Surface Segmentation Storage",
};

/// Real World Value Mapping Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.67"
pub const RealWorldValueMappingStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.67",
    ident: "RealWorldValueMappingStorage",
    name: "Real World Value Mapping Storage",
};

/// VL Image Storage : Trial (Retired) -, "1.2.840.10008.5.1.4.1.1.77.1"
pub const VLImageStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1",
    ident: "VLImageStorageTrialRetired",
    name: "VL Multi-frame Image Storage - Trial (Retired)",
};

/// VL Multi-frame Image Storage : Trial (Retired) -, "1.2.840.10008.5.1.4.1.1.77.2"
pub const VLMultiFrameImageStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.2",
    ident: "VLMultiFrameImageStorageTrialRetired",
    name: "VL Multi-frame Image Storage - Trial (Retired)",
};

/// VL Endoscopic Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.77.1.1"
pub const VLEndoscopicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.1",
    ident: "VLEndoscopicImageStorage",
    name: "VL Endoscopic Image Storage",
};

/// Video Endoscopic Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.77.1.1.1"
pub const VideoEndoscopicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.1.1",
    ident: "VideoEndoscopicImageStorage",
    name: "Video Endoscopic Image Storage",
};

/// VL Microscopic Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.77.1.2"
pub const VLMicroscopicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.2",
    ident: "VLMicroscopicImageStorage",
    name: "VL Microscopic Image Storage",
};

/// Video Microscopic Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.77.1.2.1"
pub const VideoMicroscopicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.2.1",
    ident: "VideoMicroscopicImageStorage",
    name: "Video Microscopic Image Storage",
};

/// VL Slide-Coordinates Microscopic Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.77.1.3"
pub const VLSlideCoordinatesMicroscopicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.3",
    ident: "VLSlideCoordinatesMicroscopicImageStorage",
    name: "VL Slide-Coordinates Microscopic Image Storage",
};

/// VL Photographic Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.77.1.4"
pub const VLPhotographicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.4",
    ident: "VLPhotographicImageStorage",
    name: "VL Photographic Image Storage",
};

/// Video Photographic Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.77.1.4.1"
pub const VideoPhotographicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.4.1",
    ident: "VideoPhotographicImageStorage",
    name: "Video Photographic Image Storage",
};

/// Ophthalmic Photography 8 Bit Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.77.1.5.1"
pub const OphthalmicPhotography8BitImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.1",
    ident: "OphthalmicPhotography8BitImageStorage",
    name: "Ophthalmic Photography 8 Bit Image Storage",
};

/// Ophthalmic Photography 16 Bit Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.77.1.5.2"
pub const OphthalmicPhotography16BitImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.2",
    ident: "OphthalmicPhotography16BitImageStorage",
    name: "Ophthalmic Photography 16 Bit Image Storage",
};

/// Stereometric Relationship Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.77.1.5.3"
pub const StereometricRelationshipStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.3",
    ident: "StereometricRelationshipStorage",
    name: "Stereometric Relationship Storage",
};

/// Ophthalmic Tomography Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.77.1.5.4"
pub const OphthalmicTomographyImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.4",
    ident: "OphthalmicTomographyImageStorage",
    name: "Ophthalmic Tomography Image Storage",
};

/// VL Whole Slide Microscopy Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.77.1.6"
pub const VLWholeSlideMicroscopyImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.6",
    ident: "VLWholeSlideMicroscopyImageStorage",
    name: "VL Whole Slide Microscopy Image Storage",
};

/// Lensometry Measurements Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.78.1"
pub const LensometryMeasurementsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.1",
    ident: "LensometryMeasurementsStorage",
    name: "Lensometry Measurements Storage",
};

/// Autorefraction Measurements Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.78.2"
pub const AutorefractionMeasurementsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.2",
    ident: "AutorefractionMeasurementsStorage",
    name: "Autorefraction Measurements Storage",
};

/// Keratometry Measurements Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.78.3"
pub const KeratometryMeasurementsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.3",
    ident: "KeratometryMeasurementsStorage",
    name: "Keratometry Measurements Storage",
};

/// Subjective Refraction Measurements Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.78.4"
pub const SubjectiveRefractionMeasurementsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.4",
    ident: "SubjectiveRefractionMeasurementsStorage",
    name: "Subjective Refraction Measurements Storage",
};

/// Visual Acuity Measurements Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.78.5"
pub const VisualAcuityMeasurementsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.5",
    ident: "VisualAcuityMeasurementsStorage",
    name: "Visual Acuity Measurements Storage",
};

/// Spectacle Prescription Report Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.78.6"
pub const SpectaclePrescriptionReportStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.6",
    ident: "SpectaclePrescriptionReportStorage",
    name: "Spectacle Prescription Report Storage",
};

/// Ophthalmic Axial Measurements Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.78.7"
pub const OphthalmicAxialMeasurementsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.7",
    ident: "OphthalmicAxialMeasurementsStorage",
    name: "Ophthalmic Axial Measurements Storage",
};

/// Intraocular Lens Calculations Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.78.8"
pub const IntraocularLensCalculationsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.8",
    ident: "IntraocularLensCalculationsStorage",
    name: "Intraocular Lens Calculations Storage",
};

/// Macular Grid Thickness and Volume Report Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.79.1"
pub const MacularGridThicknessAndVolumeReportStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.79.1",
    ident: "MacularGridThicknessAndVolumeReportStorage",
    name: "Macular Grid Thickness and Volume Report Storage",
};

/// Ophthalmic Visual Field Static Perimetry Measurements Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.80.1"
pub const OphthalmicVisualFieldStaticPerimetryMeasurementsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.80.1",
    ident: "OphthalmicVisualFieldStaticPerimetryMeasurementsStorage",
    name: "Ophthalmic Visual Field Static Perimetry Measurements Storage",
};

/// Text SR Storage : Trial (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.1.88.1"
pub const TextSRStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.1",
    ident: "TextSRStorageTrialRetired",
    name: "Text SR Storage - Trial (Retired)",
};

/// Audio SR Storage : Trial (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.1.88.2"
pub const AudioSRStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.2",
    ident: "AudioSRStorageTrialRetired",
    name: "Audio SR Storage - Trial (Retired)",
};

/// Detail SR Storage : Trial (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.1.88.3"
pub const DetailSRStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.3",
    ident: "DetailSRStorageTrialRetired",
    name: "Detail SR Storage - Trial (Retired)",
};

/// Comprehensive SR Storage : Trial (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.1.88.4"
pub const ComprehensiveSRStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.4",
    ident: "ComprehensiveSRStorageTrialRetired",
    name: "Comprehensive SR Storage - Trial (Retired)",
};

/// Basic Text SR Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.88.11"
pub const BasicTextSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.11",
    ident: "BasicTextSRStorage",
    name: "Basic Text SR Storage",
};

/// Enhanced SR Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.88.22"
pub const EnhancedSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.22",
    ident: "EnhancedSRStorage",
    name: "Enhanced SR Storage",
};

/// Comprehensive SR Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.88.33"
pub const ComprehensiveSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.33",
    ident: "ComprehensiveSRStorage",
    name: "Comprehensive SR Storage",
};

/// Procedure Log Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.88.40"
pub const ProcedureLogStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.40",
    ident: "ProcedureLogStorage",
    name: "Procedure Log Storage",
};

/// Mammography CAD SR Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.88.50"
pub const MammographyCADSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.50",
    ident: "MammographyCADSRStorage",
    name: "Mammography CAD SR Storage",
};

/// Key Object Selection Document Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.88.59"
pub const KeyObjectSelectionDocumentStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.59",
    ident: "KeyObjectSelectionDocumentStorage",
    name: "Key Object Selection Document Storage",
};

/// Chest CAD SR Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.88.65"
pub const ChestCADSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.65",
    ident: "ChestCADSRStorage",
    name: "Chest CAD SR Storage",
};

/// X-Ray Radiation Dose SR Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.88.67"
pub const XRayRadiationDoseSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.67",
    ident: "XRayRadiationDoseSRStorage",
    name: "X-Ray Radiation Dose SR Storage",
};

/// Colon CAD SR Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.88.69"
pub const ColonCADSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.69",
    ident: "ColonCADSRStorage",
    name: "Colon CAD SR Storage",
};

/// Implantation Plan SR Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.88.70"
pub const ImplantationPlanSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.70",
    ident: "ImplantationPlanSRStorage",
    name: "Implantation Plan SR Storage",
};

/// Encapsulated PDF Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.104.1"
pub const EncapsulatedPDFStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.104.1",
    ident: "EncapsulatedPDFStorage",
    name: "Encapsulated PDF Storage",
};

/// Encapsulated CDA Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.104.2"
pub const EncapsulatedCDAStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.104.2",
    ident: "EncapsulatedCDAStorage",
    name: "Encapsulated CDA Storage",
};

/// Positron Emission Tomography Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.128"
pub const PositronEmissionTomographyImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.128",
    ident: "PositronEmissionTomographyImageStorage",
    name: "Positron Emission Tomography Image Storage",
};

/// Standalone PET Curve Storage (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.1.129"
pub const StandalonePETCurveStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.129",
    ident: "StandalonePETCurveStorageRetired",
    name: "Standalone PET Curve Storage (Retired)",
};

/// Enhanced PET Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.130"
pub const EnhancedPETImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.130",
    ident: "EnhancedPETImageStorage",
    name: "Enhanced PET Image Storage",
};

/// Basic Structured Display Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.131"
pub const BasicStructuredDisplayStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.131",
    ident: "BasicStructuredDisplayStorage",
    name: "Basic Structured Display Storage",
};

/// RT Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.481.1"
pub const RTImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.1",
    ident: "RTImageStorage",
    name: "RT Image Storage",
};

/// RT Dose Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.481.2"
pub const RTDoseStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.2",
    ident: "RTDoseStorage",
    name: "RT Dose Storage",
};

/// RT Structure Set Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.481.3"
pub const RTStructureSetStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.3",
    ident: "RTStructureSetStorage",
    name: "RT Structure Set Storage",
};

/// RT Beams Treatment Record Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.481.4"
pub const RTBeamsTreatmentRecordStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.4",
    ident: "RTBeamsTreatmentRecordStorage",
    name: "RT Beams Treatment Record Storage",
};

/// RT Plan Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.481.5"
pub const RTPlanStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.5",
    ident: "RTPlanStorage",
    name: "RT Plan Storage",
};

/// RT Brachy Treatment Record Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.481.6"
pub const RTBrachyTreatmentRecordStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.6",
    ident: "RTBrachyTreatmentRecordStorage",
    name: "RT Brachy Treatment Record Storage",
};

/// RT Treatment Summary Record Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.481.7"
pub const RTTreatmentSummaryRecordStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.7",
    ident: "RTTreatmentSummaryRecordStorage",
    name: "RT Treatment Summary Record Storage",
};

/// RT Ion Plan Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.481.8"
pub const RTIonPlanStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.8",
    ident: "RTIonPlanStorage",
    name: "RT Ion Plan Storage",
};

/// RT Ion Beams Treatment Record Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.481.9"
pub const RTIonBeamsTreatmentRecordStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.9",
    ident: "RTIonBeamsTreatmentRecordStorage",
    name: "RT Ion Beams Treatment Record Storage",
};

/// DICOS CT Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.501.1"
pub const DICOSCTImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.501.1",
    ident: "DICOSCTImageStorage",
    name: "DICOS CT Image Storage",
};

/// DICOS Digital X-Ray Image Storage : For Presentation - SOP Class, "1.2.840.10008.5.1.4.1.1.501.2.1"
pub const DICOSDigitalXRayImageStorageForPresentation: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.501.2.1",
    ident: "DICOSDigitalXRayImageStorageForPresentation",
    name: "DICOS Digital X-Ray Image Storage - For Presentation",
};

/// DICOS Digital X-Ray Image Storage : For Processing - SOP Class, "1.2.840.10008.5.1.4.1.1.501.2.2"
pub const DICOSDigitalXRayImageStorageForProcessing: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.501.2.2",
    ident: "DICOSDigitalXRayImageStorageForProcessing",
    name: "DICOS Digital X-Ray Image Storage - For Processing",
};

/// DICOS Threat Detection Report Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.501.3"
pub const DICOSThreatDetectionReportStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.501.3",
    ident: "DICOSThreatDetectionReportStorage",
    name: "DICOS Threat Detection Report Storage",
};

/// Eddy Current Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.601.1"
pub const EddyCurrentImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.601.1",
    ident: "EddyCurrentImageStorage",
    name: "Eddy Current Image Storage",
};

/// Eddy Current Multi-frame Image Storage - SOP Class, "1.2.840.10008.5.1.4.1.1.601.2"
pub const EddyCurrentMultiFrameImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.601.2",
    ident: "EddyCurrentMultiFrameImageStorage",
    name: "Eddy Current Multi-frame Image Storage",
};

/// Patient Root Query/Retrieve Information Model : FIND - SOP Class, "1.2.840.10008.5.1.4.1.2.1.1"
pub const PatientRootQueryRetrieveInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.1.1",
    ident: "PatientRootQueryRetrieveInformationModelFIND",
    name: "Patient Root Query/Retrieve Information Model - FIND",
};

/// Patient Root Query/Retrieve Information Model : MOVE - SOP Class, "1.2.840.10008.5.1.4.1.2.1.2"
pub const PatientRootQueryRetrieveInformationModelMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.1.2",
    ident: "PatientRootQueryRetrieveInformationModelMOVE",
    name: "Patient Root Query/Retrieve Information Model - MOVE",
};

/// Patient Root Query/Retrieve Information Model : GET - SOP Class, "1.2.840.10008.5.1.4.1.2.1.3"
pub const PatientRootQueryRetrieveInformationModelGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.1.3",
    ident: "PatientRootQueryRetrieveInformationModelGET",
    name: "Patient Root Query/Retrieve Information Model - GET",
};

/// Study Root Query/Retrieve Information Model : FIND - SOP Class, "1.2.840.10008.5.1.4.1.2.2.1"
pub const StudyRootQueryRetrieveInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.2.1",
    ident: "StudyRootQueryRetrieveInformationModelFIND",
    name: "Study Root Query/Retrieve Information Model - FIND",
};

/// Study Root Query/Retrieve Information Model : MOVE - SOP Class, "1.2.840.10008.5.1.4.1.2.2.2"
pub const StudyRootQueryRetrieveInformationModelMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.2.2",
    ident: "StudyRootQueryRetrieveInformationModelMOVE",
    name: "Study Root Query/Retrieve Information Model - MOVE",
};

/// Study Root Query/Retrieve Information Model : GET - SOP Class, "1.2.840.10008.5.1.4.1.2.2.3"
pub const StudyRootQueryRetrieveInformationModelGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.2.3",
    ident: "StudyRootQueryRetrieveInformationModelGET",
    name: "Study Root Query/Retrieve Information Model - GET",
};

/// Patient/Study Only Query/Retrieve Information Model : FIND (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.2.3.1"
pub const PatientStudyOnlyQueryRetrieveInformationModelFINDRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.3.1",
    ident: "PatientStudyOnlyQueryRetrieveInformationModelFINDRetired",
    name: "Patient/Study Only Query/Retrieve Information Model - FIND (Retired)",
};

/// Patient/Study Only Query/Retrieve Information Model : MOVE (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.2.3.2"
pub const PatientStudyOnlyQueryRetrieveInformationModelMOVERetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.3.2",
    ident: "PatientStudyOnlyQueryRetrieveInformationModelMOVERetired",
    name: "Patient/Study Only Query/Retrieve Information Model - MOVE (Retired)",
};

/// Patient/Study Only Query/Retrieve Information Model : GET (Retired) - SOP Class, "1.2.840.10008.5.1.4.1.2.3.3"
pub const PatientStudyOnlyQueryRetrieveInformationModelGETRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.3.3",
    ident: "PatientStudyOnlyQueryRetrieveInformationModelGETRetired",
    name: "Patient/Study Only Query/Retrieve Information Model - GET (Retired)",
};

/// Composite Instance Root Retrieve : MOVE - SOP Class, "1.2.840.10008.5.1.4.1.2.4.2"
pub const CompositeInstanceRootRetrieveMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.4.2",
    ident: "CompositeInstanceRootRetrieveMOVE",
    name: "Composite Instance Root Retrieve - MOVE",
};

/// Composite Instance Root Retrieve : GET - SOP Class, "1.2.840.10008.5.1.4.1.2.4.3"
pub const CompositeInstanceRootRetrieveGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.4.3",
    ident: "CompositeInstanceRootRetrieveGET",
    name: "Composite Instance Root Retrieve - GET",
};

/// Composite Instance Retrieve Without Bulk Data : GET - SOP Class, "1.2.840.10008.5.1.4.1.2.5.3"
pub const CompositeInstanceRetrieveWithoutBulkDataGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.5.3",
    ident: "CompositeInstanceRetrieveWithoutBulkDataGET",
    name: "Composite Instance Retrieve Without Bulk Data - GET",
};

/// Modality Worklist Information Model : FIND - SOP Class, "1.2.840.10008.5.1.4.31"
pub const ModalityWorklistInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.31",
    ident: "ModalityWorklistInformationModelFIND",
    name: "Modality Worklist Information Model - FIND",
};

/// General Purpose Worklist Information Model : FIND - SOP Class, "1.2.840.10008.5.1.4.32.1"
pub const GeneralPurposeWorklistInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.32.1",
    ident: "GeneralPurposeWorklistInformationModelFIND",
    name: "General Purpose Worklist Information Model - FIND",
};

/// General Purpose Scheduled Procedure Step SOP Class - SOP Class, "1.2.840.10008.5.1.4.32.2"
pub const GeneralPurposeScheduledProcedureStepSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.32.2",
    ident: "GeneralPurposeScheduledProcedureStepSOPClass",
    name: "General Purpose Scheduled Procedure Step SOP Class",
};

/// General Purpose Performed Procedure Step SOP Class - SOP Class, "1.2.840.10008.5.1.4.32.3"
pub const GeneralPurposePerformedProcedureStepSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.32.3",
    ident: "GeneralPurposePerformedProcedureStepSOPClass",
    name: "General Purpose Performed Procedure Step SOP Class",
};

/// General Purpose Worklist Management Meta SOP Class - Meta SOP Class, "1.2.840.10008.5.1.4.32"
pub const GeneralPurposeWorklistManagementMetaSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.32",
    ident: "GeneralPurposeWorklistManagementMetaSOPClass",
    name: "General Purpose Worklist Management Meta SOP Class",
};

/// Instance Availability Notification SOP Class - SOP Class, "1.2.840.10008.5.1.4.33"
pub const InstanceAvailabilityNotificationSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.33",
    ident: "InstanceAvailabilityNotificationSOPClass",
    name: "Instance Availability Notification SOP Class",
};

/// RT Beams Delivery Instruction Storage : Trial (Retired) - SOP Class, "1.2.840.10008.5.1.4.34.1"
pub const RTBeamsDeliveryInstructionStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.1",
    ident: "RTBeamsDeliveryInstructionStorageTrialRetired",
    name: "RT Beams Delivery Instruction Storage - Trial (Retired)",
};

/// RT Conventional Machine Verification : Trial (Retired) - SOP Class, "1.2.840.10008.5.1.4.34.2"
pub const RTConventionalMachineVerificationTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.2",
    ident: "RTConventionalMachineVerificationTrialRetired",
    name: "RT Conventional Machine Verification - Trial (Retired)",
};

/// RT Ion Machine Verification : Trial (Retired) - SOP Class, "1.2.840.10008.5.1.4.34.3"
pub const RTIonMachineVerificationTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.3",
    ident: "RTIonMachineVerificationTrialRetired",
    name: "RT Ion Machine Verification - Trial (Retired)",
};

/// Unified Worklist and Procedure Step Service Class : Trial (Retired) - Service Class, "1.2.840.10008.5.1.4.34.4"
pub const UnifiedWorklistAndProcedureStepServiceClassTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.4",
    ident: "UnifiedWorklistAndProcedureStepServiceClassTrialRetired",
    name: "Unified Worklist and Procedure Step Service Class - Trial (Retired)",
};

/// Unified Procedure Step : Push SOP Class : Trial (Retired) - SOP Class, "1.2.840.10008.5.1.4.34.4.1"
pub const UnifiedProcedureStepPushSOPClassTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.4.1",
    ident: "UnifiedProcedureStepPushSOPClassTrialRetired",
    name: "Unified Procedure Step - Push SOP Class - Trial (Retired)",
};

/// Unified Procedure Step : Watch SOP Class : Trial (Retired) - SOP Class, "1.2.840.10008.5.1.4.34.4.2"
pub const UnifiedProcedureStepWatchSOPClassTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.4.2",
    ident: "UnifiedProcedureStepWatchSOPClassTrialRetired",
    name: "Unified Procedure Step - Watch SOP Class - Trial (Retired)",
};

/// Unified Procedure Step : Pull SOP Class : Trial (Retired) - SOP Class, "1.2.840.10008.5.1.4.34.4.3"
pub const UnifiedProcedureStepPullSOPClassTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.4.3",
    ident: "UnifiedProcedureStepPullSOPClassTrialRetired",
    name: "Unified Procedure Step - Pull SOP Class - Trial (Retired)",
};

/// Unified Procedure Step : Event SOP Class : Trial (Retired) - SOP Class, "1.2.840.10008.5.1.4.34.4.4"
pub const UnifiedProcedureStepEventSOPClassTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.4.4",
    ident: "UnifiedProcedureStepEventSOPClassTrialRetired",
    name: "Unified Procedure Step - Event SOP Class - Trial (Retired)",
};

/// Unified Worklist and Procedure Step SOP Instance - Well-known SOP Instance, "1.2.840.10008.5.1.4.34.5"
pub const UnifiedWorklistAndProcedureStepSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.5",
    ident: "UnifiedWorklistAndProcedureStepSOPInstance",
    name: "Unified Worklist and Procedure Step SOP Instance",
};

/// Unified Worklist and Procedure Step Service Class - Service Class, "1.2.840.10008.5.1.4.34.6"
pub const UnifiedWorklistAndProcedureStepServiceClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.6",
    ident: "UnifiedWorklistAndProcedureStepServiceClass",
    name: "Unified Worklist and Procedure Step Service Class",
};

/// Unified Procedure Step : Push SOP Class - SOP Class, "1.2.840.10008.5.1.4.34.6.1"
pub const UnifiedProcedureStepPushSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.6.1",
    ident: "UnifiedProcedureStepPushSOPClass",
    name: "Unified Procedure Step - Push SOP Class",
};

/// Unified Procedure Step : Watch SOP Class - SOP Class, "1.2.840.10008.5.1.4.34.6.2"
pub const UnifiedProcedureStepWatchSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.6.2",
    ident: "UnifiedProcedureStepWatchSOPClass",
    name: "Unified Procedure Step - Watch SOP Class",
};

/// Unified Procedure Step : Pull SOP Class - SOP Class, "1.2.840.10008.5.1.4.34.6.3"
pub const UnifiedProcedureStepPullSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.6.3",
    ident: "UnifiedProcedureStepPullSOPClass",
    name: "Unified Procedure Step - Pull SOP Class",
};

/// Unified Procedure Step : Event SOP Class - SOP Class, "1.2.840.10008.5.1.4.34.6.4"
pub const UnifiedProcedureStepEventSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.6.4",
    ident: "UnifiedProcedureStepEventSOPClass",
    name: "Unified Procedure Step - Event SOP Class",
};

/// RT Beams Delivery Instruction Storage - SOP Class, "1.2.840.10008.5.1.4.34.7"
pub const RTBeamsDeliveryInstructionStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.7",
    ident: "RTBeamsDeliveryInstructionStorage",
    name: "RT Beams Delivery Instruction Storage",
};

/// RT Conventional Machine Verification - SOP Class, "1.2.840.10008.5.1.4.34.8"
pub const RTConventionalMachineVerification: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.8",
    ident: "RTConventionalMachineVerification",
    name: "RT Conventional Machine Verification",
};

/// RT Ion Machine Verification - SOP Class, "1.2.840.10008.5.1.4.34.9"
pub const RTIonMachineVerification: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.9",
    ident: "RTIonMachineVerification",
    name: "RT Ion Machine Verification",
};

/// General Relevant Patient Information Query - SOP Class, "1.2.840.10008.5.1.4.37.1"
pub const GeneralRelevantPatientInformationQuery: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.37.1",
    ident: "GeneralRelevantPatientInformationQuery",
    name: "General Relevant Patient Information Query",
};

/// Breast Imaging Relevant Patient Information Query - SOP Class, "1.2.840.10008.5.1.4.37.2"
pub const BreastImagingRelevantPatientInformationQuery: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.37.2",
    ident: "BreastImagingRelevantPatientInformationQuery",
    name: "Breast Imaging Relevant Patient Information Query",
};

/// Cardiac Relevant Patient Information Query - SOP Class, "1.2.840.10008.5.1.4.37.3"
pub const CardiacRelevantPatientInformationQuery: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.37.3",
    ident: "CardiacRelevantPatientInformationQuery",
    name: "Cardiac Relevant Patient Information Query",
};

/// Hanging Protocol Storage - SOP Class, "1.2.840.10008.5.1.4.38.1"
pub const HangingProtocolStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.38.1",
    ident: "HangingProtocolStorage",
    name: "Hanging Protocol Storage",
};

/// Hanging Protocol Information Model : FIND - SOP Class, "1.2.840.10008.5.1.4.38.2"
pub const HangingProtocolInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.38.2",
    ident: "HangingProtocolInformationModelFIND",
    name: "Hanging Protocol Information Model - FIND",
};

/// Hanging Protocol Information Model : MOVE - SOP Class, "1.2.840.10008.5.1.4.38.3"
pub const HangingProtocolInformationModelMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.38.3",
    ident: "HangingProtocolInformationModelMOVE",
    name: "Hanging Protocol Information Model - MOVE",
};

/// Hanging Protocol Information Model : GET - SOP Class, "1.2.840.10008.5.1.4.38.4"
pub const HangingProtocolInformationModelGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.38.4",
    ident: "HangingProtocolInformationModelGET",
    name: "Hanging Protocol Information Model - GET",
};

/// Color Palette Storage - Transfer, "1.2.840.10008.5.1.4.39.1"
pub const ColorPaletteStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.39.1",
    ident: "ColorPaletteStorage",
    name: "Color Palette Storage",
};

/// Color Palette Information Model : FIND - Query/Retrieve, "1.2.840.10008.5.1.4.39.2"
pub const ColorPaletteInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.39.2",
    ident: "ColorPaletteInformationModelFIND",
    name: "Color Palette Information Model - FIND",
};

/// Color Palette Information Model : MOVE - Query/Retrieve, "1.2.840.10008.5.1.4.39.3"
pub const ColorPaletteInformationModelMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.39.3",
    ident: "ColorPaletteInformationModelMOVE",
    name: "Color Palette Information Model - MOVE",
};

/// Color Palette Information Model : GET - Query/Retrieve, "1.2.840.10008.5.1.4.39.4"
pub const ColorPaletteInformationModelGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.39.4",
    ident: "ColorPaletteInformationModelGET",
    name: "Color Palette Information Model - GET",
};

/// Product Characteristics Query SOP Class - SOP Class, "1.2.840.10008.5.1.4.41"
pub const ProductCharacteristicsQuerySOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.41",
    ident: "ProductCharacteristicsQuerySOPClass",
    name: "Product Characteristics Query SOP Class",
};

/// Substance Approval Query SOP Class - SOP Class, "1.2.840.10008.5.1.4.42"
pub const SubstanceApprovalQuerySOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.42",
    ident: "SubstanceApprovalQuerySOPClass",
    name: "Substance Approval Query SOP Class",
};

/// Generic Implant Template Storage - SOP Class, "1.2.840.10008.5.1.4.43.1"
pub const GenericImplantTemplateStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.43.1",
    ident: "GenericImplantTemplateStorage",
    name: "Generic Implant Template Storage",
};

/// Generic Implant Template Information Model : FIND - SOP Class, "1.2.840.10008.5.1.4.43.2"
pub const GenericImplantTemplateInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.43.2",
    ident: "GenericImplantTemplateInformationModelFIND",
    name: "Generic Implant Template Information Model - FIND",
};

/// Generic Implant Template Information Model : MOVE - SOP Class, "1.2.840.10008.5.1.4.43.3"
pub const GenericImplantTemplateInformationModelMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.43.3",
    ident: "GenericImplantTemplateInformationModelMOVE",
    name: "Generic Implant Template Information Model - MOVE",
};

/// Generic Implant Template Information Model : GET - SOP Class, "1.2.840.10008.5.1.4.43.4"
pub const GenericImplantTemplateInformationModelGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.43.4",
    ident: "GenericImplantTemplateInformationModelGET",
    name: "Generic Implant Template Information Model - GET",
};

/// Implant Assembly Template Storage - SOP Class, "1.2.840.10008.5.1.4.44.1"
pub const ImplantAssemblyTemplateStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.44.1",
    ident: "ImplantAssemblyTemplateStorage",
    name: "Implant Assembly Template Storage",
};

/// Implant Assembly Template Information Model : FIND - SOP Class, "1.2.840.10008.5.1.4.44.2"
pub const ImplantAssemblyTemplateInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.44.2",
    ident: "ImplantAssemblyTemplateInformationModelFIND",
    name: "Implant Assembly Template Information Model - FIND",
};

/// Implant Assembly Template Information Model : MOVE - SOP Class, "1.2.840.10008.5.1.4.44.3"
pub const ImplantAssemblyTemplateInformationModelMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.44.3",
    ident: "ImplantAssemblyTemplateInformationModelMOVE",
    name: "Implant Assembly Template Information Model - MOVE",
};

/// Implant Assembly Template Information Model : GET - SOP Class, "1.2.840.10008.5.1.4.44.4"
pub const ImplantAssemblyTemplateInformationModelGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.44.4",
    ident: "ImplantAssemblyTemplateInformationModelGET",
    name: "Implant Assembly Template Information Model - GET",
};

/// Implant Template Group Storage - SOP Class, "1.2.840.10008.5.1.4.45.1"
pub const ImplantTemplateGroupStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.45.1",
    ident: "ImplantTemplateGroupStorage",
    name: "Implant Template Group Storage",
};

/// Implant Template Group Information Model : FIND - SOP Class, "1.2.840.10008.5.1.4.45.2"
pub const ImplantTemplateGroupInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.45.2",
    ident: "ImplantTemplateGroupInformationModelFIND",
    name: "Implant Template Group Information Model - FIND",
};

/// Implant Template Group Information Model : MOVE - SOP Class, "1.2.840.10008.5.1.4.45.3"
pub const ImplantTemplateGroupInformationModelMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.45.3",
    ident: "ImplantTemplateGroupInformationModelMOVE",
    name: "Implant Template Group Information Model - MOVE",
};

/// Implant Template Group Information Model : GET - SOP Class, "1.2.840.10008.5.1.4.45.4"
pub const ImplantTemplateGroupInformationModelGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.45.4",
    ident: "ImplantTemplateGroupInformationModelGET",
    name: "Implant Template Group Information Model - GET",
};

/// Native DICOM Model - Application Hosting Model, "1.2.840.10008.7.1.1"
pub const NativeDICOMModel: &'static UID = &UID {
    uid: "1.2.840.10008.7.1.1",
    ident: "NativeDICOMModel",
    name: "Native DICOM Model",
};

/// Abstract Multi-Dimensional Image Model - Application Hosting Model, "1.2.840.10008.7.1.2"
pub const AbstractMultiDimensionalImageModel: &'static UID = &UID {
    uid: "1.2.840.10008.7.1.2",
    ident: "AbstractMultiDimensionalImageModel",
    name: "Abstract Multi-Dimensional Image Model",
};

/// dicomDeviceName - LDAP OID, "1.2.840.10008.15.0.3.1"
pub const dicomDeviceName: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.1",
    ident: "dicomDeviceName",
    name: "dicomDeviceName",
};

/// dicomDescription - LDAP OID, "1.2.840.10008.15.0.3.2"
pub const dicomDescription: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.2",
    ident: "dicomDescription",
    name: "dicomDescription",
};

/// dicomManufacturer - LDAP OID, "1.2.840.10008.15.0.3.3"
pub const dicomManufacturer: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.3",
    ident: "dicomManufacturer",
    name: "dicomManufacturer",
};

/// dicomManufacturerModelName - LDAP OID, "1.2.840.10008.15.0.3.4"
pub const dicomManufacturerModelName: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.4",
    ident: "dicomManufacturerModelName",
    name: "dicomManufacturerModelName",
};

/// dicomSoftwareVersion - LDAP OID, "1.2.840.10008.15.0.3.5"
pub const dicomSoftwareVersion: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.5",
    ident: "dicomSoftwareVersion",
    name: "dicomSoftwareVersion",
};

/// dicomVendorData - LDAP OID, "1.2.840.10008.15.0.3.6"
pub const dicomVendorData: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.6",
    ident: "dicomVendorData",
    name: "dicomVendorData",
};

/// dicomAETitle - LDAP OID, "1.2.840.10008.15.0.3.7"
pub const dicomAETitle: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.7",
    ident: "dicomAETitle",
    name: "dicomAETitle",
};

/// dicomNetworkConnectionReference - LDAP OID, "1.2.840.10008.15.0.3.8"
pub const dicomNetworkConnectionReference: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.8",
    ident: "dicomNetworkConnectionReference",
    name: "dicomNetworkConnectionReference",
};

/// dicomApplicationCluster - LDAP OID, "1.2.840.10008.15.0.3.9"
pub const dicomApplicationCluster: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.9",
    ident: "dicomApplicationCluster",
    name: "dicomApplicationCluster",
};

/// dicomAssociationInitiator - LDAP OID, "1.2.840.10008.15.0.3.10"
pub const dicomAssociationInitiator: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.10",
    ident: "dicomAssociationInitiator",
    name: "dicomAssociationInitiator",
};

/// dicomAssociationAcceptor - LDAP OID, "1.2.840.10008.15.0.3.11"
pub const dicomAssociationAcceptor: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.11",
    ident: "dicomAssociationAcceptor",
    name: "dicomAssociationAcceptor",
};

/// dicomHostname - LDAP OID, "1.2.840.10008.15.0.3.12"
pub const dicomHostname: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.12",
    ident: "dicomHostname",
    name: "dicomHostname",
};

/// dicomPort - LDAP OID, "1.2.840.10008.15.0.3.13"
pub const dicomPort: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.13",
    ident: "dicomPort",
    name: "dicomPort",
};

/// dicomSOPClass - LDAP OID, "1.2.840.10008.15.0.3.14"
pub const dicomSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.14",
    ident: "dicomSOPClass",
    name: "dicomSOPClass",
};

/// dicomTransferRole - LDAP OID, "1.2.840.10008.15.0.3.15"
pub const dicomTransferRole: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.15",
    ident: "dicomTransferRole",
    name: "dicomTransferRole",
};

/// dicomTransferSyntax - LDAP OID, "1.2.840.10008.15.0.3.16"
pub const dicomTransferSyntax: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.16",
    ident: "dicomTransferSyntax",
    name: "dicomTransferSyntax",
};

/// dicomPrimaryDeviceType - LDAP OID, "1.2.840.10008.15.0.3.17"
pub const dicomPrimaryDeviceType: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.17",
    ident: "dicomPrimaryDeviceType",
    name: "dicomPrimaryDeviceType",
};

/// dicomRelatedDeviceReference - LDAP OID, "1.2.840.10008.15.0.3.18"
pub const dicomRelatedDeviceReference: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.18",
    ident: "dicomRelatedDeviceReference",
    name: "dicomRelatedDeviceReference",
};

/// dicomPreferredCalledAETitle - LDAP OID, "1.2.840.10008.15.0.3.19"
pub const dicomPreferredCalledAETitle: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.19",
    ident: "dicomPreferredCalledAETitle",
    name: "dicomPreferredCalledAETitle",
};

/// dicomTLSCyphersuite - LDAP OID, "1.2.840.10008.15.0.3.20"
pub const dicomTLSCyphersuite: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.20",
    ident: "dicomTLSCyphersuite",
    name: "dicomTLSCyphersuite",
};

/// dicomAuthorizedNodeCertificateReference - LDAP OID, "1.2.840.10008.15.0.3.21"
pub const dicomAuthorizedNodeCertificateReference: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.21",
    ident: "dicomAuthorizedNodeCertificateReference",
    name: "dicomAuthorizedNodeCertificateReference",
};

/// dicomThisNodeCertificateReference - LDAP OID, "1.2.840.10008.15.0.3.22"
pub const dicomThisNodeCertificateReference: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.22",
    ident: "dicomThisNodeCertificateReference",
    name: "dicomThisNodeCertificateReference",
};

/// dicomInstalled - LDAP OID, "1.2.840.10008.15.0.3.23"
pub const dicomInstalled: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.23",
    ident: "dicomInstalled",
    name: "dicomInstalled",
};

/// dicomStationName - LDAP OID, "1.2.840.10008.15.0.3.24"
pub const dicomStationName: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.24",
    ident: "dicomStationName",
    name: "dicomStationName",
};

/// dicomDeviceSerialNumber - LDAP OID, "1.2.840.10008.15.0.3.25"
pub const dicomDeviceSerialNumber: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.25",
    ident: "dicomDeviceSerialNumber",
    name: "dicomDeviceSerialNumber",
};

/// dicomInstitutionName - LDAP OID, "1.2.840.10008.15.0.3.26"
pub const dicomInstitutionName: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.26",
    ident: "dicomInstitutionName",
    name: "dicomInstitutionName",
};

/// dicomInstitutionAddress - LDAP OID, "1.2.840.10008.15.0.3.27"
pub const dicomInstitutionAddress: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.27",
    ident: "dicomInstitutionAddress",
    name: "dicomInstitutionAddress",
};

/// dicomInstitutionDepartmentName - LDAP OID, "1.2.840.10008.15.0.3.28"
pub const dicomInstitutionDepartmentName: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.28",
    ident: "dicomInstitutionDepartmentName",
    name: "dicomInstitutionDepartmentName",
};

/// dicomIssuerOfPatientID - LDAP OID, "1.2.840.10008.15.0.3.29"
pub const dicomIssuerOfPatientID: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.29",
    ident: "dicomIssuerOfPatientID",
    name: "dicomIssuerOfPatientID",
};

/// dicomPreferredCallingAETitle - LDAP OID, "1.2.840.10008.15.0.3.30"
pub const dicomPreferredCallingAETitle: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.30",
    ident: "dicomPreferredCallingAETitle",
    name: "dicomPreferredCallingAETitle",
};

/// dicomSupportedCharacterSet - LDAP OID, "1.2.840.10008.15.0.3.31"
pub const dicomSupportedCharacterSet: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.31",
    ident: "dicomSupportedCharacterSet",
    name: "dicomSupportedCharacterSet",
};

/// dicomConfigurationRoot - LDAP OID, "1.2.840.10008.15.0.4.1"
pub const dicomConfigurationRoot: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.1",
    ident: "dicomConfigurationRoot",
    name: "dicomConfigurationRoot",
};

/// dicomDevicesRoot - LDAP OID, "1.2.840.10008.15.0.4.2"
pub const dicomDevicesRoot: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.2",
    ident: "dicomDevicesRoot",
    name: "dicomDevicesRoot",
};

/// dicomUniqueAETitlesRegistryRoot - LDAP OID, "1.2.840.10008.15.0.4.3"
pub const dicomUniqueAETitlesRegistryRoot: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.3",
    ident: "dicomUniqueAETitlesRegistryRoot",
    name: "dicomUniqueAETitlesRegistryRoot",
};

/// dicomDevice - LDAP OID, "1.2.840.10008.15.0.4.4"
pub const dicomDevice: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.4",
    ident: "dicomDevice",
    name: "dicomDevice",
};

/// dicomNetworkAE - LDAP OID, "1.2.840.10008.15.0.4.5"
pub const dicomNetworkAE: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.5",
    ident: "dicomNetworkAE",
    name: "dicomNetworkAE",
};

/// dicomNetworkConnection - LDAP OID, "1.2.840.10008.15.0.4.6"
pub const dicomNetworkConnection: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.6",
    ident: "dicomNetworkConnection",
    name: "dicomNetworkConnection",
};

/// dicomUniqueAETitle - LDAP OID, "1.2.840.10008.15.0.4.7"
pub const dicomUniqueAETitle: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.7",
    ident: "dicomUniqueAETitle",
    name: "dicomUniqueAETitle",
};

/// dicomTransferCapability - LDAP OID, "1.2.840.10008.15.0.4.8"
pub const dicomTransferCapability: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.8",
    ident: "dicomTransferCapability",
    name: "dicomTransferCapability",
};

/// Universal Coordinated Time - Synchronization Frame of Reference, "1.2.840.10008.15.1.1"
pub const UniversalCoordinatedTime: &'static UID = &UID {
    uid: "1.2.840.10008.15.1.1",
    ident: "UniversalCoordinatedTime",
    name: "Universal Coordinated Time",
};

/// Anatomic Modifier (2) - Context Group Name, "1.2.840.10008.6.1.1"
pub const AnatomicModifier2: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.1",
    ident: "AnatomicModifier2",
    name: "Anatomic Modifier (2)",
};

/// Anatomic Region (4) - Context Group Name, "1.2.840.10008.6.1.2"
pub const AnatomicRegion4: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.2",
    ident: "AnatomicRegion4",
    name: "Anatomic Region (4)",
};

/// Transducer Approach (5) - Context Group Name, "1.2.840.10008.6.1.3"
pub const TransducerApproach5: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.3",
    ident: "TransducerApproach5",
    name: "Transducer Approach (5)",
};

/// Transducer Orientation (6) - Context Group Name, "1.2.840.10008.6.1.4"
pub const TransducerOrientation6: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.4",
    ident: "TransducerOrientation6",
    name: "Transducer Orientation (6)",
};

/// Ultrasound Beam Path (7) - Context Group Name, "1.2.840.10008.6.1.5"
pub const UltrasoundBeamPath7: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.5",
    ident: "UltrasoundBeamPath7",
    name: "Ultrasound Beam Path (7)",
};

/// Angiographic Interventional Devices (8) - Context Group Name, "1.2.840.10008.6.1.6"
pub const AngiographicInterventionalDevices8: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.6",
    ident: "AngiographicInterventionalDevices8",
    name: "Angiographic Interventional Devices (8)",
};

/// Image Guided Therapeutic Procedures (9) - Context Group Name, "1.2.840.10008.6.1.7"
pub const ImageGuidedTherapeuticProcedures9: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.7",
    ident: "ImageGuidedTherapeuticProcedures9",
    name: "Image Guided Therapeutic Procedures (9)",
};

/// Interventional Drug (10) - Context Group Name, "1.2.840.10008.6.1.8"
pub const InterventionalDrug10: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.8",
    ident: "InterventionalDrug10",
    name: "Interventional Drug (10)",
};

/// Route of Administration (11) - Context Group Name, "1.2.840.10008.6.1.9"
pub const RouteOfAdministration11: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.9",
    ident: "RouteOfAdministration11",
    name: "Route of Administration (11)",
};

/// Radiographic Contrast Agent (12) - Context Group Name, "1.2.840.10008.6.1.10"
pub const RadiographicContrastAgent12: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.10",
    ident: "RadiographicContrastAgent12",
    name: "Radiographic Contrast Agent (12)",
};

/// Radiographic Contrast Agent Ingredient (13) - Context Group Name, "1.2.840.10008.6.1.11"
pub const RadiographicContrastAgentIngredient13: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.11",
    ident: "RadiographicContrastAgentIngredient13",
    name: "Radiographic Contrast Agent Ingredient (13)",
};

/// Isotopes in Radiopharmaceuticals (18) - Context Group Name, "1.2.840.10008.6.1.12"
pub const IsotopesInRadiopharmaceuticals18: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.12",
    ident: "IsotopesInRadiopharmaceuticals18",
    name: "Isotopes in Radiopharmaceuticals (18)",
};

/// Patient Orientation (19) - Context Group Name, "1.2.840.10008.6.1.13"
pub const PatientOrientation19: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.13",
    ident: "PatientOrientation19",
    name: "Patient Orientation (19)",
};

/// Patient Orientation Modifier (20) - Context Group Name, "1.2.840.10008.6.1.14"
pub const PatientOrientationModifier20: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.14",
    ident: "PatientOrientationModifier20",
    name: "Patient Orientation Modifier (20)",
};

/// Patient Gantry Relationship (21) - Context Group Name, "1.2.840.10008.6.1.15"
pub const PatientGantryRelationship21: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.15",
    ident: "PatientGantryRelationship21",
    name: "Patient Gantry Relationship (21)",
};

/// Cranio-caudad Angulation (23) - Context Group Name, "1.2.840.10008.6.1.16"
pub const CranioCaudadAngulation23: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.16",
    ident: "CranioCaudadAngulation23",
    name: "Cranio-caudad Angulation (23)",
};

/// Radiopharmaceuticals (25) - Context Group Name, "1.2.840.10008.6.1.17"
pub const Radiopharmaceuticals25: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.17",
    ident: "Radiopharmaceuticals25",
    name: "Radiopharmaceuticals (25)",
};

/// Nuclear Medicine Projections (26) - Context Group Name, "1.2.840.10008.6.1.18"
pub const NuclearMedicineProjections26: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.18",
    ident: "NuclearMedicineProjections26",
    name: "Nuclear Medicine Projections (26)",
};

/// Acquisition Modality (29) - Context Group Name, "1.2.840.10008.6.1.19"
pub const AcquisitionModality29: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.19",
    ident: "AcquisitionModality29",
    name: "Acquisition Modality (29)",
};

/// DICOM Devices (30) - Context Group Name, "1.2.840.10008.6.1.20"
pub const DICOMDevices30: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.20",
    ident: "DICOMDevices30",
    name: "DICOM Devices (30)",
};

/// Abstract Priors (31) - Context Group Name, "1.2.840.10008.6.1.21"
pub const AbstractPriors31: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.21",
    ident: "AbstractPriors31",
    name: "Abstract Priors (31)",
};

/// Numeric Value Qualifier (42) - Context Group Name, "1.2.840.10008.6.1.22"
pub const NumericValueQualifier42: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.22",
    ident: "NumericValueQualifier42",
    name: "Numeric Value Qualifier (42)",
};

/// Units of Measurement (82) - Context Group Name, "1.2.840.10008.6.1.23"
pub const UnitsOfMeasurement82: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.23",
    ident: "UnitsOfMeasurement82",
    name: "Units of Measurement (82)",
};

/// Units for Real World Value Mapping (83) - Context Group Name, "1.2.840.10008.6.1.24"
pub const UnitsForRealWorldValueMapping83: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.24",
    ident: "UnitsForRealWorldValueMapping83",
    name: "Units for Real World Value Mapping (83)",
};

/// Level of Significance (220) - Context Group Name, "1.2.840.10008.6.1.25"
pub const LevelOfSignificance220: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.25",
    ident: "LevelOfSignificance220",
    name: "Level of Significance (220)",
};

/// Measurement Range Concepts (221) - Context Group Name, "1.2.840.10008.6.1.26"
pub const MeasurementRangeConcepts221: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.26",
    ident: "MeasurementRangeConcepts221",
    name: "Measurement Range Concepts (221)",
};

/// Normality Codes (222) - Context Group Name, "1.2.840.10008.6.1.27"
pub const NormalityCodes222: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.27",
    ident: "NormalityCodes222",
    name: "Normality Codes (222)",
};

/// Normal Range Values (223) - Context Group Name, "1.2.840.10008.6.1.28"
pub const NormalRangeValues223: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.28",
    ident: "NormalRangeValues223",
    name: "Normal Range Values (223)",
};

/// Selection Method (224) - Context Group Name, "1.2.840.10008.6.1.29"
pub const SelectionMethod224: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.29",
    ident: "SelectionMethod224",
    name: "Selection Method (224)",
};

/// Measurement Uncertainty Concepts (225) - Context Group Name, "1.2.840.10008.6.1.30"
pub const MeasurementUncertaintyConcepts225: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.30",
    ident: "MeasurementUncertaintyConcepts225",
    name: "Measurement Uncertainty Concepts (225)",
};

/// Population Statistical Descriptors (226) - Context Group Name, "1.2.840.10008.6.1.31"
pub const PopulationStatisticalDescriptors226: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.31",
    ident: "PopulationStatisticalDescriptors226",
    name: "Population Statistical Descriptors (226)",
};

/// Sample Statistical Descriptors (227) - Context Group Name, "1.2.840.10008.6.1.32"
pub const SampleStatisticalDescriptors227: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.32",
    ident: "SampleStatisticalDescriptors227",
    name: "Sample Statistical Descriptors (227)",
};

/// Equation or Table (228) - Context Group Name, "1.2.840.10008.6.1.33"
pub const EquationOrTable228: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.33",
    ident: "EquationOrTable228",
    name: "Equation or Table (228)",
};

/// Yes-No (230) - Context Group Name, "1.2.840.10008.6.1.34"
pub const YesNo230: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.34",
    ident: "YesNo230",
    name: "Yes-No (230)",
};

/// Present-Absent (240) - Context Group Name, "1.2.840.10008.6.1.35"
pub const PresentAbsent240: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.35",
    ident: "PresentAbsent240",
    name: "Present-Absent (240)",
};

/// Normal-Abnormal (242) - Context Group Name, "1.2.840.10008.6.1.36"
pub const NormalAbnormal242: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.36",
    ident: "NormalAbnormal242",
    name: "Normal-Abnormal (242)",
};

/// Laterality (244) - Context Group Name, "1.2.840.10008.6.1.37"
pub const Laterality244: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.37",
    ident: "Laterality244",
    name: "Laterality (244)",
};

/// Positive-Negative (250) - Context Group Name, "1.2.840.10008.6.1.38"
pub const PositiveNegative250: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.38",
    ident: "PositiveNegative250",
    name: "Positive-Negative (250)",
};

/// Severity of Complication (251) - Context Group Name, "1.2.840.10008.6.1.39"
pub const SeverityOfComplication251: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.39",
    ident: "SeverityOfComplication251",
    name: "Severity of Complication (251)",
};

/// Observer Type (270) - Context Group Name, "1.2.840.10008.6.1.40"
pub const ObserverType270: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.40",
    ident: "ObserverType270",
    name: "Observer Type (270)",
};

/// Observation Subject Class (271) - Context Group Name, "1.2.840.10008.6.1.41"
pub const ObservationSubjectClass271: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.41",
    ident: "ObservationSubjectClass271",
    name: "Observation Subject Class (271)",
};

/// Audio Channel Source (3000) - Context Group Name, "1.2.840.10008.6.1.42"
pub const AudioChannelSource3000: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.42",
    ident: "AudioChannelSource3000",
    name: "Audio Channel Source (3000)",
};

/// ECG Leads (3001) - Context Group Name, "1.2.840.10008.6.1.43"
pub const ECGLeads3001: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.43",
    ident: "ECGLeads3001",
    name: "ECG Leads (3001)",
};

/// Hemodynamic Waveform Sources (3003) - Context Group Name, "1.2.840.10008.6.1.44"
pub const HemodynamicWaveformSources3003: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.44",
    ident: "HemodynamicWaveformSources3003",
    name: "Hemodynamic Waveform Sources (3003)",
};

/// Cardiovascular Anatomic Locations (3010) - Context Group Name, "1.2.840.10008.6.1.45"
pub const CardiovascularAnatomicLocations3010: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.45",
    ident: "CardiovascularAnatomicLocations3010",
    name: "Cardiovascular Anatomic Locations (3010)",
};

/// Electrophysiology Anatomic Locations (3011) - Context Group Name, "1.2.840.10008.6.1.46"
pub const ElectrophysiologyAnatomicLocations3011: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.46",
    ident: "ElectrophysiologyAnatomicLocations3011",
    name: "Electrophysiology Anatomic Locations (3011)",
};

/// Coronary Artery Segments (3014) - Context Group Name, "1.2.840.10008.6.1.47"
pub const CoronaryArterySegments3014: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.47",
    ident: "CoronaryArterySegments3014",
    name: "Coronary Artery Segments (3014)",
};

/// Coronary Arteries (3015) - Context Group Name, "1.2.840.10008.6.1.48"
pub const CoronaryArteries3015: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.48",
    ident: "CoronaryArteries3015",
    name: "Coronary Arteries (3015)",
};

/// Cardiovascular Anatomic Location Modifiers (3019) - Context Group Name, "1.2.840.10008.6.1.49"
pub const CardiovascularAnatomicLocationModifiers3019: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.49",
    ident: "CardiovascularAnatomicLocationModifiers3019",
    name: "Cardiovascular Anatomic Location Modifiers (3019)",
};

/// Cardiology Units of Measurement (3082) - Context Group Name, "1.2.840.10008.6.1.50"
pub const CardiologyUnitsOfMeasurement3082: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.50",
    ident: "CardiologyUnitsOfMeasurement3082",
    name: "Cardiology Units of Measurement (3082)",
};

/// Time Synchronization Channel Types (3090) - Context Group Name, "1.2.840.10008.6.1.51"
pub const TimeSynchronizationChannelTypes3090: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.51",
    ident: "TimeSynchronizationChannelTypes3090",
    name: "Time Synchronization Channel Types (3090)",
};

/// NM Procedural State Values (3101) - Context Group Name, "1.2.840.10008.6.1.52"
pub const NMProceduralStateValues3101: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.52",
    ident: "NMProceduralStateValues3101",
    name: "NM Procedural State Values (3101)",
};

/// Electrophysiology Measurement Functions and Techniques (3240) - Context Group Name, "1.2.840.10008.6.1.53"
pub const ElectrophysiologyMeasurementFunctionsAndTechniques3240: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.53",
    ident: "ElectrophysiologyMeasurementFunctionsAndTechniques3240",
    name: "Electrophysiology Measurement Functions and Techniques (3240)",
};

/// Hemodynamic Measurement Techniques (3241) - Context Group Name, "1.2.840.10008.6.1.54"
pub const HemodynamicMeasurementTechniques3241: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.54",
    ident: "HemodynamicMeasurementTechniques3241",
    name: "Hemodynamic Measurement Techniques (3241)",
};

/// Catheterization Procedure Phase (3250) - Context Group Name, "1.2.840.10008.6.1.55"
pub const CatheterizationProcedurePhase3250: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.55",
    ident: "CatheterizationProcedurePhase3250",
    name: "Catheterization Procedure Phase (3250)",
};

/// Electrophysiology Procedure Phase (3254) - Context Group Name, "1.2.840.10008.6.1.56"
pub const ElectrophysiologyProcedurePhase3254: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.56",
    ident: "ElectrophysiologyProcedurePhase3254",
    name: "Electrophysiology Procedure Phase (3254)",
};

/// Stress Protocols (3261) - Context Group Name, "1.2.840.10008.6.1.57"
pub const StressProtocols3261: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.57",
    ident: "StressProtocols3261",
    name: "Stress Protocols (3261)",
};

/// ECG Patient State Values (3262) - Context Group Name, "1.2.840.10008.6.1.58"
pub const ECGPatientStateValues3262: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.58",
    ident: "ECGPatientStateValues3262",
    name: "ECG Patient State Values (3262)",
};

/// Electrode Placement Values (3263) - Context Group Name, "1.2.840.10008.6.1.59"
pub const ElectrodePlacementValues3263: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.59",
    ident: "ElectrodePlacementValues3263",
    name: "Electrode Placement Values (3263)",
};

/// XYZ Electrode Placement Values (3264) - Context Group Name, "1.2.840.10008.6.1.60"
pub const XYZElectrodePlacementValues3264: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.60",
    ident: "XYZElectrodePlacementValues3264",
    name: "XYZ Electrode Placement Values (3264)",
};

/// Hemodynamic Physiological Challenges (3271) - Context Group Name, "1.2.840.10008.6.1.61"
pub const HemodynamicPhysiologicalChallenges3271: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.61",
    ident: "HemodynamicPhysiologicalChallenges3271",
    name: "Hemodynamic Physiological Challenges (3271)",
};

/// ECG Annotations (3335) - Context Group Name, "1.2.840.10008.6.1.62"
pub const ECGAnnotations3335: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.62",
    ident: "ECGAnnotations3335",
    name: "ECG Annotations (3335)",
};

/// Hemodynamic Annotations (3337) - Context Group Name, "1.2.840.10008.6.1.63"
pub const HemodynamicAnnotations3337: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.63",
    ident: "HemodynamicAnnotations3337",
    name: "Hemodynamic Annotations (3337)",
};

/// Electrophysiology Annotations (3339) - Context Group Name, "1.2.840.10008.6.1.64"
pub const ElectrophysiologyAnnotations3339: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.64",
    ident: "ElectrophysiologyAnnotations3339",
    name: "Electrophysiology Annotations (3339)",
};

/// Procedure Log Titles (3400) - Context Group Name, "1.2.840.10008.6.1.65"
pub const ProcedureLogTitles3400: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.65",
    ident: "ProcedureLogTitles3400",
    name: "Procedure Log Titles (3400)",
};

/// Types of Log Notes (3401) - Context Group Name, "1.2.840.10008.6.1.66"
pub const TypesOfLogNotes3401: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.66",
    ident: "TypesOfLogNotes3401",
    name: "Types of Log Notes (3401)",
};

/// Patient Status and Events (3402) - Context Group Name, "1.2.840.10008.6.1.67"
pub const PatientStatusAndEvents3402: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.67",
    ident: "PatientStatusAndEvents3402",
    name: "Patient Status and Events (3402)",
};

/// Percutaneous Entry (3403) - Context Group Name, "1.2.840.10008.6.1.68"
pub const PercutaneousEntry3403: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.68",
    ident: "PercutaneousEntry3403",
    name: "Percutaneous Entry (3403)",
};

/// Staff Actions (3404) - Context Group Name, "1.2.840.10008.6.1.69"
pub const StaffActions3404: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.69",
    ident: "StaffActions3404",
    name: "Staff Actions (3404)",
};

/// Procedure Action Values (3405) - Context Group Name, "1.2.840.10008.6.1.70"
pub const ProcedureActionValues3405: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.70",
    ident: "ProcedureActionValues3405",
    name: "Procedure Action Values (3405)",
};

/// Non-Coronary Transcatheter Interventions (3406) - Context Group Name, "1.2.840.10008.6.1.71"
pub const NonCoronaryTranscatheterInterventions3406: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.71",
    ident: "NonCoronaryTranscatheterInterventions3406",
    name: "Non-Coronary Transcatheter Interventions (3406)",
};

/// Purpose of Reference to Object (3407) - Context Group Name, "1.2.840.10008.6.1.72"
pub const PurposeOfReferenceToObject3407: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.72",
    ident: "PurposeOfReferenceToObject3407",
    name: "Purpose of Reference to Object (3407)",
};

/// Actions with Consumables (3408) - Context Group Name, "1.2.840.10008.6.1.73"
pub const ActionsWithConsumables3408: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.73",
    ident: "ActionsWithConsumables3408",
    name: "Actions with Consumables (3408)",
};

/// Administration of Drugs/Contrast (3409) - Context Group Name, "1.2.840.10008.6.1.74"
pub const AdministrationOfDrugsContrast3409: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.74",
    ident: "AdministrationOfDrugsContrast3409",
    name: "Administration of Drugs/Contrast (3409)",
};

/// Numeric Parameters of Drugs/Contrast (3410) - Context Group Name, "1.2.840.10008.6.1.75"
pub const NumericParametersOfDrugsContrast3410: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.75",
    ident: "NumericParametersOfDrugsContrast3410",
    name: "Numeric Parameters of Drugs/Contrast (3410)",
};

/// Intracoronary Devices (3411) - Context Group Name, "1.2.840.10008.6.1.76"
pub const IntracoronaryDevices3411: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.76",
    ident: "IntracoronaryDevices3411",
    name: "Intracoronary Devices (3411)",
};

/// Intervention Actions and Status (3412) - Context Group Name, "1.2.840.10008.6.1.77"
pub const InterventionActionsAndStatus3412: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.77",
    ident: "InterventionActionsAndStatus3412",
    name: "Intervention Actions and Status (3412)",
};

/// Adverse Outcomes (3413) - Context Group Name, "1.2.840.10008.6.1.78"
pub const AdverseOutcomes3413: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.78",
    ident: "AdverseOutcomes3413",
    name: "Adverse Outcomes (3413)",
};

/// Procedure Urgency (3414) - Context Group Name, "1.2.840.10008.6.1.79"
pub const ProcedureUrgency3414: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.79",
    ident: "ProcedureUrgency3414",
    name: "Procedure Urgency (3414)",
};

/// Cardiac Rhythms (3415) - Context Group Name, "1.2.840.10008.6.1.80"
pub const CardiacRhythms3415: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.80",
    ident: "CardiacRhythms3415",
    name: "Cardiac Rhythms (3415)",
};

/// Respiration Rhythms (3416) - Context Group Name, "1.2.840.10008.6.1.81"
pub const RespirationRhythms3416: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.81",
    ident: "RespirationRhythms3416",
    name: "Respiration Rhythms (3416)",
};

/// Lesion Risk (3418) - Context Group Name, "1.2.840.10008.6.1.82"
pub const LesionRisk3418: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.82",
    ident: "LesionRisk3418",
    name: "Lesion Risk (3418)",
};

/// Findings Titles (3419) - Context Group Name, "1.2.840.10008.6.1.83"
pub const FindingsTitles3419: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.83",
    ident: "FindingsTitles3419",
    name: "Findings Titles (3419)",
};

/// Procedure Action (3421) - Context Group Name, "1.2.840.10008.6.1.84"
pub const ProcedureAction3421: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.84",
    ident: "ProcedureAction3421",
    name: "Procedure Action (3421)",
};

/// Device Use Actions (3422) - Context Group Name, "1.2.840.10008.6.1.85"
pub const DeviceUseActions3422: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.85",
    ident: "DeviceUseActions3422",
    name: "Device Use Actions (3422)",
};

/// Numeric Device Characteristics (3423) - Context Group Name, "1.2.840.10008.6.1.86"
pub const NumericDeviceCharacteristics3423: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.86",
    ident: "NumericDeviceCharacteristics3423",
    name: "Numeric Device Characteristics (3423)",
};

/// Intervention Parameters (3425) - Context Group Name, "1.2.840.10008.6.1.87"
pub const InterventionParameters3425: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.87",
    ident: "InterventionParameters3425",
    name: "Intervention Parameters (3425)",
};

/// Consumables Parameters (3426) - Context Group Name, "1.2.840.10008.6.1.88"
pub const ConsumablesParameters3426: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.88",
    ident: "ConsumablesParameters3426",
    name: "Consumables Parameters (3426)",
};

/// Equipment Events (3427) - Context Group Name, "1.2.840.10008.6.1.89"
pub const EquipmentEvents3427: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.89",
    ident: "EquipmentEvents3427",
    name: "Equipment Events (3427)",
};

/// Imaging Procedures (3428) - Context Group Name, "1.2.840.10008.6.1.90"
pub const ImagingProcedures3428: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.90",
    ident: "ImagingProcedures3428",
    name: "Imaging Procedures (3428)",
};

/// Catheterization Devices (3429) - Context Group Name, "1.2.840.10008.6.1.91"
pub const CatheterizationDevices3429: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.91",
    ident: "CatheterizationDevices3429",
    name: "Catheterization Devices (3429)",
};

/// DateTime Qualifiers (3430) - Context Group Name, "1.2.840.10008.6.1.92"
pub const DateTimeQualifiers3430: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.92",
    ident: "DateTimeQualifiers3430",
    name: "DateTime Qualifiers (3430)",
};

/// Peripheral Pulse Locations (3440) - Context Group Name, "1.2.840.10008.6.1.93"
pub const PeripheralPulseLocations3440: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.93",
    ident: "PeripheralPulseLocations3440",
    name: "Peripheral Pulse Locations (3440)",
};

/// Patient assessments (3441) - Context Group Name, "1.2.840.10008.6.1.94"
pub const PatientAssessments3441: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.94",
    ident: "PatientAssessments3441",
    name: "Patient assessments (3441)",
};

/// Peripheral Pulse Methods (3442) - Context Group Name, "1.2.840.10008.6.1.95"
pub const PeripheralPulseMethods3442: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.95",
    ident: "PeripheralPulseMethods3442",
    name: "Peripheral Pulse Methods (3442)",
};

/// Skin Condition (3446) - Context Group Name, "1.2.840.10008.6.1.96"
pub const SkinCondition3446: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.96",
    ident: "SkinCondition3446",
    name: "Skin Condition (3446)",
};

/// Airway Assessment (3448) - Context Group Name, "1.2.840.10008.6.1.97"
pub const AirwayAssessment3448: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.97",
    ident: "AirwayAssessment3448",
    name: "Airway Assessment (3448)",
};

/// Calibration Objects (3451) - Context Group Name, "1.2.840.10008.6.1.98"
pub const CalibrationObjects3451: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.98",
    ident: "CalibrationObjects3451",
    name: "Calibration Objects (3451)",
};

/// Calibration Methods (3452) - Context Group Name, "1.2.840.10008.6.1.99"
pub const CalibrationMethods3452: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.99",
    ident: "CalibrationMethods3452",
    name: "Calibration Methods (3452)",
};

/// Cardiac Volume Methods (3453) - Context Group Name, "1.2.840.10008.6.1.100"
pub const CardiacVolumeMethods3453: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.100",
    ident: "CardiacVolumeMethods3453",
    name: "Cardiac Volume Methods (3453)",
};

/// Index Methods (3455) - Context Group Name, "1.2.840.10008.6.1.101"
pub const IndexMethods3455: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.101",
    ident: "IndexMethods3455",
    name: "Index Methods (3455)",
};

/// Sub-segment Methods (3456) - Context Group Name, "1.2.840.10008.6.1.102"
pub const SubSegmentMethods3456: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.102",
    ident: "SubSegmentMethods3456",
    name: "Sub-segment Methods (3456)",
};

/// Contour Realignment (3458) - Context Group Name, "1.2.840.10008.6.1.103"
pub const ContourRealignment3458: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.103",
    ident: "ContourRealignment3458",
    name: "Contour Realignment (3458)",
};

/// Circumferential ExtenT (3460) - Context Group Name, "1.2.840.10008.6.1.104"
pub const CircumferentialExtenT3460: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.104",
    ident: "CircumferentialExtenT3460",
    name: "Circumferential ExtenT (3460)",
};

/// Regional Extent (3461) - Context Group Name, "1.2.840.10008.6.1.105"
pub const RegionalExtent3461: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.105",
    ident: "RegionalExtent3461",
    name: "Regional Extent (3461)",
};

/// Chamber Identification (3462) - Context Group Name, "1.2.840.10008.6.1.106"
pub const ChamberIdentification3462: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.106",
    ident: "ChamberIdentification3462",
    name: "Chamber Identification (3462)",
};

/// QA Reference MethodS (3465) - Context Group Name, "1.2.840.10008.6.1.107"
pub const QAReferenceMethodS3465: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.107",
    ident: "QAReferenceMethodS3465",
    name: "QA Reference MethodS (3465)",
};

/// Plane Identification (3466) - Context Group Name, "1.2.840.10008.6.1.108"
pub const PlaneIdentification3466: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.108",
    ident: "PlaneIdentification3466",
    name: "Plane Identification (3466)",
};

/// Ejection Fraction (3467) - Context Group Name, "1.2.840.10008.6.1.109"
pub const EjectionFraction3467: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.109",
    ident: "EjectionFraction3467",
    name: "Ejection Fraction (3467)",
};

/// ED Volume (3468) - Context Group Name, "1.2.840.10008.6.1.110"
pub const EDVolume3468: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.110",
    ident: "EDVolume3468",
    name: "ED Volume (3468)",
};

/// ES Volume (3469) - Context Group Name, "1.2.840.10008.6.1.111"
pub const ESVolume3469: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.111",
    ident: "ESVolume3469",
    name: "ES Volume (3469)",
};

/// Vessel Lumen Cross-Sectional Area Calculation Methods (3470) - Context Group Name, "1.2.840.10008.6.1.112"
pub const VesselLumenCrossSectionalAreaCalculationMethods3470: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.112",
    ident: "VesselLumenCrossSectionalAreaCalculationMethods3470",
    name: "Vessel Lumen Cross-Sectional Area Calculation Methods (3470)",
};

/// Estimated Volumes (3471) - Context Group Name, "1.2.840.10008.6.1.113"
pub const EstimatedVolumes3471: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.113",
    ident: "EstimatedVolumes3471",
    name: "Estimated Volumes (3471)",
};

/// Cardiac Contraction Phase (3472) - Context Group Name, "1.2.840.10008.6.1.114"
pub const CardiacContractionPhase3472: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.114",
    ident: "CardiacContractionPhase3472",
    name: "Cardiac Contraction Phase (3472)",
};

/// IVUS Procedure Phases (3480) - Context Group Name, "1.2.840.10008.6.1.115"
pub const IVUSProcedurePhases3480: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.115",
    ident: "IVUSProcedurePhases3480",
    name: "IVUS Procedure Phases (3480)",
};

/// IVUS Distance Measurements (3481) - Context Group Name, "1.2.840.10008.6.1.116"
pub const IVUSDistanceMeasurements3481: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.116",
    ident: "IVUSDistanceMeasurements3481",
    name: "IVUS Distance Measurements (3481)",
};

/// IVUS Area Measurements (3482) - Context Group Name, "1.2.840.10008.6.1.117"
pub const IVUSAreaMeasurements3482: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.117",
    ident: "IVUSAreaMeasurements3482",
    name: "IVUS Area Measurements (3482)",
};

/// IVUS Longitudinal Measurements (3483) - Context Group Name, "1.2.840.10008.6.1.118"
pub const IVUSLongitudinalMeasurements3483: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.118",
    ident: "IVUSLongitudinalMeasurements3483",
    name: "IVUS Longitudinal Measurements (3483)",
};

/// IVUS Indices and Ratios (3484) - Context Group Name, "1.2.840.10008.6.1.119"
pub const IVUSIndicesAndRatios3484: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.119",
    ident: "IVUSIndicesAndRatios3484",
    name: "IVUS Indices and Ratios (3484)",
};

/// IVUS Volume Measurements (3485) - Context Group Name, "1.2.840.10008.6.1.120"
pub const IVUSVolumeMeasurements3485: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.120",
    ident: "IVUSVolumeMeasurements3485",
    name: "IVUS Volume Measurements (3485)",
};

/// Vascular Measurement Sites (3486) - Context Group Name, "1.2.840.10008.6.1.121"
pub const VascularMeasurementSites3486: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.121",
    ident: "VascularMeasurementSites3486",
    name: "Vascular Measurement Sites (3486)",
};

/// Intravascular Volumetric Regions (3487) - Context Group Name, "1.2.840.10008.6.1.122"
pub const IntravascularVolumetricRegions3487: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.122",
    ident: "IntravascularVolumetricRegions3487",
    name: "Intravascular Volumetric Regions (3487)",
};

/// Min/Max/Mean (3488) - Context Group Name, "1.2.840.10008.6.1.123"
pub const MinMaxMean3488: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.123",
    ident: "MinMaxMean3488",
    name: "Min/Max/Mean (3488)",
};

/// Calcium Distribution (3489) - Context Group Name, "1.2.840.10008.6.1.124"
pub const CalciumDistribution3489: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.124",
    ident: "CalciumDistribution3489",
    name: "Calcium Distribution (3489)",
};

/// IVUS Lesion Morphologies (3491) - Context Group Name, "1.2.840.10008.6.1.125"
pub const IVUSLesionMorphologies3491: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.125",
    ident: "IVUSLesionMorphologies3491",
    name: "IVUS Lesion Morphologies (3491)",
};

/// Vascular Dissection Classifications (3492) - Context Group Name, "1.2.840.10008.6.1.126"
pub const VascularDissectionClassifications3492: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.126",
    ident: "VascularDissectionClassifications3492",
    name: "Vascular Dissection Classifications (3492)",
};

/// IVUS Relative Stenosis Severities (3493) - Context Group Name, "1.2.840.10008.6.1.127"
pub const IVUSRelativeStenosisSeverities3493: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.127",
    ident: "IVUSRelativeStenosisSeverities3493",
    name: "IVUS Relative Stenosis Severities (3493)",
};

/// IVUS Non Morphological Findings (3494) - Context Group Name, "1.2.840.10008.6.1.128"
pub const IVUSNonMorphologicalFindings3494: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.128",
    ident: "IVUSNonMorphologicalFindings3494",
    name: "IVUS Non Morphological Findings (3494)",
};

/// IVUS Plaque Composition (3495) - Context Group Name, "1.2.840.10008.6.1.129"
pub const IVUSPlaqueComposition3495: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.129",
    ident: "IVUSPlaqueComposition3495",
    name: "IVUS Plaque Composition (3495)",
};

/// IVUS Fiducial Points (3496) - Context Group Name, "1.2.840.10008.6.1.130"
pub const IVUSFiducialPoints3496: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.130",
    ident: "IVUSFiducialPoints3496",
    name: "IVUS Fiducial Points (3496)",
};

/// IVUS Arterial Morphology (3497) - Context Group Name, "1.2.840.10008.6.1.131"
pub const IVUSArterialMorphology3497: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.131",
    ident: "IVUSArterialMorphology3497",
    name: "IVUS Arterial Morphology (3497)",
};

/// Pressure Units (3500) - Context Group Name, "1.2.840.10008.6.1.132"
pub const PressureUnits3500: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.132",
    ident: "PressureUnits3500",
    name: "Pressure Units (3500)",
};

/// Hemodynamic Resistance Units (3502) - Context Group Name, "1.2.840.10008.6.1.133"
pub const HemodynamicResistanceUnits3502: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.133",
    ident: "HemodynamicResistanceUnits3502",
    name: "Hemodynamic Resistance Units (3502)",
};

/// Indexed Hemodynamic Resistance Units (3503) - Context Group Name, "1.2.840.10008.6.1.134"
pub const IndexedHemodynamicResistanceUnits3503: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.134",
    ident: "IndexedHemodynamicResistanceUnits3503",
    name: "Indexed Hemodynamic Resistance Units (3503)",
};

/// Catheter Size Units (3510) - Context Group Name, "1.2.840.10008.6.1.135"
pub const CatheterSizeUnits3510: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.135",
    ident: "CatheterSizeUnits3510",
    name: "Catheter Size Units (3510)",
};

/// Specimen Collection (3515) - Context Group Name, "1.2.840.10008.6.1.136"
pub const SpecimenCollection3515: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.136",
    ident: "SpecimenCollection3515",
    name: "Specimen Collection (3515)",
};

/// Blood Source Type (3520) - Context Group Name, "1.2.840.10008.6.1.137"
pub const BloodSourceType3520: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.137",
    ident: "BloodSourceType3520",
    name: "Blood Source Type (3520)",
};

/// Blood Gas Pressures (3524) - Context Group Name, "1.2.840.10008.6.1.138"
pub const BloodGasPressures3524: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.138",
    ident: "BloodGasPressures3524",
    name: "Blood Gas Pressures (3524)",
};

/// Blood Gas Content (3525) - Context Group Name, "1.2.840.10008.6.1.139"
pub const BloodGasContent3525: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.139",
    ident: "BloodGasContent3525",
    name: "Blood Gas Content (3525)",
};

/// Blood Gas Saturation (3526) - Context Group Name, "1.2.840.10008.6.1.140"
pub const BloodGasSaturation3526: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.140",
    ident: "BloodGasSaturation3526",
    name: "Blood Gas Saturation (3526)",
};

/// Blood Base Excess (3527) - Context Group Name, "1.2.840.10008.6.1.141"
pub const BloodBaseExcess3527: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.141",
    ident: "BloodBaseExcess3527",
    name: "Blood Base Excess (3527)",
};

/// Blood pH (3528) - Context Group Name, "1.2.840.10008.6.1.142"
pub const BloodPH3528: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.142",
    ident: "BloodPH3528",
    name: "Blood pH (3528)",
};

/// Arterial / Venous Content (3529) - Context Group Name, "1.2.840.10008.6.1.143"
pub const ArterialVenousContent3529: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.143",
    ident: "ArterialVenousContent3529",
    name: "Arterial / Venous Content (3529)",
};

/// Oxygen Administration Actions (3530) - Context Group Name, "1.2.840.10008.6.1.144"
pub const OxygenAdministrationActions3530: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.144",
    ident: "OxygenAdministrationActions3530",
    name: "Oxygen Administration Actions (3530)",
};

/// Oxygen Administration (3531) - Context Group Name, "1.2.840.10008.6.1.145"
pub const OxygenAdministration3531: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.145",
    ident: "OxygenAdministration3531",
    name: "Oxygen Administration (3531)",
};

/// Circulatory Support Actions (3550) - Context Group Name, "1.2.840.10008.6.1.146"
pub const CirculatorySupportActions3550: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.146",
    ident: "CirculatorySupportActions3550",
    name: "Circulatory Support Actions (3550)",
};

/// Ventilation Actions (3551) - Context Group Name, "1.2.840.10008.6.1.147"
pub const VentilationActions3551: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.147",
    ident: "VentilationActions3551",
    name: "Ventilation Actions (3551)",
};

/// Pacing Actions (3552) - Context Group Name, "1.2.840.10008.6.1.148"
pub const PacingActions3552: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.148",
    ident: "PacingActions3552",
    name: "Pacing Actions (3552)",
};

/// Circulatory Support (3553) - Context Group Name, "1.2.840.10008.6.1.149"
pub const CirculatorySupport3553: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.149",
    ident: "CirculatorySupport3553",
    name: "Circulatory Support (3553)",
};

/// Ventilation (3554) - Context Group Name, "1.2.840.10008.6.1.150"
pub const Ventilation3554: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.150",
    ident: "Ventilation3554",
    name: "Ventilation (3554)",
};

/// Pacing (3555) - Context Group Name, "1.2.840.10008.6.1.151"
pub const Pacing3555: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.151",
    ident: "Pacing3555",
    name: "Pacing (3555)",
};

/// Blood Pressure Methods (3560) - Context Group Name, "1.2.840.10008.6.1.152"
pub const BloodPressureMethods3560: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.152",
    ident: "BloodPressureMethods3560",
    name: "Blood Pressure Methods (3560)",
};

/// Relative times (3600) - Context Group Name, "1.2.840.10008.6.1.153"
pub const RelativeTimes3600: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.153",
    ident: "RelativeTimes3600",
    name: "Relative times (3600)",
};

/// Hemodynamic Patient State (3602) - Context Group Name, "1.2.840.10008.6.1.154"
pub const HemodynamicPatientState3602: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.154",
    ident: "HemodynamicPatientState3602",
    name: "Hemodynamic Patient State (3602)",
};

/// Arterial lesion locations (3604) - Context Group Name, "1.2.840.10008.6.1.155"
pub const ArterialLesionLocations3604: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.155",
    ident: "ArterialLesionLocations3604",
    name: "Arterial lesion locations (3604)",
};

/// Arterial source locations (3606) - Context Group Name, "1.2.840.10008.6.1.156"
pub const ArterialSourceLocations3606: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.156",
    ident: "ArterialSourceLocations3606",
    name: "Arterial source locations (3606)",
};

/// Venous Source locations (3607) - Context Group Name, "1.2.840.10008.6.1.157"
pub const VenousSourceLocations3607: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.157",
    ident: "VenousSourceLocations3607",
    name: "Venous Source locations (3607)",
};

/// Atrial source locations (3608) - Context Group Name, "1.2.840.10008.6.1.158"
pub const AtrialSourceLocations3608: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.158",
    ident: "AtrialSourceLocations3608",
    name: "Atrial source locations (3608)",
};

/// Ventricular source locations (3609) - Context Group Name, "1.2.840.10008.6.1.159"
pub const VentricularSourceLocations3609: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.159",
    ident: "VentricularSourceLocations3609",
    name: "Ventricular source locations (3609)",
};

/// Gradient Source Locations (3610) - Context Group Name, "1.2.840.10008.6.1.160"
pub const GradientSourceLocations3610: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.160",
    ident: "GradientSourceLocations3610",
    name: "Gradient Source Locations (3610)",
};

/// Pressure Measurements (3611) - Context Group Name, "1.2.840.10008.6.1.161"
pub const PressureMeasurements3611: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.161",
    ident: "PressureMeasurements3611",
    name: "Pressure Measurements (3611)",
};

/// Blood Velocity Measurements (3612) - Context Group Name, "1.2.840.10008.6.1.162"
pub const BloodVelocityMeasurements3612: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.162",
    ident: "BloodVelocityMeasurements3612",
    name: "Blood Velocity Measurements (3612)",
};

/// Hemodynamic Time Measurements (3613) - Context Group Name, "1.2.840.10008.6.1.163"
pub const HemodynamicTimeMeasurements3613: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.163",
    ident: "HemodynamicTimeMeasurements3613",
    name: "Hemodynamic Time Measurements (3613)",
};

/// Valve Areas, non-Mitral (3614) - Context Group Name, "1.2.840.10008.6.1.164"
pub const ValveAreasNonMitral3614: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.164",
    ident: "ValveAreasNonMitral3614",
    name: "Valve Areas, non-Mitral (3614)",
};

/// Valve Areas (3615) - Context Group Name, "1.2.840.10008.6.1.165"
pub const ValveAreas3615: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.165",
    ident: "ValveAreas3615",
    name: "Valve Areas (3615)",
};

/// Hemodynamic Period Measurements (3616) - Context Group Name, "1.2.840.10008.6.1.166"
pub const HemodynamicPeriodMeasurements3616: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.166",
    ident: "HemodynamicPeriodMeasurements3616",
    name: "Hemodynamic Period Measurements (3616)",
};

/// Valve Flows (3617) - Context Group Name, "1.2.840.10008.6.1.167"
pub const ValveFlows3617: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.167",
    ident: "ValveFlows3617",
    name: "Valve Flows (3617)",
};

/// Hemodynamic Flows (3618) - Context Group Name, "1.2.840.10008.6.1.168"
pub const HemodynamicFlows3618: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.168",
    ident: "HemodynamicFlows3618",
    name: "Hemodynamic Flows (3618)",
};

/// Hemodynamic Resistance Measurements (3619) - Context Group Name, "1.2.840.10008.6.1.169"
pub const HemodynamicResistanceMeasurements3619: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.169",
    ident: "HemodynamicResistanceMeasurements3619",
    name: "Hemodynamic Resistance Measurements (3619)",
};

/// Hemodynamic Ratios (3620) - Context Group Name, "1.2.840.10008.6.1.170"
pub const HemodynamicRatios3620: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.170",
    ident: "HemodynamicRatios3620",
    name: "Hemodynamic Ratios (3620)",
};

/// Fractional Flow Reserve (3621) - Context Group Name, "1.2.840.10008.6.1.171"
pub const FractionalFlowReserve3621: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.171",
    ident: "FractionalFlowReserve3621",
    name: "Fractional Flow Reserve (3621)",
};

/// Measurement Type (3627) - Context Group Name, "1.2.840.10008.6.1.172"
pub const MeasurementType3627: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.172",
    ident: "MeasurementType3627",
    name: "Measurement Type (3627)",
};

/// Cardiac Output Methods (3628) - Context Group Name, "1.2.840.10008.6.1.173"
pub const CardiacOutputMethods3628: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.173",
    ident: "CardiacOutputMethods3628",
    name: "Cardiac Output Methods (3628)",
};

/// Procedure Intent (3629) - Context Group Name, "1.2.840.10008.6.1.174"
pub const ProcedureIntent3629: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.174",
    ident: "ProcedureIntent3629",
    name: "Procedure Intent (3629)",
};

/// Cardiovascular Anatomic Locations (3630) - Context Group Name, "1.2.840.10008.6.1.175"
pub const CardiovascularAnatomicLocations3630: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.175",
    ident: "CardiovascularAnatomicLocations3630",
    name: "Cardiovascular Anatomic Locations (3630)",
};

/// Hypertension (3640) - Context Group Name, "1.2.840.10008.6.1.176"
pub const Hypertension3640: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.176",
    ident: "Hypertension3640",
    name: "Hypertension (3640)",
};

/// Hemodynamic Assessments (3641) - Context Group Name, "1.2.840.10008.6.1.177"
pub const HemodynamicAssessments3641: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.177",
    ident: "HemodynamicAssessments3641",
    name: "Hemodynamic Assessments (3641)",
};

/// Degree Findings (3642) - Context Group Name, "1.2.840.10008.6.1.178"
pub const DegreeFindings3642: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.178",
    ident: "DegreeFindings3642",
    name: "Degree Findings (3642)",
};

/// Hemodynamic Measurement Phase (3651) - Context Group Name, "1.2.840.10008.6.1.179"
pub const HemodynamicMeasurementPhase3651: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.179",
    ident: "HemodynamicMeasurementPhase3651",
    name: "Hemodynamic Measurement Phase (3651)",
};

/// Body Surface Area Equations (3663) - Context Group Name, "1.2.840.10008.6.1.180"
pub const BodySurfaceAreaEquations3663: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.180",
    ident: "BodySurfaceAreaEquations3663",
    name: "Body Surface Area Equations (3663)",
};

/// Oxygen Consumption Equations and Tables (3664) - Context Group Name, "1.2.840.10008.6.1.181"
pub const OxygenConsumptionEquationsAndTables3664: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.181",
    ident: "OxygenConsumptionEquationsAndTables3664",
    name: "Oxygen Consumption Equations and Tables (3664)",
};

/// P50 Equations (3666) - Context Group Name, "1.2.840.10008.6.1.182"
pub const P50Equations3666: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.182",
    ident: "P50Equations3666",
    name: "P50 Equations (3666)",
};

/// Framingham Scores (3667) - Context Group Name, "1.2.840.10008.6.1.183"
pub const FraminghamScores3667: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.183",
    ident: "FraminghamScores3667",
    name: "Framingham Scores (3667)",
};

/// Framingham Tables (3668) - Context Group Name, "1.2.840.10008.6.1.184"
pub const FraminghamTables3668: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.184",
    ident: "FraminghamTables3668",
    name: "Framingham Tables (3668)",
};

/// ECG Procedure Types (3670) - Context Group Name, "1.2.840.10008.6.1.185"
pub const ECGProcedureTypes3670: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.185",
    ident: "ECGProcedureTypes3670",
    name: "ECG Procedure Types (3670)",
};

/// Reason for ECG Exam (3671) - Context Group Name, "1.2.840.10008.6.1.186"
pub const ReasonForECGExam3671: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.186",
    ident: "ReasonForECGExam3671",
    name: "Reason for ECG Exam (3671)",
};

/// Pacemakers (3672) - Context Group Name, "1.2.840.10008.6.1.187"
pub const Pacemakers3672: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.187",
    ident: "Pacemakers3672",
    name: "Pacemakers (3672)",
};

/// Diagnosis (3673) - Context Group Name, "1.2.840.10008.6.1.188"
pub const Diagnosis3673: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.188",
    ident: "Diagnosis3673",
    name: "Diagnosis (3673)",
};

/// Other Filters (3675) - Context Group Name, "1.2.840.10008.6.1.189"
pub const OtherFilters3675: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.189",
    ident: "OtherFilters3675",
    name: "Other Filters (3675)",
};

/// Lead Measurement Technique (3676) - Context Group Name, "1.2.840.10008.6.1.190"
pub const LeadMeasurementTechnique3676: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.190",
    ident: "LeadMeasurementTechnique3676",
    name: "Lead Measurement Technique (3676)",
};

/// Summary Codes ECG (3677) - Context Group Name, "1.2.840.10008.6.1.191"
pub const SummaryCodesECG3677: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.191",
    ident: "SummaryCodesECG3677",
    name: "Summary Codes ECG (3677)",
};

/// QT Correction Algorithms (3678) - Context Group Name, "1.2.840.10008.6.1.192"
pub const QTCorrectionAlgorithms3678: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.192",
    ident: "QTCorrectionAlgorithms3678",
    name: "QT Correction Algorithms (3678)",
};

/// ECG Morphology Descriptions (3679) - Context Group Name, "1.2.840.10008.6.1.193"
pub const ECGMorphologyDescriptions3679: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.193",
    ident: "ECGMorphologyDescriptions3679",
    name: "ECG Morphology Descriptions (3679)",
};

/// ECG Lead Noise Descriptions (3680) - Context Group Name, "1.2.840.10008.6.1.194"
pub const ECGLeadNoiseDescriptions3680: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.194",
    ident: "ECGLeadNoiseDescriptions3680",
    name: "ECG Lead Noise Descriptions (3680)",
};

/// ECG Lead Noise Modifiers (3681) - Context Group Name, "1.2.840.10008.6.1.195"
pub const ECGLeadNoiseModifiers3681: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.195",
    ident: "ECGLeadNoiseModifiers3681",
    name: "ECG Lead Noise Modifiers (3681)",
};

/// Probability (3682) - Context Group Name, "1.2.840.10008.6.1.196"
pub const Probability3682: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.196",
    ident: "Probability3682",
    name: "Probability (3682)",
};

/// Modifiers (3683) - Context Group Name, "1.2.840.10008.6.1.197"
pub const Modifiers3683: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.197",
    ident: "Modifiers3683",
    name: "Modifiers (3683)",
};

/// Trend (3684) - Context Group Name, "1.2.840.10008.6.1.198"
pub const Trend3684: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.198",
    ident: "Trend3684",
    name: "Trend (3684)",
};

/// Conjunctive Terms (3685) - Context Group Name, "1.2.840.10008.6.1.199"
pub const ConjunctiveTerms3685: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.199",
    ident: "ConjunctiveTerms3685",
    name: "Conjunctive Terms (3685)",
};

/// ECG Interpretive Statements (3686) - Context Group Name, "1.2.840.10008.6.1.200"
pub const ECGInterpretiveStatements3686: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.200",
    ident: "ECGInterpretiveStatements3686",
    name: "ECG Interpretive Statements (3686)",
};

/// Electrophysiology Waveform Durations (3687) - Context Group Name, "1.2.840.10008.6.1.201"
pub const ElectrophysiologyWaveformDurations3687: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.201",
    ident: "ElectrophysiologyWaveformDurations3687",
    name: "Electrophysiology Waveform Durations (3687)",
};

/// Electrophysiology Waveform Voltages (3688) - Context Group Name, "1.2.840.10008.6.1.202"
pub const ElectrophysiologyWaveformVoltages3688: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.202",
    ident: "ElectrophysiologyWaveformVoltages3688",
    name: "Electrophysiology Waveform Voltages (3688)",
};

/// Cath Diagnosis (3700) - Context Group Name, "1.2.840.10008.6.1.203"
pub const CathDiagnosis3700: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.203",
    ident: "CathDiagnosis3700",
    name: "Cath Diagnosis (3700)",
};

/// Cardiac Valves and Tracts (3701) - Context Group Name, "1.2.840.10008.6.1.204"
pub const CardiacValvesAndTracts3701: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.204",
    ident: "CardiacValvesAndTracts3701",
    name: "Cardiac Valves and Tracts (3701)",
};

/// Wall Motion (3703) - Context Group Name, "1.2.840.10008.6.1.205"
pub const WallMotion3703: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.205",
    ident: "WallMotion3703",
    name: "Wall Motion (3703)",
};

/// Myocardium Wall Morphology Findings (3704) - Context Group Name, "1.2.840.10008.6.1.206"
pub const MyocardiumWallMorphologyFindings3704: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.206",
    ident: "MyocardiumWallMorphologyFindings3704",
    name: "Myocardium Wall Morphology Findings (3704)",
};

/// Chamber Size (3705) - Context Group Name, "1.2.840.10008.6.1.207"
pub const ChamberSize3705: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.207",
    ident: "ChamberSize3705",
    name: "Chamber Size (3705)",
};

/// Overall Contractility (3706) - Context Group Name, "1.2.840.10008.6.1.208"
pub const OverallContractility3706: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.208",
    ident: "OverallContractility3706",
    name: "Overall Contractility (3706)",
};

/// VSD Description (3707) - Context Group Name, "1.2.840.10008.6.1.209"
pub const VSDDescription3707: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.209",
    ident: "VSDDescription3707",
    name: "VSD Description (3707)",
};

/// Aortic Root Description (3709) - Context Group Name, "1.2.840.10008.6.1.210"
pub const AorticRootDescription3709: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.210",
    ident: "AorticRootDescription3709",
    name: "Aortic Root Description (3709)",
};

/// Coronary Dominance (3710) - Context Group Name, "1.2.840.10008.6.1.211"
pub const CoronaryDominance3710: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.211",
    ident: "CoronaryDominance3710",
    name: "Coronary Dominance (3710)",
};

/// Valvular Abnormalities (3711) - Context Group Name, "1.2.840.10008.6.1.212"
pub const ValvularAbnormalities3711: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.212",
    ident: "ValvularAbnormalities3711",
    name: "Valvular Abnormalities (3711)",
};

/// Vessel Descriptors (3712) - Context Group Name, "1.2.840.10008.6.1.213"
pub const VesselDescriptors3712: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.213",
    ident: "VesselDescriptors3712",
    name: "Vessel Descriptors (3712)",
};

/// TIMI Flow Characteristics (3713) - Context Group Name, "1.2.840.10008.6.1.214"
pub const TIMIFlowCharacteristics3713: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.214",
    ident: "TIMIFlowCharacteristics3713",
    name: "TIMI Flow Characteristics (3713)",
};

/// Thrombus (3714) - Context Group Name, "1.2.840.10008.6.1.215"
pub const Thrombus3714: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.215",
    ident: "Thrombus3714",
    name: "Thrombus (3714)",
};

/// Lesion Margin (3715) - Context Group Name, "1.2.840.10008.6.1.216"
pub const LesionMargin3715: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.216",
    ident: "LesionMargin3715",
    name: "Lesion Margin (3715)",
};

/// Severity (3716) - Context Group Name, "1.2.840.10008.6.1.217"
pub const Severity3716: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.217",
    ident: "Severity3716",
    name: "Severity (3716)",
};

/// Myocardial Wall Segments (3717) - Context Group Name, "1.2.840.10008.6.1.218"
pub const MyocardialWallSegments3717: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.218",
    ident: "MyocardialWallSegments3717",
    name: "Myocardial Wall Segments (3717)",
};

/// Myocardial Wall Segments in Projection (3718) - Context Group Name, "1.2.840.10008.6.1.219"
pub const MyocardialWallSegmentsInProjection3718: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.219",
    ident: "MyocardialWallSegmentsInProjection3718",
    name: "Myocardial Wall Segments in Projection (3718)",
};

/// Canadian Clinical Classification (3719) - Context Group Name, "1.2.840.10008.6.1.220"
pub const CanadianClinicalClassification3719: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.220",
    ident: "CanadianClinicalClassification3719",
    name: "Canadian Clinical Classification (3719)",
};

/// Cardiac History Dates (Retired) (3720) - Context Group Name, "1.2.840.10008.6.1.221"
pub const CardiacHistoryDatesRetired3720: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.221",
    ident: "CardiacHistoryDatesRetired3720",
    name: "Cardiac History Dates (Retired) (3720)",
};

/// Cardiovascular Surgeries (3721) - Context Group Name, "1.2.840.10008.6.1.222"
pub const CardiovascularSurgeries3721: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.222",
    ident: "CardiovascularSurgeries3721",
    name: "Cardiovascular Surgeries (3721)",
};

/// Diabetic Therapy (3722) - Context Group Name, "1.2.840.10008.6.1.223"
pub const DiabeticTherapy3722: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.223",
    ident: "DiabeticTherapy3722",
    name: "Diabetic Therapy (3722)",
};

/// MI Types (3723) - Context Group Name, "1.2.840.10008.6.1.224"
pub const MITypes3723: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.224",
    ident: "MITypes3723",
    name: "MI Types (3723)",
};

/// Smoking History (3724) - Context Group Name, "1.2.840.10008.6.1.225"
pub const SmokingHistory3724: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.225",
    ident: "SmokingHistory3724",
    name: "Smoking History (3724)",
};

/// Indications for Coronary Intervention (3726) - Context Group Name, "1.2.840.10008.6.1.226"
pub const IndicationsForCoronaryIntervention3726: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.226",
    ident: "IndicationsForCoronaryIntervention3726",
    name: "Indications for Coronary Intervention (3726)",
};

/// Indications for Catheterization (3727) - Context Group Name, "1.2.840.10008.6.1.227"
pub const IndicationsForCatheterization3727: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.227",
    ident: "IndicationsForCatheterization3727",
    name: "Indications for Catheterization (3727)",
};

/// Cath Findings (3728) - Context Group Name, "1.2.840.10008.6.1.228"
pub const CathFindings3728: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.228",
    ident: "CathFindings3728",
    name: "Cath Findings (3728)",
};

/// Admission Status (3729) - Context Group Name, "1.2.840.10008.6.1.229"
pub const AdmissionStatus3729: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.229",
    ident: "AdmissionStatus3729",
    name: "Admission Status (3729)",
};

/// Insurance Payor (3730) - Context Group Name, "1.2.840.10008.6.1.230"
pub const InsurancePayor3730: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.230",
    ident: "InsurancePayor3730",
    name: "Insurance Payor (3730)",
};

/// Primary Cause of Death (3733) - Context Group Name, "1.2.840.10008.6.1.231"
pub const PrimaryCauseOfDeath3733: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.231",
    ident: "PrimaryCauseOfDeath3733",
    name: "Primary Cause of Death (3733)",
};

/// Acute Coronary Syndrome Time Period (3735) - Context Group Name, "1.2.840.10008.6.1.232"
pub const AcuteCoronarySyndromeTimePeriod3735: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.232",
    ident: "AcuteCoronarySyndromeTimePeriod3735",
    name: "Acute Coronary Syndrome Time Period (3735)",
};

/// NYHA Classification (3736) - Context Group Name, "1.2.840.10008.6.1.233"
pub const NYHAClassification3736: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.233",
    ident: "NYHAClassification3736",
    name: "NYHA Classification (3736)",
};

/// Non-Invasive Test - Ischemia (3737) - Context Group Name, "1.2.840.10008.6.1.234"
pub const NonInvasiveTestIschemia3737: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.234",
    ident: "NonInvasiveTestIschemia3737",
    name: "Non-Invasive Test - Ischemia (3737)",
};

/// Pre-Cath Angina Type (3738) - Context Group Name, "1.2.840.10008.6.1.235"
pub const PreCathAnginaType3738: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.235",
    ident: "PreCathAnginaType3738",
    name: "Pre-Cath Angina Type (3738)",
};

/// Cath Procedure Type (3739) - Context Group Name, "1.2.840.10008.6.1.236"
pub const CathProcedureType3739: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.236",
    ident: "CathProcedureType3739",
    name: "Cath Procedure Type (3739)",
};

/// Thrombolytic Administration (3740) - Context Group Name, "1.2.840.10008.6.1.237"
pub const ThrombolyticAdministration3740: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.237",
    ident: "ThrombolyticAdministration3740",
    name: "Thrombolytic Administration (3740)",
};

/// Medication Administration, Lab Visit (3741) - Context Group Name, "1.2.840.10008.6.1.238"
pub const MedicationAdministrationLabVisit3741: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.238",
    ident: "MedicationAdministrationLabVisit3741",
    name: "Medication Administration, Lab Visit (3741)",
};

/// Medication Administration, PCI (3742) - Context Group Name, "1.2.840.10008.6.1.239"
pub const MedicationAdministrationPCI3742: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.239",
    ident: "MedicationAdministrationPCI3742",
    name: "Medication Administration, PCI (3742)",
};

/// Clopidogrel/Ticlopidine Administration (3743) - Context Group Name, "1.2.840.10008.6.1.240"
pub const ClopidogrelTiclopidineAdministration3743: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.240",
    ident: "ClopidogrelTiclopidineAdministration3743",
    name: "Clopidogrel/Ticlopidine Administration (3743)",
};

/// EF Testing Method (3744) - Context Group Name, "1.2.840.10008.6.1.241"
pub const EFTestingMethod3744: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.241",
    ident: "EFTestingMethod3744",
    name: "EF Testing Method (3744)",
};

/// Calculation Method (3745) - Context Group Name, "1.2.840.10008.6.1.242"
pub const CalculationMethod3745: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.242",
    ident: "CalculationMethod3745",
    name: "Calculation Method (3745)",
};

/// Percutaneous Entry Site (3746) - Context Group Name, "1.2.840.10008.6.1.243"
pub const PercutaneousEntrySite3746: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.243",
    ident: "PercutaneousEntrySite3746",
    name: "Percutaneous Entry Site (3746)",
};

/// Percutaneous Closure (3747) - Context Group Name, "1.2.840.10008.6.1.244"
pub const PercutaneousClosure3747: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.244",
    ident: "PercutaneousClosure3747",
    name: "Percutaneous Closure (3747)",
};

/// Angiographic EF Testing Method (3748) - Context Group Name, "1.2.840.10008.6.1.245"
pub const AngiographicEFTestingMethod3748: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.245",
    ident: "AngiographicEFTestingMethod3748",
    name: "Angiographic EF Testing Method (3748)",
};

/// PCI Procedure Result (3749) - Context Group Name, "1.2.840.10008.6.1.246"
pub const PCIProcedureResult3749: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.246",
    ident: "PCIProcedureResult3749",
    name: "PCI Procedure Result (3749)",
};

/// Previously Dilated Lesion (3750) - Context Group Name, "1.2.840.10008.6.1.247"
pub const PreviouslyDilatedLesion3750: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.247",
    ident: "PreviouslyDilatedLesion3750",
    name: "Previously Dilated Lesion (3750)",
};

/// Guidewire Crossing (3752) - Context Group Name, "1.2.840.10008.6.1.248"
pub const GuidewireCrossing3752: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.248",
    ident: "GuidewireCrossing3752",
    name: "Guidewire Crossing (3752)",
};

/// Vascular Complications (3754) - Context Group Name, "1.2.840.10008.6.1.249"
pub const VascularComplications3754: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.249",
    ident: "VascularComplications3754",
    name: "Vascular Complications (3754)",
};

/// Cath Complications (3755) - Context Group Name, "1.2.840.10008.6.1.250"
pub const CathComplications3755: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.250",
    ident: "CathComplications3755",
    name: "Cath Complications (3755)",
};

/// Cardiac Patient Risk Factors (3756) - Context Group Name, "1.2.840.10008.6.1.251"
pub const CardiacPatientRiskFactors3756: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.251",
    ident: "CardiacPatientRiskFactors3756",
    name: "Cardiac Patient Risk Factors (3756)",
};

/// Cardiac Diagnostic Procedures (3757) - Context Group Name, "1.2.840.10008.6.1.252"
pub const CardiacDiagnosticProcedures3757: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.252",
    ident: "CardiacDiagnosticProcedures3757",
    name: "Cardiac Diagnostic Procedures (3757)",
};

/// Cardiovascular Family History (3758) - Context Group Name, "1.2.840.10008.6.1.253"
pub const CardiovascularFamilyHistory3758: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.253",
    ident: "CardiovascularFamilyHistory3758",
    name: "Cardiovascular Family History (3758)",
};

/// Hypertension Therapy (3760) - Context Group Name, "1.2.840.10008.6.1.254"
pub const HypertensionTherapy3760: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.254",
    ident: "HypertensionTherapy3760",
    name: "Hypertension Therapy (3760)",
};

/// Antilipemic agents (3761) - Context Group Name, "1.2.840.10008.6.1.255"
pub const AntilipemicAgents3761: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.255",
    ident: "AntilipemicAgents3761",
    name: "Antilipemic agents (3761)",
};

/// Antiarrhythmic agents (3762) - Context Group Name, "1.2.840.10008.6.1.256"
pub const AntiarrhythmicAgents3762: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.256",
    ident: "AntiarrhythmicAgents3762",
    name: "Antiarrhythmic agents (3762)",
};

/// Myocardial Infarction Therapies (3764) - Context Group Name, "1.2.840.10008.6.1.257"
pub const MyocardialInfarctionTherapies3764: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.257",
    ident: "MyocardialInfarctionTherapies3764",
    name: "Myocardial Infarction Therapies (3764)",
};

/// Concern Types (3769) - Context Group Name, "1.2.840.10008.6.1.258"
pub const ConcernTypes3769: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.258",
    ident: "ConcernTypes3769",
    name: "Concern Types (3769)",
};

/// Problem Status (3770) - Context Group Name, "1.2.840.10008.6.1.259"
pub const ProblemStatus3770: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.259",
    ident: "ProblemStatus3770",
    name: "Problem Status (3770)",
};

/// Health Status (3772) - Context Group Name, "1.2.840.10008.6.1.260"
pub const HealthStatus3772: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.260",
    ident: "HealthStatus3772",
    name: "Health Status (3772)",
};

/// Use Status (3773) - Context Group Name, "1.2.840.10008.6.1.261"
pub const UseStatus3773: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.261",
    ident: "UseStatus3773",
    name: "Use Status (3773)",
};

/// Social History (3774) - Context Group Name, "1.2.840.10008.6.1.262"
pub const SocialHistory3774: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.262",
    ident: "SocialHistory3774",
    name: "Social History (3774)",
};

/// Implanted Devices (3777) - Context Group Name, "1.2.840.10008.6.1.263"
pub const ImplantedDevices3777: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.263",
    ident: "ImplantedDevices3777",
    name: "Implanted Devices (3777)",
};

/// Plaque Structures (3802) - Context Group Name, "1.2.840.10008.6.1.264"
pub const PlaqueStructures3802: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.264",
    ident: "PlaqueStructures3802",
    name: "Plaque Structures (3802)",
};

/// Stenosis Measurement Methods (3804) - Context Group Name, "1.2.840.10008.6.1.265"
pub const StenosisMeasurementMethods3804: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.265",
    ident: "StenosisMeasurementMethods3804",
    name: "Stenosis Measurement Methods (3804)",
};

/// Stenosis Types (3805) - Context Group Name, "1.2.840.10008.6.1.266"
pub const StenosisTypes3805: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.266",
    ident: "StenosisTypes3805",
    name: "Stenosis Types (3805)",
};

/// Stenosis Shape (3806) - Context Group Name, "1.2.840.10008.6.1.267"
pub const StenosisShape3806: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.267",
    ident: "StenosisShape3806",
    name: "Stenosis Shape (3806)",
};

/// Volume Measurement Methods (3807) - Context Group Name, "1.2.840.10008.6.1.268"
pub const VolumeMeasurementMethods3807: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.268",
    ident: "VolumeMeasurementMethods3807",
    name: "Volume Measurement Methods (3807)",
};

/// Aneurysm Types (3808) - Context Group Name, "1.2.840.10008.6.1.269"
pub const AneurysmTypes3808: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.269",
    ident: "AneurysmTypes3808",
    name: "Aneurysm Types (3808)",
};

/// Associated Conditions (3809) - Context Group Name, "1.2.840.10008.6.1.270"
pub const AssociatedConditions3809: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.270",
    ident: "AssociatedConditions3809",
    name: "Associated Conditions (3809)",
};

/// Vascular Morphology (3810) - Context Group Name, "1.2.840.10008.6.1.271"
pub const VascularMorphology3810: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.271",
    ident: "VascularMorphology3810",
    name: "Vascular Morphology (3810)",
};

/// Stent Findings (3813) - Context Group Name, "1.2.840.10008.6.1.272"
pub const StentFindings3813: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.272",
    ident: "StentFindings3813",
    name: "Stent Findings (3813)",
};

/// Stent Composition (3814) - Context Group Name, "1.2.840.10008.6.1.273"
pub const StentComposition3814: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.273",
    ident: "StentComposition3814",
    name: "Stent Composition (3814)",
};

/// Source of Vascular Finding (3815) - Context Group Name, "1.2.840.10008.6.1.274"
pub const SourceOfVascularFinding3815: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.274",
    ident: "SourceOfVascularFinding3815",
    name: "Source of Vascular Finding (3815)",
};

/// Vascular Sclerosis Types (3817) - Context Group Name, "1.2.840.10008.6.1.275"
pub const VascularSclerosisTypes3817: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.275",
    ident: "VascularSclerosisTypes3817",
    name: "Vascular Sclerosis Types (3817)",
};

/// Non-invasive Vascular Procedures (3820) - Context Group Name, "1.2.840.10008.6.1.276"
pub const NonInvasiveVascularProcedures3820: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.276",
    ident: "NonInvasiveVascularProcedures3820",
    name: "Non-invasive Vascular Procedures (3820)",
};

/// Papillary Muscle Included/Excluded (3821) - Context Group Name, "1.2.840.10008.6.1.277"
pub const PapillaryMuscleIncludedExcluded3821: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.277",
    ident: "PapillaryMuscleIncludedExcluded3821",
    name: "Papillary Muscle Included/Excluded (3821)",
};

/// Respiratory Status (3823) - Context Group Name, "1.2.840.10008.6.1.278"
pub const RespiratoryStatus3823: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.278",
    ident: "RespiratoryStatus3823",
    name: "Respiratory Status (3823)",
};

/// Heart Rhythm (3826) - Context Group Name, "1.2.840.10008.6.1.279"
pub const HeartRhythm3826: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.279",
    ident: "HeartRhythm3826",
    name: "Heart Rhythm (3826)",
};

/// Vessel Segments (3827) - Context Group Name, "1.2.840.10008.6.1.280"
pub const VesselSegments3827: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.280",
    ident: "VesselSegments3827",
    name: "Vessel Segments (3827)",
};

/// Pulmonary Arteries (3829) - Context Group Name, "1.2.840.10008.6.1.281"
pub const PulmonaryArteries3829: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.281",
    ident: "PulmonaryArteries3829",
    name: "Pulmonary Arteries (3829)",
};

/// Stenosis Length (3831) - Context Group Name, "1.2.840.10008.6.1.282"
pub const StenosisLength3831: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.282",
    ident: "StenosisLength3831",
    name: "Stenosis Length (3831)",
};

/// Stenosis Grade (3832) - Context Group Name, "1.2.840.10008.6.1.283"
pub const StenosisGrade3832: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.283",
    ident: "StenosisGrade3832",
    name: "Stenosis Grade (3832)",
};

/// Cardiac Ejection Fraction (3833) - Context Group Name, "1.2.840.10008.6.1.284"
pub const CardiacEjectionFraction3833: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.284",
    ident: "CardiacEjectionFraction3833",
    name: "Cardiac Ejection Fraction (3833)",
};

/// Cardiac Volume Measurements (3835) - Context Group Name, "1.2.840.10008.6.1.285"
pub const CardiacVolumeMeasurements3835: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.285",
    ident: "CardiacVolumeMeasurements3835",
    name: "Cardiac Volume Measurements (3835)",
};

/// Time-based Perfusion Measurements (3836) - Context Group Name, "1.2.840.10008.6.1.286"
pub const TimeBasedPerfusionMeasurements3836: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.286",
    ident: "TimeBasedPerfusionMeasurements3836",
    name: "Time-based Perfusion Measurements (3836)",
};

/// Fiducial Feature (3837) - Context Group Name, "1.2.840.10008.6.1.287"
pub const FiducialFeature3837: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.287",
    ident: "FiducialFeature3837",
    name: "Fiducial Feature (3837)",
};

/// Diameter Derivation (3838) - Context Group Name, "1.2.840.10008.6.1.288"
pub const DiameterDerivation3838: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.288",
    ident: "DiameterDerivation3838",
    name: "Diameter Derivation (3838)",
};

/// Coronary Veins (3839) - Context Group Name, "1.2.840.10008.6.1.289"
pub const CoronaryVeins3839: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.289",
    ident: "CoronaryVeins3839",
    name: "Coronary Veins (3839)",
};

/// Pulmonary Veins (3840) - Context Group Name, "1.2.840.10008.6.1.290"
pub const PulmonaryVeins3840: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.290",
    ident: "PulmonaryVeins3840",
    name: "Pulmonary Veins (3840)",
};

/// Myocardial Subsegment (3843) - Context Group Name, "1.2.840.10008.6.1.291"
pub const MyocardialSubsegment3843: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.291",
    ident: "MyocardialSubsegment3843",
    name: "Myocardial Subsegment (3843)",
};

/// Partial View Section for Mammography (4005) - Context Group Name, "1.2.840.10008.6.1.292"
pub const PartialViewSectionForMammography4005: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.292",
    ident: "PartialViewSectionForMammography4005",
    name: "Partial View Section for Mammography (4005)",
};

/// DX Anatomy Imaged (4009) - Context Group Name, "1.2.840.10008.6.1.293"
pub const DXAnatomyImaged4009: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.293",
    ident: "DXAnatomyImaged4009",
    name: "DX Anatomy Imaged (4009)",
};

/// DX View (4010) - Context Group Name, "1.2.840.10008.6.1.294"
pub const DXView4010: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.294",
    ident: "DXView4010",
    name: "DX View (4010)",
};

/// DX View Modifier (4011) - Context Group Name, "1.2.840.10008.6.1.295"
pub const DXViewModifier4011: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.295",
    ident: "DXViewModifier4011",
    name: "DX View Modifier (4011)",
};

/// Projection Eponymous Name (4012) - Context Group Name, "1.2.840.10008.6.1.296"
pub const ProjectionEponymousName4012: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.296",
    ident: "ProjectionEponymousName4012",
    name: "Projection Eponymous Name (4012)",
};

/// Anatomic Region for Mammography (4013) - Context Group Name, "1.2.840.10008.6.1.297"
pub const AnatomicRegionForMammography4013: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.297",
    ident: "AnatomicRegionForMammography4013",
    name: "Anatomic Region for Mammography (4013)",
};

/// View for Mammography (4014) - Context Group Name, "1.2.840.10008.6.1.298"
pub const ViewForMammography4014: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.298",
    ident: "ViewForMammography4014",
    name: "View for Mammography (4014)",
};

/// View Modifier for Mammography (4015) - Context Group Name, "1.2.840.10008.6.1.299"
pub const ViewModifierForMammography4015: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.299",
    ident: "ViewModifierForMammography4015",
    name: "View Modifier for Mammography (4015)",
};

/// Anatomic Region for Intra-oral Radiography (4016) - Context Group Name, "1.2.840.10008.6.1.300"
pub const AnatomicRegionForIntraOralRadiography4016: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.300",
    ident: "AnatomicRegionForIntraOralRadiography4016",
    name: "Anatomic Region for Intra-oral Radiography (4016)",
};

/// Anatomic Region Modifier for Intra-oral Radiography (4017) - Context Group Name, "1.2.840.10008.6.1.301"
pub const AnatomicRegionModifierForIntraOralRadiography4017: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.301",
    ident: "AnatomicRegionModifierForIntraOralRadiography4017",
    name: "Anatomic Region Modifier for Intra-oral Radiography (4017)",
};

/// Primary Anatomic Structure for Intra-oral Radiography (Permanent Dentition / Designation of Teeth) (4018) - Context Group Name, "1.2.840.10008.6.1.302"
pub const PrimaryAnatomicStructureForIntraOralRadiographyPermanentDentitionDesignationOfTeeth4018: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.302",
    ident: "PrimaryAnatomicStructureForIntraOralRadiographyPermanentDentitionDesignationOfTeeth4018",
    name: "Primary Anatomic Structure for Intra-oral Radiography (Permanent Dentition / Designation of Teeth) (4018)",
};

/// Primary Anatomic Structure for Intra-oral Radiography (Deciduous Dentition / Designation of Teeth) (4019) - Context Group Name, "1.2.840.10008.6.1.303"
pub const PrimaryAnatomicStructureForIntraOralRadiographyDeciduousDentitionDesignationOfTeeth4019: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.303",
    ident: "PrimaryAnatomicStructureForIntraOralRadiographyDeciduousDentitionDesignationOfTeeth4019",
    name: "Primary Anatomic Structure for Intra-oral Radiography (Deciduous Dentition / Designation of Teeth) (4019)",
};

/// PET Radionuclide (4020) - Context Group Name, "1.2.840.10008.6.1.304"
pub const PETRadionuclide4020: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.304",
    ident: "PETRadionuclide4020",
    name: "PET Radionuclide (4020)",
};

/// PET Radiopharmaceutical (4021) - Context Group Name, "1.2.840.10008.6.1.305"
pub const PETRadiopharmaceutical4021: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.305",
    ident: "PETRadiopharmaceutical4021",
    name: "PET Radiopharmaceutical (4021)",
};

/// Craniofacial Anatomic Regions (4028) - Context Group Name, "1.2.840.10008.6.1.306"
pub const CraniofacialAnatomicRegions4028: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.306",
    ident: "CraniofacialAnatomicRegions4028",
    name: "Craniofacial Anatomic Regions (4028)",
};

/// CT and MR Anatomy Imaged (4030) - Context Group Name, "1.2.840.10008.6.1.307"
pub const CTAndMRAnatomyImaged4030: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.307",
    ident: "CTAndMRAnatomyImaged4030",
    name: "CT and MR Anatomy Imaged (4030)",
};

/// Common Anatomic Regions (4031) - Context Group Name, "1.2.840.10008.6.1.308"
pub const CommonAnatomicRegions4031: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.308",
    ident: "CommonAnatomicRegions4031",
    name: "Common Anatomic Regions (4031)",
};

/// MR Spectroscopy Metabolites (4032) - Context Group Name, "1.2.840.10008.6.1.309"
pub const MRSpectroscopyMetabolites4032: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.309",
    ident: "MRSpectroscopyMetabolites4032",
    name: "MR Spectroscopy Metabolites (4032)",
};

/// MR Proton Spectroscopy Metabolites (4033) - Context Group Name, "1.2.840.10008.6.1.310"
pub const MRProtonSpectroscopyMetabolites4033: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.310",
    ident: "MRProtonSpectroscopyMetabolites4033",
    name: "MR Proton Spectroscopy Metabolites (4033)",
};

/// Endoscopy Anatomic Regions (4040) - Context Group Name, "1.2.840.10008.6.1.311"
pub const EndoscopyAnatomicRegions4040: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.311",
    ident: "EndoscopyAnatomicRegions4040",
    name: "Endoscopy Anatomic Regions (4040)",
};

/// XA/XRF Anatomy Imaged (4042) - Context Group Name, "1.2.840.10008.6.1.312"
pub const XAXRFAnatomyImaged4042: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.312",
    ident: "XAXRFAnatomyImaged4042",
    name: "XA/XRF Anatomy Imaged (4042)",
};

/// Drug or Contrast Agent Characteristics (4050) - Context Group Name, "1.2.840.10008.6.1.313"
pub const DrugOrContrastAgentCharacteristics4050: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.313",
    ident: "DrugOrContrastAgentCharacteristics4050",
    name: "Drug or Contrast Agent Characteristics (4050)",
};

/// General Devices (4051) - Context Group Name, "1.2.840.10008.6.1.314"
pub const GeneralDevices4051: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.314",
    ident: "GeneralDevices4051",
    name: "General Devices (4051)",
};

/// Phantom Devices (4052) - Context Group Name, "1.2.840.10008.6.1.315"
pub const PhantomDevices4052: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.315",
    ident: "PhantomDevices4052",
    name: "Phantom Devices (4052)",
};

/// Ophthalmic Imaging Agent (4200) - Context Group Name, "1.2.840.10008.6.1.316"
pub const OphthalmicImagingAgent4200: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.316",
    ident: "OphthalmicImagingAgent4200",
    name: "Ophthalmic Imaging Agent (4200)",
};

/// Patient Eye Movement Command (4201) - Context Group Name, "1.2.840.10008.6.1.317"
pub const PatientEyeMovementCommand4201: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.317",
    ident: "PatientEyeMovementCommand4201",
    name: "Patient Eye Movement Command (4201)",
};

/// Ophthalmic Photography Acquisition Device (4202) - Context Group Name, "1.2.840.10008.6.1.318"
pub const OphthalmicPhotographyAcquisitionDevice4202: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.318",
    ident: "OphthalmicPhotographyAcquisitionDevice4202",
    name: "Ophthalmic Photography Acquisition Device (4202)",
};

/// Ophthalmic Photography Illumination (4203) - Context Group Name, "1.2.840.10008.6.1.319"
pub const OphthalmicPhotographyIllumination4203: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.319",
    ident: "OphthalmicPhotographyIllumination4203",
    name: "Ophthalmic Photography Illumination (4203)",
};

/// Ophthalmic Filter (4204) - Context Group Name, "1.2.840.10008.6.1.320"
pub const OphthalmicFilter4204: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.320",
    ident: "OphthalmicFilter4204",
    name: "Ophthalmic Filter (4204)",
};

/// Ophthalmic Lens (4205) - Context Group Name, "1.2.840.10008.6.1.321"
pub const OphthalmicLens4205: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.321",
    ident: "OphthalmicLens4205",
    name: "Ophthalmic Lens (4205)",
};

/// Ophthalmic Channel Description (4206) - Context Group Name, "1.2.840.10008.6.1.322"
pub const OphthalmicChannelDescription4206: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.322",
    ident: "OphthalmicChannelDescription4206",
    name: "Ophthalmic Channel Description (4206)",
};

/// Ophthalmic Image Position (4207) - Context Group Name, "1.2.840.10008.6.1.323"
pub const OphthalmicImagePosition4207: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.323",
    ident: "OphthalmicImagePosition4207",
    name: "Ophthalmic Image Position (4207)",
};

/// Mydriatic Agent (4208) - Context Group Name, "1.2.840.10008.6.1.324"
pub const MydriaticAgent4208: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.324",
    ident: "MydriaticAgent4208",
    name: "Mydriatic Agent (4208)",
};

/// Ophthalmic Anatomic Structure Imaged (4209) - Context Group Name, "1.2.840.10008.6.1.325"
pub const OphthalmicAnatomicStructureImaged4209: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.325",
    ident: "OphthalmicAnatomicStructureImaged4209",
    name: "Ophthalmic Anatomic Structure Imaged (4209)",
};

/// Ophthalmic Tomography Acquisition Device (4210) - Context Group Name, "1.2.840.10008.6.1.326"
pub const OphthalmicTomographyAcquisitionDevice4210: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.326",
    ident: "OphthalmicTomographyAcquisitionDevice4210",
    name: "Ophthalmic Tomography Acquisition Device (4210)",
};

/// Ophthalmic OCT Anatomic Structure Imaged (4211) - Context Group Name, "1.2.840.10008.6.1.327"
pub const OphthalmicOCTAnatomicStructureImaged4211: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.327",
    ident: "OphthalmicOCTAnatomicStructureImaged4211",
    name: "Ophthalmic OCT Anatomic Structure Imaged (4211)",
};

/// Languages (5000) - Context Group Name, "1.2.840.10008.6.1.328"
pub const Languages5000: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.328",
    ident: "Languages5000",
    name: "Languages (5000)",
};

/// Countries (5001) - Context Group Name, "1.2.840.10008.6.1.329"
pub const Countries5001: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.329",
    ident: "Countries5001",
    name: "Countries (5001)",
};

/// Overall Breast Composition (6000) - Context Group Name, "1.2.840.10008.6.1.330"
pub const OverallBreastComposition6000: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.330",
    ident: "OverallBreastComposition6000",
    name: "Overall Breast Composition (6000)",
};

/// Overall Breast Composition from BI-RADS® (6001) - Context Group Name, "1.2.840.10008.6.1.331"
pub const OverallBreastCompositionFromBIRADS6001: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.331",
    ident: "OverallBreastCompositionFromBIRADS6001",
    name: "Overall Breast Composition from BI-RADS® (6001)",
};

/// Change Since Last Mammogram or Prior Surgery (6002) - Context Group Name, "1.2.840.10008.6.1.332"
pub const ChangeSinceLastMammogramOrPriorSurgery6002: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.332",
    ident: "ChangeSinceLastMammogramOrPriorSurgery6002",
    name: "Change Since Last Mammogram or Prior Surgery (6002)",
};

/// Change Since Last Mammogram or Prior Surgery from BI-RADS® (6003) - Context Group Name, "1.2.840.10008.6.1.333"
pub const ChangeSinceLastMammogramOrPriorSurgeryFromBIRADS6003: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.333",
    ident: "ChangeSinceLastMammogramOrPriorSurgeryFromBIRADS6003",
    name: "Change Since Last Mammogram or Prior Surgery from BI-RADS® (6003)",
};

/// Mammography Characteristics of Shape (6004) - Context Group Name, "1.2.840.10008.6.1.334"
pub const MammographyCharacteristicsOfShape6004: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.334",
    ident: "MammographyCharacteristicsOfShape6004",
    name: "Mammography Characteristics of Shape (6004)",
};

/// Characteristics of Shape from BI-RADS® (6005) - Context Group Name, "1.2.840.10008.6.1.335"
pub const CharacteristicsOfShapeFromBIRADS6005: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.335",
    ident: "CharacteristicsOfShapeFromBIRADS6005",
    name: "Characteristics of Shape from BI-RADS® (6005)",
};

/// Mammography Characteristics of Margin (6006) - Context Group Name, "1.2.840.10008.6.1.336"
pub const MammographyCharacteristicsOfMargin6006: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.336",
    ident: "MammographyCharacteristicsOfMargin6006",
    name: "Mammography Characteristics of Margin (6006)",
};

/// Characteristics of Margin from BI-RADS® (6007) - Context Group Name, "1.2.840.10008.6.1.337"
pub const CharacteristicsOfMarginFromBIRADS6007: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.337",
    ident: "CharacteristicsOfMarginFromBIRADS6007",
    name: "Characteristics of Margin from BI-RADS® (6007)",
};

/// Density Modifier (6008) - Context Group Name, "1.2.840.10008.6.1.338"
pub const DensityModifier6008: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.338",
    ident: "DensityModifier6008",
    name: "Density Modifier (6008)",
};

/// Density Modifier from BI-RADS® (6009) - Context Group Name, "1.2.840.10008.6.1.339"
pub const DensityModifierFromBIRADS6009: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.339",
    ident: "DensityModifierFromBIRADS6009",
    name: "Density Modifier from BI-RADS® (6009)",
};

/// Mammography Calcification Types (6010) - Context Group Name, "1.2.840.10008.6.1.340"
pub const MammographyCalcificationTypes6010: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.340",
    ident: "MammographyCalcificationTypes6010",
    name: "Mammography Calcification Types (6010)",
};

/// Calcification Types from BI-RADS® (6011) - Context Group Name, "1.2.840.10008.6.1.341"
pub const CalcificationTypesFromBIRADS6011: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.341",
    ident: "CalcificationTypesFromBIRADS6011",
    name: "Calcification Types from BI-RADS® (6011)",
};

/// Calcification Distribution Modifier (6012) - Context Group Name, "1.2.840.10008.6.1.342"
pub const CalcificationDistributionModifier6012: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.342",
    ident: "CalcificationDistributionModifier6012",
    name: "Calcification Distribution Modifier (6012)",
};

/// Calcification Distribution Modifier from BI-RADS® (6013) - Context Group Name, "1.2.840.10008.6.1.343"
pub const CalcificationDistributionModifierFromBIRADS6013: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.343",
    ident: "CalcificationDistributionModifierFromBIRADS6013",
    name: "Calcification Distribution Modifier from BI-RADS® (6013)",
};

/// Mammography Single Image Finding (6014) - Context Group Name, "1.2.840.10008.6.1.344"
pub const MammographySingleImageFinding6014: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.344",
    ident: "MammographySingleImageFinding6014",
    name: "Mammography Single Image Finding (6014)",
};

/// Single Image Finding from BI-RADS® (6015) - Context Group Name, "1.2.840.10008.6.1.345"
pub const SingleImageFindingFromBIRADS6015: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.345",
    ident: "SingleImageFindingFromBIRADS6015",
    name: "Single Image Finding from BI-RADS® (6015)",
};

/// Mammography Composite Feature (6016) - Context Group Name, "1.2.840.10008.6.1.346"
pub const MammographyCompositeFeature6016: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.346",
    ident: "MammographyCompositeFeature6016",
    name: "Mammography Composite Feature (6016)",
};

/// Composite Feature from BI-RADS® (6017) - Context Group Name, "1.2.840.10008.6.1.347"
pub const CompositeFeatureFromBIRADS6017: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.347",
    ident: "CompositeFeatureFromBIRADS6017",
    name: "Composite Feature from BI-RADS® (6017)",
};

/// Clockface Location or Region (6018) - Context Group Name, "1.2.840.10008.6.1.348"
pub const ClockfaceLocationOrRegion6018: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.348",
    ident: "ClockfaceLocationOrRegion6018",
    name: "Clockface Location or Region (6018)",
};

/// Clockface Location or Region from BI-RADS® (6019) - Context Group Name, "1.2.840.10008.6.1.349"
pub const ClockfaceLocationOrRegionFromBIRADS6019: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.349",
    ident: "ClockfaceLocationOrRegionFromBIRADS6019",
    name: "Clockface Location or Region from BI-RADS® (6019)",
};

/// Quadrant Location (6020) - Context Group Name, "1.2.840.10008.6.1.350"
pub const QuadrantLocation6020: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.350",
    ident: "QuadrantLocation6020",
    name: "Quadrant Location (6020)",
};

/// Quadrant Location from BI-RADS® (6021) - Context Group Name, "1.2.840.10008.6.1.351"
pub const QuadrantLocationFromBIRADS6021: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.351",
    ident: "QuadrantLocationFromBIRADS6021",
    name: "Quadrant Location from BI-RADS® (6021)",
};

/// Side (6022) - Context Group Name, "1.2.840.10008.6.1.352"
pub const Side6022: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.352",
    ident: "Side6022",
    name: "Side (6022)",
};

/// Side from BI-RADS® (6023) - Context Group Name, "1.2.840.10008.6.1.353"
pub const SideFromBIRADS6023: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.353",
    ident: "SideFromBIRADS6023",
    name: "Side from BI-RADS® (6023)",
};

/// Depth (6024) - Context Group Name, "1.2.840.10008.6.1.354"
pub const Depth6024: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.354",
    ident: "Depth6024",
    name: "Depth (6024)",
};

/// Depth from BI-RADS® (6025) - Context Group Name, "1.2.840.10008.6.1.355"
pub const DepthFromBIRADS6025: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.355",
    ident: "DepthFromBIRADS6025",
    name: "Depth from BI-RADS® (6025)",
};

/// Mammography Assessment (6026) - Context Group Name, "1.2.840.10008.6.1.356"
pub const MammographyAssessment6026: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.356",
    ident: "MammographyAssessment6026",
    name: "Mammography Assessment (6026)",
};

/// Assessment from BI-RADS® (6027) - Context Group Name, "1.2.840.10008.6.1.357"
pub const AssessmentFromBIRADS6027: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.357",
    ident: "AssessmentFromBIRADS6027",
    name: "Assessment from BI-RADS® (6027)",
};

/// Mammography Recommended Follow-up (6028) - Context Group Name, "1.2.840.10008.6.1.358"
pub const MammographyRecommendedFollowUp6028: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.358",
    ident: "MammographyRecommendedFollowUp6028",
    name: "Mammography Recommended Follow-up (6028)",
};

/// Recommended Follow-up from BI-RADS® (6029) - Context Group Name, "1.2.840.10008.6.1.359"
pub const RecommendedFollowUpFromBIRADS6029: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.359",
    ident: "RecommendedFollowUpFromBIRADS6029",
    name: "Recommended Follow-up from BI-RADS® (6029)",
};

/// Mammography Pathology Codes (6030) - Context Group Name, "1.2.840.10008.6.1.360"
pub const MammographyPathologyCodes6030: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.360",
    ident: "MammographyPathologyCodes6030",
    name: "Mammography Pathology Codes (6030)",
};

/// Benign Pathology Codes from BI-RADS® (6031) - Context Group Name, "1.2.840.10008.6.1.361"
pub const BenignPathologyCodesFromBIRADS6031: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.361",
    ident: "BenignPathologyCodesFromBIRADS6031",
    name: "Benign Pathology Codes from BI-RADS® (6031)",
};

/// High Risk Lesions Pathology Codes from BI-RADS® (6032) - Context Group Name, "1.2.840.10008.6.1.362"
pub const HighRiskLesionsPathologyCodesFromBIRADS6032: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.362",
    ident: "HighRiskLesionsPathologyCodesFromBIRADS6032",
    name: "High Risk Lesions Pathology Codes from BI-RADS® (6032)",
};

/// Malignant Pathology Codes from BI-RADS® (6033) - Context Group Name, "1.2.840.10008.6.1.363"
pub const MalignantPathologyCodesFromBIRADS6033: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.363",
    ident: "MalignantPathologyCodesFromBIRADS6033",
    name: "Malignant Pathology Codes from BI-RADS® (6033)",
};

/// Intended Use of CAD Output (6034) - Context Group Name, "1.2.840.10008.6.1.364"
pub const IntendedUseOfCADOutput6034: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.364",
    ident: "IntendedUseOfCADOutput6034",
    name: "Intended Use of CAD Output (6034)",
};

/// Composite Feature Relations (6035) - Context Group Name, "1.2.840.10008.6.1.365"
pub const CompositeFeatureRelations6035: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.365",
    ident: "CompositeFeatureRelations6035",
    name: "Composite Feature Relations (6035)",
};

/// Scope of Feature (6036) - Context Group Name, "1.2.840.10008.6.1.366"
pub const ScopeOfFeature6036: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.366",
    ident: "ScopeOfFeature6036",
    name: "Scope of Feature (6036)",
};

/// Mammography Quantitative Temporal Difference Type (6037) - Context Group Name, "1.2.840.10008.6.1.367"
pub const MammographyQuantitativeTemporalDifferenceType6037: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.367",
    ident: "MammographyQuantitativeTemporalDifferenceType6037",
    name: "Mammography Quantitative Temporal Difference Type (6037)",
};

/// Mammography Qualitative Temporal Difference Type (6038) - Context Group Name, "1.2.840.10008.6.1.368"
pub const MammographyQualitativeTemporalDifferenceType6038: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.368",
    ident: "MammographyQualitativeTemporalDifferenceType6038",
    name: "Mammography Qualitative Temporal Difference Type (6038)",
};

/// Nipple Characteristic (6039) - Context Group Name, "1.2.840.10008.6.1.369"
pub const NippleCharacteristic6039: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.369",
    ident: "NippleCharacteristic6039",
    name: "Nipple Characteristic (6039)",
};

/// Non-Lesion Object Type (6040) - Context Group Name, "1.2.840.10008.6.1.370"
pub const NonLesionObjectType6040: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.370",
    ident: "NonLesionObjectType6040",
    name: "Non-Lesion Object Type (6040)",
};

/// Mammography Image Quality Finding (6041) - Context Group Name, "1.2.840.10008.6.1.371"
pub const MammographyImageQualityFinding6041: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.371",
    ident: "MammographyImageQualityFinding6041",
    name: "Mammography Image Quality Finding (6041)",
};

/// Status of Results (6042) - Context Group Name, "1.2.840.10008.6.1.372"
pub const StatusOfResults6042: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.372",
    ident: "StatusOfResults6042",
    name: "Status of Results (6042)",
};

/// Types of Mammography CAD Analysis (6043) - Context Group Name, "1.2.840.10008.6.1.373"
pub const TypesOfMammographyCADAnalysis6043: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.373",
    ident: "TypesOfMammographyCADAnalysis6043",
    name: "Types of Mammography CAD Analysis (6043)",
};

/// Types of Image Quality Assessment (6044) - Context Group Name, "1.2.840.10008.6.1.374"
pub const TypesOfImageQualityAssessment6044: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.374",
    ident: "TypesOfImageQualityAssessment6044",
    name: "Types of Image Quality Assessment (6044)",
};

/// Mammography Types of Quality Control Standard (6045) - Context Group Name, "1.2.840.10008.6.1.375"
pub const MammographyTypesOfQualityControlStandard6045: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.375",
    ident: "MammographyTypesOfQualityControlStandard6045",
    name: "Mammography Types of Quality Control Standard (6045)",
};

/// Units of Follow-up Interval (6046) - Context Group Name, "1.2.840.10008.6.1.376"
pub const UnitsOfFollowUpInterval6046: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.376",
    ident: "UnitsOfFollowUpInterval6046",
    name: "Units of Follow-up Interval (6046)",
};

/// CAD Processing and Findings Summary (6047) - Context Group Name, "1.2.840.10008.6.1.377"
pub const CADProcessingAndFindingsSummary6047: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.377",
    ident: "CADProcessingAndFindingsSummary6047",
    name: "CAD Processing and Findings Summary (6047)",
};

/// CAD Operating Point Axis Label (6048) - Context Group Name, "1.2.840.10008.6.1.378"
pub const CADOperatingPointAxisLabel6048: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.378",
    ident: "CADOperatingPointAxisLabel6048",
    name: "CAD Operating Point Axis Label (6048)",
};

/// Breast Procedure Reported (6050) - Context Group Name, "1.2.840.10008.6.1.379"
pub const BreastProcedureReported6050: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.379",
    ident: "BreastProcedureReported6050",
    name: "Breast Procedure Reported (6050)",
};

/// Breast Procedure Reason (6051) - Context Group Name, "1.2.840.10008.6.1.380"
pub const BreastProcedureReason6051: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.380",
    ident: "BreastProcedureReason6051",
    name: "Breast Procedure Reason (6051)",
};

/// Breast Imaging Report section title (6052) - Context Group Name, "1.2.840.10008.6.1.381"
pub const BreastImagingReportSectionTitle6052: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.381",
    ident: "BreastImagingReportSectionTitle6052",
    name: "Breast Imaging Report section title (6052)",
};

/// Breast Imaging Report Elements (6053) - Context Group Name, "1.2.840.10008.6.1.382"
pub const BreastImagingReportElements6053: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.382",
    ident: "BreastImagingReportElements6053",
    name: "Breast Imaging Report Elements (6053)",
};

/// Breast Imaging Findings (6054) - Context Group Name, "1.2.840.10008.6.1.383"
pub const BreastImagingFindings6054: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.383",
    ident: "BreastImagingFindings6054",
    name: "Breast Imaging Findings (6054)",
};

/// Breast Clinical Finding or Indicated Problem (6055) - Context Group Name, "1.2.840.10008.6.1.384"
pub const BreastClinicalFindingOrIndicatedProblem6055: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.384",
    ident: "BreastClinicalFindingOrIndicatedProblem6055",
    name: "Breast Clinical Finding or Indicated Problem (6055)",
};

/// Associated Findings for Breast (6056) - Context Group Name, "1.2.840.10008.6.1.385"
pub const AssociatedFindingsForBreast6056: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.385",
    ident: "AssociatedFindingsForBreast6056",
    name: "Associated Findings for Breast (6056)",
};

/// Ductography Findings for Breast (6057) - Context Group Name, "1.2.840.10008.6.1.386"
pub const DuctographyFindingsForBreast6057: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.386",
    ident: "DuctographyFindingsForBreast6057",
    name: "Ductography Findings for Breast (6057)",
};

/// Procedure Modifiers for Breast (6058) - Context Group Name, "1.2.840.10008.6.1.387"
pub const ProcedureModifiersForBreast6058: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.387",
    ident: "ProcedureModifiersForBreast6058",
    name: "Procedure Modifiers for Breast (6058)",
};

/// Breast Implant Types (6059) - Context Group Name, "1.2.840.10008.6.1.388"
pub const BreastImplantTypes6059: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.388",
    ident: "BreastImplantTypes6059",
    name: "Breast Implant Types (6059)",
};

/// Breast Biopsy Techniques (6060) - Context Group Name, "1.2.840.10008.6.1.389"
pub const BreastBiopsyTechniques6060: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.389",
    ident: "BreastBiopsyTechniques6060",
    name: "Breast Biopsy Techniques (6060)",
};

/// Breast Imaging Procedure Modifiers (6061) - Context Group Name, "1.2.840.10008.6.1.390"
pub const BreastImagingProcedureModifiers6061: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.390",
    ident: "BreastImagingProcedureModifiers6061",
    name: "Breast Imaging Procedure Modifiers (6061)",
};

/// Interventional Procedure Complications (6062) - Context Group Name, "1.2.840.10008.6.1.391"
pub const InterventionalProcedureComplications6062: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.391",
    ident: "InterventionalProcedureComplications6062",
    name: "Interventional Procedure Complications (6062)",
};

/// Interventional Procedure Results (6063) - Context Group Name, "1.2.840.10008.6.1.392"
pub const InterventionalProcedureResults6063: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.392",
    ident: "InterventionalProcedureResults6063",
    name: "Interventional Procedure Results (6063)",
};

/// Ultrasound Findings for Breast (6064) - Context Group Name, "1.2.840.10008.6.1.393"
pub const UltrasoundFindingsForBreast6064: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.393",
    ident: "UltrasoundFindingsForBreast6064",
    name: "Ultrasound Findings for Breast (6064)",
};

/// Instrument Approach (6065) - Context Group Name, "1.2.840.10008.6.1.394"
pub const InstrumentApproach6065: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.394",
    ident: "InstrumentApproach6065",
    name: "Instrument Approach (6065)",
};

/// Target Confirmation (6066) - Context Group Name, "1.2.840.10008.6.1.395"
pub const TargetConfirmation6066: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.395",
    ident: "TargetConfirmation6066",
    name: "Target Confirmation (6066)",
};

/// Fluid Color (6067) - Context Group Name, "1.2.840.10008.6.1.396"
pub const FluidColor6067: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.396",
    ident: "FluidColor6067",
    name: "Fluid Color (6067)",
};

/// Tumor Stages from AJCC (6068) - Context Group Name, "1.2.840.10008.6.1.397"
pub const TumorStagesFromAJCC6068: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.397",
    ident: "TumorStagesFromAJCC6068",
    name: "Tumor Stages from AJCC (6068)",
};

/// Nottingham Combined Histologic Grade (6069) - Context Group Name, "1.2.840.10008.6.1.398"
pub const NottinghamCombinedHistologicGrade6069: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.398",
    ident: "NottinghamCombinedHistologicGrade6069",
    name: "Nottingham Combined Histologic Grade (6069)",
};

/// Bloom-Richardson Histologic Grade (6070) - Context Group Name, "1.2.840.10008.6.1.399"
pub const BloomRichardsonHistologicGrade6070: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.399",
    ident: "BloomRichardsonHistologicGrade6070",
    name: "Bloom-Richardson Histologic Grade (6070)",
};

/// Histologic Grading Method (6071) - Context Group Name, "1.2.840.10008.6.1.400"
pub const HistologicGradingMethod6071: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.400",
    ident: "HistologicGradingMethod6071",
    name: "Histologic Grading Method (6071)",
};

/// Breast Implant Findings (6072) - Context Group Name, "1.2.840.10008.6.1.401"
pub const BreastImplantFindings6072: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.401",
    ident: "BreastImplantFindings6072",
    name: "Breast Implant Findings (6072)",
};

/// Gynecological Hormones (6080) - Context Group Name, "1.2.840.10008.6.1.402"
pub const GynecologicalHormones6080: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.402",
    ident: "GynecologicalHormones6080",
    name: "Gynecological Hormones (6080)",
};

/// Breast Cancer Risk Factors (6081) - Context Group Name, "1.2.840.10008.6.1.403"
pub const BreastCancerRiskFactors6081: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.403",
    ident: "BreastCancerRiskFactors6081",
    name: "Breast Cancer Risk Factors (6081)",
};

/// Gynecological Procedures (6082) - Context Group Name, "1.2.840.10008.6.1.404"
pub const GynecologicalProcedures6082: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.404",
    ident: "GynecologicalProcedures6082",
    name: "Gynecological Procedures (6082)",
};

/// Procedures for Breast (6083) - Context Group Name, "1.2.840.10008.6.1.405"
pub const ProceduresForBreast6083: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.405",
    ident: "ProceduresForBreast6083",
    name: "Procedures for Breast (6083)",
};

/// Mammoplasty Procedures (6084) - Context Group Name, "1.2.840.10008.6.1.406"
pub const MammoplastyProcedures6084: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.406",
    ident: "MammoplastyProcedures6084",
    name: "Mammoplasty Procedures (6084)",
};

/// Therapies for Breast (6085) - Context Group Name, "1.2.840.10008.6.1.407"
pub const TherapiesForBreast6085: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.407",
    ident: "TherapiesForBreast6085",
    name: "Therapies for Breast (6085)",
};

/// Menopausal Phase (6086) - Context Group Name, "1.2.840.10008.6.1.408"
pub const MenopausalPhase6086: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.408",
    ident: "MenopausalPhase6086",
    name: "Menopausal Phase (6086)",
};

/// General Risk Factors (6087) - Context Group Name, "1.2.840.10008.6.1.409"
pub const GeneralRiskFactors6087: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.409",
    ident: "GeneralRiskFactors6087",
    name: "General Risk Factors (6087)",
};

/// OB-GYN Maternal Risk Factors (6088) - Context Group Name, "1.2.840.10008.6.1.410"
pub const OBGYNMaternalRiskFactors6088: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.410",
    ident: "OBGYNMaternalRiskFactors6088",
    name: "OB-GYN Maternal Risk Factors (6088)",
};

/// Substances (6089) - Context Group Name, "1.2.840.10008.6.1.411"
pub const Substances6089: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.411",
    ident: "Substances6089",
    name: "Substances (6089)",
};

/// Relative Usage, Exposure Amount (6090) - Context Group Name, "1.2.840.10008.6.1.412"
pub const RelativeUsageExposureAmount6090: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.412",
    ident: "RelativeUsageExposureAmount6090",
    name: "Relative Usage, Exposure Amount (6090)",
};

/// Relative Frequency of Event Values (6091) - Context Group Name, "1.2.840.10008.6.1.413"
pub const RelativeFrequencyOfEventValues6091: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.413",
    ident: "RelativeFrequencyOfEventValues6091",
    name: "Relative Frequency of Event Values (6091)",
};

/// Quantitative Concepts for Usage, Exposure (6092) - Context Group Name, "1.2.840.10008.6.1.414"
pub const QuantitativeConceptsForUsageExposure6092: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.414",
    ident: "QuantitativeConceptsForUsageExposure6092",
    name: "Quantitative Concepts for Usage, Exposure (6092)",
};

/// Qualitative Concepts for Usage, Exposure Amount (6093) - Context Group Name, "1.2.840.10008.6.1.415"
pub const QualitativeConceptsForUsageExposureAmount6093: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.415",
    ident: "QualitativeConceptsForUsageExposureAmount6093",
    name: "Qualitative Concepts for Usage, Exposure Amount (6093)",
};

/// QuaLItative Concepts for Usage, Exposure Frequency (6094) - Context Group Name, "1.2.840.10008.6.1.416"
pub const QuaLItativeConceptsForUsageExposureFrequency6094: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.416",
    ident: "QuaLItativeConceptsForUsageExposureFrequency6094",
    name: "QuaLItative Concepts for Usage, Exposure Frequency (6094)",
};

/// Numeric Properties of Procedures (6095) - Context Group Name, "1.2.840.10008.6.1.417"
pub const NumericPropertiesOfProcedures6095: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.417",
    ident: "NumericPropertiesOfProcedures6095",
    name: "Numeric Properties of Procedures (6095)",
};

/// Pregnancy Status (6096) - Context Group Name, "1.2.840.10008.6.1.418"
pub const PregnancyStatus6096: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.418",
    ident: "PregnancyStatus6096",
    name: "Pregnancy Status (6096)",
};

/// Side of Family (6097) - Context Group Name, "1.2.840.10008.6.1.419"
pub const SideOfFamily6097: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.419",
    ident: "SideOfFamily6097",
    name: "Side of Family (6097)",
};

/// Chest Component Categories (6100) - Context Group Name, "1.2.840.10008.6.1.420"
pub const ChestComponentCategories6100: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.420",
    ident: "ChestComponentCategories6100",
    name: "Chest Component Categories (6100)",
};

/// Chest Finding or Feature (6101) - Context Group Name, "1.2.840.10008.6.1.421"
pub const ChestFindingOrFeature6101: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.421",
    ident: "ChestFindingOrFeature6101",
    name: "Chest Finding or Feature (6101)",
};

/// Chest Finding or Feature Modifier (6102) - Context Group Name, "1.2.840.10008.6.1.422"
pub const ChestFindingOrFeatureModifier6102: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.422",
    ident: "ChestFindingOrFeatureModifier6102",
    name: "Chest Finding or Feature Modifier (6102)",
};

/// Abnormal Lines Finding or Feature (6103) - Context Group Name, "1.2.840.10008.6.1.423"
pub const AbnormalLinesFindingOrFeature6103: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.423",
    ident: "AbnormalLinesFindingOrFeature6103",
    name: "Abnormal Lines Finding or Feature (6103)",
};

/// Abnormal Opacity Finding or Feature (6104) - Context Group Name, "1.2.840.10008.6.1.424"
pub const AbnormalOpacityFindingOrFeature6104: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.424",
    ident: "AbnormalOpacityFindingOrFeature6104",
    name: "Abnormal Opacity Finding or Feature (6104)",
};

/// Abnormal Lucency Finding or Feature (6105) - Context Group Name, "1.2.840.10008.6.1.425"
pub const AbnormalLucencyFindingOrFeature6105: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.425",
    ident: "AbnormalLucencyFindingOrFeature6105",
    name: "Abnormal Lucency Finding or Feature (6105)",
};

/// Abnormal Texture Finding or Feature (6106) - Context Group Name, "1.2.840.10008.6.1.426"
pub const AbnormalTextureFindingOrFeature6106: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.426",
    ident: "AbnormalTextureFindingOrFeature6106",
    name: "Abnormal Texture Finding or Feature (6106)",
};

/// Width Descriptor (6107) - Context Group Name, "1.2.840.10008.6.1.427"
pub const WidthDescriptor6107: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.427",
    ident: "WidthDescriptor6107",
    name: "Width Descriptor (6107)",
};

/// Chest Anatomic Structure Abnormal Distribution (6108) - Context Group Name, "1.2.840.10008.6.1.428"
pub const ChestAnatomicStructureAbnormalDistribution6108: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.428",
    ident: "ChestAnatomicStructureAbnormalDistribution6108",
    name: "Chest Anatomic Structure Abnormal Distribution (6108)",
};

/// Radiographic Anatomy Finding or Feature (6109) - Context Group Name, "1.2.840.10008.6.1.429"
pub const RadiographicAnatomyFindingOrFeature6109: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.429",
    ident: "RadiographicAnatomyFindingOrFeature6109",
    name: "Radiographic Anatomy Finding or Feature (6109)",
};

/// Lung Anatomy Finding or Feature (6110) - Context Group Name, "1.2.840.10008.6.1.430"
pub const LungAnatomyFindingOrFeature6110: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.430",
    ident: "LungAnatomyFindingOrFeature6110",
    name: "Lung Anatomy Finding or Feature (6110)",
};

/// Bronchovascular Anatomy Finding or Feature (6111) - Context Group Name, "1.2.840.10008.6.1.431"
pub const BronchovascularAnatomyFindingOrFeature6111: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.431",
    ident: "BronchovascularAnatomyFindingOrFeature6111",
    name: "Bronchovascular Anatomy Finding or Feature (6111)",
};

/// Pleura Anatomy Finding or Feature (6112) - Context Group Name, "1.2.840.10008.6.1.432"
pub const PleuraAnatomyFindingOrFeature6112: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.432",
    ident: "PleuraAnatomyFindingOrFeature6112",
    name: "Pleura Anatomy Finding or Feature (6112)",
};

/// Mediastinum Anatomy Finding or Feature (6113) - Context Group Name, "1.2.840.10008.6.1.433"
pub const MediastinumAnatomyFindingOrFeature6113: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.433",
    ident: "MediastinumAnatomyFindingOrFeature6113",
    name: "Mediastinum Anatomy Finding or Feature (6113)",
};

/// Osseous Anatomy Finding or Feature (6114) - Context Group Name, "1.2.840.10008.6.1.434"
pub const OsseousAnatomyFindingOrFeature6114: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.434",
    ident: "OsseousAnatomyFindingOrFeature6114",
    name: "Osseous Anatomy Finding or Feature (6114)",
};

/// Osseous Anatomy Modifiers (6115) - Context Group Name, "1.2.840.10008.6.1.435"
pub const OsseousAnatomyModifiers6115: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.435",
    ident: "OsseousAnatomyModifiers6115",
    name: "Osseous Anatomy Modifiers (6115)",
};

/// Muscular Anatomy (6116) - Context Group Name, "1.2.840.10008.6.1.436"
pub const MuscularAnatomy6116: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.436",
    ident: "MuscularAnatomy6116",
    name: "Muscular Anatomy (6116)",
};

/// Vascular Anatomy (6117) - Context Group Name, "1.2.840.10008.6.1.437"
pub const VascularAnatomy6117: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.437",
    ident: "VascularAnatomy6117",
    name: "Vascular Anatomy (6117)",
};

/// Size Descriptor (6118) - Context Group Name, "1.2.840.10008.6.1.438"
pub const SizeDescriptor6118: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.438",
    ident: "SizeDescriptor6118",
    name: "Size Descriptor (6118)",
};

/// Chest Border Shape (6119) - Context Group Name, "1.2.840.10008.6.1.439"
pub const ChestBorderShape6119: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.439",
    ident: "ChestBorderShape6119",
    name: "Chest Border Shape (6119)",
};

/// Chest Border Definition (6120) - Context Group Name, "1.2.840.10008.6.1.440"
pub const ChestBorderDefinition6120: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.440",
    ident: "ChestBorderDefinition6120",
    name: "Chest Border Definition (6120)",
};

/// Chest Orientation Descriptor (6121) - Context Group Name, "1.2.840.10008.6.1.441"
pub const ChestOrientationDescriptor6121: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.441",
    ident: "ChestOrientationDescriptor6121",
    name: "Chest Orientation Descriptor (6121)",
};

/// Chest Content Descriptor (6122) - Context Group Name, "1.2.840.10008.6.1.442"
pub const ChestContentDescriptor6122: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.442",
    ident: "ChestContentDescriptor6122",
    name: "Chest Content Descriptor (6122)",
};

/// Chest Opacity Descriptor (6123) - Context Group Name, "1.2.840.10008.6.1.443"
pub const ChestOpacityDescriptor6123: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.443",
    ident: "ChestOpacityDescriptor6123",
    name: "Chest Opacity Descriptor (6123)",
};

/// Location in Chest (6124) - Context Group Name, "1.2.840.10008.6.1.444"
pub const LocationInChest6124: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.444",
    ident: "LocationInChest6124",
    name: "Location in Chest (6124)",
};

/// General Chest Location (6125) - Context Group Name, "1.2.840.10008.6.1.445"
pub const GeneralChestLocation6125: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.445",
    ident: "GeneralChestLocation6125",
    name: "General Chest Location (6125)",
};

/// Location in Lung (6126) - Context Group Name, "1.2.840.10008.6.1.446"
pub const LocationInLung6126: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.446",
    ident: "LocationInLung6126",
    name: "Location in Lung (6126)",
};

/// Segment Location in Lung (6127) - Context Group Name, "1.2.840.10008.6.1.447"
pub const SegmentLocationInLung6127: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.447",
    ident: "SegmentLocationInLung6127",
    name: "Segment Location in Lung (6127)",
};

/// Chest Distribution Descriptor (6128) - Context Group Name, "1.2.840.10008.6.1.448"
pub const ChestDistributionDescriptor6128: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.448",
    ident: "ChestDistributionDescriptor6128",
    name: "Chest Distribution Descriptor (6128)",
};

/// Chest Site Involvement (6129) - Context Group Name, "1.2.840.10008.6.1.449"
pub const ChestSiteInvolvement6129: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.449",
    ident: "ChestSiteInvolvement6129",
    name: "Chest Site Involvement (6129)",
};

/// Severity Descriptor (6130) - Context Group Name, "1.2.840.10008.6.1.450"
pub const SeverityDescriptor6130: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.450",
    ident: "SeverityDescriptor6130",
    name: "Severity Descriptor (6130)",
};

/// Chest Texture Descriptor (6131) - Context Group Name, "1.2.840.10008.6.1.451"
pub const ChestTextureDescriptor6131: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.451",
    ident: "ChestTextureDescriptor6131",
    name: "Chest Texture Descriptor (6131)",
};

/// Chest Calcification Descriptor (6132) - Context Group Name, "1.2.840.10008.6.1.452"
pub const ChestCalcificationDescriptor6132: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.452",
    ident: "ChestCalcificationDescriptor6132",
    name: "Chest Calcification Descriptor (6132)",
};

/// Chest Quantitative Temporal Difference Type (6133) - Context Group Name, "1.2.840.10008.6.1.453"
pub const ChestQuantitativeTemporalDifferenceType6133: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.453",
    ident: "ChestQuantitativeTemporalDifferenceType6133",
    name: "Chest Quantitative Temporal Difference Type (6133)",
};

/// Qualitative Temporal Difference Type (6134) - Context Group Name, "1.2.840.10008.6.1.454"
pub const QualitativeTemporalDifferenceType6134: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.454",
    ident: "QualitativeTemporalDifferenceType6134",
    name: "Qualitative Temporal Difference Type (6134)",
};

/// Image Quality Finding (6135) - Context Group Name, "1.2.840.10008.6.1.455"
pub const ImageQualityFinding6135: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.455",
    ident: "ImageQualityFinding6135",
    name: "Image Quality Finding (6135)",
};

/// Chest Types of Quality Control Standard (6136) - Context Group Name, "1.2.840.10008.6.1.456"
pub const ChestTypesOfQualityControlStandard6136: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.456",
    ident: "ChestTypesOfQualityControlStandard6136",
    name: "Chest Types of Quality Control Standard (6136)",
};

/// Types of CAD Analysis (6137) - Context Group Name, "1.2.840.10008.6.1.457"
pub const TypesOfCADAnalysis6137: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.457",
    ident: "TypesOfCADAnalysis6137",
    name: "Types of CAD Analysis (6137)",
};

/// Chest Non-Lesion Object Type (6138) - Context Group Name, "1.2.840.10008.6.1.458"
pub const ChestNonLesionObjectType6138: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.458",
    ident: "ChestNonLesionObjectType6138",
    name: "Chest Non-Lesion Object Type (6138)",
};

/// Non-Lesion Modifiers (6139) - Context Group Name, "1.2.840.10008.6.1.459"
pub const NonLesionModifiers6139: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.459",
    ident: "NonLesionModifiers6139",
    name: "Non-Lesion Modifiers (6139)",
};

/// Calculation Methods (6140) - Context Group Name, "1.2.840.10008.6.1.460"
pub const CalculationMethods6140: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.460",
    ident: "CalculationMethods6140",
    name: "Calculation Methods (6140)",
};

/// Attenuation Coefficient Measurements (6141) - Context Group Name, "1.2.840.10008.6.1.461"
pub const AttenuationCoefficientMeasurements6141: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.461",
    ident: "AttenuationCoefficientMeasurements6141",
    name: "Attenuation Coefficient Measurements (6141)",
};

/// Calculated Value (6142) - Context Group Name, "1.2.840.10008.6.1.462"
pub const CalculatedValue6142: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.462",
    ident: "CalculatedValue6142",
    name: "Calculated Value (6142)",
};

/// Response Criteria (6143) - Context Group Name, "1.2.840.10008.6.1.463"
pub const ResponseCriteria6143: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.463",
    ident: "ResponseCriteria6143",
    name: "Response Criteria (6143)",
};

/// RECIST Response Criteria (6144) - Context Group Name, "1.2.840.10008.6.1.464"
pub const RECISTResponseCriteria6144: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.464",
    ident: "RECISTResponseCriteria6144",
    name: "RECIST Response Criteria (6144)",
};

/// Baseline Category (6145) - Context Group Name, "1.2.840.10008.6.1.465"
pub const BaselineCategory6145: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.465",
    ident: "BaselineCategory6145",
    name: "Baseline Category (6145)",
};

/// Background echotexture (6151) - Context Group Name, "1.2.840.10008.6.1.466"
pub const BackgroundEchotexture6151: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.466",
    ident: "BackgroundEchotexture6151",
    name: "Background echotexture (6151)",
};

/// Orientation (6152) - Context Group Name, "1.2.840.10008.6.1.467"
pub const Orientation6152: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.467",
    ident: "Orientation6152",
    name: "Orientation (6152)",
};

/// Lesion boundary (6153) - Context Group Name, "1.2.840.10008.6.1.468"
pub const LesionBoundary6153: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.468",
    ident: "LesionBoundary6153",
    name: "Lesion boundary (6153)",
};

/// Echo pattern (6154) - Context Group Name, "1.2.840.10008.6.1.469"
pub const EchoPattern6154: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.469",
    ident: "EchoPattern6154",
    name: "Echo pattern (6154)",
};

/// Posterior acoustic features (6155) - Context Group Name, "1.2.840.10008.6.1.470"
pub const PosteriorAcousticFeatures6155: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.470",
    ident: "PosteriorAcousticFeatures6155",
    name: "Posterior acoustic features (6155)",
};

/// Vascularity (6157) - Context Group Name, "1.2.840.10008.6.1.471"
pub const Vascularity6157: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.471",
    ident: "Vascularity6157",
    name: "Vascularity (6157)",
};

/// Correlation to Other Findings (6158) - Context Group Name, "1.2.840.10008.6.1.472"
pub const CorrelationToOtherFindings6158: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.472",
    ident: "CorrelationToOtherFindings6158",
    name: "Correlation to Other Findings (6158)",
};

/// Malignancy Type (6159) - Context Group Name, "1.2.840.10008.6.1.473"
pub const MalignancyType6159: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.473",
    ident: "MalignancyType6159",
    name: "Malignancy Type (6159)",
};

/// Breast Primary Tumor Assessment from AJCC (6160) - Context Group Name, "1.2.840.10008.6.1.474"
pub const BreastPrimaryTumorAssessmentFromAJCC6160: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.474",
    ident: "BreastPrimaryTumorAssessmentFromAJCC6160",
    name: "Breast Primary Tumor Assessment from AJCC (6160)",
};

/// Clinical Regional Lymph Node Assessment for Breast (6161) - Context Group Name, "1.2.840.10008.6.1.475"
pub const ClinicalRegionalLymphNodeAssessmentForBreast6161: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.475",
    ident: "ClinicalRegionalLymphNodeAssessmentForBreast6161",
    name: "Clinical Regional Lymph Node Assessment for Breast (6161)",
};

/// Assessment of Metastasis for Breast (6162) - Context Group Name, "1.2.840.10008.6.1.476"
pub const AssessmentOfMetastasisForBreast6162: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.476",
    ident: "AssessmentOfMetastasisForBreast6162",
    name: "Assessment of Metastasis for Breast (6162)",
};

/// Menstrual Cycle Phase (6163) - Context Group Name, "1.2.840.10008.6.1.477"
pub const MenstrualCyclePhase6163: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.477",
    ident: "MenstrualCyclePhase6163",
    name: "Menstrual Cycle Phase (6163)",
};

/// Time Intervals (6164) - Context Group Name, "1.2.840.10008.6.1.478"
pub const TimeIntervals6164: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.478",
    ident: "TimeIntervals6164",
    name: "Time Intervals (6164)",
};

/// Breast Linear Measurements (6165) - Context Group Name, "1.2.840.10008.6.1.479"
pub const BreastLinearMeasurements6165: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.479",
    ident: "BreastLinearMeasurements6165",
    name: "Breast Linear Measurements (6165)",
};

/// CAD Geometry Secondary Graphical Representation (6166) - Context Group Name, "1.2.840.10008.6.1.480"
pub const CADGeometrySecondaryGraphicalRepresentation6166: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.480",
    ident: "CADGeometrySecondaryGraphicalRepresentation6166",
    name: "CAD Geometry Secondary Graphical Representation (6166)",
};

/// Diagnostic Imaging Report Document Titles (7000) - Context Group Name, "1.2.840.10008.6.1.481"
pub const DiagnosticImagingReportDocumentTitles7000: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.481",
    ident: "DiagnosticImagingReportDocumentTitles7000",
    name: "Diagnostic Imaging Report Document Titles (7000)",
};

/// Diagnostic Imaging Report Headings (7001) - Context Group Name, "1.2.840.10008.6.1.482"
pub const DiagnosticImagingReportHeadings7001: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.482",
    ident: "DiagnosticImagingReportHeadings7001",
    name: "Diagnostic Imaging Report Headings (7001)",
};

/// Diagnostic Imaging Report Elements (7002) - Context Group Name, "1.2.840.10008.6.1.483"
pub const DiagnosticImagingReportElements7002: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.483",
    ident: "DiagnosticImagingReportElements7002",
    name: "Diagnostic Imaging Report Elements (7002)",
};

/// Diagnostic Imaging Report Purposes of Reference (7003) - Context Group Name, "1.2.840.10008.6.1.484"
pub const DiagnosticImagingReportPurposesOfReference7003: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.484",
    ident: "DiagnosticImagingReportPurposesOfReference7003",
    name: "Diagnostic Imaging Report Purposes of Reference (7003)",
};

/// Waveform Purposes of Reference (7004) - Context Group Name, "1.2.840.10008.6.1.485"
pub const WaveformPurposesOfReference7004: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.485",
    ident: "WaveformPurposesOfReference7004",
    name: "Waveform Purposes of Reference (7004)",
};

/// Contributing Equipment Purposes of Reference (7005) - Context Group Name, "1.2.840.10008.6.1.486"
pub const ContributingEquipmentPurposesOfReference7005: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.486",
    ident: "ContributingEquipmentPurposesOfReference7005",
    name: "Contributing Equipment Purposes of Reference (7005)",
};

/// SR Document Purposes of Reference (7006) - Context Group Name, "1.2.840.10008.6.1.487"
pub const SRDocumentPurposesOfReference7006: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.487",
    ident: "SRDocumentPurposesOfReference7006",
    name: "SR Document Purposes of Reference (7006)",
};

/// Signature Purpose (7007) - Context Group Name, "1.2.840.10008.6.1.488"
pub const SignaturePurpose7007: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.488",
    ident: "SignaturePurpose7007",
    name: "Signature Purpose (7007)",
};

/// Media Import (7008) - Context Group Name, "1.2.840.10008.6.1.489"
pub const MediaImport7008: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.489",
    ident: "MediaImport7008",
    name: "Media Import (7008)",
};

/// Key Object Selection Document Title (7010) - Context Group Name, "1.2.840.10008.6.1.490"
pub const KeyObjectSelectionDocumentTitle7010: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.490",
    ident: "KeyObjectSelectionDocumentTitle7010",
    name: "Key Object Selection Document Title (7010)",
};

/// Rejected for Quality Reasons (7011) - Context Group Name, "1.2.840.10008.6.1.491"
pub const RejectedForQualityReasons7011: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.491",
    ident: "RejectedForQualityReasons7011",
    name: "Rejected for Quality Reasons (7011)",
};

/// Best In Set (7012) - Context Group Name, "1.2.840.10008.6.1.492"
pub const BestInSet7012: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.492",
    ident: "BestInSet7012",
    name: "Best In Set (7012)",
};

/// Document Titles (7020) - Context Group Name, "1.2.840.10008.6.1.493"
pub const DocumentTitles7020: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.493",
    ident: "DocumentTitles7020",
    name: "Document Titles (7020)",
};

/// RCS Registration Method Type (7100) - Context Group Name, "1.2.840.10008.6.1.494"
pub const RCSRegistrationMethodType7100: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.494",
    ident: "RCSRegistrationMethodType7100",
    name: "RCS Registration Method Type (7100)",
};

/// Brain Atlas Fiducials (7101) - Context Group Name, "1.2.840.10008.6.1.495"
pub const BrainAtlasFiducials7101: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.495",
    ident: "BrainAtlasFiducials7101",
    name: "Brain Atlas Fiducials (7101)",
};

/// Segmentation Property Categories (7150) - Context Group Name, "1.2.840.10008.6.1.496"
pub const SegmentationPropertyCategories7150: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.496",
    ident: "SegmentationPropertyCategories7150",
    name: "Segmentation Property Categories (7150)",
};

/// Segmentation Property Types (7151) - Context Group Name, "1.2.840.10008.6.1.497"
pub const SegmentationPropertyTypes7151: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.497",
    ident: "SegmentationPropertyTypes7151",
    name: "Segmentation Property Types (7151)",
};

/// Cardiac Structure Segmentation Types (7152) - Context Group Name, "1.2.840.10008.6.1.498"
pub const CardiacStructureSegmentationTypes7152: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.498",
    ident: "CardiacStructureSegmentationTypes7152",
    name: "Cardiac Structure Segmentation Types (7152)",
};

/// Brain Tissue Segmentation Types (7153) - Context Group Name, "1.2.840.10008.6.1.499"
pub const BrainTissueSegmentationTypes7153: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.499",
    ident: "BrainTissueSegmentationTypes7153",
    name: "Brain Tissue Segmentation Types (7153)",
};

/// Abdominal Organ Segmentation Types (7154) - Context Group Name, "1.2.840.10008.6.1.500"
pub const AbdominalOrganSegmentationTypes7154: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.500",
    ident: "AbdominalOrganSegmentationTypes7154",
    name: "Abdominal Organ Segmentation Types (7154)",
};

/// Thoracic Tissue Segmentation Types (7155) - Context Group Name, "1.2.840.10008.6.1.501"
pub const ThoracicTissueSegmentationTypes7155: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.501",
    ident: "ThoracicTissueSegmentationTypes7155",
    name: "Thoracic Tissue Segmentation Types (7155)",
};

/// Vascular Tissue Segmentation Types (7156) - Context Group Name, "1.2.840.10008.6.1.502"
pub const VascularTissueSegmentationTypes7156: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.502",
    ident: "VascularTissueSegmentationTypes7156",
    name: "Vascular Tissue Segmentation Types (7156)",
};

/// Device Segmentation Types (7157) - Context Group Name, "1.2.840.10008.6.1.503"
pub const DeviceSegmentationTypes7157: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.503",
    ident: "DeviceSegmentationTypes7157",
    name: "Device Segmentation Types (7157)",
};

/// Artifact Segmentation Types (7158) - Context Group Name, "1.2.840.10008.6.1.504"
pub const ArtifactSegmentationTypes7158: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.504",
    ident: "ArtifactSegmentationTypes7158",
    name: "Artifact Segmentation Types (7158)",
};

/// Lesion Segmentation Types (7159) - Context Group Name, "1.2.840.10008.6.1.505"
pub const LesionSegmentationTypes7159: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.505",
    ident: "LesionSegmentationTypes7159",
    name: "Lesion Segmentation Types (7159)",
};

/// Pelvic Organ Segmentation Types (7160) - Context Group Name, "1.2.840.10008.6.1.506"
pub const PelvicOrganSegmentationTypes7160: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.506",
    ident: "PelvicOrganSegmentationTypes7160",
    name: "Pelvic Organ Segmentation Types (7160)",
};

/// Physiology Segmentation Types (7161) - Context Group Name, "1.2.840.10008.6.1.507"
pub const PhysiologySegmentationTypes7161: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.507",
    ident: "PhysiologySegmentationTypes7161",
    name: "Physiology Segmentation Types (7161)",
};

/// Referenced Image Purposes of Reference (7201) - Context Group Name, "1.2.840.10008.6.1.508"
pub const ReferencedImagePurposesOfReference7201: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.508",
    ident: "ReferencedImagePurposesOfReference7201",
    name: "Referenced Image Purposes of Reference (7201)",
};

/// Source Image Purposes of Reference (7202) - Context Group Name, "1.2.840.10008.6.1.509"
pub const SourceImagePurposesOfReference7202: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.509",
    ident: "SourceImagePurposesOfReference7202",
    name: "Source Image Purposes of Reference (7202)",
};

/// Image Derivation (7203) - Context Group Name, "1.2.840.10008.6.1.510"
pub const ImageDerivation7203: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.510",
    ident: "ImageDerivation7203",
    name: "Image Derivation (7203)",
};

/// Purpose Of Reference to Alternate Representation (7205) - Context Group Name, "1.2.840.10008.6.1.511"
pub const PurposeOfReferenceToAlternateRepresentation7205: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.511",
    ident: "PurposeOfReferenceToAlternateRepresentation7205",
    name: "Purpose Of Reference to Alternate Representation (7205)",
};

/// Related Series Purposes Of Reference (7210) - Context Group Name, "1.2.840.10008.6.1.512"
pub const RelatedSeriesPurposesOfReference7210: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.512",
    ident: "RelatedSeriesPurposesOfReference7210",
    name: "Related Series Purposes Of Reference (7210)",
};

/// Multi-frame Subset Type (7250) - Context Group Name, "1.2.840.10008.6.1.513"
pub const MultiFrameSubsetType7250: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.513",
    ident: "MultiFrameSubsetType7250",
    name: "Multi-frame Subset Type (7250)",
};

/// Person Roles (7450) - Context Group Name, "1.2.840.10008.6.1.514"
pub const PersonRoles7450: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.514",
    ident: "PersonRoles7450",
    name: "Person Roles (7450)",
};

/// Family Member (7451) - Context Group Name, "1.2.840.10008.6.1.515"
pub const FamilyMember7451: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.515",
    ident: "FamilyMember7451",
    name: "Family Member (7451)",
};

/// Organizational Roles (7452) - Context Group Name, "1.2.840.10008.6.1.516"
pub const OrganizationalRoles7452: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.516",
    ident: "OrganizationalRoles7452",
    name: "Organizational Roles (7452)",
};

/// Performing Roles (7453) - Context Group Name, "1.2.840.10008.6.1.517"
pub const PerformingRoles7453: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.517",
    ident: "PerformingRoles7453",
    name: "Performing Roles (7453)",
};

/// Species (7454) - Context Group Name, "1.2.840.10008.6.1.518"
pub const Species7454: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.518",
    ident: "Species7454",
    name: "Species (7454)",
};

/// Sex (7455) - Context Group Name, "1.2.840.10008.6.1.519"
pub const Sex7455: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.519",
    ident: "Sex7455",
    name: "Sex (7455)",
};

/// Units of Measure for Age (7456) - Context Group Name, "1.2.840.10008.6.1.520"
pub const UnitsOfMeasureForAge7456: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.520",
    ident: "UnitsOfMeasureForAge7456",
    name: "Units of Measure for Age (7456)",
};

/// Units of Linear Measurement (7460) - Context Group Name, "1.2.840.10008.6.1.521"
pub const UnitsOfLinearMeasurement7460: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.521",
    ident: "UnitsOfLinearMeasurement7460",
    name: "Units of Linear Measurement (7460)",
};

/// Units of Area Measurement (7461) - Context Group Name, "1.2.840.10008.6.1.522"
pub const UnitsOfAreaMeasurement7461: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.522",
    ident: "UnitsOfAreaMeasurement7461",
    name: "Units of Area Measurement (7461)",
};

/// Units of Volume Measurement (7462) - Context Group Name, "1.2.840.10008.6.1.523"
pub const UnitsOfVolumeMeasurement7462: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.523",
    ident: "UnitsOfVolumeMeasurement7462",
    name: "Units of Volume Measurement (7462)",
};

/// Linear Measurements (7470) - Context Group Name, "1.2.840.10008.6.1.524"
pub const LinearMeasurements7470: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.524",
    ident: "LinearMeasurements7470",
    name: "Linear Measurements (7470)",
};

/// Area Measurements (7471) - Context Group Name, "1.2.840.10008.6.1.525"
pub const AreaMeasurements7471: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.525",
    ident: "AreaMeasurements7471",
    name: "Area Measurements (7471)",
};

/// Volume Measurements (7472) - Context Group Name, "1.2.840.10008.6.1.526"
pub const VolumeMeasurements7472: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.526",
    ident: "VolumeMeasurements7472",
    name: "Volume Measurements (7472)",
};

/// General Area Calculation Methods (7473) - Context Group Name, "1.2.840.10008.6.1.527"
pub const GeneralAreaCalculationMethods7473: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.527",
    ident: "GeneralAreaCalculationMethods7473",
    name: "General Area Calculation Methods (7473)",
};

/// General Volume Calculation Methods (7474) - Context Group Name, "1.2.840.10008.6.1.528"
pub const GeneralVolumeCalculationMethods7474: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.528",
    ident: "GeneralVolumeCalculationMethods7474",
    name: "General Volume Calculation Methods (7474)",
};

/// Breed (7480) - Context Group Name, "1.2.840.10008.6.1.529"
pub const Breed7480: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.529",
    ident: "Breed7480",
    name: "Breed (7480)",
};

/// Breed Registry (7481) - Context Group Name, "1.2.840.10008.6.1.530"
pub const BreedRegistry7481: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.530",
    ident: "BreedRegistry7481",
    name: "Breed Registry (7481)",
};

/// General Purpose Workitem Definition (9231) - Context Group Name, "1.2.840.10008.6.1.531"
pub const GeneralPurposeWorkitemDefinition9231: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.531",
    ident: "GeneralPurposeWorkitemDefinition9231",
    name: "General Purpose Workitem Definition (9231)",
};

/// Non-DICOM Output Types (9232) - Context Group Name, "1.2.840.10008.6.1.532"
pub const NonDICOMOutputTypes9232: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.532",
    ident: "NonDICOMOutputTypes9232",
    name: "Non-DICOM Output Types (9232)",
};

/// Procedure Discontinuation Reasons (9300) - Context Group Name, "1.2.840.10008.6.1.533"
pub const ProcedureDiscontinuationReasons9300: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.533",
    ident: "ProcedureDiscontinuationReasons9300",
    name: "Procedure Discontinuation Reasons (9300)",
};

/// Scope of Accumulation (10000) - Context Group Name, "1.2.840.10008.6.1.534"
pub const ScopeOfAccumulation10000: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.534",
    ident: "ScopeOfAccumulation10000",
    name: "Scope of Accumulation (10000)",
};

/// UID Types (10001) - Context Group Name, "1.2.840.10008.6.1.535"
pub const UIDTypes10001: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.535",
    ident: "UIDTypes10001",
    name: "UID Types (10001)",
};

/// Irradiation Event Types (10002) - Context Group Name, "1.2.840.10008.6.1.536"
pub const IrradiationEventTypes10002: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.536",
    ident: "IrradiationEventTypes10002",
    name: "Irradiation Event Types (10002)",
};

/// Equipment Plane Identification (10003) - Context Group Name, "1.2.840.10008.6.1.537"
pub const EquipmentPlaneIdentification10003: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.537",
    ident: "EquipmentPlaneIdentification10003",
    name: "Equipment Plane Identification (10003)",
};

/// Fluoro Modes (10004) - Context Group Name, "1.2.840.10008.6.1.538"
pub const FluoroModes10004: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.538",
    ident: "FluoroModes10004",
    name: "Fluoro Modes (10004)",
};

/// X-Ray Filter Materials (10006) - Context Group Name, "1.2.840.10008.6.1.539"
pub const XRayFilterMaterials10006: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.539",
    ident: "XRayFilterMaterials10006",
    name: "X-Ray Filter Materials (10006)",
};

/// X-Ray Filter Types (10007) - Context Group Name, "1.2.840.10008.6.1.540"
pub const XRayFilterTypes10007: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.540",
    ident: "XRayFilterTypes10007",
    name: "X-Ray Filter Types (10007)",
};

/// Dose Related Distance Measurements (10008) - Context Group Name, "1.2.840.10008.6.1.541"
pub const DoseRelatedDistanceMeasurements10008: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.541",
    ident: "DoseRelatedDistanceMeasurements10008",
    name: "Dose Related Distance Measurements (10008)",
};

/// Measured/Calculated (10009) - Context Group Name, "1.2.840.10008.6.1.542"
pub const MeasuredCalculated10009: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.542",
    ident: "MeasuredCalculated10009",
    name: "Measured/Calculated (10009)",
};

/// Dose Measurement Devices (10010) - Context Group Name, "1.2.840.10008.6.1.543"
pub const DoseMeasurementDevices10010: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.543",
    ident: "DoseMeasurementDevices10010",
    name: "Dose Measurement Devices (10010)",
};

/// Effective Dose Evaluation Method (10011) - Context Group Name, "1.2.840.10008.6.1.544"
pub const EffectiveDoseEvaluationMethod10011: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.544",
    ident: "EffectiveDoseEvaluationMethod10011",
    name: "Effective Dose Evaluation Method (10011)",
};

/// CT Acquisition Type (10013) - Context Group Name, "1.2.840.10008.6.1.545"
pub const CTAcquisitionType10013: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.545",
    ident: "CTAcquisitionType10013",
    name: "CT Acquisition Type (10013)",
};

/// Contrast Imaging Technique (10014) - Context Group Name, "1.2.840.10008.6.1.546"
pub const ContrastImagingTechnique10014: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.546",
    ident: "ContrastImagingTechnique10014",
    name: "Contrast Imaging Technique (10014)",
};

/// CT Dose Reference Authorities (10015) - Context Group Name, "1.2.840.10008.6.1.547"
pub const CTDoseReferenceAuthorities10015: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.547",
    ident: "CTDoseReferenceAuthorities10015",
    name: "CT Dose Reference Authorities (10015)",
};

/// Anode Target Material (10016) - Context Group Name, "1.2.840.10008.6.1.548"
pub const AnodeTargetMaterial10016: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.548",
    ident: "AnodeTargetMaterial10016",
    name: "Anode Target Material (10016)",
};

/// X-Ray Grid (10017) - Context Group Name, "1.2.840.10008.6.1.549"
pub const XRayGrid10017: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.549",
    ident: "XRayGrid10017",
    name: "X-Ray Grid (10017)",
};

/// Ultrasound Protocol Types (12001) - Context Group Name, "1.2.840.10008.6.1.550"
pub const UltrasoundProtocolTypes12001: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.550",
    ident: "UltrasoundProtocolTypes12001",
    name: "Ultrasound Protocol Types (12001)",
};

/// Ultrasound Protocol Stage Types (12002) - Context Group Name, "1.2.840.10008.6.1.551"
pub const UltrasoundProtocolStageTypes12002: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.551",
    ident: "UltrasoundProtocolStageTypes12002",
    name: "Ultrasound Protocol Stage Types (12002)",
};

/// OB-GYN Dates (12003) - Context Group Name, "1.2.840.10008.6.1.552"
pub const OBGYNDates12003: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.552",
    ident: "OBGYNDates12003",
    name: "OB-GYN Dates (12003)",
};

/// Fetal Biometry Ratios (12004) - Context Group Name, "1.2.840.10008.6.1.553"
pub const FetalBiometryRatios12004: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.553",
    ident: "FetalBiometryRatios12004",
    name: "Fetal Biometry Ratios (12004)",
};

/// Fetal Biometry Measurements (12005) - Context Group Name, "1.2.840.10008.6.1.554"
pub const FetalBiometryMeasurements12005: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.554",
    ident: "FetalBiometryMeasurements12005",
    name: "Fetal Biometry Measurements (12005)",
};

/// Fetal Long Bones Biometry Measurements (12006) - Context Group Name, "1.2.840.10008.6.1.555"
pub const FetalLongBonesBiometryMeasurements12006: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.555",
    ident: "FetalLongBonesBiometryMeasurements12006",
    name: "Fetal Long Bones Biometry Measurements (12006)",
};

/// Fetal Cranium (12007) - Context Group Name, "1.2.840.10008.6.1.556"
pub const FetalCranium12007: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.556",
    ident: "FetalCranium12007",
    name: "Fetal Cranium (12007)",
};

/// OB-GYN Amniotic Sac (12008) - Context Group Name, "1.2.840.10008.6.1.557"
pub const OBGYNAmnioticSac12008: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.557",
    ident: "OBGYNAmnioticSac12008",
    name: "OB-GYN Amniotic Sac (12008)",
};

/// Early Gestation Biometry Measurements (12009) - Context Group Name, "1.2.840.10008.6.1.558"
pub const EarlyGestationBiometryMeasurements12009: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.558",
    ident: "EarlyGestationBiometryMeasurements12009",
    name: "Early Gestation Biometry Measurements (12009)",
};

/// Ultrasound Pelvis and Uterus (12011) - Context Group Name, "1.2.840.10008.6.1.559"
pub const UltrasoundPelvisAndUterus12011: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.559",
    ident: "UltrasoundPelvisAndUterus12011",
    name: "Ultrasound Pelvis and Uterus (12011)",
};

/// OB Equations and Tables (12012) - Context Group Name, "1.2.840.10008.6.1.560"
pub const OBEquationsAndTables12012: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.560",
    ident: "OBEquationsAndTables12012",
    name: "OB Equations and Tables (12012)",
};

/// Gestational Age Equations and Tables (12013) - Context Group Name, "1.2.840.10008.6.1.561"
pub const GestationalAgeEquationsAndTables12013: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.561",
    ident: "GestationalAgeEquationsAndTables12013",
    name: "Gestational Age Equations and Tables (12013)",
};

/// OB Fetal Body Weight Equations and Tables (12014) - Context Group Name, "1.2.840.10008.6.1.562"
pub const OBFetalBodyWeightEquationsAndTables12014: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.562",
    ident: "OBFetalBodyWeightEquationsAndTables12014",
    name: "OB Fetal Body Weight Equations and Tables (12014)",
};

/// Fetal Growth Equations and Tables (12015) - Context Group Name, "1.2.840.10008.6.1.563"
pub const FetalGrowthEquationsAndTables12015: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.563",
    ident: "FetalGrowthEquationsAndTables12015",
    name: "Fetal Growth Equations and Tables (12015)",
};

/// Estimated Fetal Weight Percentile Equations and Tables (12016) - Context Group Name, "1.2.840.10008.6.1.564"
pub const EstimatedFetalWeightPercentileEquationsAndTables12016: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.564",
    ident: "EstimatedFetalWeightPercentileEquationsAndTables12016",
    name: "Estimated Fetal Weight Percentile Equations and Tables (12016)",
};

/// Growth Distribution Rank (12017) - Context Group Name, "1.2.840.10008.6.1.565"
pub const GrowthDistributionRank12017: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.565",
    ident: "GrowthDistributionRank12017",
    name: "Growth Distribution Rank (12017)",
};

/// OB-GYN Summary (12018) - Context Group Name, "1.2.840.10008.6.1.566"
pub const OBGYNSummary12018: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.566",
    ident: "OBGYNSummary12018",
    name: "OB-GYN Summary (12018)",
};

/// OB-GYN Fetus Summary (12019) - Context Group Name, "1.2.840.10008.6.1.567"
pub const OBGYNFetusSummary12019: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.567",
    ident: "OBGYNFetusSummary12019",
    name: "OB-GYN Fetus Summary (12019)",
};

/// Vascular Summary (12101) - Context Group Name, "1.2.840.10008.6.1.568"
pub const VascularSummary12101: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.568",
    ident: "VascularSummary12101",
    name: "Vascular Summary (12101)",
};

/// Temporal Periods Relating to Procedure or Therapy (12102) - Context Group Name, "1.2.840.10008.6.1.569"
pub const TemporalPeriodsRelatingToProcedureOrTherapy12102: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.569",
    ident: "TemporalPeriodsRelatingToProcedureOrTherapy12102",
    name: "Temporal Periods Relating to Procedure or Therapy (12102)",
};

/// Vascular Ultrasound Anatomic Location (12103) - Context Group Name, "1.2.840.10008.6.1.570"
pub const VascularUltrasoundAnatomicLocation12103: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.570",
    ident: "VascularUltrasoundAnatomicLocation12103",
    name: "Vascular Ultrasound Anatomic Location (12103)",
};

/// Extracranial Arteries (12104) - Context Group Name, "1.2.840.10008.6.1.571"
pub const ExtracranialArteries12104: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.571",
    ident: "ExtracranialArteries12104",
    name: "Extracranial Arteries (12104)",
};

/// Intracranial Cerebral Vessels (12105) - Context Group Name, "1.2.840.10008.6.1.572"
pub const IntracranialCerebralVessels12105: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.572",
    ident: "IntracranialCerebralVessels12105",
    name: "Intracranial Cerebral Vessels (12105)",
};

/// Intracranial Cerebral Vessels (unilateral) (12106) - Context Group Name, "1.2.840.10008.6.1.573"
pub const IntracranialCerebralVesselsUnilateral12106: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.573",
    ident: "IntracranialCerebralVesselsUnilateral12106",
    name: "Intracranial Cerebral Vessels (unilateral) (12106)",
};

/// Upper Extremity Arteries (12107) - Context Group Name, "1.2.840.10008.6.1.574"
pub const UpperExtremityArteries12107: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.574",
    ident: "UpperExtremityArteries12107",
    name: "Upper Extremity Arteries (12107)",
};

/// Upper Extremity Veins (12108) - Context Group Name, "1.2.840.10008.6.1.575"
pub const UpperExtremityVeins12108: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.575",
    ident: "UpperExtremityVeins12108",
    name: "Upper Extremity Veins (12108)",
};

/// Lower Extremity Arteries (12109) - Context Group Name, "1.2.840.10008.6.1.576"
pub const LowerExtremityArteries12109: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.576",
    ident: "LowerExtremityArteries12109",
    name: "Lower Extremity Arteries (12109)",
};

/// Lower Extremity Veins (12110) - Context Group Name, "1.2.840.10008.6.1.577"
pub const LowerExtremityVeins12110: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.577",
    ident: "LowerExtremityVeins12110",
    name: "Lower Extremity Veins (12110)",
};

/// Abdominal Arteries (lateral) (12111) - Context Group Name, "1.2.840.10008.6.1.578"
pub const AbdominalArteriesLateral12111: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.578",
    ident: "AbdominalArteriesLateral12111",
    name: "Abdominal Arteries (lateral) (12111)",
};

/// Abdominal Arteries (unilateral) (12112) - Context Group Name, "1.2.840.10008.6.1.579"
pub const AbdominalArteriesUnilateral12112: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.579",
    ident: "AbdominalArteriesUnilateral12112",
    name: "Abdominal Arteries (unilateral) (12112)",
};

/// Abdominal Veins (lateral) (12113) - Context Group Name, "1.2.840.10008.6.1.580"
pub const AbdominalVeinsLateral12113: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.580",
    ident: "AbdominalVeinsLateral12113",
    name: "Abdominal Veins (lateral) (12113)",
};

/// Abdominal Veins (unilateral) (12114) - Context Group Name, "1.2.840.10008.6.1.581"
pub const AbdominalVeinsUnilateral12114: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.581",
    ident: "AbdominalVeinsUnilateral12114",
    name: "Abdominal Veins (unilateral) (12114)",
};

/// Renal Vessels (12115) - Context Group Name, "1.2.840.10008.6.1.582"
pub const RenalVessels12115: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.582",
    ident: "RenalVessels12115",
    name: "Renal Vessels (12115)",
};

/// Vessel Segment Modifiers (12116) - Context Group Name, "1.2.840.10008.6.1.583"
pub const VesselSegmentModifiers12116: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.583",
    ident: "VesselSegmentModifiers12116",
    name: "Vessel Segment Modifiers (12116)",
};

/// Vessel Branch Modifiers (12117) - Context Group Name, "1.2.840.10008.6.1.584"
pub const VesselBranchModifiers12117: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.584",
    ident: "VesselBranchModifiers12117",
    name: "Vessel Branch Modifiers (12117)",
};

/// Vascular Ultrasound Property (12119) - Context Group Name, "1.2.840.10008.6.1.585"
pub const VascularUltrasoundProperty12119: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.585",
    ident: "VascularUltrasoundProperty12119",
    name: "Vascular Ultrasound Property (12119)",
};

/// Blood Velocity Measurements by Ultrasound (12120) - Context Group Name, "1.2.840.10008.6.1.586"
pub const BloodVelocityMeasurementsByUltrasound12120: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.586",
    ident: "BloodVelocityMeasurementsByUltrasound12120",
    name: "Blood Velocity Measurements by Ultrasound (12120)",
};

/// Vascular Indices and Ratios (12121) - Context Group Name, "1.2.840.10008.6.1.587"
pub const VascularIndicesAndRatios12121: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.587",
    ident: "VascularIndicesAndRatios12121",
    name: "Vascular Indices and Ratios (12121)",
};

/// Other Vascular Properties (12122) - Context Group Name, "1.2.840.10008.6.1.588"
pub const OtherVascularProperties12122: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.588",
    ident: "OtherVascularProperties12122",
    name: "Other Vascular Properties (12122)",
};

/// Carotid Ratios (12123) - Context Group Name, "1.2.840.10008.6.1.589"
pub const CarotidRatios12123: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.589",
    ident: "CarotidRatios12123",
    name: "Carotid Ratios (12123)",
};

/// Renal Ratios (12124) - Context Group Name, "1.2.840.10008.6.1.590"
pub const RenalRatios12124: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.590",
    ident: "RenalRatios12124",
    name: "Renal Ratios (12124)",
};

/// Pelvic Vasculature Anatomical Location (12140) - Context Group Name, "1.2.840.10008.6.1.591"
pub const PelvicVasculatureAnatomicalLocation12140: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.591",
    ident: "PelvicVasculatureAnatomicalLocation12140",
    name: "Pelvic Vasculature Anatomical Location (12140)",
};

/// Fetal Vasculature Anatomical Location (12141) - Context Group Name, "1.2.840.10008.6.1.592"
pub const FetalVasculatureAnatomicalLocation12141: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.592",
    ident: "FetalVasculatureAnatomicalLocation12141",
    name: "Fetal Vasculature Anatomical Location (12141)",
};

/// Echocardiography Left Ventricle (12200) - Context Group Name, "1.2.840.10008.6.1.593"
pub const EchocardiographyLeftVentricle12200: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.593",
    ident: "EchocardiographyLeftVentricle12200",
    name: "Echocardiography Left Ventricle (12200)",
};

/// Left Ventricle Linear (12201) - Context Group Name, "1.2.840.10008.6.1.594"
pub const LeftVentricleLinear12201: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.594",
    ident: "LeftVentricleLinear12201",
    name: "Left Ventricle Linear (12201)",
};

/// Left Ventricle Volume (12202) - Context Group Name, "1.2.840.10008.6.1.595"
pub const LeftVentricleVolume12202: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.595",
    ident: "LeftVentricleVolume12202",
    name: "Left Ventricle Volume (12202)",
};

/// Left Ventricle Other (12203) - Context Group Name, "1.2.840.10008.6.1.596"
pub const LeftVentricleOther12203: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.596",
    ident: "LeftVentricleOther12203",
    name: "Left Ventricle Other (12203)",
};

/// Echocardiography Right Ventricle (12204) - Context Group Name, "1.2.840.10008.6.1.597"
pub const EchocardiographyRightVentricle12204: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.597",
    ident: "EchocardiographyRightVentricle12204",
    name: "Echocardiography Right Ventricle (12204)",
};

/// Echocardiography Left Atrium (12205) - Context Group Name, "1.2.840.10008.6.1.598"
pub const EchocardiographyLeftAtrium12205: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.598",
    ident: "EchocardiographyLeftAtrium12205",
    name: "Echocardiography Left Atrium (12205)",
};

/// Echocardiography Right Atrium (12206) - Context Group Name, "1.2.840.10008.6.1.599"
pub const EchocardiographyRightAtrium12206: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.599",
    ident: "EchocardiographyRightAtrium12206",
    name: "Echocardiography Right Atrium (12206)",
};

/// Echocardiography Mitral Valve (12207) - Context Group Name, "1.2.840.10008.6.1.600"
pub const EchocardiographyMitralValve12207: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.600",
    ident: "EchocardiographyMitralValve12207",
    name: "Echocardiography Mitral Valve (12207)",
};

/// Echocardiography Tricuspid Valve (12208) - Context Group Name, "1.2.840.10008.6.1.601"
pub const EchocardiographyTricuspidValve12208: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.601",
    ident: "EchocardiographyTricuspidValve12208",
    name: "Echocardiography Tricuspid Valve (12208)",
};

/// Echocardiography Pulmonic Valve (12209) - Context Group Name, "1.2.840.10008.6.1.602"
pub const EchocardiographyPulmonicValve12209: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.602",
    ident: "EchocardiographyPulmonicValve12209",
    name: "Echocardiography Pulmonic Valve (12209)",
};

/// Echocardiography Pulmonary Artery (12210) - Context Group Name, "1.2.840.10008.6.1.603"
pub const EchocardiographyPulmonaryArtery12210: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.603",
    ident: "EchocardiographyPulmonaryArtery12210",
    name: "Echocardiography Pulmonary Artery (12210)",
};

/// Echocardiography Aortic Valve (12211) - Context Group Name, "1.2.840.10008.6.1.604"
pub const EchocardiographyAorticValve12211: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.604",
    ident: "EchocardiographyAorticValve12211",
    name: "Echocardiography Aortic Valve (12211)",
};

/// Echocardiography Aorta (12212) - Context Group Name, "1.2.840.10008.6.1.605"
pub const EchocardiographyAorta12212: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.605",
    ident: "EchocardiographyAorta12212",
    name: "Echocardiography Aorta (12212)",
};

/// Echocardiography Pulmonary Veins (12214) - Context Group Name, "1.2.840.10008.6.1.606"
pub const EchocardiographyPulmonaryVeins12214: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.606",
    ident: "EchocardiographyPulmonaryVeins12214",
    name: "Echocardiography Pulmonary Veins (12214)",
};

/// Echocardiography Vena Cavae (12215) - Context Group Name, "1.2.840.10008.6.1.607"
pub const EchocardiographyVenaCavae12215: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.607",
    ident: "EchocardiographyVenaCavae12215",
    name: "Echocardiography Vena Cavae (12215)",
};

/// Echocardiography Hepatic Veins (12216) - Context Group Name, "1.2.840.10008.6.1.608"
pub const EchocardiographyHepaticVeins12216: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.608",
    ident: "EchocardiographyHepaticVeins12216",
    name: "Echocardiography Hepatic Veins (12216)",
};

/// Echocardiography Cardiac Shunt (12217) - Context Group Name, "1.2.840.10008.6.1.609"
pub const EchocardiographyCardiacShunt12217: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.609",
    ident: "EchocardiographyCardiacShunt12217",
    name: "Echocardiography Cardiac Shunt (12217)",
};

/// Echocardiography Congenital (12218) - Context Group Name, "1.2.840.10008.6.1.610"
pub const EchocardiographyCongenital12218: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.610",
    ident: "EchocardiographyCongenital12218",
    name: "Echocardiography Congenital (12218)",
};

/// Pulmonary Vein Modifiers (12219) - Context Group Name, "1.2.840.10008.6.1.611"
pub const PulmonaryVeinModifiers12219: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.611",
    ident: "PulmonaryVeinModifiers12219",
    name: "Pulmonary Vein Modifiers (12219)",
};

/// Echocardiography Common Measurements (12220) - Context Group Name, "1.2.840.10008.6.1.612"
pub const EchocardiographyCommonMeasurements12220: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.612",
    ident: "EchocardiographyCommonMeasurements12220",
    name: "Echocardiography Common Measurements (12220)",
};

/// Flow Direction (12221) - Context Group Name, "1.2.840.10008.6.1.613"
pub const FlowDirection12221: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.613",
    ident: "FlowDirection12221",
    name: "Flow Direction (12221)",
};

/// Orifice Flow Properties (12222) - Context Group Name, "1.2.840.10008.6.1.614"
pub const OrificeFlowProperties12222: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.614",
    ident: "OrificeFlowProperties12222",
    name: "Orifice Flow Properties (12222)",
};

/// Echocardiography Stroke Volume Origin (12223) - Context Group Name, "1.2.840.10008.6.1.615"
pub const EchocardiographyStrokeVolumeOrigin12223: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.615",
    ident: "EchocardiographyStrokeVolumeOrigin12223",
    name: "Echocardiography Stroke Volume Origin (12223)",
};

/// Ultrasound Image Modes (12224) - Context Group Name, "1.2.840.10008.6.1.616"
pub const UltrasoundImageModes12224: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.616",
    ident: "UltrasoundImageModes12224",
    name: "Ultrasound Image Modes (12224)",
};

/// Echocardiography Image View (12226) - Context Group Name, "1.2.840.10008.6.1.617"
pub const EchocardiographyImageView12226: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.617",
    ident: "EchocardiographyImageView12226",
    name: "Echocardiography Image View (12226)",
};

/// Echocardiography Measurement Method (12227) - Context Group Name, "1.2.840.10008.6.1.618"
pub const EchocardiographyMeasurementMethod12227: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.618",
    ident: "EchocardiographyMeasurementMethod12227",
    name: "Echocardiography Measurement Method (12227)",
};

/// Echocardiography Volume Methods (12228) - Context Group Name, "1.2.840.10008.6.1.619"
pub const EchocardiographyVolumeMethods12228: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.619",
    ident: "EchocardiographyVolumeMethods12228",
    name: "Echocardiography Volume Methods (12228)",
};

/// Echocardiography Area Methods (12229) - Context Group Name, "1.2.840.10008.6.1.620"
pub const EchocardiographyAreaMethods12229: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.620",
    ident: "EchocardiographyAreaMethods12229",
    name: "Echocardiography Area Methods (12229)",
};

/// Gradient Methods (12230) - Context Group Name, "1.2.840.10008.6.1.621"
pub const GradientMethods12230: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.621",
    ident: "GradientMethods12230",
    name: "Gradient Methods (12230)",
};

/// Volume Flow Methods (12231) - Context Group Name, "1.2.840.10008.6.1.622"
pub const VolumeFlowMethods12231: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.622",
    ident: "VolumeFlowMethods12231",
    name: "Volume Flow Methods (12231)",
};

/// Myocardium Mass Methods (12232) - Context Group Name, "1.2.840.10008.6.1.623"
pub const MyocardiumMassMethods12232: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.623",
    ident: "MyocardiumMassMethods12232",
    name: "Myocardium Mass Methods (12232)",
};

/// Cardiac Phase (12233) - Context Group Name, "1.2.840.10008.6.1.624"
pub const CardiacPhase12233: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.624",
    ident: "CardiacPhase12233",
    name: "Cardiac Phase (12233)",
};

/// Respiration State (12234) - Context Group Name, "1.2.840.10008.6.1.625"
pub const RespirationState12234: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.625",
    ident: "RespirationState12234",
    name: "Respiration State (12234)",
};

/// Mitral Valve Anatomic Sites (12235) - Context Group Name, "1.2.840.10008.6.1.626"
pub const MitralValveAnatomicSites12235: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.626",
    ident: "MitralValveAnatomicSites12235",
    name: "Mitral Valve Anatomic Sites (12235)",
};

/// Echo Anatomic Sites (12236) - Context Group Name, "1.2.840.10008.6.1.627"
pub const EchoAnatomicSites12236: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.627",
    ident: "EchoAnatomicSites12236",
    name: "Echo Anatomic Sites (12236)",
};

/// Echocardiography Anatomic Site Modifiers (12237) - Context Group Name, "1.2.840.10008.6.1.628"
pub const EchocardiographyAnatomicSiteModifiers12237: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.628",
    ident: "EchocardiographyAnatomicSiteModifiers12237",
    name: "Echocardiography Anatomic Site Modifiers (12237)",
};

/// Wall Motion Scoring Schemes (12238) - Context Group Name, "1.2.840.10008.6.1.629"
pub const WallMotionScoringSchemes12238: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.629",
    ident: "WallMotionScoringSchemes12238",
    name: "Wall Motion Scoring Schemes (12238)",
};

/// Cardiac Output Properties (12239) - Context Group Name, "1.2.840.10008.6.1.630"
pub const CardiacOutputProperties12239: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.630",
    ident: "CardiacOutputProperties12239",
    name: "Cardiac Output Properties (12239)",
};

/// Left Ventricle Area (12240) - Context Group Name, "1.2.840.10008.6.1.631"
pub const LeftVentricleArea12240: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.631",
    ident: "LeftVentricleArea12240",
    name: "Left Ventricle Area (12240)",
};

/// Tricuspid Valve Finding Sites (12241) - Context Group Name, "1.2.840.10008.6.1.632"
pub const TricuspidValveFindingSites12241: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.632",
    ident: "TricuspidValveFindingSites12241",
    name: "Tricuspid Valve Finding Sites (12241)",
};

/// Aortic Valve Finding Sites (12242) - Context Group Name, "1.2.840.10008.6.1.633"
pub const AorticValveFindingSites12242: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.633",
    ident: "AorticValveFindingSites12242",
    name: "Aortic Valve Finding Sites (12242)",
};

/// Left Ventricle Finding Sites (12243) - Context Group Name, "1.2.840.10008.6.1.634"
pub const LeftVentricleFindingSites12243: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.634",
    ident: "LeftVentricleFindingSites12243",
    name: "Left Ventricle Finding Sites (12243)",
};

/// Congenital Finding Sites (12244) - Context Group Name, "1.2.840.10008.6.1.635"
pub const CongenitalFindingSites12244: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.635",
    ident: "CongenitalFindingSites12244",
    name: "Congenital Finding Sites (12244)",
};

/// Surface Processing Algorithm Families (7162) - Context Group Name, "1.2.840.10008.6.1.636"
pub const SurfaceProcessingAlgorithmFamilies7162: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.636",
    ident: "SurfaceProcessingAlgorithmFamilies7162",
    name: "Surface Processing Algorithm Families (7162)",
};

/// Stress Test Procedure Phases (3207) - Context Group Name, "1.2.840.10008.6.1.637"
pub const StressTestProcedurePhases3207: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.637",
    ident: "StressTestProcedurePhases3207",
    name: "Stress Test Procedure Phases (3207)",
};

/// Stages (3778) - Context Group Name, "1.2.840.10008.6.1.638"
pub const Stages3778: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.638",
    ident: "Stages3778",
    name: "Stages (3778)",
};

/// S-M-L Size Descriptor (252) - Context Group Name, "1.2.840.10008.6.1.735"
pub const SMLSizeDescriptor252: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.735",
    ident: "SMLSizeDescriptor252",
    name: "S-M-L Size Descriptor (252)",
};

/// Major Coronary Arteries (3016) - Context Group Name, "1.2.840.10008.6.1.736"
pub const MajorCoronaryArteries3016: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.736",
    ident: "MajorCoronaryArteries3016",
    name: "Major Coronary Arteries (3016)",
};

/// Units of Radioactivity (3083) - Context Group Name, "1.2.840.10008.6.1.737"
pub const UnitsOfRadioactivity3083: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.737",
    ident: "UnitsOfRadioactivity3083",
    name: "Units of Radioactivity (3083)",
};

/// Rest-Stress (3102) - Context Group Name, "1.2.840.10008.6.1.738"
pub const RestStress3102: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.738",
    ident: "RestStress3102",
    name: "Rest-Stress (3102)",
};

/// PET Cardiology Protocols (3106) - Context Group Name, "1.2.840.10008.6.1.739"
pub const PETCardiologyProtocols3106: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.739",
    ident: "PETCardiologyProtocols3106",
    name: "PET Cardiology Protocols (3106)",
};

/// PET Cardiology Radiopharmaceuticals (3107) - Context Group Name, "1.2.840.10008.6.1.740"
pub const PETCardiologyRadiopharmaceuticals3107: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.740",
    ident: "PETCardiologyRadiopharmaceuticals3107",
    name: "PET Cardiology Radiopharmaceuticals (3107)",
};

/// NM/PET Procedures (3108) - Context Group Name, "1.2.840.10008.6.1.741"
pub const NMPETProcedures3108: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.741",
    ident: "NMPETProcedures3108",
    name: "NM/PET Procedures (3108)",
};

/// Nuclear Cardiology Protocols (3110) - Context Group Name, "1.2.840.10008.6.1.742"
pub const NuclearCardiologyProtocols3110: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.742",
    ident: "NuclearCardiologyProtocols3110",
    name: "Nuclear Cardiology Protocols (3110)",
};

/// Nuclear Cardiology Radiopharmaceuticals (3111) - Context Group Name, "1.2.840.10008.6.1.743"
pub const NuclearCardiologyRadiopharmaceuticals3111: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.743",
    ident: "NuclearCardiologyRadiopharmaceuticals3111",
    name: "Nuclear Cardiology Radiopharmaceuticals (3111)",
};

/// Attenuation Correction (3112) - Context Group Name, "1.2.840.10008.6.1.744"
pub const AttenuationCorrection3112: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.744",
    ident: "AttenuationCorrection3112",
    name: "Attenuation Correction (3112)",
};

/// Types of Perfusion Defects (3113) - Context Group Name, "1.2.840.10008.6.1.745"
pub const TypesOfPerfusionDefects3113: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.745",
    ident: "TypesOfPerfusionDefects3113",
    name: "Types of Perfusion Defects (3113)",
};

/// Study Quality (3114) - Context Group Name, "1.2.840.10008.6.1.746"
pub const StudyQuality3114: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.746",
    ident: "StudyQuality3114",
    name: "Study Quality (3114)",
};

/// Stress Imaging Quality Issues (3115) - Context Group Name, "1.2.840.10008.6.1.747"
pub const StressImagingQualityIssues3115: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.747",
    ident: "StressImagingQualityIssues3115",
    name: "Stress Imaging Quality Issues (3115)",
};

/// NM Extracardiac Findings (3116) - Context Group Name, "1.2.840.10008.6.1.748"
pub const NMExtracardiacFindings3116: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.748",
    ident: "NMExtracardiacFindings3116",
    name: "NM Extracardiac Findings (3116)",
};

/// Attenuation Correction Methods (3117) - Context Group Name, "1.2.840.10008.6.1.749"
pub const AttenuationCorrectionMethods3117: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.749",
    ident: "AttenuationCorrectionMethods3117",
    name: "Attenuation Correction Methods (3117)",
};

/// Level of Risk (3118) - Context Group Name, "1.2.840.10008.6.1.750"
pub const LevelOfRisk3118: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.750",
    ident: "LevelOfRisk3118",
    name: "Level of Risk (3118)",
};

/// LV Function (3119) - Context Group Name, "1.2.840.10008.6.1.751"
pub const LVFunction3119: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.751",
    ident: "LVFunction3119",
    name: "LV Function (3119)",
};

/// Perfusion Findings (3120) - Context Group Name, "1.2.840.10008.6.1.752"
pub const PerfusionFindings3120: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.752",
    ident: "PerfusionFindings3120",
    name: "Perfusion Findings (3120)",
};

/// Perfusion Morphology (3121) - Context Group Name, "1.2.840.10008.6.1.753"
pub const PerfusionMorphology3121: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.753",
    ident: "PerfusionMorphology3121",
    name: "Perfusion Morphology (3121)",
};

/// Ventricular Enlargement (3122) - Context Group Name, "1.2.840.10008.6.1.754"
pub const VentricularEnlargement3122: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.754",
    ident: "VentricularEnlargement3122",
    name: "Ventricular Enlargement (3122)",
};

/// Stress Test Procedure (3200) - Context Group Name, "1.2.840.10008.6.1.755"
pub const StressTestProcedure3200: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.755",
    ident: "StressTestProcedure3200",
    name: "Stress Test Procedure (3200)",
};

/// Indications for Stress Test (3201) - Context Group Name, "1.2.840.10008.6.1.756"
pub const IndicationsForStressTest3201: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.756",
    ident: "IndicationsForStressTest3201",
    name: "Indications for Stress Test (3201)",
};

/// Chest Pain (3202) - Context Group Name, "1.2.840.10008.6.1.757"
pub const ChestPain3202: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.757",
    ident: "ChestPain3202",
    name: "Chest Pain (3202)",
};

/// Exerciser Device (3203) - Context Group Name, "1.2.840.10008.6.1.758"
pub const ExerciserDevice3203: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.758",
    ident: "ExerciserDevice3203",
    name: "Exerciser Device (3203)",
};

/// Stress Agents (3204) - Context Group Name, "1.2.840.10008.6.1.759"
pub const StressAgents3204: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.759",
    ident: "StressAgents3204",
    name: "Stress Agents (3204)",
};

/// Indications for Pharmacological Stress Test (3205) - Context Group Name, "1.2.840.10008.6.1.760"
pub const IndicationsForPharmacologicalStressTest3205: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.760",
    ident: "IndicationsForPharmacologicalStressTest3205",
    name: "Indications for Pharmacological Stress Test (3205)",
};

/// Non-invasive Cardiac Imaging Procedures (3206) - Context Group Name, "1.2.840.10008.6.1.761"
pub const NonInvasiveCardiacImagingProcedures3206: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.761",
    ident: "NonInvasiveCardiacImagingProcedures3206",
    name: "Non-invasive Cardiac Imaging Procedures (3206)",
};

/// Summary Codes Exercise ECG (3208) - Context Group Name, "1.2.840.10008.6.1.763"
pub const SummaryCodesExerciseECG3208: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.763",
    ident: "SummaryCodesExerciseECG3208",
    name: "Summary Codes Exercise ECG (3208)",
};

/// Summary Codes Stress Imaging (3209) - Context Group Name, "1.2.840.10008.6.1.764"
pub const SummaryCodesStressImaging3209: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.764",
    ident: "SummaryCodesStressImaging3209",
    name: "Summary Codes Stress Imaging (3209)",
};

/// Speed of Response (3210) - Context Group Name, "1.2.840.10008.6.1.765"
pub const SpeedOfResponse3210: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.765",
    ident: "SpeedOfResponse3210",
    name: "Speed of Response (3210)",
};

/// BP Response (3211) - Context Group Name, "1.2.840.10008.6.1.766"
pub const BPResponse3211: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.766",
    ident: "BPResponse3211",
    name: "BP Response (3211)",
};

/// Treadmill Speed (3212) - Context Group Name, "1.2.840.10008.6.1.767"
pub const TreadmillSpeed3212: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.767",
    ident: "TreadmillSpeed3212",
    name: "Treadmill Speed (3212)",
};

/// Stress Hemodynamic Findings (3213) - Context Group Name, "1.2.840.10008.6.1.768"
pub const StressHemodynamicFindings3213: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.768",
    ident: "StressHemodynamicFindings3213",
    name: "Stress Hemodynamic Findings (3213)",
};

/// Perfusion Finding Method (3215) - Context Group Name, "1.2.840.10008.6.1.769"
pub const PerfusionFindingMethod3215: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.769",
    ident: "PerfusionFindingMethod3215",
    name: "Perfusion Finding Method (3215)",
};

/// Comparison Finding (3217) - Context Group Name, "1.2.840.10008.6.1.770"
pub const ComparisonFinding3217: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.770",
    ident: "ComparisonFinding3217",
    name: "Comparison Finding (3217)",
};

/// Stress Symptoms (3220) - Context Group Name, "1.2.840.10008.6.1.771"
pub const StressSymptoms3220: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.771",
    ident: "StressSymptoms3220",
    name: "Stress Symptoms (3220)",
};

/// Stress Test Termination Reasons (3221) - Context Group Name, "1.2.840.10008.6.1.772"
pub const StressTestTerminationReasons3221: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.772",
    ident: "StressTestTerminationReasons3221",
    name: "Stress Test Termination Reasons (3221)",
};

/// QTc Measurements (3227) - Context Group Name, "1.2.840.10008.6.1.773"
pub const QTcMeasurements3227: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.773",
    ident: "QTcMeasurements3227",
    name: "QTc Measurements (3227)",
};

/// ECG Timing Measurements (3228) - Context Group Name, "1.2.840.10008.6.1.774"
pub const ECGTimingMeasurements3228: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.774",
    ident: "ECGTimingMeasurements3228",
    name: "ECG Timing Measurements (3228)",
};

/// ECG Axis Measurements (3229) - Context Group Name, "1.2.840.10008.6.1.775"
pub const ECGAxisMeasurements3229: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.775",
    ident: "ECGAxisMeasurements3229",
    name: "ECG Axis Measurements (3229)",
};

/// ECG Findings (3230) - Context Group Name, "1.2.840.10008.6.1.776"
pub const ECGFindings3230: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.776",
    ident: "ECGFindings3230",
    name: "ECG Findings (3230)",
};

/// ST Segment Findings (3231) - Context Group Name, "1.2.840.10008.6.1.777"
pub const STSegmentFindings3231: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.777",
    ident: "STSegmentFindings3231",
    name: "ST Segment Findings (3231)",
};

/// ST Segment Location (3232) - Context Group Name, "1.2.840.10008.6.1.778"
pub const STSegmentLocation3232: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.778",
    ident: "STSegmentLocation3232",
    name: "ST Segment Location (3232)",
};

/// ST Segment Morphology (3233) - Context Group Name, "1.2.840.10008.6.1.779"
pub const STSegmentMorphology3233: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.779",
    ident: "STSegmentMorphology3233",
    name: "ST Segment Morphology (3233)",
};

/// Ectopic Beat Morphology (3234) - Context Group Name, "1.2.840.10008.6.1.780"
pub const EctopicBeatMorphology3234: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.780",
    ident: "EctopicBeatMorphology3234",
    name: "Ectopic Beat Morphology (3234)",
};

/// Perfusion Comparison Findings (3235) - Context Group Name, "1.2.840.10008.6.1.781"
pub const PerfusionComparisonFindings3235: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.781",
    ident: "PerfusionComparisonFindings3235",
    name: "Perfusion Comparison Findings (3235)",
};

/// Tolerance Comparison Findings (3236) - Context Group Name, "1.2.840.10008.6.1.782"
pub const ToleranceComparisonFindings3236: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.782",
    ident: "ToleranceComparisonFindings3236",
    name: "Tolerance Comparison Findings (3236)",
};

/// Wall Motion Comparison Findings (3237) - Context Group Name, "1.2.840.10008.6.1.783"
pub const WallMotionComparisonFindings3237: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.783",
    ident: "WallMotionComparisonFindings3237",
    name: "Wall Motion Comparison Findings (3237)",
};

/// Stress Scoring Scales (3238) - Context Group Name, "1.2.840.10008.6.1.784"
pub const StressScoringScales3238: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.784",
    ident: "StressScoringScales3238",
    name: "Stress Scoring Scales (3238)",
};

/// Perceived Exertion Scales (3239) - Context Group Name, "1.2.840.10008.6.1.785"
pub const PerceivedExertionScales3239: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.785",
    ident: "PerceivedExertionScales3239",
    name: "Perceived Exertion Scales (3239)",
};

/// Ventricle Identification (3463) - Context Group Name, "1.2.840.10008.6.1.786"
pub const VentricleIdentification3463: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.786",
    ident: "VentricleIdentification3463",
    name: "Ventricle Identification (3463)",
};

/// Colon Overall Assessment (6200) - Context Group Name, "1.2.840.10008.6.1.787"
pub const ColonOverallAssessment6200: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.787",
    ident: "ColonOverallAssessment6200",
    name: "Colon Overall Assessment (6200)",
};

/// Colon Finding or Feature (6201) - Context Group Name, "1.2.840.10008.6.1.788"
pub const ColonFindingOrFeature6201: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.788",
    ident: "ColonFindingOrFeature6201",
    name: "Colon Finding or Feature (6201)",
};

/// Colon Finding or Feature Modifier (6202) - Context Group Name, "1.2.840.10008.6.1.789"
pub const ColonFindingOrFeatureModifier6202: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.789",
    ident: "ColonFindingOrFeatureModifier6202",
    name: "Colon Finding or Feature Modifier (6202)",
};

/// Colon Non-Lesion Object Type (6203) - Context Group Name, "1.2.840.10008.6.1.790"
pub const ColonNonLesionObjectType6203: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.790",
    ident: "ColonNonLesionObjectType6203",
    name: "Colon Non-Lesion Object Type (6203)",
};

/// Anatomic Non-Colon Findings (6204) - Context Group Name, "1.2.840.10008.6.1.791"
pub const AnatomicNonColonFindings6204: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.791",
    ident: "AnatomicNonColonFindings6204",
    name: "Anatomic Non-Colon Findings (6204)",
};

/// Clockface Location for Colon (6205) - Context Group Name, "1.2.840.10008.6.1.792"
pub const ClockfaceLocationForColon6205: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.792",
    ident: "ClockfaceLocationForColon6205",
    name: "Clockface Location for Colon (6205)",
};

/// Recumbent Patient Orientation for Colon (6206) - Context Group Name, "1.2.840.10008.6.1.793"
pub const RecumbentPatientOrientationForColon6206: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.793",
    ident: "RecumbentPatientOrientationForColon6206",
    name: "Recumbent Patient Orientation for Colon (6206)",
};

/// Colon Quantitative Temporal Difference Type (6207) - Context Group Name, "1.2.840.10008.6.1.794"
pub const ColonQuantitativeTemporalDifferenceType6207: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.794",
    ident: "ColonQuantitativeTemporalDifferenceType6207",
    name: "Colon Quantitative Temporal Difference Type (6207)",
};

/// Colon Types of Quality Control Standard (6208) - Context Group Name, "1.2.840.10008.6.1.795"
pub const ColonTypesOfQualityControlStandard6208: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.795",
    ident: "ColonTypesOfQualityControlStandard6208",
    name: "Colon Types of Quality Control Standard (6208)",
};

/// Colon Morphology Descriptor (6209) - Context Group Name, "1.2.840.10008.6.1.796"
pub const ColonMorphologyDescriptor6209: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.796",
    ident: "ColonMorphologyDescriptor6209",
    name: "Colon Morphology Descriptor (6209)",
};

/// Location in Intestinal Tract (6210) - Context Group Name, "1.2.840.10008.6.1.797"
pub const LocationInIntestinalTract6210: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.797",
    ident: "LocationInIntestinalTract6210",
    name: "Location in Intestinal Tract (6210)",
};

/// Attenuation Coefficient Descriptors (6211) - Context Group Name, "1.2.840.10008.6.1.798"
pub const AttenuationCoefficientDescriptors6211: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.798",
    ident: "AttenuationCoefficientDescriptors6211",
    name: "Attenuation Coefficient Descriptors (6211)",
};

/// Calculated Value for Colon Findings (6212) - Context Group Name, "1.2.840.10008.6.1.799"
pub const CalculatedValueForColonFindings6212: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.799",
    ident: "CalculatedValueForColonFindings6212",
    name: "Calculated Value for Colon Findings (6212)",
};

/// Ophthalmic Horizontal Directions (4214) - Context Group Name, "1.2.840.10008.6.1.800"
pub const OphthalmicHorizontalDirections4214: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.800",
    ident: "OphthalmicHorizontalDirections4214",
    name: "Ophthalmic Horizontal Directions (4214)",
};

/// Ophthalmic Vertical Directions (4215) - Context Group Name, "1.2.840.10008.6.1.801"
pub const OphthalmicVerticalDirections4215: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.801",
    ident: "OphthalmicVerticalDirections4215",
    name: "Ophthalmic Vertical Directions (4215)",
};

/// Ophthalmic Visual Acuity Type (4216) - Context Group Name, "1.2.840.10008.6.1.802"
pub const OphthalmicVisualAcuityType4216: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.802",
    ident: "OphthalmicVisualAcuityType4216",
    name: "Ophthalmic Visual Acuity Type (4216)",
};

/// Arterial Pulse Waveform (3004) - Context Group Name, "1.2.840.10008.6.1.803"
pub const ArterialPulseWaveform3004: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.803",
    ident: "ArterialPulseWaveform3004",
    name: "Arterial Pulse Waveform (3004)",
};

/// Respiration Waveform (3005) - Context Group Name, "1.2.840.10008.6.1.804"
pub const RespirationWaveform3005: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.804",
    ident: "RespirationWaveform3005",
    name: "Respiration Waveform (3005)",
};

/// Ultrasound Contrast/Bolus Agents (12030) - Context Group Name, "1.2.840.10008.6.1.805"
pub const UltrasoundContrastBolusAgents12030: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.805",
    ident: "UltrasoundContrastBolusAgents12030",
    name: "Ultrasound Contrast/Bolus Agents (12030)",
};

/// Protocol Interval Events (12031) - Context Group Name, "1.2.840.10008.6.1.806"
pub const ProtocolIntervalEvents12031: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.806",
    ident: "ProtocolIntervalEvents12031",
    name: "Protocol Interval Events (12031)",
};

/// Transducer Scan Pattern (12032) - Context Group Name, "1.2.840.10008.6.1.807"
pub const TransducerScanPattern12032: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.807",
    ident: "TransducerScanPattern12032",
    name: "Transducer Scan Pattern (12032)",
};

/// Ultrasound Transducer Geometry (12033) - Context Group Name, "1.2.840.10008.6.1.808"
pub const UltrasoundTransducerGeometry12033: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.808",
    ident: "UltrasoundTransducerGeometry12033",
    name: "Ultrasound Transducer Geometry (12033)",
};

/// Ultrasound Transducer Beam Steering (12034) - Context Group Name, "1.2.840.10008.6.1.809"
pub const UltrasoundTransducerBeamSteering12034: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.809",
    ident: "UltrasoundTransducerBeamSteering12034",
    name: "Ultrasound Transducer Beam Steering (12034)",
};

/// Ultrasound Transducer Application (12035) - Context Group Name, "1.2.840.10008.6.1.810"
pub const UltrasoundTransducerApplication12035: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.810",
    ident: "UltrasoundTransducerApplication12035",
    name: "Ultrasound Transducer Application (12035)",
};

/// Instance Availability Status (50) - Context Group Name, "1.2.840.10008.6.1.811"
pub const InstanceAvailabilityStatus50: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.811",
    ident: "InstanceAvailabilityStatus50",
    name: "Instance Availability Status (50)",
};

/// Modality PPS Discontinuation Reasons (9301) - Context Group Name, "1.2.840.10008.6.1.812"
pub const ModalityPPSDiscontinuationReasons9301: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.812",
    ident: "ModalityPPSDiscontinuationReasons9301",
    name: "Modality PPS Discontinuation Reasons (9301)",
};

/// Media Import PPS Discontinuation Reasons (9302) - Context Group Name, "1.2.840.10008.6.1.813"
pub const MediaImportPPSDiscontinuationReasons9302: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.813",
    ident: "MediaImportPPSDiscontinuationReasons9302",
    name: "Media Import PPS Discontinuation Reasons (9302)",
};

/// DX Anatomy Imaged for Animals (7482) - Context Group Name, "1.2.840.10008.6.1.814"
pub const DXAnatomyImagedForAnimals7482: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.814",
    ident: "DXAnatomyImagedForAnimals7482",
    name: "DX Anatomy Imaged for Animals (7482)",
};

/// Common Anatomic Regions for Animals (7483) - Context Group Name, "1.2.840.10008.6.1.815"
pub const CommonAnatomicRegionsForAnimals7483: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.815",
    ident: "CommonAnatomicRegionsForAnimals7483",
    name: "Common Anatomic Regions for Animals (7483)",
};

/// DX View for Animals (7484) - Context Group Name, "1.2.840.10008.6.1.816"
pub const DXViewForAnimals7484: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.816",
    ident: "DXViewForAnimals7484",
    name: "DX View for Animals (7484)",
};

/// Institutional Departments, Units and Services (7030) - Context Group Name, "1.2.840.10008.6.1.817"
pub const InstitutionalDepartmentsUnitsAndServices7030: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.817",
    ident: "InstitutionalDepartmentsUnitsAndServices7030",
    name: "Institutional Departments, Units and Services (7030)",
};

/// Purpose Of Reference to Predecessor Report (7009) - Context Group Name, "1.2.840.10008.6.1.818"
pub const PurposeOfReferenceToPredecessorReport7009: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.818",
    ident: "PurposeOfReferenceToPredecessorReport7009",
    name: "Purpose Of Reference to Predecessor Report (7009)",
};

/// Visual Fixation Quality During Acquisition (4220) - Context Group Name, "1.2.840.10008.6.1.819"
pub const VisualFixationQualityDuringAcquisition4220: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.819",
    ident: "VisualFixationQualityDuringAcquisition4220",
    name: "Visual Fixation Quality During Acquisition (4220)",
};

/// Visual Fixation Quality Problem (4221) - Context Group Name, "1.2.840.10008.6.1.820"
pub const VisualFixationQualityProblem4221: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.820",
    ident: "VisualFixationQualityProblem4221",
    name: "Visual Fixation Quality Problem (4221)",
};

/// Ophthalmic Macular Grid Problem (4222) - Context Group Name, "1.2.840.10008.6.1.821"
pub const OphthalmicMacularGridProblem4222: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.821",
    ident: "OphthalmicMacularGridProblem4222",
    name: "Ophthalmic Macular Grid Problem (4222)",
};

/// Organizations (5002) - Context Group Name, "1.2.840.10008.6.1.822"
pub const Organizations5002: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.822",
    ident: "Organizations5002",
    name: "Organizations (5002)",
};

/// Mixed Breeds (7486) - Context Group Name, "1.2.840.10008.6.1.823"
pub const MixedBreeds7486: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.823",
    ident: "MixedBreeds7486",
    name: "Mixed Breeds (7486)",
};

/// Broselow-Luten Pediatric Size Categories (7040) - Context Group Name, "1.2.840.10008.6.1.824"
pub const BroselowLutenPediatricSizeCategories7040: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.824",
    ident: "BroselowLutenPediatricSizeCategories7040",
    name: "Broselow-Luten Pediatric Size Categories (7040)",
};

/// Calcium Scoring Patient Size Categories (7042) - Context Group Name, "1.2.840.10008.6.1.825"
pub const CalciumScoringPatientSizeCategories7042: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.825",
    ident: "CalciumScoringPatientSizeCategories7042",
    name: "Calcium Scoring Patient Size Categories (7042)",
};

/// Cardiac Ultrasound Report Titles (12245) - Context Group Name, "1.2.840.10008.6.1.826"
pub const CardiacUltrasoundReportTitles12245: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.826",
    ident: "CardiacUltrasoundReportTitles12245",
    name: "Cardiac Ultrasound Report Titles (12245)",
};

/// Cardiac Ultrasound Indication for Study (12246) - Context Group Name, "1.2.840.10008.6.1.827"
pub const CardiacUltrasoundIndicationForStudy12246: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.827",
    ident: "CardiacUltrasoundIndicationForStudy12246",
    name: "Cardiac Ultrasound Indication for Study (12246)",
};

/// Pediatric, Fetal and Congenital Cardiac Surgical Interventions (12247) - Context Group Name, "1.2.840.10008.6.1.828"
pub const PediatricFetalAndCongenitalCardiacSurgicalInterventions12247: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.828",
    ident: "PediatricFetalAndCongenitalCardiacSurgicalInterventions12247",
    name: "Pediatric, Fetal and Congenital Cardiac Surgical Interventions (12247)",
};

/// Cardiac Ultrasound Summary Codes (12248) - Context Group Name, "1.2.840.10008.6.1.829"
pub const CardiacUltrasoundSummaryCodes12248: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.829",
    ident: "CardiacUltrasoundSummaryCodes12248",
    name: "Cardiac Ultrasound Summary Codes (12248)",
};

/// Cardiac Ultrasound Fetal Summary Codes (12249) - Context Group Name, "1.2.840.10008.6.1.830"
pub const CardiacUltrasoundFetalSummaryCodes12249: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.830",
    ident: "CardiacUltrasoundFetalSummaryCodes12249",
    name: "Cardiac Ultrasound Fetal Summary Codes (12249)",
};

/// Cardiac Ultrasound Common Linear Measurements (12250) - Context Group Name, "1.2.840.10008.6.1.831"
pub const CardiacUltrasoundCommonLinearMeasurements12250: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.831",
    ident: "CardiacUltrasoundCommonLinearMeasurements12250",
    name: "Cardiac Ultrasound Common Linear Measurements (12250)",
};

/// Cardiac Ultrasound Linear Valve Measurements (12251) - Context Group Name, "1.2.840.10008.6.1.832"
pub const CardiacUltrasoundLinearValveMeasurements12251: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.832",
    ident: "CardiacUltrasoundLinearValveMeasurements12251",
    name: "Cardiac Ultrasound Linear Valve Measurements (12251)",
};

/// Cardiac Ultrasound Cardiac Function (12252) - Context Group Name, "1.2.840.10008.6.1.833"
pub const CardiacUltrasoundCardiacFunction12252: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.833",
    ident: "CardiacUltrasoundCardiacFunction12252",
    name: "Cardiac Ultrasound Cardiac Function (12252)",
};

/// Cardiac Ultrasound Area Measurements (12253) - Context Group Name, "1.2.840.10008.6.1.834"
pub const CardiacUltrasoundAreaMeasurements12253: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.834",
    ident: "CardiacUltrasoundAreaMeasurements12253",
    name: "Cardiac Ultrasound Area Measurements (12253)",
};

/// Cardiac Ultrasound Hemodynamic Measurements (12254) - Context Group Name, "1.2.840.10008.6.1.835"
pub const CardiacUltrasoundHemodynamicMeasurements12254: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.835",
    ident: "CardiacUltrasoundHemodynamicMeasurements12254",
    name: "Cardiac Ultrasound Hemodynamic Measurements (12254)",
};

/// Cardiac Ultrasound Myocardium Measurements (12255) - Context Group Name, "1.2.840.10008.6.1.836"
pub const CardiacUltrasoundMyocardiumMeasurements12255: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.836",
    ident: "CardiacUltrasoundMyocardiumMeasurements12255",
    name: "Cardiac Ultrasound Myocardium Measurements (12255)",
};

/// Cardiac Ultrasound Common Linear Flow Measurements (12256) - Context Group Name, "1.2.840.10008.6.1.837"
pub const CardiacUltrasoundCommonLinearFlowMeasurements12256: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.837",
    ident: "CardiacUltrasoundCommonLinearFlowMeasurements12256",
    name: "Cardiac Ultrasound Common Linear Flow Measurements (12256)",
};

/// Cardiac Ultrasound Left Ventricle (12257) - Context Group Name, "1.2.840.10008.6.1.838"
pub const CardiacUltrasoundLeftVentricle12257: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.838",
    ident: "CardiacUltrasoundLeftVentricle12257",
    name: "Cardiac Ultrasound Left Ventricle (12257)",
};

/// Cardiac Ultrasound Right Ventricle (12258) - Context Group Name, "1.2.840.10008.6.1.839"
pub const CardiacUltrasoundRightVentricle12258: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.839",
    ident: "CardiacUltrasoundRightVentricle12258",
    name: "Cardiac Ultrasound Right Ventricle (12258)",
};

/// Cardiac Ultrasound Ventricles Measurements (12259) - Context Group Name, "1.2.840.10008.6.1.840"
pub const CardiacUltrasoundVentriclesMeasurements12259: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.840",
    ident: "CardiacUltrasoundVentriclesMeasurements12259",
    name: "Cardiac Ultrasound Ventricles Measurements (12259)",
};

/// Cardiac Ultrasound Pulmonary Artery (12260) - Context Group Name, "1.2.840.10008.6.1.841"
pub const CardiacUltrasoundPulmonaryArtery12260: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.841",
    ident: "CardiacUltrasoundPulmonaryArtery12260",
    name: "Cardiac Ultrasound Pulmonary Artery (12260)",
};

/// Cardiac Ultrasound Pulmonary Vein (12261) - Context Group Name, "1.2.840.10008.6.1.842"
pub const CardiacUltrasoundPulmonaryVein12261: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.842",
    ident: "CardiacUltrasoundPulmonaryVein12261",
    name: "Cardiac Ultrasound Pulmonary Vein (12261)",
};

/// Cardiac Ultrasound Pulmonary Valve (12262) - Context Group Name, "1.2.840.10008.6.1.843"
pub const CardiacUltrasoundPulmonaryValve12262: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.843",
    ident: "CardiacUltrasoundPulmonaryValve12262",
    name: "Cardiac Ultrasound Pulmonary Valve (12262)",
};

/// Cardiac Ultrasound Venous Return Pulmonary Measurements (12263) - Context Group Name, "1.2.840.10008.6.1.844"
pub const CardiacUltrasoundVenousReturnPulmonaryMeasurements12263: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.844",
    ident: "CardiacUltrasoundVenousReturnPulmonaryMeasurements12263",
    name: "Cardiac Ultrasound Venous Return Pulmonary Measurements (12263)",
};

/// Cardiac Ultrasound Venous Return Systemic Measurements (12264) - Context Group Name, "1.2.840.10008.6.1.845"
pub const CardiacUltrasoundVenousReturnSystemicMeasurements12264: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.845",
    ident: "CardiacUltrasoundVenousReturnSystemicMeasurements12264",
    name: "Cardiac Ultrasound Venous Return Systemic Measurements (12264)",
};

/// Cardiac Ultrasound Atria and Atrial Septum Measurements (12265) - Context Group Name, "1.2.840.10008.6.1.846"
pub const CardiacUltrasoundAtriaAndAtrialSeptumMeasurements12265: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.846",
    ident: "CardiacUltrasoundAtriaAndAtrialSeptumMeasurements12265",
    name: "Cardiac Ultrasound Atria and Atrial Septum Measurements (12265)",
};

/// Cardiac Ultrasound Mitral Valve (12266) - Context Group Name, "1.2.840.10008.6.1.847"
pub const CardiacUltrasoundMitralValve12266: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.847",
    ident: "CardiacUltrasoundMitralValve12266",
    name: "Cardiac Ultrasound Mitral Valve (12266)",
};

/// Cardiac Ultrasound Tricuspid Valve (12267) - Context Group Name, "1.2.840.10008.6.1.848"
pub const CardiacUltrasoundTricuspidValve12267: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.848",
    ident: "CardiacUltrasoundTricuspidValve12267",
    name: "Cardiac Ultrasound Tricuspid Valve (12267)",
};

/// Cardiac Ultrasound Atrioventricular Valves Measurements (12268) - Context Group Name, "1.2.840.10008.6.1.849"
pub const CardiacUltrasoundAtrioventricularValvesMeasurements12268: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.849",
    ident: "CardiacUltrasoundAtrioventricularValvesMeasurements12268",
    name: "Cardiac Ultrasound Atrioventricular Valves Measurements (12268)",
};

/// Cardiac Ultrasound Interventricular Septum Measurements (12269) - Context Group Name, "1.2.840.10008.6.1.850"
pub const CardiacUltrasoundInterventricularSeptumMeasurements12269: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.850",
    ident: "CardiacUltrasoundInterventricularSeptumMeasurements12269",
    name: "Cardiac Ultrasound Interventricular Septum Measurements (12269)",
};

/// Cardiac Ultrasound Aortic Valve (12270) - Context Group Name, "1.2.840.10008.6.1.851"
pub const CardiacUltrasoundAorticValve12270: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.851",
    ident: "CardiacUltrasoundAorticValve12270",
    name: "Cardiac Ultrasound Aortic Valve (12270)",
};

/// Cardiac Ultrasound Outflow Tracts Measurements (12271) - Context Group Name, "1.2.840.10008.6.1.852"
pub const CardiacUltrasoundOutflowTractsMeasurements12271: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.852",
    ident: "CardiacUltrasoundOutflowTractsMeasurements12271",
    name: "Cardiac Ultrasound Outflow Tracts Measurements (12271)",
};

/// Cardiac Ultrasound Semilunar Valves, Annulus and Sinuses Measurements (12272) - Context Group Name, "1.2.840.10008.6.1.853"
pub const CardiacUltrasoundSemilunarValvesAnnulusAndSinusesMeasurements12272: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.853",
    ident: "CardiacUltrasoundSemilunarValvesAnnulusAndSinusesMeasurements12272",
    name: "Cardiac Ultrasound Semilunar Valves, Annulus and Sinuses Measurements (12272)",
};

/// Cardiac Ultrasound Aortic Sinotubular Junction (12273) - Context Group Name, "1.2.840.10008.6.1.854"
pub const CardiacUltrasoundAorticSinotubularJunction12273: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.854",
    ident: "CardiacUltrasoundAorticSinotubularJunction12273",
    name: "Cardiac Ultrasound Aortic Sinotubular Junction (12273)",
};

/// Cardiac Ultrasound Aorta Measurements (12274) - Context Group Name, "1.2.840.10008.6.1.855"
pub const CardiacUltrasoundAortaMeasurements12274: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.855",
    ident: "CardiacUltrasoundAortaMeasurements12274",
    name: "Cardiac Ultrasound Aorta Measurements (12274)",
};

/// Cardiac Ultrasound Coronary Arteries Measurements (12275) - Context Group Name, "1.2.840.10008.6.1.856"
pub const CardiacUltrasoundCoronaryArteriesMeasurements12275: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.856",
    ident: "CardiacUltrasoundCoronaryArteriesMeasurements12275",
    name: "Cardiac Ultrasound Coronary Arteries Measurements (12275)",
};

/// Cardiac Ultrasound Aorto Pulmonary Connections Measurements (12276) - Context Group Name, "1.2.840.10008.6.1.857"
pub const CardiacUltrasoundAortoPulmonaryConnectionsMeasurements12276: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.857",
    ident: "CardiacUltrasoundAortoPulmonaryConnectionsMeasurements12276",
    name: "Cardiac Ultrasound Aorto Pulmonary Connections Measurements (12276)",
};

/// Cardiac Ultrasound Pericardium and Pleura Measurements (12277) - Context Group Name, "1.2.840.10008.6.1.858"
pub const CardiacUltrasoundPericardiumAndPleuraMeasurements12277: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.858",
    ident: "CardiacUltrasoundPericardiumAndPleuraMeasurements12277",
    name: "Cardiac Ultrasound Pericardium and Pleura Measurements (12277)",
};

/// Cardiac Ultrasound Fetal General Measurements (12279) - Context Group Name, "1.2.840.10008.6.1.859"
pub const CardiacUltrasoundFetalGeneralMeasurements12279: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.859",
    ident: "CardiacUltrasoundFetalGeneralMeasurements12279",
    name: "Cardiac Ultrasound Fetal General Measurements (12279)",
};

/// Cardiac Ultrasound Target Sites (12280) - Context Group Name, "1.2.840.10008.6.1.860"
pub const CardiacUltrasoundTargetSites12280: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.860",
    ident: "CardiacUltrasoundTargetSites12280",
    name: "Cardiac Ultrasound Target Sites (12280)",
};

/// Cardiac Ultrasound Target Site Modifiers (12281) - Context Group Name, "1.2.840.10008.6.1.861"
pub const CardiacUltrasoundTargetSiteModifiers12281: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.861",
    ident: "CardiacUltrasoundTargetSiteModifiers12281",
    name: "Cardiac Ultrasound Target Site Modifiers (12281)",
};

/// Cardiac Ultrasound Venous Return Systemic Finding Sites (12282) - Context Group Name, "1.2.840.10008.6.1.862"
pub const CardiacUltrasoundVenousReturnSystemicFindingSites12282: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.862",
    ident: "CardiacUltrasoundVenousReturnSystemicFindingSites12282",
    name: "Cardiac Ultrasound Venous Return Systemic Finding Sites (12282)",
};

/// Cardiac Ultrasound Venous Return Pulmonary Finding Sites (12283) - Context Group Name, "1.2.840.10008.6.1.863"
pub const CardiacUltrasoundVenousReturnPulmonaryFindingSites12283: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.863",
    ident: "CardiacUltrasoundVenousReturnPulmonaryFindingSites12283",
    name: "Cardiac Ultrasound Venous Return Pulmonary Finding Sites (12283)",
};

/// Cardiac Ultrasound Atria and Atrial Septum Finding Sites (12284) - Context Group Name, "1.2.840.10008.6.1.864"
pub const CardiacUltrasoundAtriaAndAtrialSeptumFindingSites12284: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.864",
    ident: "CardiacUltrasoundAtriaAndAtrialSeptumFindingSites12284",
    name: "Cardiac Ultrasound Atria and Atrial Septum Finding Sites (12284)",
};

/// Cardiac Ultrasound Atrioventricular Valves Findiing Sites (12285) - Context Group Name, "1.2.840.10008.6.1.865"
pub const CardiacUltrasoundAtrioventricularValvesFindiingSites12285: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.865",
    ident: "CardiacUltrasoundAtrioventricularValvesFindiingSites12285",
    name: "Cardiac Ultrasound Atrioventricular Valves Findiing Sites (12285)",
};

/// Cardiac Ultrasound Interventricular Septum Finding Sites (12286) - Context Group Name, "1.2.840.10008.6.1.866"
pub const CardiacUltrasoundInterventricularSeptumFindingSites12286: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.866",
    ident: "CardiacUltrasoundInterventricularSeptumFindingSites12286",
    name: "Cardiac Ultrasound Interventricular Septum Finding Sites (12286)",
};

/// Cardiac Ultrasound Ventricles Finding Sites (12287) - Context Group Name, "1.2.840.10008.6.1.867"
pub const CardiacUltrasoundVentriclesFindingSites12287: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.867",
    ident: "CardiacUltrasoundVentriclesFindingSites12287",
    name: "Cardiac Ultrasound Ventricles Finding Sites (12287)",
};

/// Cardiac Ultrasound Outflow Tracts Finding Sites (12288) - Context Group Name, "1.2.840.10008.6.1.868"
pub const CardiacUltrasoundOutflowTractsFindingSites12288: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.868",
    ident: "CardiacUltrasoundOutflowTractsFindingSites12288",
    name: "Cardiac Ultrasound Outflow Tracts Finding Sites (12288)",
};

/// Cardiac Ultrasound Semilunar Valves, Annulus and Sinuses Finding Sites (12289) - Context Group Name, "1.2.840.10008.6.1.869"
pub const CardiacUltrasoundSemilunarValvesAnnulusAndSinusesFindingSites12289: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.869",
    ident: "CardiacUltrasoundSemilunarValvesAnnulusAndSinusesFindingSites12289",
    name: "Cardiac Ultrasound Semilunar Valves, Annulus and Sinuses Finding Sites (12289)",
};

/// Cardiac Ultrasound Pulmonary Arteries Finding Sites (12290) - Context Group Name, "1.2.840.10008.6.1.870"
pub const CardiacUltrasoundPulmonaryArteriesFindingSites12290: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.870",
    ident: "CardiacUltrasoundPulmonaryArteriesFindingSites12290",
    name: "Cardiac Ultrasound Pulmonary Arteries Finding Sites (12290)",
};

/// Cardiac Ultrasound Aorta Finding Sites (12291) - Context Group Name, "1.2.840.10008.6.1.871"
pub const CardiacUltrasoundAortaFindingSites12291: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.871",
    ident: "CardiacUltrasoundAortaFindingSites12291",
    name: "Cardiac Ultrasound Aorta Finding Sites (12291)",
};

/// Cardiac Ultrasound Coronary Arteries Finding Sites (12292) - Context Group Name, "1.2.840.10008.6.1.872"
pub const CardiacUltrasoundCoronaryArteriesFindingSites12292: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.872",
    ident: "CardiacUltrasoundCoronaryArteriesFindingSites12292",
    name: "Cardiac Ultrasound Coronary Arteries Finding Sites (12292)",
};

/// Cardiac Ultrasound Aorto Pulmonary Connections Finding Sites (12293) - Context Group Name, "1.2.840.10008.6.1.873"
pub const CardiacUltrasoundAortoPulmonaryConnectionsFindingSites12293: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.873",
    ident: "CardiacUltrasoundAortoPulmonaryConnectionsFindingSites12293",
    name: "Cardiac Ultrasound Aorto Pulmonary Connections Finding Sites (12293)",
};

/// Cardiac Ultrasound Pericardium and Pleura Finding Sites (12294) - Context Group Name, "1.2.840.10008.6.1.874"
pub const CardiacUltrasoundPericardiumAndPleuraFindingSites12294: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.874",
    ident: "CardiacUltrasoundPericardiumAndPleuraFindingSites12294",
    name: "Cardiac Ultrasound Pericardium and Pleura Finding Sites (12294)",
};

/// Ophthalmic Ultrasound Axial Measurements Type (4230) - Context Group Name, "1.2.840.10008.6.1.876"
pub const OphthalmicUltrasoundAxialMeasurementsType4230: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.876",
    ident: "OphthalmicUltrasoundAxialMeasurementsType4230",
    name: "Ophthalmic Ultrasound Axial Measurements Type (4230)",
};

/// Lens Status (4231) - Context Group Name, "1.2.840.10008.6.1.877"
pub const LensStatus4231: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.877",
    ident: "LensStatus4231",
    name: "Lens Status (4231)",
};

/// Vitreous Status (4232) - Context Group Name, "1.2.840.10008.6.1.878"
pub const VitreousStatus4232: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.878",
    ident: "VitreousStatus4232",
    name: "Vitreous Status (4232)",
};

/// Ophthalmic Axial Length Measurements Segment Names (4233) - Context Group Name, "1.2.840.10008.6.1.879"
pub const OphthalmicAxialLengthMeasurementsSegmentNames4233: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.879",
    ident: "OphthalmicAxialLengthMeasurementsSegmentNames4233",
    name: "Ophthalmic Axial Length Measurements Segment Names (4233)",
};

/// Refractive Surgery Types (4234) - Context Group Name, "1.2.840.10008.6.1.880"
pub const RefractiveSurgeryTypes4234: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.880",
    ident: "RefractiveSurgeryTypes4234",
    name: "Refractive Surgery Types (4234)",
};

/// Keratometry Descriptors (4235) - Context Group Name, "1.2.840.10008.6.1.881"
pub const KeratometryDescriptors4235: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.881",
    ident: "KeratometryDescriptors4235",
    name: "Keratometry Descriptors (4235)",
};

/// IOL Calculation Formula (4236) - Context Group Name, "1.2.840.10008.6.1.882"
pub const IOLCalculationFormula4236: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.882",
    ident: "IOLCalculationFormula4236",
    name: "IOL Calculation Formula (4236)",
};

/// Lens Constant Type (4237) - Context Group Name, "1.2.840.10008.6.1.883"
pub const LensConstantType4237: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.883",
    ident: "LensConstantType4237",
    name: "Lens Constant Type (4237)",
};

/// Refractive Error Types (4238) - Context Group Name, "1.2.840.10008.6.1.884"
pub const RefractiveErrorTypes4238: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.884",
    ident: "RefractiveErrorTypes4238",
    name: "Refractive Error Types (4238)",
};

/// Anterior Chamber Depth Definition (4239) - Context Group Name, "1.2.840.10008.6.1.885"
pub const AnteriorChamberDepthDefinition4239: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.885",
    ident: "AnteriorChamberDepthDefinition4239",
    name: "Anterior Chamber Depth Definition (4239)",
};

/// Ophthalmic Measurement or Calculation Data Source (4240) - Context Group Name, "1.2.840.10008.6.1.886"
pub const OphthalmicMeasurementOrCalculationDataSource4240: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.886",
    ident: "OphthalmicMeasurementOrCalculationDataSource4240",
    name: "Ophthalmic Measurement or Calculation Data Source (4240)",
};

/// Ophthalmic Axial Length Selection Method (4241) - Context Group Name, "1.2.840.10008.6.1.887"
pub const OphthalmicAxialLengthSelectionMethod4241: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.887",
    ident: "OphthalmicAxialLengthSelectionMethod4241",
    name: "Ophthalmic Axial Length Selection Method (4241)",
};

/// Ophthalmic Axial Length Quality Metric Type (4243) - Context Group Name, "1.2.840.10008.6.1.889"
pub const OphthalmicAxialLengthQualityMetricType4243: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.889",
    ident: "OphthalmicAxialLengthQualityMetricType4243",
    name: "Ophthalmic Axial Length Quality Metric Type (4243)",
};

/// Ophthalmic Agent Concentration Units (4244) - Context Group Name, "1.2.840.10008.6.1.890"
pub const OphthalmicAgentConcentrationUnits4244: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.890",
    ident: "OphthalmicAgentConcentrationUnits4244",
    name: "Ophthalmic Agent Concentration Units (4244)",
};

/// Functional condition present during acquisition (91) - Context Group Name, "1.2.840.10008.6.1.891"
pub const FunctionalConditionPresentDuringAcquisition91: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.891",
    ident: "FunctionalConditionPresentDuringAcquisition91",
    name: "Functional condition present during acquisition (91)",
};

/// Joint position during acquisition (92) - Context Group Name, "1.2.840.10008.6.1.892"
pub const JointPositionDuringAcquisition92: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.892",
    ident: "JointPositionDuringAcquisition92",
    name: "Joint position during acquisition (92)",
};

/// Joint positioning method (93) - Context Group Name, "1.2.840.10008.6.1.893"
pub const JointPositioningMethod93: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.893",
    ident: "JointPositioningMethod93",
    name: "Joint positioning method (93)",
};

/// Physical force applied during acquisition (94) - Context Group Name, "1.2.840.10008.6.1.894"
pub const PhysicalForceAppliedDuringAcquisition94: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.894",
    ident: "PhysicalForceAppliedDuringAcquisition94",
    name: "Physical force applied during acquisition (94)",
};

/// ECG Control Variables Numeric (3690) - Context Group Name, "1.2.840.10008.6.1.895"
pub const ECGControlVariablesNumeric3690: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.895",
    ident: "ECGControlVariablesNumeric3690",
    name: "ECG Control Variables Numeric (3690)",
};

/// ECG Control Variables Text (3691) - Context Group Name, "1.2.840.10008.6.1.896"
pub const ECGControlVariablesText3691: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.896",
    ident: "ECGControlVariablesText3691",
    name: "ECG Control Variables Text (3691)",
};

/// WSI Referenced Image Purposes of Reference (8120) - Context Group Name, "1.2.840.10008.6.1.897"
pub const WSIReferencedImagePurposesOfReference8120: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.897",
    ident: "WSIReferencedImagePurposesOfReference8120",
    name: "WSI Referenced Image Purposes of Reference (8120)",
};

/// WSI Microscopy Lens Type (8121) - Context Group Name, "1.2.840.10008.6.1.898"
pub const WSIMicroscopyLensType8121: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.898",
    ident: "WSIMicroscopyLensType8121",
    name: "WSI Microscopy Lens Type (8121)",
};

/// Microscopy Illuminator and Sensor Color (8122) - Context Group Name, "1.2.840.10008.6.1.899"
pub const MicroscopyIlluminatorAndSensorColor8122: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.899",
    ident: "MicroscopyIlluminatorAndSensorColor8122",
    name: "Microscopy Illuminator and Sensor Color (8122)",
};

/// Microscopy Illumination Method (8123) - Context Group Name, "1.2.840.10008.6.1.900"
pub const MicroscopyIlluminationMethod8123: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.900",
    ident: "MicroscopyIlluminationMethod8123",
    name: "Microscopy Illumination Method (8123)",
};

/// Microscopy Filter (8124) - Context Group Name, "1.2.840.10008.6.1.901"
pub const MicroscopyFilter8124: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.901",
    ident: "MicroscopyFilter8124",
    name: "Microscopy Filter (8124)",
};

/// Microscopy Illuminator Type (8125) - Context Group Name, "1.2.840.10008.6.1.902"
pub const MicroscopyIlluminatorType8125: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.902",
    ident: "MicroscopyIlluminatorType8125",
    name: "Microscopy Illuminator Type (8125)",
};

/// Audit Event ID (400) - Context Group Name, "1.2.840.10008.6.1.903"
pub const AuditEventID400: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.903",
    ident: "AuditEventID400",
    name: "Audit Event ID (400)",
};

/// Audit Event Type Code (401) - Context Group Name, "1.2.840.10008.6.1.904"
pub const AuditEventTypeCode401: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.904",
    ident: "AuditEventTypeCode401",
    name: "Audit Event Type Code (401)",
};

/// Audit Active Participant Role ID Code (402) - Context Group Name, "1.2.840.10008.6.1.905"
pub const AuditActiveParticipantRoleIDCode402: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.905",
    ident: "AuditActiveParticipantRoleIDCode402",
    name: "Audit Active Participant Role ID Code (402)",
};

/// Security Alert Type Code (403) - Context Group Name, "1.2.840.10008.6.1.906"
pub const SecurityAlertTypeCode403: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.906",
    ident: "SecurityAlertTypeCode403",
    name: "Security Alert Type Code (403)",
};

/// Audit Participant Object ID Type Code (404) - Context Group Name, "1.2.840.10008.6.1.907"
pub const AuditParticipantObjectIDTypeCode404: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.907",
    ident: "AuditParticipantObjectIDTypeCode404",
    name: "Audit Participant Object ID Type Code (404)",
};

/// Media Type Code (405) - Context Group Name, "1.2.840.10008.6.1.908"
pub const MediaTypeCode405: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.908",
    ident: "MediaTypeCode405",
    name: "Media Type Code (405)",
};

/// Visual Field Static Perimetry Test Patterns (4250) - Context Group Name, "1.2.840.10008.6.1.909"
pub const VisualFieldStaticPerimetryTestPatterns4250: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.909",
    ident: "VisualFieldStaticPerimetryTestPatterns4250",
    name: "Visual Field Static Perimetry Test Patterns (4250)",
};

/// Visual Field Static Perimetry Test Strategies (4251) - Context Group Name, "1.2.840.10008.6.1.910"
pub const VisualFieldStaticPerimetryTestStrategies4251: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.910",
    ident: "VisualFieldStaticPerimetryTestStrategies4251",
    name: "Visual Field Static Perimetry Test Strategies (4251)",
};

/// Visual Field Static Perimetry Screening Test Modes (4252) - Context Group Name, "1.2.840.10008.6.1.911"
pub const VisualFieldStaticPerimetryScreeningTestModes4252: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.911",
    ident: "VisualFieldStaticPerimetryScreeningTestModes4252",
    name: "Visual Field Static Perimetry Screening Test Modes (4252)",
};

/// Visual Field Static Perimetry Fixation Strategy (4253) - Context Group Name, "1.2.840.10008.6.1.912"
pub const VisualFieldStaticPerimetryFixationStrategy4253: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.912",
    ident: "VisualFieldStaticPerimetryFixationStrategy4253",
    name: "Visual Field Static Perimetry Fixation Strategy (4253)",
};

/// Visual Field Static Perimetry Test Analysis Results (4254) - Context Group Name, "1.2.840.10008.6.1.913"
pub const VisualFieldStaticPerimetryTestAnalysisResults4254: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.913",
    ident: "VisualFieldStaticPerimetryTestAnalysisResults4254",
    name: "Visual Field Static Perimetry Test Analysis Results (4254)",
};

/// Visual Field Illumination Color (4255) - Context Group Name, "1.2.840.10008.6.1.914"
pub const VisualFieldIlluminationColor4255: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.914",
    ident: "VisualFieldIlluminationColor4255",
    name: "Visual Field Illumination Color (4255)",
};

/// Visual Field Procedure Modifier (4256) - Context Group Name, "1.2.840.10008.6.1.915"
pub const VisualFieldProcedureModifier4256: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.915",
    ident: "VisualFieldProcedureModifier4256",
    name: "Visual Field Procedure Modifier (4256)",
};

/// Visual Field Global Index Name (4257) - Context Group Name, "1.2.840.10008.6.1.916"
pub const VisualFieldGlobalIndexName4257: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.916",
    ident: "VisualFieldGlobalIndexName4257",
    name: "Visual Field Global Index Name (4257)",
};

/// Abstract Multi-Dimensional Image Model Component Semantics (7180) - Context Group Name, "1.2.840.10008.6.1.917"
pub const AbstractMultiDimensionalImageModelComponentSemantics7180: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.917",
    ident: "AbstractMultiDimensionalImageModelComponentSemantics7180",
    name: "Abstract Multi-Dimensional Image Model Component Semantics (7180)",
};

/// Abstract Multi-Dimensional Image Model Component Units (7181) - Context Group Name, "1.2.840.10008.6.1.918"
pub const AbstractMultiDimensionalImageModelComponentUnits7181: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.918",
    ident: "AbstractMultiDimensionalImageModelComponentUnits7181",
    name: "Abstract Multi-Dimensional Image Model Component Units (7181)",
};

/// Abstract Multi-Dimensional Image Model Dimension Semantics (7182) - Context Group Name, "1.2.840.10008.6.1.919"
pub const AbstractMultiDimensionalImageModelDimensionSemantics7182: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.919",
    ident: "AbstractMultiDimensionalImageModelDimensionSemantics7182",
    name: "Abstract Multi-Dimensional Image Model Dimension Semantics (7182)",
};

/// Abstract Multi-Dimensional Image Model Dimension Units (7183) - Context Group Name, "1.2.840.10008.6.1.920"
pub const AbstractMultiDimensionalImageModelDimensionUnits7183: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.920",
    ident: "AbstractMultiDimensionalImageModelDimensionUnits7183",
    name: "Abstract Multi-Dimensional Image Model Dimension Units (7183)",
};

/// Abstract Multi-Dimensional Image Model Axis Direction (7184) - Context Group Name, "1.2.840.10008.6.1.921"
pub const AbstractMultiDimensionalImageModelAxisDirection7184: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.921",
    ident: "AbstractMultiDimensionalImageModelAxisDirection7184",
    name: "Abstract Multi-Dimensional Image Model Axis Direction (7184)",
};

/// Abstract Multi-Dimensional Image Model Axis Orientation (7185) - Context Group Name, "1.2.840.10008.6.1.922"
pub const AbstractMultiDimensionalImageModelAxisOrientation7185: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.922",
    ident: "AbstractMultiDimensionalImageModelAxisOrientation7185",
    name: "Abstract Multi-Dimensional Image Model Axis Orientation (7185)",
};

/// Abstract Multi-Dimensional Image Model Qualitative Dimension Sample Semantics (7186) - Context Group Name, "1.2.840.10008.6.1.923"
pub const AbstractMultiDimensionalImageModelQualitativeDimensionSampleSemantics7186: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.923",
    ident: "AbstractMultiDimensionalImageModelQualitativeDimensionSampleSemantics7186",
    name: "Abstract Multi-Dimensional Image Model Qualitative Dimension Sample Semantics (7186)",
};

/// Planning Methods (7320) - Context Group Name, "1.2.840.10008.6.1.924"
pub const PlanningMethods7320: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.924",
    ident: "PlanningMethods7320",
    name: "Planning Methods (7320)",
};

/// De-identification Method (7050) - Context Group Name, "1.2.840.10008.6.1.925"
pub const DeIdentificationMethod7050: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.925",
    ident: "DeIdentificationMethod7050",
    name: "De-identification Method (7050)",
};

/// Measurement Orientation (12118) - Context Group Name, "1.2.840.10008.6.1.926"
pub const MeasurementOrientation12118: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.926",
    ident: "MeasurementOrientation12118",
    name: "Measurement Orientation (12118)",
};

/// ECG Global Waveform Durations (3689) - Context Group Name, "1.2.840.10008.6.1.927"
pub const ECGGlobalWaveformDurations3689: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.927",
    ident: "ECGGlobalWaveformDurations3689",
    name: "ECG Global Waveform Durations (3689)",
};

/// ICDs (3692) - Context Group Name, "1.2.840.10008.6.1.930"
pub const ICDs3692: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.930",
    ident: "ICDs3692",
    name: "ICDs (3692)",
};

/// Radiotherapy General Workitem Definition (9241) - Context Group Name, "1.2.840.10008.6.1.931"
pub const RadiotherapyGeneralWorkitemDefinition9241: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.931",
    ident: "RadiotherapyGeneralWorkitemDefinition9241",
    name: "Radiotherapy General Workitem Definition (9241)",
};

/// Radiotherapy Acquisition Workitem Definition (9242) - Context Group Name, "1.2.840.10008.6.1.932"
pub const RadiotherapyAcquisitionWorkitemDefinition9242: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.932",
    ident: "RadiotherapyAcquisitionWorkitemDefinition9242",
    name: "Radiotherapy Acquisition Workitem Definition (9242)",
};

/// Radiotherapy Registration Workitem Definition (9243) - Context Group Name, "1.2.840.10008.6.1.933"
pub const RadiotherapyRegistrationWorkitemDefinition9243: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.933",
    ident: "RadiotherapyRegistrationWorkitemDefinition9243",
    name: "Radiotherapy Registration Workitem Definition (9243)",
};

/// Intravascular OCT Flush Agent (3850) - Context Group Name, "1.2.840.10008.6.1.934"
pub const IntravascularOCTFlushAgent3850: &'static UID = &UID {
    uid: "1.2.840.10008.6.1.934",
    ident: "IntravascularOCTFlushAgent3850",
    name: "Intravascular OCT Flush Agent (3850)",
};

/// Dcm4che Attributes Modification Notification SOP Class - SOP Class, "1.2.40.0.13.1.3.1.2.3.1.1"
pub const Dcm4cheAttributesModificationNotificationSOPClass: &'static UID = &UID {
    uid: "1.2.40.0.13.1.3.1.2.3.1.1",
    ident: "Dcm4cheAttributesModificationNotificationSOPClass",
    name: "Dcm4che Attributes Modification Notification SOP Class",
};

/// Private Study Root Query/Retrieve Information Model : FIND - SOP Class, "1.2.40.0.13.1.5.1.4.1.2.2.1"
pub const PrivateStudyRootQueryRetrieveInformationModelFIND: &'static UID = &UID {
    uid: "1.2.40.0.13.1.5.1.4.1.2.2.1",
    ident: "PrivateStudyRootQueryRetrieveInformationModelFIND",
    name: "Private Study Root Query/Retrieve Information Model - FIND",
};

/// Private Blocked Study Root Query/Retrieve Information Model : FIND - SOP Class, "1.2.40.0.13.1.5.1.4.1.2.2.1.1"
pub const PrivateBlockedStudyRootQueryRetrieveInformationModelFIND: &'static UID = &UID {
    uid: "1.2.40.0.13.1.5.1.4.1.2.2.1.1",
    ident: "PrivateBlockedStudyRootQueryRetrieveInformationModelFIND",
    name: "Private Blocked Study Root Query/Retrieve Information Model - FIND",
};

/// Private Virtual Multiframe Study Root Query/Retrieve Information Model : FIND - SOP Class, "1.2.40.0.13.1.5.1.4.1.2.2.1.2"
pub const PrivateVirtualMultiframeStudyRootQueryRetrieveInformationModelFIND: &'static UID = &UID {
    uid: "1.2.40.0.13.1.5.1.4.1.2.2.1.2",
    ident: "PrivateVirtualMultiframeStudyRootQueryRetrieveInformationModelFIND",
    name: "Private Virtual Multiframe Study Root Query/Retrieve Information Model - FIND",
};

/// Siemens CSA Non-Image Storage - SOP Class, "1.3.12.2.1107.5.9.1"
pub const SiemensCSANonImageStorage: &'static UID = &UID {
    uid: "1.3.12.2.1107.5.9.1",
    ident: "SiemensCSANonImageStorage",
    name: "Siemens CSA Non-Image Storage",
};

/// Toshiba US Private Data Storage - SOP Class, "1.2.392.200036.9116.7.8.1.1.1"
pub const ToshibaUSPrivateDataStorage: &'static UID = &UID {
    uid: "1.2.392.200036.9116.7.8.1.1.1",
    ident: "ToshibaUSPrivateDataStorage",
    name: "Toshiba US Private Data Storage",
};

/// No Pixel Data - Transfer Syntax, "1.2.840.10008.1.2.4.96"
pub const NoPixelData: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.96",
    ident: "NoPixelData",
    name: "No Pixel Data",
};

/// No Pixel Data Deflate - Transfer Syntax, "1.2.840.10008.1.2.4.97"
pub const NoPixelDataDeflate: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.97",
    ident: "NoPixelDataDeflate",
    name: "No Pixel Data Deflate",
};
