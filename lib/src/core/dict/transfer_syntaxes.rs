//! This is an auto-generated file. Do not make modifications here.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::dict::uids;
use core::ts::TransferSyntax;

/// Implicit VR Little Endian: Default Transfer Syntax for DICOM
/// 
/// - **UID:** 1.2.840.10008.1.2
pub static ImplicitVRLittleEndian: TransferSyntax = TransferSyntax {
	uid: &uids::ImplicitVRLittleEndian,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// Explicit VR Little Endian
/// 
/// - **UID:** 1.2.840.10008.1.2.1
pub static ExplicitVRLittleEndian: TransferSyntax = TransferSyntax {
	uid: &uids::ExplicitVRLittleEndian,
	explicit_vr: true,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// Deflated Explicit VR Little Endian
/// 
/// - **UID:** 1.2.840.10008.1.2.1.99
pub static DeflatedExplicitVRLittleEndian: TransferSyntax = TransferSyntax {
	uid: &uids::DeflatedExplicitVRLittleEndian,
	explicit_vr: true,
	big_endian: false,
	deflated: true,
	encapsulated: false,
};

/// Explicit VR Big Endian (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.2
pub static ExplicitVRBigEndian: TransferSyntax = TransferSyntax {
	uid: &uids::ExplicitVRBigEndian,
	explicit_vr: true,
	big_endian: true,
	deflated: false,
	encapsulated: false,
};

/// JPEG Baseline (Process 1): Default Transfer Syntax for Lossy JPEG 8 Bit Image Compression
/// 
/// - **UID:** 1.2.840.10008.1.2.4.50
pub static JPEGBaselineProcess1: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGBaselineProcess1,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Extended (Process 2 & 4): Default Transfer Syntax for Lossy JPEG 12 Bit Image Compression (Process 4 only)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.51
pub static JPEGExtendedProcess2_and_4: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGExtendedProcess2_and_4,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Extended (Process 3 & 5) (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.52
pub static JPEGExtendedProcess3_and_5: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGExtendedProcess3_and_5,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Spectral Selection, Non-Hierarchical (Process 6 & 8) (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.53
pub static JPEGSpectralSelectionNonHierarchicalProcess6_and_8: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGSpectralSelectionNonHierarchicalProcess6_and_8,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Spectral Selection, Non-Hierarchical (Process 7 & 9) (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.54
pub static JPEGSpectralSelectionNonHierarchicalProcess7_and_9: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGSpectralSelectionNonHierarchicalProcess7_and_9,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Full Progression, Non-Hierarchical (Process 10 & 12) (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.55
pub static JPEGFullProgressionNonHierarchicalProcess10_and_12: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGFullProgressionNonHierarchicalProcess10_and_12,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Full Progression, Non-Hierarchical (Process 11 & 13) (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.56
pub static JPEGFullProgressionNonHierarchicalProcess11_and_13: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGFullProgressionNonHierarchicalProcess11_and_13,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Lossless, Non-Hierarchical (Process 14)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.57
pub static JPEGLosslessNonHierarchicalProcess14: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGLosslessNonHierarchicalProcess14,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Lossless, Non-Hierarchical (Process 15) (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.58
pub static JPEGLosslessNonHierarchicalProcess15: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGLosslessNonHierarchicalProcess15,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Extended, Hierarchical (Process 16 & 18) (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.59
pub static JPEGExtendedHierarchicalProcess16_and_18: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGExtendedHierarchicalProcess16_and_18,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Extended, Hierarchical (Process 17 & 19) (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.60
pub static JPEGExtendedHierarchicalProcess17_and_19: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGExtendedHierarchicalProcess17_and_19,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Spectral Selection, Hierarchical (Process 20 & 22) (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.61
pub static JPEGSpectralSelectionHierarchicalProcess20_and_22: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGSpectralSelectionHierarchicalProcess20_and_22,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Spectral Selection, Hierarchical (Process 21 & 23) (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.62
pub static JPEGSpectralSelectionHierarchicalProcess21_and_23: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGSpectralSelectionHierarchicalProcess21_and_23,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Full Progression, Hierarchical (Process 24 & 26) (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.63
pub static JPEGFullProgressionHierarchicalProcess24_and_26: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGFullProgressionHierarchicalProcess24_and_26,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Full Progression, Hierarchical (Process 25 & 27) (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.64
pub static JPEGFullProgressionHierarchicalProcess25_and_27: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGFullProgressionHierarchicalProcess25_and_27,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Lossless, Hierarchical (Process 28) (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.65
pub static JPEGLosslessHierarchicalProcess28: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGLosslessHierarchicalProcess28,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Lossless, Hierarchical (Process 29) (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.66
pub static JPEGLosslessHierarchicalProcess29: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGLosslessHierarchicalProcess29,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG Lossless, Non-Hierarchical, First-Order Prediction (Process 14 [Selection Value 1]): Default Transfer Syntax for Lossless JPEG Image Compression
/// 
/// - **UID:** 1.2.840.10008.1.2.4.70
pub static JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG-LS Lossless Image Compression
/// 
/// - **UID:** 1.2.840.10008.1.2.4.80
pub static JPEGLSLosslessImageCompression: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGLSLosslessImageCompression,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG-LS Lossy (Near-Lossless) Image Compression
/// 
/// - **UID:** 1.2.840.10008.1.2.4.81
pub static JPEGLSLossyNearLosslessImageCompression: TransferSyntax = TransferSyntax {
	uid: &uids::JPEGLSLossyNearLosslessImageCompression,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG 2000 Image Compression (Lossless Only)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.90
pub static JPEG2000ImageCompressionLosslessOnly: TransferSyntax = TransferSyntax {
	uid: &uids::JPEG2000ImageCompressionLosslessOnly,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG 2000 Image Compression
/// 
/// - **UID:** 1.2.840.10008.1.2.4.91
pub static JPEG2000ImageCompression: TransferSyntax = TransferSyntax {
	uid: &uids::JPEG2000ImageCompression,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG 2000 Part 2 Multi-component Image Compression (Lossless Only)
/// 
/// - **UID:** 1.2.840.10008.1.2.4.92
pub static JPEG2000Part2MulticomponentImageCompressionLosslessOnly: TransferSyntax = TransferSyntax {
	uid: &uids::JPEG2000Part2MulticomponentImageCompressionLosslessOnly,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPEG 2000 Part 2 Multi-component Image Compression
/// 
/// - **UID:** 1.2.840.10008.1.2.4.93
pub static JPEG2000Part2MulticomponentImageCompression: TransferSyntax = TransferSyntax {
	uid: &uids::JPEG2000Part2MulticomponentImageCompression,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPIP Referenced
/// 
/// - **UID:** 1.2.840.10008.1.2.4.94
pub static JPIPReferenced: TransferSyntax = TransferSyntax {
	uid: &uids::JPIPReferenced,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// JPIP Referenced Deflate
/// 
/// - **UID:** 1.2.840.10008.1.2.4.95
pub static JPIPReferencedDeflate: TransferSyntax = TransferSyntax {
	uid: &uids::JPIPReferencedDeflate,
	explicit_vr: false,
	big_endian: false,
	deflated: true,
	encapsulated: false,
};

/// MPEG2 Main Profile / Main Level
/// 
/// - **UID:** 1.2.840.10008.1.2.4.100
pub static MPEG2MainProfileMainLevel: TransferSyntax = TransferSyntax {
	uid: &uids::MPEG2MainProfileMainLevel,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// MPEG2 Main Profile / High Level
/// 
/// - **UID:** 1.2.840.10008.1.2.4.101
pub static MPEG2MainProfileHighLevel: TransferSyntax = TransferSyntax {
	uid: &uids::MPEG2MainProfileHighLevel,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// MPEG-4 AVC/H.264 High Profile / Level 4.1
/// 
/// - **UID:** 1.2.840.10008.1.2.4.102
pub static MPEG4AVCH264HighProfileLevel41: TransferSyntax = TransferSyntax {
	uid: &uids::MPEG4AVCH264HighProfileLevel41,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// MPEG-4 AVC/H.264 BD-compatible High Profile / Level 4.1
/// 
/// - **UID:** 1.2.840.10008.1.2.4.103
pub static MPEG4AVCH264BDcompatibleHighProfileLevel41: TransferSyntax = TransferSyntax {
	uid: &uids::MPEG4AVCH264BDcompatibleHighProfileLevel41,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// MPEG-4 AVC/H.264 High Profile / Level 4.2 For 2D Video
/// 
/// - **UID:** 1.2.840.10008.1.2.4.104
pub static MPEG4AVCH264HighProfileLevel42For2DVideo: TransferSyntax = TransferSyntax {
	uid: &uids::MPEG4AVCH264HighProfileLevel42For2DVideo,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// MPEG-4 AVC/H.264 High Profile / Level 4.2 For 3D Video
/// 
/// - **UID:** 1.2.840.10008.1.2.4.105
pub static MPEG4AVCH264HighProfileLevel42For3DVideo: TransferSyntax = TransferSyntax {
	uid: &uids::MPEG4AVCH264HighProfileLevel42For3DVideo,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// MPEG-4 AVC/H.264 Stereo High Profile / Level 4.2
/// 
/// - **UID:** 1.2.840.10008.1.2.4.106
pub static MPEG4AVCH264StereoHighProfileLevel42: TransferSyntax = TransferSyntax {
	uid: &uids::MPEG4AVCH264StereoHighProfileLevel42,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// HEVC/H.265 Main Profile / Level 5.1
/// 
/// - **UID:** 1.2.840.10008.1.2.4.107
pub static HEVCH265MainProfileLevel51: TransferSyntax = TransferSyntax {
	uid: &uids::HEVCH265MainProfileLevel51,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// HEVC/H.265 Main 10 Profile / Level 5.1
/// 
/// - **UID:** 1.2.840.10008.1.2.4.108
pub static HEVCH265Main10ProfileLevel51: TransferSyntax = TransferSyntax {
	uid: &uids::HEVCH265Main10ProfileLevel51,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// RLE Lossless
/// 
/// - **UID:** 1.2.840.10008.1.2.5
pub static RLELossless: TransferSyntax = TransferSyntax {
	uid: &uids::RLELossless,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// RFC 2557 MIME encapsulation (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.6.1
pub static RFC2557MIMEencapsulation: TransferSyntax = TransferSyntax {
	uid: &uids::RFC2557MIMEencapsulation,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// XML Encoding (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.2.6.2
pub static XMLEncoding: TransferSyntax = TransferSyntax {
	uid: &uids::XMLEncoding,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

/// Papyrus 3 Implicit VR Little Endian (Retired)
/// 
/// - **UID:** 1.2.840.10008.1.20
pub static Papyrus3ImplicitVRLittleEndian: TransferSyntax = TransferSyntax {
	uid: &uids::Papyrus3ImplicitVRLittleEndian,
	explicit_vr: false,
	big_endian: false,
	deflated: false,
	encapsulated: false,
};

