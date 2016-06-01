#![allow(dead_code)]
#![allow(non_upper_case_globals)]

pub struct UID {
    uid: &'static str,
    ident: &'static str,
    name: &'static str,
}


/// VerificationSOPClass, "1.2.840.10008.1.1"
pub static VerificationSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.1.1",
    ident: "VerificationSOPClass",
    name: "Verification SOP Class",
};

/// ImplicitVRLittleEndian, "1.2.840.10008.1.2"
pub static ImplicitVRLittleEndian: &'static UID = &UID {
    uid: "1.2.840.10008.1.2",
    ident: "ImplicitVRLittleEndian",
    name: "Implicit VR Little Endian",
};

/// ExplicitVRLittleEndian, "1.2.840.10008.1.2.1"
pub static ExplicitVRLittleEndian: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.1",
    ident: "ExplicitVRLittleEndian",
    name: "Explicit VR Little Endian",
};

/// DeflatedExplicitVRLittleEndian, "1.2.840.10008.1.2.1.99"
pub static DeflatedExplicitVRLittleEndian: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.1.99",
    ident: "DeflatedExplicitVRLittleEndian",
    name: "Deflated Explicit VR Little Endian",
};

/// ExplicitVRBigEndianRetired, "1.2.840.10008.1.2.2"
pub static ExplicitVRBigEndianRetired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.2",
    ident: "ExplicitVRBigEndianRetired",
    name: "Explicit VR Big Endian (Retired)",
};

/// JPEGBaseline1, "1.2.840.10008.1.2.4.50"
pub static JPEGBaseline1: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.50",
    ident: "JPEGBaseline1",
    name: "JPEG Baseline (Process 1)",
};

/// JPEGExtended24, "1.2.840.10008.1.2.4.51"
pub static JPEGExtended24: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.51",
    ident: "JPEGExtended24",
    name: "JPEG Extended (Process 2 & 4)",
};

/// JPEGExtended35Retired, "1.2.840.10008.1.2.4.52"
pub static JPEGExtended35Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.52",
    ident: "JPEGExtended35Retired",
    name: "JPEG Extended (Process 3 & 5) (Retired)",
};

/// JPEGSpectralSelectionNonHierarchical68Retired, "1.2.840.10008.1.2.4.53"
pub static JPEGSpectralSelectionNonHierarchical68Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.53",
    ident: "JPEGSpectralSelectionNonHierarchical68Retired",
    name: "JPEG Spectral Selection, Non-Hierarchical (Process 6 & 8) (Retired)",
};

/// JPEGSpectralSelectionNonHierarchical79Retired, "1.2.840.10008.1.2.4.54"
pub static JPEGSpectralSelectionNonHierarchical79Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.54",
    ident: "JPEGSpectralSelectionNonHierarchical79Retired",
    name: "JPEG Spectral Selection, Non-Hierarchical (Process 7 & 9) (Retired)",
};

/// JPEGFullProgressionNonHierarchical1012Retired, "1.2.840.10008.1.2.4.55"
pub static JPEGFullProgressionNonHierarchical1012Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.55",
    ident: "JPEGFullProgressionNonHierarchical1012Retired",
    name: "JPEG Full Progression, Non-Hierarchical (Process 10 & 12) (Retired)",
};

/// JPEGFullProgressionNonHierarchical1113Retired, "1.2.840.10008.1.2.4.56"
pub static JPEGFullProgressionNonHierarchical1113Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.56",
    ident: "JPEGFullProgressionNonHierarchical1113Retired",
    name: "JPEG Full Progression, Non-Hierarchical (Process 11 & 13) (Retired)",
};

/// JPEGLosslessNonHierarchical14, "1.2.840.10008.1.2.4.57"
pub static JPEGLosslessNonHierarchical14: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.57",
    ident: "JPEGLosslessNonHierarchical14",
    name: "JPEG Lossless, Non-Hierarchical (Process 14)",
};

/// JPEGLosslessNonHierarchical15Retired, "1.2.840.10008.1.2.4.58"
pub static JPEGLosslessNonHierarchical15Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.58",
    ident: "JPEGLosslessNonHierarchical15Retired",
    name: "JPEG Lossless, Non-Hierarchical (Process 15) (Retired)",
};

/// JPEGExtendedHierarchical1618Retired, "1.2.840.10008.1.2.4.59"
pub static JPEGExtendedHierarchical1618Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.59",
    ident: "JPEGExtendedHierarchical1618Retired",
    name: "JPEG Extended, Hierarchical (Process 16 & 18) (Retired)",
};

/// JPEGExtendedHierarchical1719Retired, "1.2.840.10008.1.2.4.60"
pub static JPEGExtendedHierarchical1719Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.60",
    ident: "JPEGExtendedHierarchical1719Retired",
    name: "JPEG Extended, Hierarchical (Process 17 & 19) (Retired)",
};

/// JPEGSpectralSelectionHierarchical2022Retired, "1.2.840.10008.1.2.4.61"
pub static JPEGSpectralSelectionHierarchical2022Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.61",
    ident: "JPEGSpectralSelectionHierarchical2022Retired",
    name: "JPEG Spectral Selection, Hierarchical (Process 20 & 22) (Retired)",
};

/// JPEGSpectralSelectionHierarchical2123Retired, "1.2.840.10008.1.2.4.62"
pub static JPEGSpectralSelectionHierarchical2123Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.62",
    ident: "JPEGSpectralSelectionHierarchical2123Retired",
    name: "JPEG Spectral Selection, Hierarchical (Process 21 & 23) (Retired)",
};

/// JPEGFullProgressionHierarchical2426Retired, "1.2.840.10008.1.2.4.63"
pub static JPEGFullProgressionHierarchical2426Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.63",
    ident: "JPEGFullProgressionHierarchical2426Retired",
    name: "JPEG Full Progression, Hierarchical (Process 24 & 26) (Retired)",
};

/// JPEGFullProgressionHierarchical2527Retired, "1.2.840.10008.1.2.4.64"
pub static JPEGFullProgressionHierarchical2527Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.64",
    ident: "JPEGFullProgressionHierarchical2527Retired",
    name: "JPEG Full Progression, Hierarchical (Process 25 & 27) (Retired)",
};

/// JPEGLosslessHierarchical28Retired, "1.2.840.10008.1.2.4.65"
pub static JPEGLosslessHierarchical28Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.65",
    ident: "JPEGLosslessHierarchical28Retired",
    name: "JPEG Lossless, Hierarchical (Process 28) (Retired)",
};

/// JPEGLosslessHierarchical29Retired, "1.2.840.10008.1.2.4.66"
pub static JPEGLosslessHierarchical29Retired: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.66",
    ident: "JPEGLosslessHierarchical29Retired",
    name: "JPEG Lossless, Hierarchical (Process 29) (Retired)",
};

/// JPEGLossless, "1.2.840.10008.1.2.4.70"
pub static JPEGLossless: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.70",
    ident: "JPEGLossless",
    name: "JPEG Lossless, Non-Hierarchical, First-Order Prediction (Process 14 [Selection Value \
           1])",
};

/// JPEGLSLossless, "1.2.840.10008.1.2.4.80"
pub static JPEGLSLossless: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.80",
    ident: "JPEGLSLossless",
    name: "JPEG-LS Lossless Image Compression",
};

/// JPEGLSLossyNearLossless, "1.2.840.10008.1.2.4.81"
pub static JPEGLSLossyNearLossless: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.81",
    ident: "JPEGLSLossyNearLossless",
    name: "JPEG-LS Lossy (Near-Lossless) Image Compression",
};

/// JPEG2000LosslessOnly, "1.2.840.10008.1.2.4.90"
pub static JPEG2000LosslessOnly: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.90",
    ident: "JPEG2000LosslessOnly",
    name: "JPEG 2000 Image Compression (Lossless Only)",
};

/// JPEG2000, "1.2.840.10008.1.2.4.91"
pub static JPEG2000: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.91",
    ident: "JPEG2000",
    name: "JPEG 2000 Image Compression",
};

/// JPEG2000Part2MultiComponentLosslessOnly, "1.2.840.10008.1.2.4.92"
pub static JPEG2000Part2MultiComponentLosslessOnly: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.92",
    ident: "JPEG2000Part2MultiComponentLosslessOnly",
    name: "JPEG 2000 Part 2 Multi-component Image Compression (Lossless Only)",
};

/// JPEG2000Part2MultiComponent, "1.2.840.10008.1.2.4.93"
pub static JPEG2000Part2MultiComponent: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.93",
    ident: "JPEG2000Part2MultiComponent",
    name: "JPEG 2000 Part 2 Multi-component Image Compression",
};

/// JPIPReferenced, "1.2.840.10008.1.2.4.94"
pub static JPIPReferenced: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.94",
    ident: "JPIPReferenced",
    name: "JPIP Referenced",
};

/// JPIPReferencedDeflate, "1.2.840.10008.1.2.4.95"
pub static JPIPReferencedDeflate: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.95",
    ident: "JPIPReferencedDeflate",
    name: "JPIP Referenced Deflate",
};

/// MPEG2, "1.2.840.10008.1.2.4.100"
pub static MPEG2: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.100",
    ident: "MPEG2",
    name: "MPEG2 Main Profile @ Main Level",
};

/// MPEG2MainProfileHighLevel, "1.2.840.10008.1.2.4.101"
pub static MPEG2MainProfileHighLevel: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.101",
    ident: "MPEG2MainProfileHighLevel",
    name: "MPEG2 Main Profile @ High Level",
};

/// MPEG4AVCH264HighProfileLevel41, "1.2.840.10008.1.2.4.102"
pub static MPEG4AVCH264HighProfileLevel41: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.102",
    ident: "MPEG4AVCH264HighProfileLevel41",
    name: "MPEG-4 AVC/H.264 High Profile / Level 4.1",
};

/// MPEG4AVCH264BDCompatibleHighProfileLevel41, "1.2.840.10008.1.2.4.103"
pub static MPEG4AVCH264BDCompatibleHighProfileLevel41: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.4.103",
    ident: "MPEG4AVCH264BDCompatibleHighProfileLevel41",
    name: "MPEG-4 AVC/H.264 BD-compatible High Profile / Level 4.1",
};

/// RLELossless, "1.2.840.10008.1.2.5"
pub static RLELossless: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.5",
    ident: "RLELossless",
    name: "RLE Lossless",
};

/// RFC2557MIMEEncapsulation, "1.2.840.10008.1.2.6.1"
pub static RFC2557MIMEEncapsulation: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.6.1",
    ident: "RFC2557MIMEEncapsulation",
    name: "RFC 2557 MIME encapsulation",
};

/// XMLEncoding, "1.2.840.10008.1.2.6.2"
pub static XMLEncoding: &'static UID = &UID {
    uid: "1.2.840.10008.1.2.6.2",
    ident: "XMLEncoding",
    name: "XML Encoding",
};

/// MediaStorageDirectoryStorage, "1.2.840.10008.1.3.10"
pub static MediaStorageDirectoryStorage: &'static UID = &UID {
    uid: "1.2.840.10008.1.3.10",
    ident: "MediaStorageDirectoryStorage",
    name: "Media Storage Directory Storage",
};

/// TalairachBrainAtlasFrameOfReference, "1.2.840.10008.1.4.1.1"
pub static TalairachBrainAtlasFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.1",
    ident: "TalairachBrainAtlasFrameOfReference",
    name: "Talairach Brain Atlas Frame of Reference",
};

/// SPM2T1FrameOfReference, "1.2.840.10008.1.4.1.2"
pub static SPM2T1FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.2",
    ident: "SPM2T1FrameOfReference",
    name: "SPM2 T1 Frame of Reference",
};

/// SPM2T2FrameOfReference, "1.2.840.10008.1.4.1.3"
pub static SPM2T2FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.3",
    ident: "SPM2T2FrameOfReference",
    name: "SPM2 T2 Frame of Reference",
};

/// SPM2PDFrameOfReference, "1.2.840.10008.1.4.1.4"
pub static SPM2PDFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.4",
    ident: "SPM2PDFrameOfReference",
    name: "SPM2 PD Frame of Reference",
};

/// SPM2EPIFrameOfReference, "1.2.840.10008.1.4.1.5"
pub static SPM2EPIFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.5",
    ident: "SPM2EPIFrameOfReference",
    name: "SPM2 EPI Frame of Reference",
};

/// SPM2FILT1FrameOfReference, "1.2.840.10008.1.4.1.6"
pub static SPM2FILT1FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.6",
    ident: "SPM2FILT1FrameOfReference",
    name: "SPM2 FIL T1 Frame of Reference",
};

/// SPM2PETFrameOfReference, "1.2.840.10008.1.4.1.7"
pub static SPM2PETFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.7",
    ident: "SPM2PETFrameOfReference",
    name: "SPM2 PET Frame of Reference",
};

/// SPM2TRANSMFrameOfReference, "1.2.840.10008.1.4.1.8"
pub static SPM2TRANSMFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.8",
    ident: "SPM2TRANSMFrameOfReference",
    name: "SPM2 TRANSM Frame of Reference",
};

/// SPM2SPECTFrameOfReference, "1.2.840.10008.1.4.1.9"
pub static SPM2SPECTFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.9",
    ident: "SPM2SPECTFrameOfReference",
    name: "SPM2 SPECT Frame of Reference",
};

/// SPM2GRAYFrameOfReference, "1.2.840.10008.1.4.1.10"
pub static SPM2GRAYFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.10",
    ident: "SPM2GRAYFrameOfReference",
    name: "SPM2 GRAY Frame of Reference",
};

/// SPM2WHITEFrameOfReference, "1.2.840.10008.1.4.1.11"
pub static SPM2WHITEFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.11",
    ident: "SPM2WHITEFrameOfReference",
    name: "SPM2 WHITE Frame of Reference",
};

/// SPM2CSFFrameOfReference, "1.2.840.10008.1.4.1.12"
pub static SPM2CSFFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.12",
    ident: "SPM2CSFFrameOfReference",
    name: "SPM2 CSF Frame of Reference",
};

/// SPM2BRAINMASKFrameOfReference, "1.2.840.10008.1.4.1.13"
pub static SPM2BRAINMASKFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.13",
    ident: "SPM2BRAINMASKFrameOfReference",
    name: "SPM2 BRAINMASK Frame of Reference",
};

/// SPM2AVG305T1FrameOfReference, "1.2.840.10008.1.4.1.14"
pub static SPM2AVG305T1FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.14",
    ident: "SPM2AVG305T1FrameOfReference",
    name: "SPM2 AVG305T1 Frame of Reference",
};

/// SPM2AVG152T1FrameOfReference, "1.2.840.10008.1.4.1.15"
pub static SPM2AVG152T1FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.15",
    ident: "SPM2AVG152T1FrameOfReference",
    name: "SPM2 AVG152T1 Frame of Reference",
};

/// SPM2AVG152T2FrameOfReference, "1.2.840.10008.1.4.1.16"
pub static SPM2AVG152T2FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.16",
    ident: "SPM2AVG152T2FrameOfReference",
    name: "SPM2 AVG152T2 Frame of Reference",
};

/// SPM2AVG152PDFrameOfReference, "1.2.840.10008.1.4.1.17"
pub static SPM2AVG152PDFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.17",
    ident: "SPM2AVG152PDFrameOfReference",
    name: "SPM2 AVG152PD Frame of Reference",
};

/// SPM2SINGLESUBJT1FrameOfReference, "1.2.840.10008.1.4.1.18"
pub static SPM2SINGLESUBJT1FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.1.18",
    ident: "SPM2SINGLESUBJT1FrameOfReference",
    name: "SPM2 SINGLESUBJT1 Frame of Reference",
};

/// ICBM452T1FrameOfReference, "1.2.840.10008.1.4.2.1"
pub static ICBM452T1FrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.2.1",
    ident: "ICBM452T1FrameOfReference",
    name: "ICBM 452 T1 Frame of Reference",
};

/// ICBMSingleSubjectMRIFrameOfReference, "1.2.840.10008.1.4.2.2"
pub static ICBMSingleSubjectMRIFrameOfReference: &'static UID = &UID {
    uid: "1.2.840.10008.1.4.2.2",
    ident: "ICBMSingleSubjectMRIFrameOfReference",
    name: "ICBM Single Subject MRI Frame of Reference",
};

/// HotIronColorPaletteSOPInstance, "1.2.840.10008.1.5.1"
pub static HotIronColorPaletteSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.1.5.1",
    ident: "HotIronColorPaletteSOPInstance",
    name: "Hot Iron Color Palette SOP Instance",
};

/// PETColorPaletteSOPInstance, "1.2.840.10008.1.5.2"
pub static PETColorPaletteSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.1.5.2",
    ident: "PETColorPaletteSOPInstance",
    name: "PET Color Palette SOP Instance",
};

/// HotMetalBlueColorPaletteSOPInstance, "1.2.840.10008.1.5.3"
pub static HotMetalBlueColorPaletteSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.1.5.3",
    ident: "HotMetalBlueColorPaletteSOPInstance",
    name: "Hot Metal Blue Color Palette SOP Instance",
};

/// PET20StepColorPaletteSOPInstance, "1.2.840.10008.1.5.4"
pub static PET20StepColorPaletteSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.1.5.4",
    ident: "PET20StepColorPaletteSOPInstance",
    name: "PET 20 Step Color Palette SOP Instance",
};

/// BasicStudyContentNotificationSOPClassRetired, "1.2.840.10008.1.9"
pub static BasicStudyContentNotificationSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.1.9",
    ident: "BasicStudyContentNotificationSOPClassRetired",
    name: "Basic Study Content Notification SOP Class (Retired)",
};

/// StorageCommitmentPushModelSOPClass, "1.2.840.10008.1.20.1"
pub static StorageCommitmentPushModelSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.1.20.1",
    ident: "StorageCommitmentPushModelSOPClass",
    name: "Storage Commitment Push Model SOP Class",
};

/// StorageCommitmentPushModelSOPInstance, "1.2.840.10008.1.20.1.1"
pub static StorageCommitmentPushModelSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.1.20.1.1",
    ident: "StorageCommitmentPushModelSOPInstance",
    name: "Storage Commitment Push Model SOP Instance",
};

/// StorageCommitmentPullModelSOPClassRetired, "1.2.840.10008.1.20.2"
pub static StorageCommitmentPullModelSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.1.20.2",
    ident: "StorageCommitmentPullModelSOPClassRetired",
    name: "Storage Commitment Pull Model SOP Class (Retired)",
};

/// StorageCommitmentPullModelSOPInstanceRetired, "1.2.840.10008.1.20.2.1"
pub static StorageCommitmentPullModelSOPInstanceRetired: &'static UID = &UID {
    uid: "1.2.840.10008.1.20.2.1",
    ident: "StorageCommitmentPullModelSOPInstanceRetired",
    name: "Storage Commitment Pull Model SOP Instance (Retired)",
};

/// ProceduralEventLoggingSOPClass, "1.2.840.10008.1.40"
pub static ProceduralEventLoggingSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.1.40",
    ident: "ProceduralEventLoggingSOPClass",
    name: "Procedural Event Logging SOP Class",
};

/// ProceduralEventLoggingSOPInstance, "1.2.840.10008.1.40.1"
pub static ProceduralEventLoggingSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.1.40.1",
    ident: "ProceduralEventLoggingSOPInstance",
    name: "Procedural Event Logging SOP Instance",
};

/// SubstanceAdministrationLoggingSOPClass, "1.2.840.10008.1.42"
pub static SubstanceAdministrationLoggingSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.1.42",
    ident: "SubstanceAdministrationLoggingSOPClass",
    name: "Substance Administration Logging SOP Class",
};

/// SubstanceAdministrationLoggingSOPInstance, "1.2.840.10008.1.42.1"
pub static SubstanceAdministrationLoggingSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.1.42.1",
    ident: "SubstanceAdministrationLoggingSOPInstance",
    name: "Substance Administration Logging SOP Instance",
};

/// DICOMUIDRegistry, "1.2.840.10008.2.6.1"
pub static DICOMUIDRegistry: &'static UID = &UID {
    uid: "1.2.840.10008.2.6.1",
    ident: "DICOMUIDRegistry",
    name: "DICOM UID Registry",
};

/// DICOMControlledTerminology, "1.2.840.10008.2.16.4"
pub static DICOMControlledTerminology: &'static UID = &UID {
    uid: "1.2.840.10008.2.16.4",
    ident: "DICOMControlledTerminology",
    name: "DICOM Controlled Terminology",
};

/// DICOMApplicationContextName, "1.2.840.10008.3.1.1.1"
pub static DICOMApplicationContextName: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.1.1",
    ident: "DICOMApplicationContextName",
    name: "DICOM Application Context Name",
};

/// DetachedPatientManagementSOPClassRetired, "1.2.840.10008.3.1.2.1.1"
pub static DetachedPatientManagementSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.1.1",
    ident: "DetachedPatientManagementSOPClassRetired",
    name: "Detached Patient Management SOP Class (Retired)",
};

/// DetachedPatientManagementMetaSOPClassRetired, "1.2.840.10008.3.1.2.1.4"
pub static DetachedPatientManagementMetaSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.1.4",
    ident: "DetachedPatientManagementMetaSOPClassRetired",
    name: "Detached Patient Management Meta SOP Class (Retired)",
};

/// DetachedVisitManagementSOPClassRetired, "1.2.840.10008.3.1.2.2.1"
pub static DetachedVisitManagementSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.2.1",
    ident: "DetachedVisitManagementSOPClassRetired",
    name: "Detached Visit Management SOP Class (Retired)",
};

/// DetachedStudyManagementSOPClassRetired, "1.2.840.10008.3.1.2.3.1"
pub static DetachedStudyManagementSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.3.1",
    ident: "DetachedStudyManagementSOPClassRetired",
    name: "Detached Study Management SOP Class (Retired)",
};

/// StudyComponentManagementSOPClassRetired, "1.2.840.10008.3.1.2.3.2"
pub static StudyComponentManagementSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.3.2",
    ident: "StudyComponentManagementSOPClassRetired",
    name: "Study Component Management SOP Class (Retired)",
};

/// ModalityPerformedProcedureStepSOPClass, "1.2.840.10008.3.1.2.3.3"
pub static ModalityPerformedProcedureStepSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.3.3",
    ident: "ModalityPerformedProcedureStepSOPClass",
    name: "Modality Performed Procedure Step SOP Class",
};

/// ModalityPerformedProcedureStepRetrieveSOPClass, "1.2.840.10008.3.1.2.3.4"
pub static ModalityPerformedProcedureStepRetrieveSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.3.4",
    ident: "ModalityPerformedProcedureStepRetrieveSOPClass",
    name: "Modality Performed Procedure Step Retrieve SOP Class",
};

/// ModalityPerformedProcedureStepNotificationSOPClass, "1.2.840.10008.3.1.2.3.5"
pub static ModalityPerformedProcedureStepNotificationSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.3.5",
    ident: "ModalityPerformedProcedureStepNotificationSOPClass",
    name: "Modality Performed Procedure Step Notification SOP Class",
};

/// DetachedResultsManagementSOPClassRetired, "1.2.840.10008.3.1.2.5.1"
pub static DetachedResultsManagementSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.5.1",
    ident: "DetachedResultsManagementSOPClassRetired",
    name: "Detached Results Management SOP Class (Retired)",
};

/// DetachedResultsManagementMetaSOPClassRetired, "1.2.840.10008.3.1.2.5.4"
pub static DetachedResultsManagementMetaSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.5.4",
    ident: "DetachedResultsManagementMetaSOPClassRetired",
    name: "Detached Results Management Meta SOP Class (Retired)",
};

/// DetachedStudyManagementMetaSOPClassRetired, "1.2.840.10008.3.1.2.5.5"
pub static DetachedStudyManagementMetaSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.5.5",
    ident: "DetachedStudyManagementMetaSOPClassRetired",
    name: "Detached Study Management Meta SOP Class (Retired)",
};

/// DetachedInterpretationManagementSOPClassRetired, "1.2.840.10008.3.1.2.6.1"
pub static DetachedInterpretationManagementSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.3.1.2.6.1",
    ident: "DetachedInterpretationManagementSOPClassRetired",
    name: "Detached Interpretation Management SOP Class (Retired)",
};

/// StorageServiceClass, "1.2.840.10008.4.2"
pub static StorageServiceClass: &'static UID = &UID {
    uid: "1.2.840.10008.4.2",
    ident: "StorageServiceClass",
    name: "Storage Service Class",
};

/// BasicFilmSessionSOPClass, "1.2.840.10008.5.1.1.1"
pub static BasicFilmSessionSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.1",
    ident: "BasicFilmSessionSOPClass",
    name: "Basic Film Session SOP Class",
};

/// BasicFilmBoxSOPClass, "1.2.840.10008.5.1.1.2"
pub static BasicFilmBoxSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.2",
    ident: "BasicFilmBoxSOPClass",
    name: "Basic Film Box SOP Class",
};

/// BasicGrayscaleImageBoxSOPClass, "1.2.840.10008.5.1.1.4"
pub static BasicGrayscaleImageBoxSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.4",
    ident: "BasicGrayscaleImageBoxSOPClass",
    name: "Basic Grayscale Image Box SOP Class",
};

/// BasicColorImageBoxSOPClass, "1.2.840.10008.5.1.1.4.1"
pub static BasicColorImageBoxSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.4.1",
    ident: "BasicColorImageBoxSOPClass",
    name: "Basic Color Image Box SOP Class",
};

/// ReferencedImageBoxSOPClassRetired, "1.2.840.10008.5.1.1.4.2"
pub static ReferencedImageBoxSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.4.2",
    ident: "ReferencedImageBoxSOPClassRetired",
    name: "Referenced Image Box SOP Class (Retired)",
};

/// BasicGrayscalePrintManagementMetaSOPClass, "1.2.840.10008.5.1.1.9"
pub static BasicGrayscalePrintManagementMetaSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.9",
    ident: "BasicGrayscalePrintManagementMetaSOPClass",
    name: "Basic Grayscale Print Management Meta SOP Class",
};

/// ReferencedGrayscalePrintManagementMetaSOPClassRetired, "1.2.840.10008.5.1.1.9.1"
pub static ReferencedGrayscalePrintManagementMetaSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.9.1",
    ident: "ReferencedGrayscalePrintManagementMetaSOPClassRetired",
    name: "Referenced Grayscale Print Management Meta SOP Class (Retired)",
};

/// PrintJobSOPClass, "1.2.840.10008.5.1.1.14"
pub static PrintJobSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.14",
    ident: "PrintJobSOPClass",
    name: "Print Job SOP Class",
};

/// BasicAnnotationBoxSOPClass, "1.2.840.10008.5.1.1.15"
pub static BasicAnnotationBoxSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.15",
    ident: "BasicAnnotationBoxSOPClass",
    name: "Basic Annotation Box SOP Class",
};

/// PrinterSOPClass, "1.2.840.10008.5.1.1.16"
pub static PrinterSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.16",
    ident: "PrinterSOPClass",
    name: "Printer SOP Class",
};

/// PrinterConfigurationRetrievalSOPClass, "1.2.840.10008.5.1.1.16.376"
pub static PrinterConfigurationRetrievalSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.16.376",
    ident: "PrinterConfigurationRetrievalSOPClass",
    name: "Printer Configuration Retrieval SOP Class",
};

/// PrinterSOPInstance, "1.2.840.10008.5.1.1.17"
pub static PrinterSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.17",
    ident: "PrinterSOPInstance",
    name: "Printer SOP Instance",
};

/// PrinterConfigurationRetrievalSOPInstance, "1.2.840.10008.5.1.1.17.376"
pub static PrinterConfigurationRetrievalSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.17.376",
    ident: "PrinterConfigurationRetrievalSOPInstance",
    name: "Printer Configuration Retrieval SOP Instance",
};

/// BasicColorPrintManagementMetaSOPClass, "1.2.840.10008.5.1.1.18"
pub static BasicColorPrintManagementMetaSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.18",
    ident: "BasicColorPrintManagementMetaSOPClass",
    name: "Basic Color Print Management Meta SOP Class",
};

/// ReferencedColorPrintManagementMetaSOPClassRetired, "1.2.840.10008.5.1.1.18.1"
pub static ReferencedColorPrintManagementMetaSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.18.1",
    ident: "ReferencedColorPrintManagementMetaSOPClassRetired",
    name: "Referenced Color Print Management Meta SOP Class (Retired)",
};

/// VOILUTBoxSOPClass, "1.2.840.10008.5.1.1.22"
pub static VOILUTBoxSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.22",
    ident: "VOILUTBoxSOPClass",
    name: "VOI LUT Box SOP Class",
};

/// PresentationLUTSOPClass, "1.2.840.10008.5.1.1.23"
pub static PresentationLUTSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.23",
    ident: "PresentationLUTSOPClass",
    name: "Presentation LUT SOP Class",
};

/// ImageOverlayBoxSOPClassRetired, "1.2.840.10008.5.1.1.24"
pub static ImageOverlayBoxSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.24",
    ident: "ImageOverlayBoxSOPClassRetired",
    name: "Image Overlay Box SOP Class (Retired)",
};

/// BasicPrintImageOverlayBoxSOPClassRetired, "1.2.840.10008.5.1.1.24.1"
pub static BasicPrintImageOverlayBoxSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.24.1",
    ident: "BasicPrintImageOverlayBoxSOPClassRetired",
    name: "Basic Print Image Overlay Box SOP Class (Retired)",
};

/// PrintQueueSOPInstanceRetired, "1.2.840.10008.5.1.1.25"
pub static PrintQueueSOPInstanceRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.25",
    ident: "PrintQueueSOPInstanceRetired",
    name: "Print Queue SOP Instance (Retired)",
};

/// PrintQueueManagementSOPClassRetired, "1.2.840.10008.5.1.1.26"
pub static PrintQueueManagementSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.26",
    ident: "PrintQueueManagementSOPClassRetired",
    name: "Print Queue Management SOP Class (Retired)",
};

/// StoredPrintStorageSOPClassRetired, "1.2.840.10008.5.1.1.27"
pub static StoredPrintStorageSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.27",
    ident: "StoredPrintStorageSOPClassRetired",
    name: "Stored Print Storage SOP Class (Retired)",
};

/// HardcopyGrayscaleImageStorageSOPClassRetired, "1.2.840.10008.5.1.1.29"
pub static HardcopyGrayscaleImageStorageSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.29",
    ident: "HardcopyGrayscaleImageStorageSOPClassRetired",
    name: "Hardcopy Grayscale Image Storage SOP Class (Retired)",
};

/// HardcopyColorImageStorageSOPClassRetired, "1.2.840.10008.5.1.1.30"
pub static HardcopyColorImageStorageSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.30",
    ident: "HardcopyColorImageStorageSOPClassRetired",
    name: "Hardcopy Color Image Storage SOP Class (Retired)",
};

/// PullPrintRequestSOPClassRetired, "1.2.840.10008.5.1.1.31"
pub static PullPrintRequestSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.31",
    ident: "PullPrintRequestSOPClassRetired",
    name: "Pull Print Request SOP Class (Retired)",
};

/// PullStoredPrintManagementMetaSOPClassRetired, "1.2.840.10008.5.1.1.32"
pub static PullStoredPrintManagementMetaSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.32",
    ident: "PullStoredPrintManagementMetaSOPClassRetired",
    name: "Pull Stored Print Management Meta SOP Class (Retired)",
};

/// MediaCreationManagementSOPClassUID, "1.2.840.10008.5.1.1.33"
pub static MediaCreationManagementSOPClassUID: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.33",
    ident: "MediaCreationManagementSOPClassUID",
    name: "Media Creation Management SOP Class UID",
};

/// DisplaySystemSOPClass, "1.2.840.10008.5.1.1.40"
pub static DisplaySystemSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.40",
    ident: "DisplaySystemSOPClass",
    name: "Display System SOP Class",
};

/// DisplaySystemSOPInstance, "1.2.840.10008.5.1.1.40.1"
pub static DisplaySystemSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.1.40.1",
    ident: "DisplaySystemSOPInstance",
    name: "Display System SOP Instance",
};

/// ComputedRadiographyImageStorage, "1.2.840.10008.5.1.4.1.1.1"
pub static ComputedRadiographyImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.1",
    ident: "ComputedRadiographyImageStorage",
    name: "Computed Radiography Image Storage",
};

/// DigitalXRayImageStorageForPresentation, "1.2.840.10008.5.1.4.1.1.1.1"
pub static DigitalXRayImageStorageForPresentation: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.1.1",
    ident: "DigitalXRayImageStorageForPresentation",
    name: "Digital X-Ray Image Storage - For Presentation",
};

/// DigitalXRayImageStorageForProcessing, "1.2.840.10008.5.1.4.1.1.1.1.1"
pub static DigitalXRayImageStorageForProcessing: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.1.1.1",
    ident: "DigitalXRayImageStorageForProcessing",
    name: "Digital X-Ray Image Storage - For Processing",
};

/// DigitalMammographyXRayImageStorageForPresentation, "1.2.840.10008.5.1.4.1.1.1.2"
pub static DigitalMammographyXRayImageStorageForPresentation: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.1.2",
    ident: "DigitalMammographyXRayImageStorageForPresentation",
    name: "Digital Mammography X-Ray Image Storage - For Presentation",
};

/// DigitalMammographyXRayImageStorageForProcessing, "1.2.840.10008.5.1.4.1.1.1.2.1"
pub static DigitalMammographyXRayImageStorageForProcessing: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.1.2.1",
    ident: "DigitalMammographyXRayImageStorageForProcessing",
    name: "Digital Mammography X-Ray Image Storage - For Processing",
};

/// DigitalIntraOralXRayImageStorageForPresentation, "1.2.840.10008.5.1.4.1.1.1.3"
pub static DigitalIntraOralXRayImageStorageForPresentation: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.1.3",
    ident: "DigitalIntraOralXRayImageStorageForPresentation",
    name: "Digital Intra-Oral X-Ray Image Storage - For Presentation",
};

/// DigitalIntraOralXRayImageStorageForProcessing, "1.2.840.10008.5.1.4.1.1.1.3.1"
pub static DigitalIntraOralXRayImageStorageForProcessing: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.1.3.1",
    ident: "DigitalIntraOralXRayImageStorageForProcessing",
    name: "Digital Intra-Oral X-Ray Image Storage - For Processing",
};

/// CTImageStorage, "1.2.840.10008.5.1.4.1.1.2"
pub static CTImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.2",
    ident: "CTImageStorage",
    name: "CT Image Storage",
};

/// EnhancedCTImageStorage, "1.2.840.10008.5.1.4.1.1.2.1"
pub static EnhancedCTImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.2.1",
    ident: "EnhancedCTImageStorage",
    name: "Enhanced CT Image Storage",
};

/// LegacyConvertedEnhancedCTImageStorage, "1.2.840.10008.5.1.4.1.1.2.2"
pub static LegacyConvertedEnhancedCTImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.2.2",
    ident: "LegacyConvertedEnhancedCTImageStorage",
    name: "Legacy Converted Enhanced CT Image Storage",
};

/// UltrasoundMultiFrameImageStorageRetired, "1.2.840.10008.5.1.4.1.1.3"
pub static UltrasoundMultiFrameImageStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.3",
    ident: "UltrasoundMultiFrameImageStorageRetired",
    name: "Ultrasound Multi-frame Image Storage (Retired)",
};

/// UltrasoundMultiFrameImageStorage, "1.2.840.10008.5.1.4.1.1.3.1"
pub static UltrasoundMultiFrameImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.3.1",
    ident: "UltrasoundMultiFrameImageStorage",
    name: "Ultrasound Multi-frame Image Storage",
};

/// MRImageStorage, "1.2.840.10008.5.1.4.1.1.4"
pub static MRImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.4",
    ident: "MRImageStorage",
    name: "MR Image Storage",
};

/// EnhancedMRImageStorage, "1.2.840.10008.5.1.4.1.1.4.1"
pub static EnhancedMRImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.4.1",
    ident: "EnhancedMRImageStorage",
    name: "Enhanced MR Image Storage",
};

/// MRSpectroscopyStorage, "1.2.840.10008.5.1.4.1.1.4.2"
pub static MRSpectroscopyStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.4.2",
    ident: "MRSpectroscopyStorage",
    name: "MR Spectroscopy Storage",
};

/// EnhancedMRColorImageStorage, "1.2.840.10008.5.1.4.1.1.4.3"
pub static EnhancedMRColorImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.4.3",
    ident: "EnhancedMRColorImageStorage",
    name: "Enhanced MR Color Image Storage",
};

/// LegacyConvertedEnhancedMRImageStorage, "1.2.840.10008.5.1.4.1.1.4.4"
pub static LegacyConvertedEnhancedMRImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.4.4",
    ident: "LegacyConvertedEnhancedMRImageStorage",
    name: "Legacy Converted Enhanced MR Image Storage",
};

/// NuclearMedicineImageStorageRetired, "1.2.840.10008.5.1.4.1.1.5"
pub static NuclearMedicineImageStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.5",
    ident: "NuclearMedicineImageStorageRetired",
    name: "Nuclear Medicine Image Storage (Retired)",
};

/// UltrasoundImageStorageRetired, "1.2.840.10008.5.1.4.1.1.6"
pub static UltrasoundImageStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.6",
    ident: "UltrasoundImageStorageRetired",
    name: "Ultrasound Image Storage (Retired)",
};

/// UltrasoundImageStorage, "1.2.840.10008.5.1.4.1.1.6.1"
pub static UltrasoundImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.6.1",
    ident: "UltrasoundImageStorage",
    name: "Ultrasound Image Storage",
};

/// EnhancedUSVolumeStorage, "1.2.840.10008.5.1.4.1.1.6.2"
pub static EnhancedUSVolumeStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.6.2",
    ident: "EnhancedUSVolumeStorage",
    name: "Enhanced US Volume Storage",
};

/// SecondaryCaptureImageStorage, "1.2.840.10008.5.1.4.1.1.7"
pub static SecondaryCaptureImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.7",
    ident: "SecondaryCaptureImageStorage",
    name: "Secondary Capture Image Storage",
};

/// MultiFrameSingleBitSecondaryCaptureImageStorage, "1.2.840.10008.5.1.4.1.1.7.1"
pub static MultiFrameSingleBitSecondaryCaptureImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.7.1",
    ident: "MultiFrameSingleBitSecondaryCaptureImageStorage",
    name: "Multi-frame Single Bit Secondary Capture Image Storage",
};

/// MultiFrameGrayscaleByteSecondaryCaptureImageStorage, "1.2.840.10008.5.1.4.1.1.7.2"
pub static MultiFrameGrayscaleByteSecondaryCaptureImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.7.2",
    ident: "MultiFrameGrayscaleByteSecondaryCaptureImageStorage",
    name: "Multi-frame Grayscale Byte Secondary Capture Image Storage",
};

/// MultiFrameGrayscaleWordSecondaryCaptureImageStorage, "1.2.840.10008.5.1.4.1.1.7.3"
pub static MultiFrameGrayscaleWordSecondaryCaptureImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.7.3",
    ident: "MultiFrameGrayscaleWordSecondaryCaptureImageStorage",
    name: "Multi-frame Grayscale Word Secondary Capture Image Storage",
};

/// MultiFrameTrueColorSecondaryCaptureImageStorage, "1.2.840.10008.5.1.4.1.1.7.4"
pub static MultiFrameTrueColorSecondaryCaptureImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.7.4",
    ident: "MultiFrameTrueColorSecondaryCaptureImageStorage",
    name: "Multi-frame True Color Secondary Capture Image Storage",
};

/// StandaloneOverlayStorageRetired, "1.2.840.10008.5.1.4.1.1.8"
pub static StandaloneOverlayStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.8",
    ident: "StandaloneOverlayStorageRetired",
    name: "Standalone Overlay Storage (Retired)",
};

/// StandaloneCurveStorageRetired, "1.2.840.10008.5.1.4.1.1.9"
pub static StandaloneCurveStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9",
    ident: "StandaloneCurveStorageRetired",
    name: "Standalone Curve Storage (Retired)",
};

/// WaveformStorageTrialRetired, "1.2.840.10008.5.1.4.1.1.9.1"
pub static WaveformStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.1",
    ident: "WaveformStorageTrialRetired",
    name: "Waveform Storage - Trial (Retired)",
};

/// TwelveLeadECGWaveformStorage, "1.2.840.10008.5.1.4.1.1.9.1.1"
pub static TwelveLeadECGWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.1.1",
    ident: "TwelveLeadECGWaveformStorage",
    name: "12-lead ECG Waveform Storage",
};

/// GeneralECGWaveformStorage, "1.2.840.10008.5.1.4.1.1.9.1.2"
pub static GeneralECGWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.1.2",
    ident: "GeneralECGWaveformStorage",
    name: "General ECG Waveform Storage",
};

/// AmbulatoryECGWaveformStorage, "1.2.840.10008.5.1.4.1.1.9.1.3"
pub static AmbulatoryECGWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.1.3",
    ident: "AmbulatoryECGWaveformStorage",
    name: "Ambulatory ECG Waveform Storage",
};

/// HemodynamicWaveformStorage, "1.2.840.10008.5.1.4.1.1.9.2.1"
pub static HemodynamicWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.2.1",
    ident: "HemodynamicWaveformStorage",
    name: "Hemodynamic Waveform Storage",
};

/// CardiacElectrophysiologyWaveformStorage, "1.2.840.10008.5.1.4.1.1.9.3.1"
pub static CardiacElectrophysiologyWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.3.1",
    ident: "CardiacElectrophysiologyWaveformStorage",
    name: "Cardiac Electrophysiology Waveform Storage",
};

/// BasicVoiceAudioWaveformStorage, "1.2.840.10008.5.1.4.1.1.9.4.1"
pub static BasicVoiceAudioWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.4.1",
    ident: "BasicVoiceAudioWaveformStorage",
    name: "Basic Voice Audio Waveform Storage",
};

/// GeneralAudioWaveformStorage, "1.2.840.10008.5.1.4.1.1.9.4.2"
pub static GeneralAudioWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.4.2",
    ident: "GeneralAudioWaveformStorage",
    name: "General Audio Waveform Storage",
};

/// ArterialPulseWaveformStorage, "1.2.840.10008.5.1.4.1.1.9.5.1"
pub static ArterialPulseWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.5.1",
    ident: "ArterialPulseWaveformStorage",
    name: "Arterial Pulse Waveform Storage",
};

/// RespiratoryWaveformStorage, "1.2.840.10008.5.1.4.1.1.9.6.1"
pub static RespiratoryWaveformStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.9.6.1",
    ident: "RespiratoryWaveformStorage",
    name: "Respiratory Waveform Storage",
};

/// StandaloneModalityLUTStorageRetired, "1.2.840.10008.5.1.4.1.1.10"
pub static StandaloneModalityLUTStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.10",
    ident: "StandaloneModalityLUTStorageRetired",
    name: "Standalone Modality LUT Storage (Retired)",
};

/// StandaloneVOILUTStorageRetired, "1.2.840.10008.5.1.4.1.1.11"
pub static StandaloneVOILUTStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.11",
    ident: "StandaloneVOILUTStorageRetired",
    name: "Standalone VOI LUT Storage (Retired)",
};

/// GrayscaleSoftcopyPresentationStateStorageSOPClass, "1.2.840.10008.5.1.4.1.1.11.1"
pub static GrayscaleSoftcopyPresentationStateStorageSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.11.1",
    ident: "GrayscaleSoftcopyPresentationStateStorageSOPClass",
    name: "Grayscale Softcopy Presentation State Storage SOP Class",
};

/// ColorSoftcopyPresentationStateStorageSOPClass, "1.2.840.10008.5.1.4.1.1.11.2"
pub static ColorSoftcopyPresentationStateStorageSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.11.2",
    ident: "ColorSoftcopyPresentationStateStorageSOPClass",
    name: "Color Softcopy Presentation State Storage SOP Class",
};

/// PseudoColorSoftcopyPresentationStateStorageSOPClass, "1.2.840.10008.5.1.4.1.1.11.3"
pub static PseudoColorSoftcopyPresentationStateStorageSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.11.3",
    ident: "PseudoColorSoftcopyPresentationStateStorageSOPClass",
    name: "Pseudo-Color Softcopy Presentation State Storage SOP Class",
};

/// BlendingSoftcopyPresentationStateStorageSOPClass, "1.2.840.10008.5.1.4.1.1.11.4"
pub static BlendingSoftcopyPresentationStateStorageSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.11.4",
    ident: "BlendingSoftcopyPresentationStateStorageSOPClass",
    name: "Blending Softcopy Presentation State Storage SOP Class",
};

/// XAXRFGrayscaleSoftcopyPresentationStateStorage, "1.2.840.10008.5.1.4.1.1.11.5"
pub static XAXRFGrayscaleSoftcopyPresentationStateStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.11.5",
    ident: "XAXRFGrayscaleSoftcopyPresentationStateStorage",
    name: "XA/XRF Grayscale Softcopy Presentation State Storage",
};

/// XRayAngiographicImageStorage, "1.2.840.10008.5.1.4.1.1.12.1"
pub static XRayAngiographicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.12.1",
    ident: "XRayAngiographicImageStorage",
    name: "X-Ray Angiographic Image Storage",
};

/// EnhancedXAImageStorage, "1.2.840.10008.5.1.4.1.1.12.1.1"
pub static EnhancedXAImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.12.1.1",
    ident: "EnhancedXAImageStorage",
    name: "Enhanced XA Image Storage",
};

/// XRayRadiofluoroscopicImageStorage, "1.2.840.10008.5.1.4.1.1.12.2"
pub static XRayRadiofluoroscopicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.12.2",
    ident: "XRayRadiofluoroscopicImageStorage",
    name: "X-Ray Radiofluoroscopic Image Storage",
};

/// EnhancedXRFImageStorage, "1.2.840.10008.5.1.4.1.1.12.2.1"
pub static EnhancedXRFImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.12.2.1",
    ident: "EnhancedXRFImageStorage",
    name: "Enhanced XRF Image Storage",
};

/// XRayAngiographicBiPlaneImageStorageRetired, "1.2.840.10008.5.1.4.1.1.12.3"
pub static XRayAngiographicBiPlaneImageStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.12.3",
    ident: "XRayAngiographicBiPlaneImageStorageRetired",
    name: "X-Ray Angiographic Bi-Plane Image Storage (Retired)",
};

/// XRay3DAngiographicImageStorage, "1.2.840.10008.5.1.4.1.1.13.1.1"
pub static XRay3DAngiographicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.13.1.1",
    ident: "XRay3DAngiographicImageStorage",
    name: "X-Ray 3D Angiographic Image Storage",
};

/// XRay3DCraniofacialImageStorage, "1.2.840.10008.5.1.4.1.1.13.1.2"
pub static XRay3DCraniofacialImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.13.1.2",
    ident: "XRay3DCraniofacialImageStorage",
    name: "X-Ray 3D Craniofacial Image Storage",
};

/// BreastTomosynthesisImageStorage, "1.2.840.10008.5.1.4.1.1.13.1.3"
pub static BreastTomosynthesisImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.13.1.3",
    ident: "BreastTomosynthesisImageStorage",
    name: "Breast Tomosynthesis Image Storage",
};

/// BreastProjectionXRayImageStorageForPresentation, "1.2.840.10008.5.1.4.1.1.13.1.4"
pub static BreastProjectionXRayImageStorageForPresentation: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.13.1.4",
    ident: "BreastProjectionXRayImageStorageForPresentation",
    name: "Breast Projection X-Ray Image Storage - For Presentation",
};

/// BreastProjectionXRayImageStorageForProcessing, "1.2.840.10008.5.1.4.1.1.13.1.5"
pub static BreastProjectionXRayImageStorageForProcessing: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.13.1.5",
    ident: "BreastProjectionXRayImageStorageForProcessing",
    name: "Breast Projection X-Ray Image Storage - For Processing",
};

/// IntravascularOpticalCoherenceTomographyImageStorageForPresentation, "1.2.840.10008.5.1.4.1.1.14.1" 
pub static IntravascularOpticalCoherenceTomographyImageStorageForPresentation: &'static UID = &UID {
    ident: "IntravascularOpticalCoherenceTomographyImageStorageForPresentation",
    uid: "1.2.840.10008.5.1.4.1.1.14.1",
    name: "Intravascular Optical Coherence Tomography Image Storage - For Presentation",
};
    
/// IntravascularOpticalCoherenceTomographyImageStorageForProcessing, "1.2.840.10008.5.1.4.1.1.14.2"
pub static IntravascularOpticalCoherenceTomographyImageStorageForProcessing: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.14.2",
    ident: "IntravascularOpticalCoherenceTomographyImageStorageForProcessing",
    name: "Intravascular Optical Coherence Tomography Image Storage - For Processing",
};

/// NuclearMedicineImageStorage, "1.2.840.10008.5.1.4.1.1.20"
pub static NuclearMedicineImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.20",
    ident: "NuclearMedicineImageStorage",
    name: "Nuclear Medicine Image Storage",
};

/// ParametricMapStorage, "1.2.840.10008.5.1.4.1.1.30"
pub static ParametricMapStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.30",
    ident: "ParametricMapStorage",
    name: "Parametric Map Storage",
};

/// RawDataStorage, "1.2.840.10008.5.1.4.1.1.66"
pub static RawDataStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.66",
    ident: "RawDataStorage",
    name: "Raw Data Storage",
};

/// SpatialRegistrationStorage, "1.2.840.10008.5.1.4.1.1.66.1"
pub static SpatialRegistrationStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.66.1",
    ident: "SpatialRegistrationStorage",
    name: "Spatial Registration Storage",
};

/// SpatialFiducialsStorage, "1.2.840.10008.5.1.4.1.1.66.2"
pub static SpatialFiducialsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.66.2",
    ident: "SpatialFiducialsStorage",
    name: "Spatial Fiducials Storage",
};

/// DeformableSpatialRegistrationStorage, "1.2.840.10008.5.1.4.1.1.66.3"
pub static DeformableSpatialRegistrationStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.66.3",
    ident: "DeformableSpatialRegistrationStorage",
    name: "Deformable Spatial Registration Storage",
};

/// SegmentationStorage, "1.2.840.10008.5.1.4.1.1.66.4"
pub static SegmentationStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.66.4",
    ident: "SegmentationStorage",
    name: "Segmentation Storage",
};

/// SurfaceSegmentationStorage, "1.2.840.10008.5.1.4.1.1.66.5"
pub static SurfaceSegmentationStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.66.5",
    ident: "SurfaceSegmentationStorage",
    name: "Surface Segmentation Storage",
};

/// RealWorldValueMappingStorage, "1.2.840.10008.5.1.4.1.1.67"
pub static RealWorldValueMappingStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.67",
    ident: "RealWorldValueMappingStorage",
    name: "Real World Value Mapping Storage",
};

/// SurfaceScanMeshStorage, "1.2.840.10008.5.1.4.1.1.68.1"
pub static SurfaceScanMeshStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.68.1",
    ident: "SurfaceScanMeshStorage",
    name: "Surface Scan Mesh Storage",
};

/// SurfaceScanPointCloudStorage, "1.2.840.10008.5.1.4.1.1.68.2"
pub static SurfaceScanPointCloudStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.68.2",
    ident: "SurfaceScanPointCloudStorage",
    name: "Surface Scan Point Cloud Storage",
};

/// VLImageStorageTrialRetired, "1.2.840.10008.5.1.4.1.1.77.1"
pub static VLImageStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1",
    ident: "VLImageStorageTrialRetired",
    name: "VL Image Storage - Trial (Retired)",
};

/// VLMultiFrameImageStorageTrialRetired, "1.2.840.10008.5.1.4.1.1.77.2"
pub static VLMultiFrameImageStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.2",
    ident: "VLMultiFrameImageStorageTrialRetired",
    name: "VL Multi-frame Image Storage - Trial (Retired)",
};

/// VLEndoscopicImageStorage, "1.2.840.10008.5.1.4.1.1.77.1.1"
pub static VLEndoscopicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.1",
    ident: "VLEndoscopicImageStorage",
    name: "VL Endoscopic Image Storage",
};

/// VideoEndoscopicImageStorage, "1.2.840.10008.5.1.4.1.1.77.1.1.1"
pub static VideoEndoscopicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.1.1",
    ident: "VideoEndoscopicImageStorage",
    name: "Video Endoscopic Image Storage",
};

/// VLMicroscopicImageStorage, "1.2.840.10008.5.1.4.1.1.77.1.2"
pub static VLMicroscopicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.2",
    ident: "VLMicroscopicImageStorage",
    name: "VL Microscopic Image Storage",
};

/// VideoMicroscopicImageStorage, "1.2.840.10008.5.1.4.1.1.77.1.2.1"
pub static VideoMicroscopicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.2.1",
    ident: "VideoMicroscopicImageStorage",
    name: "Video Microscopic Image Storage",
};

/// VLSlideCoordinatesMicroscopicImageStorage, "1.2.840.10008.5.1.4.1.1.77.1.3"
pub static VLSlideCoordinatesMicroscopicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.3",
    ident: "VLSlideCoordinatesMicroscopicImageStorage",
    name: "VL Slide-Coordinates Microscopic Image Storage",
};

/// VLPhotographicImageStorage, "1.2.840.10008.5.1.4.1.1.77.1.4"
pub static VLPhotographicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.4",
    ident: "VLPhotographicImageStorage",
    name: "VL Photographic Image Storage",
};

/// VideoPhotographicImageStorage, "1.2.840.10008.5.1.4.1.1.77.1.4.1"
pub static VideoPhotographicImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.4.1",
    ident: "VideoPhotographicImageStorage",
    name: "Video Photographic Image Storage",
};

/// OphthalmicPhotography8BitImageStorage, "1.2.840.10008.5.1.4.1.1.77.1.5.1"
pub static OphthalmicPhotography8BitImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.1",
    ident: "OphthalmicPhotography8BitImageStorage",
    name: "Ophthalmic Photography 8 Bit Image Storage",
};

/// OphthalmicPhotography16BitImageStorage, "1.2.840.10008.5.1.4.1.1.77.1.5.2"
pub static OphthalmicPhotography16BitImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.2",
    ident: "OphthalmicPhotography16BitImageStorage",
    name: "Ophthalmic Photography 16 Bit Image Storage",
};

/// StereometricRelationshipStorage, "1.2.840.10008.5.1.4.1.1.77.1.5.3"
pub static StereometricRelationshipStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.3",
    ident: "StereometricRelationshipStorage",
    name: "Stereometric Relationship Storage",
};

/// OphthalmicTomographyImageStorage, "1.2.840.10008.5.1.4.1.1.77.1.5.4"
pub static OphthalmicTomographyImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.4",
    ident: "OphthalmicTomographyImageStorage",
    name: "Ophthalmic Tomography Image Storage",
};

/// WideFieldOphthalmicPhotographyStereographicProjectionImageStorage, "1.2.840.10008.5.1.4.1.1.77.1.5.5"
pub static WideFieldOphthalmicPhotographyStereographicProjectionImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.5",
    ident: "WideFieldOphthalmicPhotographyStereographicProjectionImageStorage",
    name: "Wide Field Ophthalmic Photography Stereographic Projection Image Storage",
};

/// WideFieldOphthalmicPhotography3DCoordinatesImageStorage, "1.2.840.10008.5.1.4.1.1.77.1.5.6"
pub static WideFieldOphthalmicPhotography3DCoordinatesImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.6",
    ident: "WideFieldOphthalmicPhotography3DCoordinatesImageStorage",
    name: "Wide Field Ophthalmic Photography 3D Coordinates Image Storage",
};

/// VLWholeSlideMicroscopyImageStorage, "1.2.840.10008.5.1.4.1.1.77.1.6"
pub static VLWholeSlideMicroscopyImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.77.1.6",
    ident: "VLWholeSlideMicroscopyImageStorage",
    name: "VL Whole Slide Microscopy Image Storage",
};

/// LensometryMeasurementsStorage, "1.2.840.10008.5.1.4.1.1.78.1"
pub static LensometryMeasurementsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.1",
    ident: "LensometryMeasurementsStorage",
    name: "Lensometry Measurements Storage",
};

/// AutorefractionMeasurementsStorage, "1.2.840.10008.5.1.4.1.1.78.2"
pub static AutorefractionMeasurementsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.2",
    ident: "AutorefractionMeasurementsStorage",
    name: "Autorefraction Measurements Storage",
};

/// KeratometryMeasurementsStorage, "1.2.840.10008.5.1.4.1.1.78.3"
pub static KeratometryMeasurementsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.3",
    ident: "KeratometryMeasurementsStorage",
    name: "Keratometry Measurements Storage",
};

/// SubjectiveRefractionMeasurementsStorage, "1.2.840.10008.5.1.4.1.1.78.4"
pub static SubjectiveRefractionMeasurementsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.4",
    ident: "SubjectiveRefractionMeasurementsStorage",
    name: "Subjective Refraction Measurements Storage",
};

/// VisualAcuityMeasurementsStorage, "1.2.840.10008.5.1.4.1.1.78.5"
pub static VisualAcuityMeasurementsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.5",
    ident: "VisualAcuityMeasurementsStorage",
    name: "Visual Acuity Measurements Storage",
};

/// SpectaclePrescriptionReportStorage, "1.2.840.10008.5.1.4.1.1.78.6"
pub static SpectaclePrescriptionReportStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.6",
    ident: "SpectaclePrescriptionReportStorage",
    name: "Spectacle Prescription Report Storage",
};

/// OphthalmicAxialMeasurementsStorage, "1.2.840.10008.5.1.4.1.1.78.7"
pub static OphthalmicAxialMeasurementsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.7",
    ident: "OphthalmicAxialMeasurementsStorage",
    name: "Ophthalmic Axial Measurements Storage",
};

/// IntraocularLensCalculationsStorage, "1.2.840.10008.5.1.4.1.1.78.8"
pub static IntraocularLensCalculationsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.78.8",
    ident: "IntraocularLensCalculationsStorage",
    name: "Intraocular Lens Calculations Storage",
};

/// MacularGridThicknessAndVolumeReportStorage, "1.2.840.10008.5.1.4.1.1.79.1"
pub static MacularGridThicknessAndVolumeReportStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.79.1",
    ident: "MacularGridThicknessAndVolumeReportStorage",
    name: "Macular Grid Thickness and Volume Report Storage",
};

/// OphthalmicVisualFieldStaticPerimetryMeasurementsStorage, "1.2.840.10008.5.1.4.1.1.80.1"
pub static OphthalmicVisualFieldStaticPerimetryMeasurementsStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.80.1",
    ident: "OphthalmicVisualFieldStaticPerimetryMeasurementsStorage",
    name: "Ophthalmic Visual Field Static Perimetry Measurements Storage",
};

/// OphthalmicThicknessMapStorage, "1.2.840.10008.5.1.4.1.1.81.1"
pub static OphthalmicThicknessMapStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.81.1",
    ident: "OphthalmicThicknessMapStorage",
    name: "Ophthalmic Thickness Map Storage",
};

/// CornealTopographyMapStorage, "1.2.840.10008.5.1.4.1.1.82.1"
pub static CornealTopographyMapStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.82.1",
    ident: "CornealTopographyMapStorage",
    name: "Corneal Topography Map Storage",
};

/// TextSRStorageTrialRetired, "1.2.840.10008.5.1.4.1.1.88.1"
pub static TextSRStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.1",
    ident: "TextSRStorageTrialRetired",
    name: "Text SR Storage - Trial (Retired)",
};

/// AudioSRStorageTrialRetired, "1.2.840.10008.5.1.4.1.1.88.2"
pub static AudioSRStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.2",
    ident: "AudioSRStorageTrialRetired",
    name: "Audio SR Storage - Trial (Retired)",
};

/// DetailSRStorageTrialRetired, "1.2.840.10008.5.1.4.1.1.88.3"
pub static DetailSRStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.3",
    ident: "DetailSRStorageTrialRetired",
    name: "Detail SR Storage - Trial (Retired)",
};

/// ComprehensiveSRStorageTrialRetired, "1.2.840.10008.5.1.4.1.1.88.4"
pub static ComprehensiveSRStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.4",
    ident: "ComprehensiveSRStorageTrialRetired",
    name: "Comprehensive SR Storage - Trial (Retired)",
};

/// BasicTextSRStorage, "1.2.840.10008.5.1.4.1.1.88.11"
pub static BasicTextSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.11",
    ident: "BasicTextSRStorage",
    name: "Basic Text SR Storage",
};

/// EnhancedSRStorage, "1.2.840.10008.5.1.4.1.1.88.22"
pub static EnhancedSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.22",
    ident: "EnhancedSRStorage",
    name: "Enhanced SR Storage",
};

/// ComprehensiveSRStorage, "1.2.840.10008.5.1.4.1.1.88.33"
pub static ComprehensiveSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.33",
    ident: "ComprehensiveSRStorage",
    name: "Comprehensive SR Storage",
};

/// Comprehensive3DSRStorage, "1.2.840.10008.5.1.4.1.1.88.34"
pub static Comprehensive3DSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.34",
    ident: "Comprehensive3DSRStorage",
    name: "Comprehensive 3D SR Storage",
};

/// ProcedureLogStorage, "1.2.840.10008.5.1.4.1.1.88.40"
pub static ProcedureLogStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.40",
    ident: "ProcedureLogStorage",
    name: "Procedure Log Storage",
};

/// MammographyCADSRStorage, "1.2.840.10008.5.1.4.1.1.88.50"
pub static MammographyCADSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.50",
    ident: "MammographyCADSRStorage",
    name: "Mammography CAD SR Storage",
};

/// KeyObjectSelectionDocumentStorage, "1.2.840.10008.5.1.4.1.1.88.59"
pub static KeyObjectSelectionDocumentStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.59",
    ident: "KeyObjectSelectionDocumentStorage",
    name: "Key Object Selection Document Storage",
};

/// ChestCADSRStorage, "1.2.840.10008.5.1.4.1.1.88.65"
pub static ChestCADSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.65",
    ident: "ChestCADSRStorage",
    name: "Chest CAD SR Storage",
};

/// XRayRadiationDoseSRStorage, "1.2.840.10008.5.1.4.1.1.88.67"
pub static XRayRadiationDoseSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.67",
    ident: "XRayRadiationDoseSRStorage",
    name: "X-Ray Radiation Dose SR Storage",
};

/// RadiopharmaceuticalRadiationDoseSRStorage, "1.2.840.10008.5.1.4.1.1.88.68"
pub static RadiopharmaceuticalRadiationDoseSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.68",
    ident: "RadiopharmaceuticalRadiationDoseSRStorage",
    name: "Radiopharmaceutical Radiation Dose SR Storage",
};

/// ColonCADSRStorage, "1.2.840.10008.5.1.4.1.1.88.69"
pub static ColonCADSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.69",
    ident: "ColonCADSRStorage",
    name: "Colon CAD SR Storage",
};

/// ImplantationPlanSRStorage, "1.2.840.10008.5.1.4.1.1.88.70"
pub static ImplantationPlanSRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.88.70",
    ident: "ImplantationPlanSRStorage",
    name: "Implantation Plan SR Storage",
};

/// EncapsulatedPDFStorage, "1.2.840.10008.5.1.4.1.1.104.1"
pub static EncapsulatedPDFStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.104.1",
    ident: "EncapsulatedPDFStorage",
    name: "Encapsulated PDF Storage",
};

/// EncapsulatedCDAStorage, "1.2.840.10008.5.1.4.1.1.104.2"
pub static EncapsulatedCDAStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.104.2",
    ident: "EncapsulatedCDAStorage",
    name: "Encapsulated CDA Storage",
};

/// PositronEmissionTomographyImageStorage, "1.2.840.10008.5.1.4.1.1.128"
pub static PositronEmissionTomographyImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.128",
    ident: "PositronEmissionTomographyImageStorage",
    name: "Positron Emission Tomography Image Storage",
};

/// LegacyConvertedEnhancedPETImageStorage, "1.2.840.10008.5.1.4.1.1.128.1"
pub static LegacyConvertedEnhancedPETImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.128.1",
    ident: "LegacyConvertedEnhancedPETImageStorage",
    name: "Legacy Converted Enhanced PET Image Storage",
};

/// StandalonePETCurveStorageRetired, "1.2.840.10008.5.1.4.1.1.129"
pub static StandalonePETCurveStorageRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.129",
    ident: "StandalonePETCurveStorageRetired",
    name: "Standalone PET Curve Storage (Retired)",
};

/// EnhancedPETImageStorage, "1.2.840.10008.5.1.4.1.1.130"
pub static EnhancedPETImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.130",
    ident: "EnhancedPETImageStorage",
    name: "Enhanced PET Image Storage",
};

/// BasicStructuredDisplayStorage, "1.2.840.10008.5.1.4.1.1.131"
pub static BasicStructuredDisplayStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.131",
    ident: "BasicStructuredDisplayStorage",
    name: "Basic Structured Display Storage",
};

/// RTImageStorage, "1.2.840.10008.5.1.4.1.1.481.1"
pub static RTImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.1",
    ident: "RTImageStorage",
    name: "RT Image Storage",
};

/// RTDoseStorage, "1.2.840.10008.5.1.4.1.1.481.2"
pub static RTDoseStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.2",
    ident: "RTDoseStorage",
    name: "RT Dose Storage",
};

/// RTStructureSetStorage, "1.2.840.10008.5.1.4.1.1.481.3"
pub static RTStructureSetStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.3",
    ident: "RTStructureSetStorage",
    name: "RT Structure Set Storage",
};

/// RTBeamsTreatmentRecordStorage, "1.2.840.10008.5.1.4.1.1.481.4"
pub static RTBeamsTreatmentRecordStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.4",
    ident: "RTBeamsTreatmentRecordStorage",
    name: "RT Beams Treatment Record Storage",
};

/// RTPlanStorage, "1.2.840.10008.5.1.4.1.1.481.5"
pub static RTPlanStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.5",
    ident: "RTPlanStorage",
    name: "RT Plan Storage",
};

/// RTBrachyTreatmentRecordStorage, "1.2.840.10008.5.1.4.1.1.481.6"
pub static RTBrachyTreatmentRecordStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.6",
    ident: "RTBrachyTreatmentRecordStorage",
    name: "RT Brachy Treatment Record Storage",
};

/// RTTreatmentSummaryRecordStorage, "1.2.840.10008.5.1.4.1.1.481.7"
pub static RTTreatmentSummaryRecordStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.7",
    ident: "RTTreatmentSummaryRecordStorage",
    name: "RT Treatment Summary Record Storage",
};

/// RTIonPlanStorage, "1.2.840.10008.5.1.4.1.1.481.8"
pub static RTIonPlanStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.8",
    ident: "RTIonPlanStorage",
    name: "RT Ion Plan Storage",
};

/// RTIonBeamsTreatmentRecordStorage, "1.2.840.10008.5.1.4.1.1.481.9"
pub static RTIonBeamsTreatmentRecordStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.481.9",
    ident: "RTIonBeamsTreatmentRecordStorage",
    name: "RT Ion Beams Treatment Record Storage",
};

/// DICOSCTImageStorage, "1.2.840.10008.5.1.4.1.1.501.1"
pub static DICOSCTImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.501.1",
    ident: "DICOSCTImageStorage",
    name: "DICOS CT Image Storage",
};

/// DICOSDigitalXRayImageStorageForPresentation, "1.2.840.10008.5.1.4.1.1.501.2.1"
pub static DICOSDigitalXRayImageStorageForPresentation: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.501.2.1",
    ident: "DICOSDigitalXRayImageStorageForPresentation",
    name: "DICOS Digital X-Ray Image Storage - For Presentation",
};

/// DICOSDigitalXRayImageStorageForProcessing, "1.2.840.10008.5.1.4.1.1.501.2.2"
pub static DICOSDigitalXRayImageStorageForProcessing: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.501.2.2",
    ident: "DICOSDigitalXRayImageStorageForProcessing",
    name: "DICOS Digital X-Ray Image Storage - For Processing",
};

/// DICOSThreatDetectionReportStorage, "1.2.840.10008.5.1.4.1.1.501.3"
pub static DICOSThreatDetectionReportStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.501.3",
    ident: "DICOSThreatDetectionReportStorage",
    name: "DICOS Threat Detection Report Storage",
};

/// DICOS2DAITStorage, "1.2.840.10008.5.1.4.1.1.501.4"
pub static DICOS2DAITStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.501.4",
    ident: "DICOS2DAITStorage",
    name: "DICOS 2D AIT Storage",
};

/// DICOS3DAITStorage, "1.2.840.10008.5.1.4.1.1.501.5"
pub static DICOS3DAITStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.501.5",
    ident: "DICOS3DAITStorage",
    name: "DICOS 3D AIT Storage",
};

/// DICOSQuadrupoleResonanceQRStorage, "1.2.840.10008.5.1.4.1.1.501.6"
pub static DICOSQuadrupoleResonanceQRStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.501.6",
    ident: "DICOSQuadrupoleResonanceQRStorage",
    name: "DICOS Quadrupole Resonance (QR) Storage",
};

/// EddyCurrentImageStorage, "1.2.840.10008.5.1.4.1.1.601.1"
pub static EddyCurrentImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.601.1",
    ident: "EddyCurrentImageStorage",
    name: "Eddy Current Image Storage",
};

/// EddyCurrentMultiFrameImageStorage, "1.2.840.10008.5.1.4.1.1.601.2"
pub static EddyCurrentMultiFrameImageStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.1.601.2",
    ident: "EddyCurrentMultiFrameImageStorage",
    name: "Eddy Current Multi-frame Image Storage",
};

/// PatientRootQueryRetrieveInformationModelFIND, "1.2.840.10008.5.1.4.1.2.1.1"
pub static PatientRootQueryRetrieveInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.1.1",
    ident: "PatientRootQueryRetrieveInformationModelFIND",
    name: "Patient Root Query/Retrieve Information Model - FIND",
};

/// PatientRootQueryRetrieveInformationModelMOVE, "1.2.840.10008.5.1.4.1.2.1.2"
pub static PatientRootQueryRetrieveInformationModelMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.1.2",
    ident: "PatientRootQueryRetrieveInformationModelMOVE",
    name: "Patient Root Query/Retrieve Information Model - MOVE",
};

/// PatientRootQueryRetrieveInformationModelGET, "1.2.840.10008.5.1.4.1.2.1.3"
pub static PatientRootQueryRetrieveInformationModelGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.1.3",
    ident: "PatientRootQueryRetrieveInformationModelGET",
    name: "Patient Root Query/Retrieve Information Model - GET",
};

/// StudyRootQueryRetrieveInformationModelFIND, "1.2.840.10008.5.1.4.1.2.2.1"
pub static StudyRootQueryRetrieveInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.2.1",
    ident: "StudyRootQueryRetrieveInformationModelFIND",
    name: "Study Root Query/Retrieve Information Model - FIND",
};

/// StudyRootQueryRetrieveInformationModelMOVE, "1.2.840.10008.5.1.4.1.2.2.2"
pub static StudyRootQueryRetrieveInformationModelMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.2.2",
    ident: "StudyRootQueryRetrieveInformationModelMOVE",
    name: "Study Root Query/Retrieve Information Model - MOVE",
};

/// StudyRootQueryRetrieveInformationModelGET, "1.2.840.10008.5.1.4.1.2.2.3"
pub static StudyRootQueryRetrieveInformationModelGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.2.3",
    ident: "StudyRootQueryRetrieveInformationModelGET",
    name: "Study Root Query/Retrieve Information Model - GET",
};

/// PatientStudyOnlyQueryRetrieveInformationModelFINDRetired, "1.2.840.10008.5.1.4.1.2.3.1"
pub static PatientStudyOnlyQueryRetrieveInformationModelFINDRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.3.1",
    ident: "PatientStudyOnlyQueryRetrieveInformationModelFINDRetired",
    name: "Patient/Study Only Query/Retrieve Information Model - FIND (Retired)",
};

/// PatientStudyOnlyQueryRetrieveInformationModelMOVERetired, "1.2.840.10008.5.1.4.1.2.3.2"
pub static PatientStudyOnlyQueryRetrieveInformationModelMOVERetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.3.2",
    ident: "PatientStudyOnlyQueryRetrieveInformationModelMOVERetired",
    name: "Patient/Study Only Query/Retrieve Information Model - MOVE (Retired)",
};

/// PatientStudyOnlyQueryRetrieveInformationModelGETRetired, "1.2.840.10008.5.1.4.1.2.3.3"
pub static PatientStudyOnlyQueryRetrieveInformationModelGETRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.3.3",
    ident: "PatientStudyOnlyQueryRetrieveInformationModelGETRetired",
    name: "Patient/Study Only Query/Retrieve Information Model - GET (Retired)",
};

/// CompositeInstanceRootRetrieveMOVE, "1.2.840.10008.5.1.4.1.2.4.2"
pub static CompositeInstanceRootRetrieveMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.4.2",
    ident: "CompositeInstanceRootRetrieveMOVE",
    name: "Composite Instance Root Retrieve - MOVE",
};

/// CompositeInstanceRootRetrieveGET, "1.2.840.10008.5.1.4.1.2.4.3"
pub static CompositeInstanceRootRetrieveGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.4.3",
    ident: "CompositeInstanceRootRetrieveGET",
    name: "Composite Instance Root Retrieve - GET",
};

/// CompositeInstanceRetrieveWithoutBulkDataGET, "1.2.840.10008.5.1.4.1.2.5.3"
pub static CompositeInstanceRetrieveWithoutBulkDataGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.1.2.5.3",
    ident: "CompositeInstanceRetrieveWithoutBulkDataGET",
    name: "Composite Instance Retrieve Without Bulk Data - GET",
};

/// ModalityWorklistInformationModelFIND, "1.2.840.10008.5.1.4.31"
pub static ModalityWorklistInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.31",
    ident: "ModalityWorklistInformationModelFIND",
    name: "Modality Worklist Information Model - FIND",
};

/// GeneralPurposeWorklistManagementMetaSOPClassRetired, "1.2.840.10008.5.1.4.32"
pub static GeneralPurposeWorklistManagementMetaSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.32",
    ident: "GeneralPurposeWorklistManagementMetaSOPClassRetired",
    name: "General Purpose Worklist Management Meta SOP Class (Retired)",
};

/// GeneralPurposeWorklistInformationModelFINDRetired, "1.2.840.10008.5.1.4.32.1"
pub static GeneralPurposeWorklistInformationModelFINDRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.32.1",
    ident: "GeneralPurposeWorklistInformationModelFINDRetired",
    name: "General Purpose Worklist Information Model - FIND (Retired)",
};

/// GeneralPurposeScheduledProcedureStepSOPClassRetired, "1.2.840.10008.5.1.4.32.2"
pub static GeneralPurposeScheduledProcedureStepSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.32.2",
    ident: "GeneralPurposeScheduledProcedureStepSOPClassRetired",
    name: "General Purpose Scheduled Procedure Step SOP Class (Retired)",
};

/// GeneralPurposePerformedProcedureStepSOPClassRetired, "1.2.840.10008.5.1.4.32.3"
pub static GeneralPurposePerformedProcedureStepSOPClassRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.32.3",
    ident: "GeneralPurposePerformedProcedureStepSOPClassRetired",
    name: "General Purpose Performed Procedure Step SOP Class (Retired)",
};

/// InstanceAvailabilityNotificationSOPClass, "1.2.840.10008.5.1.4.33"
pub static InstanceAvailabilityNotificationSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.33",
    ident: "InstanceAvailabilityNotificationSOPClass",
    name: "Instance Availability Notification SOP Class",
};

/// RTBeamsDeliveryInstructionStorageTrialRetired, "1.2.840.10008.5.1.4.34.1"
pub static RTBeamsDeliveryInstructionStorageTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.1",
    ident: "RTBeamsDeliveryInstructionStorageTrialRetired",
    name: "RT Beams Delivery Instruction Storage - Trial (Retired)",
};

/// RTConventionalMachineVerificationTrialRetired, "1.2.840.10008.5.1.4.34.2"
pub static RTConventionalMachineVerificationTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.2",
    ident: "RTConventionalMachineVerificationTrialRetired",
    name: "RT Conventional Machine Verification - Trial (Retired)",
};

/// RTIonMachineVerificationTrialRetired, "1.2.840.10008.5.1.4.34.3"
pub static RTIonMachineVerificationTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.3",
    ident: "RTIonMachineVerificationTrialRetired",
    name: "RT Ion Machine Verification - Trial (Retired)",
};

/// UnifiedWorklistAndProcedureStepServiceClassTrialRetired, "1.2.840.10008.5.1.4.34.4"
pub static UnifiedWorklistAndProcedureStepServiceClassTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.4",
    ident: "UnifiedWorklistAndProcedureStepServiceClassTrialRetired",
    name: "Unified Worklist and Procedure Step Service Class - Trial (Retired)",
};

/// UnifiedProcedureStepPushSOPClassTrialRetired, "1.2.840.10008.5.1.4.34.4.1"
pub static UnifiedProcedureStepPushSOPClassTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.4.1",
    ident: "UnifiedProcedureStepPushSOPClassTrialRetired",
    name: "Unified Procedure Step - Push SOP Class - Trial (Retired)",
};

/// UnifiedProcedureStepWatchSOPClassTrialRetired, "1.2.840.10008.5.1.4.34.4.2"
pub static UnifiedProcedureStepWatchSOPClassTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.4.2",
    ident: "UnifiedProcedureStepWatchSOPClassTrialRetired",
    name: "Unified Procedure Step - Watch SOP Class - Trial (Retired)",
};

/// UnifiedProcedureStepPullSOPClassTrialRetired, "1.2.840.10008.5.1.4.34.4.3"
pub static UnifiedProcedureStepPullSOPClassTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.4.3",
    ident: "UnifiedProcedureStepPullSOPClassTrialRetired",
    name: "Unified Procedure Step - Pull SOP Class - Trial (Retired)",
};

/// UnifiedProcedureStepEventSOPClassTrialRetired, "1.2.840.10008.5.1.4.34.4.4"
pub static UnifiedProcedureStepEventSOPClassTrialRetired: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.4.4",
    ident: "UnifiedProcedureStepEventSOPClassTrialRetired",
    name: "Unified Procedure Step - Event SOP Class - Trial (Retired)",
};

/// UPSGlobalSubscriptionSOPInstance, "1.2.840.10008.5.1.4.34.5"
pub static UPSGlobalSubscriptionSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.5",
    ident: "UPSGlobalSubscriptionSOPInstance",
    name: "UPS Global Subscription SOP Instance",
};

/// UPSFilteredGlobalSubscriptionSOPInstance, "1.2.840.10008.5.1.4.34.5.1"
pub static UPSFilteredGlobalSubscriptionSOPInstance: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.5.1",
    ident: "UPSFilteredGlobalSubscriptionSOPInstance",
    name: "UPS Filtered Global Subscription SOP Instance",
};

/// UnifiedWorklistAndProcedureStepServiceClass, "1.2.840.10008.5.1.4.34.6"
pub static UnifiedWorklistAndProcedureStepServiceClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.6",
    ident: "UnifiedWorklistAndProcedureStepServiceClass",
    name: "Unified Worklist and Procedure Step Service Class",
};

/// UnifiedProcedureStepPushSOPClass, "1.2.840.10008.5.1.4.34.6.1"
pub static UnifiedProcedureStepPushSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.6.1",
    ident: "UnifiedProcedureStepPushSOPClass",
    name: "Unified Procedure Step - Push SOP Class",
};

/// UnifiedProcedureStepWatchSOPClass, "1.2.840.10008.5.1.4.34.6.2"
pub static UnifiedProcedureStepWatchSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.6.2",
    ident: "UnifiedProcedureStepWatchSOPClass",
    name: "Unified Procedure Step - Watch SOP Class",
};

/// UnifiedProcedureStepPullSOPClass, "1.2.840.10008.5.1.4.34.6.3"
pub static UnifiedProcedureStepPullSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.6.3",
    ident: "UnifiedProcedureStepPullSOPClass",
    name: "Unified Procedure Step - Pull SOP Class",
};

/// UnifiedProcedureStepEventSOPClass, "1.2.840.10008.5.1.4.34.6.4"
pub static UnifiedProcedureStepEventSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.6.4",
    ident: "UnifiedProcedureStepEventSOPClass",
    name: "Unified Procedure Step - Event SOP Class",
};

/// RTBeamsDeliveryInstructionStorage, "1.2.840.10008.5.1.4.34.7"
pub static RTBeamsDeliveryInstructionStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.7",
    ident: "RTBeamsDeliveryInstructionStorage",
    name: "RT Beams Delivery Instruction Storage",
};

/// RTConventionalMachineVerification, "1.2.840.10008.5.1.4.34.8"
pub static RTConventionalMachineVerification: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.8",
    ident: "RTConventionalMachineVerification",
    name: "RT Conventional Machine Verification",
};

/// RTIonMachineVerification, "1.2.840.10008.5.1.4.34.9"
pub static RTIonMachineVerification: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.34.9",
    ident: "RTIonMachineVerification",
    name: "RT Ion Machine Verification",
};

/// GeneralRelevantPatientInformationQuery, "1.2.840.10008.5.1.4.37.1"
pub static GeneralRelevantPatientInformationQuery: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.37.1",
    ident: "GeneralRelevantPatientInformationQuery",
    name: "General Relevant Patient Information Query",
};

/// BreastImagingRelevantPatientInformationQuery, "1.2.840.10008.5.1.4.37.2"
pub static BreastImagingRelevantPatientInformationQuery: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.37.2",
    ident: "BreastImagingRelevantPatientInformationQuery",
    name: "Breast Imaging Relevant Patient Information Query",
};

/// CardiacRelevantPatientInformationQuery, "1.2.840.10008.5.1.4.37.3"
pub static CardiacRelevantPatientInformationQuery: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.37.3",
    ident: "CardiacRelevantPatientInformationQuery",
    name: "Cardiac Relevant Patient Information Query",
};

/// HangingProtocolStorage, "1.2.840.10008.5.1.4.38.1"
pub static HangingProtocolStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.38.1",
    ident: "HangingProtocolStorage",
    name: "Hanging Protocol Storage",
};

/// HangingProtocolInformationModelFIND, "1.2.840.10008.5.1.4.38.2"
pub static HangingProtocolInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.38.2",
    ident: "HangingProtocolInformationModelFIND",
    name: "Hanging Protocol Information Model - FIND",
};

/// HangingProtocolInformationModelMOVE, "1.2.840.10008.5.1.4.38.3"
pub static HangingProtocolInformationModelMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.38.3",
    ident: "HangingProtocolInformationModelMOVE",
    name: "Hanging Protocol Information Model - MOVE",
};

/// HangingProtocolInformationModelGET, "1.2.840.10008.5.1.4.38.4"
pub static HangingProtocolInformationModelGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.38.4",
    ident: "HangingProtocolInformationModelGET",
    name: "Hanging Protocol Information Model - GET",
};

/// ColorPaletteStorage, "1.2.840.10008.5.1.4.39.1"
pub static ColorPaletteStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.39.1",
    ident: "ColorPaletteStorage",
    name: "Color Palette Storage",
};

/// ColorPaletteInformationModelFIND, "1.2.840.10008.5.1.4.39.2"
pub static ColorPaletteInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.39.2",
    ident: "ColorPaletteInformationModelFIND",
    name: "Color Palette Information Model - FIND",
};

/// ColorPaletteInformationModelMOVE, "1.2.840.10008.5.1.4.39.3"
pub static ColorPaletteInformationModelMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.39.3",
    ident: "ColorPaletteInformationModelMOVE",
    name: "Color Palette Information Model - MOVE",
};

/// ColorPaletteInformationModelGET, "1.2.840.10008.5.1.4.39.4"
pub static ColorPaletteInformationModelGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.39.4",
    ident: "ColorPaletteInformationModelGET",
    name: "Color Palette Information Model - GET",
};

/// ProductCharacteristicsQuerySOPClass, "1.2.840.10008.5.1.4.41"
pub static ProductCharacteristicsQuerySOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.41",
    ident: "ProductCharacteristicsQuerySOPClass",
    name: "Product Characteristics Query SOP Class",
};

/// SubstanceApprovalQuerySOPClass, "1.2.840.10008.5.1.4.42"
pub static SubstanceApprovalQuerySOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.42",
    ident: "SubstanceApprovalQuerySOPClass",
    name: "Substance Approval Query SOP Class",
};

/// GenericImplantTemplateStorage, "1.2.840.10008.5.1.4.43.1"
pub static GenericImplantTemplateStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.43.1",
    ident: "GenericImplantTemplateStorage",
    name: "Generic Implant Template Storage",
};

/// GenericImplantTemplateInformationModelFIND, "1.2.840.10008.5.1.4.43.2"
pub static GenericImplantTemplateInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.43.2",
    ident: "GenericImplantTemplateInformationModelFIND",
    name: "Generic Implant Template Information Model - FIND",
};

/// GenericImplantTemplateInformationModelMOVE, "1.2.840.10008.5.1.4.43.3"
pub static GenericImplantTemplateInformationModelMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.43.3",
    ident: "GenericImplantTemplateInformationModelMOVE",
    name: "Generic Implant Template Information Model - MOVE",
};

/// GenericImplantTemplateInformationModelGET, "1.2.840.10008.5.1.4.43.4"
pub static GenericImplantTemplateInformationModelGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.43.4",
    ident: "GenericImplantTemplateInformationModelGET",
    name: "Generic Implant Template Information Model - GET",
};

/// ImplantAssemblyTemplateStorage, "1.2.840.10008.5.1.4.44.1"
pub static ImplantAssemblyTemplateStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.44.1",
    ident: "ImplantAssemblyTemplateStorage",
    name: "Implant Assembly Template Storage",
};

/// ImplantAssemblyTemplateInformationModelFIND, "1.2.840.10008.5.1.4.44.2"
pub static ImplantAssemblyTemplateInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.44.2",
    ident: "ImplantAssemblyTemplateInformationModelFIND",
    name: "Implant Assembly Template Information Model - FIND",
};

/// ImplantAssemblyTemplateInformationModelMOVE, "1.2.840.10008.5.1.4.44.3"
pub static ImplantAssemblyTemplateInformationModelMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.44.3",
    ident: "ImplantAssemblyTemplateInformationModelMOVE",
    name: "Implant Assembly Template Information Model - MOVE",
};

/// ImplantAssemblyTemplateInformationModelGET, "1.2.840.10008.5.1.4.44.4"
pub static ImplantAssemblyTemplateInformationModelGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.44.4",
    ident: "ImplantAssemblyTemplateInformationModelGET",
    name: "Implant Assembly Template Information Model - GET",
};

/// ImplantTemplateGroupStorage, "1.2.840.10008.5.1.4.45.1"
pub static ImplantTemplateGroupStorage: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.45.1",
    ident: "ImplantTemplateGroupStorage",
    name: "Implant Template Group Storage",
};

/// ImplantTemplateGroupInformationModelFIND, "1.2.840.10008.5.1.4.45.2"
pub static ImplantTemplateGroupInformationModelFIND: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.45.2",
    ident: "ImplantTemplateGroupInformationModelFIND",
    name: "Implant Template Group Information Model - FIND",
};

/// ImplantTemplateGroupInformationModelMOVE, "1.2.840.10008.5.1.4.45.3"
pub static ImplantTemplateGroupInformationModelMOVE: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.45.3",
    ident: "ImplantTemplateGroupInformationModelMOVE",
    name: "Implant Template Group Information Model - MOVE",
};

/// ImplantTemplateGroupInformationModelGET, "1.2.840.10008.5.1.4.45.4"
pub static ImplantTemplateGroupInformationModelGET: &'static UID = &UID {
    uid: "1.2.840.10008.5.1.4.45.4",
    ident: "ImplantTemplateGroupInformationModelGET",
    name: "Implant Template Group Information Model - GET",
};

/// NativeDICOMModel, "1.2.840.10008.7.1.1"
pub static NativeDICOMModel: &'static UID = &UID {
    uid: "1.2.840.10008.7.1.1",
    ident: "NativeDICOMModel",
    name: "Native DICOM Model",
};

/// AbstractMultiDimensionalImageModel, "1.2.840.10008.7.1.2"
pub static AbstractMultiDimensionalImageModel: &'static UID = &UID {
    uid: "1.2.840.10008.7.1.2",
    ident: "AbstractMultiDimensionalImageModel",
    name: "Abstract Multi-Dimensional Image Model",
};

/// DICOMContentMappingResource, "1.2.840.10008.8.1.1"
pub static DICOMContentMappingResource: &'static UID = &UID {
    uid: "1.2.840.10008.8.1.1",
    ident: "DICOMContentMappingResource",
    name: "DICOM Content Mapping Resource",
};

/// dicomDeviceName, "1.2.840.10008.15.0.3.1"
pub static dicomDeviceName: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.1",
    ident: "dicomDeviceName",
    name: "dicomDeviceName",
};

/// dicomDescription, "1.2.840.10008.15.0.3.2"
pub static dicomDescription: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.2",
    ident: "dicomDescription",
    name: "dicomDescription",
};

/// dicomManufacturer, "1.2.840.10008.15.0.3.3"
pub static dicomManufacturer: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.3",
    ident: "dicomManufacturer",
    name: "dicomManufacturer",
};

/// dicomManufacturerModelName, "1.2.840.10008.15.0.3.4"
pub static dicomManufacturerModelName: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.4",
    ident: "dicomManufacturerModelName",
    name: "dicomManufacturerModelName",
};

/// dicomSoftwareVersion, "1.2.840.10008.15.0.3.5"
pub static dicomSoftwareVersion: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.5",
    ident: "dicomSoftwareVersion",
    name: "dicomSoftwareVersion",
};

/// dicomVendorData, "1.2.840.10008.15.0.3.6"
pub static dicomVendorData: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.6",
    ident: "dicomVendorData",
    name: "dicomVendorData",
};

/// dicomAETitle, "1.2.840.10008.15.0.3.7"
pub static dicomAETitle: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.7",
    ident: "dicomAETitle",
    name: "dicomAETitle",
};

/// dicomNetworkConnectionReference, "1.2.840.10008.15.0.3.8"
pub static dicomNetworkConnectionReference: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.8",
    ident: "dicomNetworkConnectionReference",
    name: "dicomNetworkConnectionReference",
};

/// dicomApplicationCluster, "1.2.840.10008.15.0.3.9"
pub static dicomApplicationCluster: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.9",
    ident: "dicomApplicationCluster",
    name: "dicomApplicationCluster",
};

/// dicomAssociationInitiator, "1.2.840.10008.15.0.3.10"
pub static dicomAssociationInitiator: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.10",
    ident: "dicomAssociationInitiator",
    name: "dicomAssociationInitiator",
};

/// dicomAssociationAcceptor, "1.2.840.10008.15.0.3.11"
pub static dicomAssociationAcceptor: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.11",
    ident: "dicomAssociationAcceptor",
    name: "dicomAssociationAcceptor",
};

/// dicomHostname, "1.2.840.10008.15.0.3.12"
pub static dicomHostname: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.12",
    ident: "dicomHostname",
    name: "dicomHostname",
};

/// dicomPort, "1.2.840.10008.15.0.3.13"
pub static dicomPort: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.13",
    ident: "dicomPort",
    name: "dicomPort",
};

/// dicomSOPClass, "1.2.840.10008.15.0.3.14"
pub static dicomSOPClass: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.14",
    ident: "dicomSOPClass",
    name: "dicomSOPClass",
};

/// dicomTransferRole, "1.2.840.10008.15.0.3.15"
pub static dicomTransferRole: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.15",
    ident: "dicomTransferRole",
    name: "dicomTransferRole",
};

/// dicomTransferSyntax, "1.2.840.10008.15.0.3.16"
pub static dicomTransferSyntax: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.16",
    ident: "dicomTransferSyntax",
    name: "dicomTransferSyntax",
};

/// dicomPrimaryDeviceType, "1.2.840.10008.15.0.3.17"
pub static dicomPrimaryDeviceType: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.17",
    ident: "dicomPrimaryDeviceType",
    name: "dicomPrimaryDeviceType",
};

/// dicomRelatedDeviceReference, "1.2.840.10008.15.0.3.18"
pub static dicomRelatedDeviceReference: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.18",
    ident: "dicomRelatedDeviceReference",
    name: "dicomRelatedDeviceReference",
};

/// dicomPreferredCalledAETitle, "1.2.840.10008.15.0.3.19"
pub static dicomPreferredCalledAETitle: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.19",
    ident: "dicomPreferredCalledAETitle",
    name: "dicomPreferredCalledAETitle",
};

/// dicomTLSCyphersuite, "1.2.840.10008.15.0.3.20"
pub static dicomTLSCyphersuite: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.20",
    ident: "dicomTLSCyphersuite",
    name: "dicomTLSCyphersuite",
};

/// dicomAuthorizedNodeCertificateReference, "1.2.840.10008.15.0.3.21"
pub static dicomAuthorizedNodeCertificateReference: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.21",
    ident: "dicomAuthorizedNodeCertificateReference",
    name: "dicomAuthorizedNodeCertificateReference",
};

/// dicomThisNodeCertificateReference, "1.2.840.10008.15.0.3.22"
pub static dicomThisNodeCertificateReference: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.22",
    ident: "dicomThisNodeCertificateReference",
    name: "dicomThisNodeCertificateReference",
};

/// dicomInstalled, "1.2.840.10008.15.0.3.23"
pub static dicomInstalled: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.23",
    ident: "dicomInstalled",
    name: "dicomInstalled",
};

/// dicomStationName, "1.2.840.10008.15.0.3.24"
pub static dicomStationName: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.24",
    ident: "dicomStationName",
    name: "dicomStationName",
};

/// dicomDeviceSerialNumber, "1.2.840.10008.15.0.3.25"
pub static dicomDeviceSerialNumber: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.25",
    ident: "dicomDeviceSerialNumber",
    name: "dicomDeviceSerialNumber",
};

/// dicomInstitutionName, "1.2.840.10008.15.0.3.26"
pub static dicomInstitutionName: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.26",
    ident: "dicomInstitutionName",
    name: "dicomInstitutionName",
};

/// dicomInstitutionAddress, "1.2.840.10008.15.0.3.27"
pub static dicomInstitutionAddress: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.27",
    ident: "dicomInstitutionAddress",
    name: "dicomInstitutionAddress",
};

/// dicomInstitutionDepartmentName, "1.2.840.10008.15.0.3.28"
pub static dicomInstitutionDepartmentName: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.28",
    ident: "dicomInstitutionDepartmentName",
    name: "dicomInstitutionDepartmentName",
};

/// dicomIssuerOfPatientID, "1.2.840.10008.15.0.3.29"
pub static dicomIssuerOfPatientID: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.29",
    ident: "dicomIssuerOfPatientID",
    name: "dicomIssuerOfPatientID",
};

/// dicomPreferredCallingAETitle, "1.2.840.10008.15.0.3.30"
pub static dicomPreferredCallingAETitle: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.30",
    ident: "dicomPreferredCallingAETitle",
    name: "dicomPreferredCallingAETitle",
};

/// dicomSupportedCharacterSet, "1.2.840.10008.15.0.3.31"
pub static dicomSupportedCharacterSet: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.3.31",
    ident: "dicomSupportedCharacterSet",
    name: "dicomSupportedCharacterSet",
};

/// dicomConfigurationRoot, "1.2.840.10008.15.0.4.1"
pub static dicomConfigurationRoot: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.1",
    ident: "dicomConfigurationRoot",
    name: "dicomConfigurationRoot",
};

/// dicomDevicesRoot, "1.2.840.10008.15.0.4.2"
pub static dicomDevicesRoot: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.2",
    ident: "dicomDevicesRoot",
    name: "dicomDevicesRoot",
};

/// dicomUniqueAETitlesRegistryRoot, "1.2.840.10008.15.0.4.3"
pub static dicomUniqueAETitlesRegistryRoot: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.3",
    ident: "dicomUniqueAETitlesRegistryRoot",
    name: "dicomUniqueAETitlesRegistryRoot",
};

/// dicomDevice, "1.2.840.10008.15.0.4.4"
pub static dicomDevice: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.4",
    ident: "dicomDevice",
    name: "dicomDevice",
};

/// dicomNetworkAE, "1.2.840.10008.15.0.4.5"
pub static dicomNetworkAE: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.5",
    ident: "dicomNetworkAE",
    name: "dicomNetworkAE",
};

/// dicomNetworkConnection, "1.2.840.10008.15.0.4.6"
pub static dicomNetworkConnection: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.6",
    ident: "dicomNetworkConnection",
    name: "dicomNetworkConnection",
};

/// dicomUniqueAETitle, "1.2.840.10008.15.0.4.7"
pub static dicomUniqueAETitle: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.7",
    ident: "dicomUniqueAETitle",
    name: "dicomUniqueAETitle",
};

/// dicomTransferCapability, "1.2.840.10008.15.0.4.8"
pub static dicomTransferCapability: &'static UID = &UID {
    uid: "1.2.840.10008.15.0.4.8",
    ident: "dicomTransferCapability",
    name: "dicomTransferCapability",
};

/// UniversalCoordinatedTime, "1.2.840.10008.15.1.1"
pub static UniversalCoordinatedTime: &'static UID = &UID {
    uid: "1.2.840.10008.15.1.1",
    ident: "UniversalCoordinatedTime",
    name: "Universal Coordinated Time",
};
