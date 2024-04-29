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

//! This is an auto-generated file. Do not make modifications here.
//!
//! This contains definitions of DICOM UIDs.

#![allow(non_upper_case_globals, clippy::doc_markdown)]

use crate::core::defn::uid::UID;

/// Verification SOP Class
///
/// - **UID:** 1.2.840.10008.1.1
/// - **UID Type:** SOP Class
pub static VerificationSOPClass: UID = UID {
    ident: "VerificationSOPClass",
    uid: "1.2.840.10008.1.1",
    name: "Verification SOP Class",
};

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

/// Encapsulated Uncompressed Explicit VR Little Endian
///
/// - **UID:** 1.2.840.10008.1.2.1.98
/// - **UID Type:** Transfer Syntax
pub static EncapsulatedUncompressedExplicitVRLittleEndian: UID = UID {
    ident: "EncapsulatedUncompressedExplicitVRLittleEndian",
    uid: "1.2.840.10008.1.2.1.98",
    name: "Encapsulated Uncompressed Explicit VR Little Endian",
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

/// JPEG Baseline (Process 1): Default Transfer Syntax for Lossy JPEG 8 Bit Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.50
/// - **UID Type:** Transfer Syntax
pub static JPEGBaselineProcess1: UID = UID {
    ident: "JPEGBaselineProcess1",
    uid: "1.2.840.10008.1.2.4.50",
    name: "JPEG Baseline (Process 1): Default Transfer Syntax for Lossy JPEG 8 Bit Image Compression",
};

/// JPEG Extended (Process 2 & 4): Default Transfer Syntax for Lossy JPEG 12 Bit Image Compression (Process 4 only)
///
/// - **UID:** 1.2.840.10008.1.2.4.51
/// - **UID Type:** Transfer Syntax
pub static JPEGExtendedProcess2_and_4: UID = UID {
    ident: "JPEGExtendedProcess2_and_4",
    uid: "1.2.840.10008.1.2.4.51",
    name: "JPEG Extended (Process 2 & 4): Default Transfer Syntax for Lossy JPEG 12 Bit Image Compression (Process 4 only)",
};

/// JPEG Extended (Process 3 & 5) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.52
/// - **UID Type:** Transfer Syntax
pub static JPEGExtendedProcess3_and_5: UID = UID {
    ident: "JPEGExtendedProcess3_and_5",
    uid: "1.2.840.10008.1.2.4.52",
    name: "JPEG Extended (Process 3 & 5) (Retired)",
};

/// JPEG Spectral Selection, Non-Hierarchical (Process 6 & 8) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.53
/// - **UID Type:** Transfer Syntax
pub static JPEGSpectralSelectionNonHierarchicalProcess6_and_8: UID = UID {
    ident: "JPEGSpectralSelectionNonHierarchicalProcess6_and_8",
    uid: "1.2.840.10008.1.2.4.53",
    name: "JPEG Spectral Selection, Non-Hierarchical (Process 6 & 8) (Retired)",
};

/// JPEG Spectral Selection, Non-Hierarchical (Process 7 & 9) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.54
/// - **UID Type:** Transfer Syntax
pub static JPEGSpectralSelectionNonHierarchicalProcess7_and_9: UID = UID {
    ident: "JPEGSpectralSelectionNonHierarchicalProcess7_and_9",
    uid: "1.2.840.10008.1.2.4.54",
    name: "JPEG Spectral Selection, Non-Hierarchical (Process 7 & 9) (Retired)",
};

/// JPEG Full Progression, Non-Hierarchical (Process 10 & 12) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.55
/// - **UID Type:** Transfer Syntax
pub static JPEGFullProgressionNonHierarchicalProcess10_and_12: UID = UID {
    ident: "JPEGFullProgressionNonHierarchicalProcess10_and_12",
    uid: "1.2.840.10008.1.2.4.55",
    name: "JPEG Full Progression, Non-Hierarchical (Process 10 & 12) (Retired)",
};

/// JPEG Full Progression, Non-Hierarchical (Process 11 & 13) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.56
/// - **UID Type:** Transfer Syntax
pub static JPEGFullProgressionNonHierarchicalProcess11_and_13: UID = UID {
    ident: "JPEGFullProgressionNonHierarchicalProcess11_and_13",
    uid: "1.2.840.10008.1.2.4.56",
    name: "JPEG Full Progression, Non-Hierarchical (Process 11 & 13) (Retired)",
};

/// JPEG Lossless, Non-Hierarchical (Process 14)
///
/// - **UID:** 1.2.840.10008.1.2.4.57
/// - **UID Type:** Transfer Syntax
pub static JPEGLosslessNonHierarchicalProcess14: UID = UID {
    ident: "JPEGLosslessNonHierarchicalProcess14",
    uid: "1.2.840.10008.1.2.4.57",
    name: "JPEG Lossless, Non-Hierarchical (Process 14)",
};

/// JPEG Lossless, Non-Hierarchical (Process 15) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.58
/// - **UID Type:** Transfer Syntax
pub static JPEGLosslessNonHierarchicalProcess15: UID = UID {
    ident: "JPEGLosslessNonHierarchicalProcess15",
    uid: "1.2.840.10008.1.2.4.58",
    name: "JPEG Lossless, Non-Hierarchical (Process 15) (Retired)",
};

/// JPEG Extended, Hierarchical (Process 16 & 18) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.59
/// - **UID Type:** Transfer Syntax
pub static JPEGExtendedHierarchicalProcess16_and_18: UID = UID {
    ident: "JPEGExtendedHierarchicalProcess16_and_18",
    uid: "1.2.840.10008.1.2.4.59",
    name: "JPEG Extended, Hierarchical (Process 16 & 18) (Retired)",
};

/// JPEG Extended, Hierarchical (Process 17 & 19) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.60
/// - **UID Type:** Transfer Syntax
pub static JPEGExtendedHierarchicalProcess17_and_19: UID = UID {
    ident: "JPEGExtendedHierarchicalProcess17_and_19",
    uid: "1.2.840.10008.1.2.4.60",
    name: "JPEG Extended, Hierarchical (Process 17 & 19) (Retired)",
};

/// JPEG Spectral Selection, Hierarchical (Process 20 & 22) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.61
/// - **UID Type:** Transfer Syntax
pub static JPEGSpectralSelectionHierarchicalProcess20_and_22: UID = UID {
    ident: "JPEGSpectralSelectionHierarchicalProcess20_and_22",
    uid: "1.2.840.10008.1.2.4.61",
    name: "JPEG Spectral Selection, Hierarchical (Process 20 & 22) (Retired)",
};

/// JPEG Spectral Selection, Hierarchical (Process 21 & 23) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.62
/// - **UID Type:** Transfer Syntax
pub static JPEGSpectralSelectionHierarchicalProcess21_and_23: UID = UID {
    ident: "JPEGSpectralSelectionHierarchicalProcess21_and_23",
    uid: "1.2.840.10008.1.2.4.62",
    name: "JPEG Spectral Selection, Hierarchical (Process 21 & 23) (Retired)",
};

/// JPEG Full Progression, Hierarchical (Process 24 & 26) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.63
/// - **UID Type:** Transfer Syntax
pub static JPEGFullProgressionHierarchicalProcess24_and_26: UID = UID {
    ident: "JPEGFullProgressionHierarchicalProcess24_and_26",
    uid: "1.2.840.10008.1.2.4.63",
    name: "JPEG Full Progression, Hierarchical (Process 24 & 26) (Retired)",
};

/// JPEG Full Progression, Hierarchical (Process 25 & 27) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.64
/// - **UID Type:** Transfer Syntax
pub static JPEGFullProgressionHierarchicalProcess25_and_27: UID = UID {
    ident: "JPEGFullProgressionHierarchicalProcess25_and_27",
    uid: "1.2.840.10008.1.2.4.64",
    name: "JPEG Full Progression, Hierarchical (Process 25 & 27) (Retired)",
};

/// JPEG Lossless, Hierarchical (Process 28) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.65
/// - **UID Type:** Transfer Syntax
pub static JPEGLosslessHierarchicalProcess28: UID = UID {
    ident: "JPEGLosslessHierarchicalProcess28",
    uid: "1.2.840.10008.1.2.4.65",
    name: "JPEG Lossless, Hierarchical (Process 28) (Retired)",
};

/// JPEG Lossless, Hierarchical (Process 29) (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.4.66
/// - **UID Type:** Transfer Syntax
pub static JPEGLosslessHierarchicalProcess29: UID = UID {
    ident: "JPEGLosslessHierarchicalProcess29",
    uid: "1.2.840.10008.1.2.4.66",
    name: "JPEG Lossless, Hierarchical (Process 29) (Retired)",
};

/// JPEG Lossless, Non-Hierarchical, First-Order Prediction (Process 14 [Selection Value 1]): Default Transfer Syntax for Lossless JPEG Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.70
/// - **UID Type:** Transfer Syntax
pub static JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1: UID = UID {
    ident: "JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1",
    uid: "1.2.840.10008.1.2.4.70",
    name: "JPEG Lossless, Non-Hierarchical, First-Order Prediction (Process 14 [Selection Value 1]): Default Transfer Syntax for Lossless JPEG Image Compression",
};

/// JPEG-LS Lossless Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.80
/// - **UID Type:** Transfer Syntax
pub static JPEGLSLosslessImageCompression: UID = UID {
    ident: "JPEGLSLosslessImageCompression",
    uid: "1.2.840.10008.1.2.4.80",
    name: "JPEG-LS Lossless Image Compression",
};

/// JPEG-LS Lossy (Near-Lossless) Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.81
/// - **UID Type:** Transfer Syntax
pub static JPEGLSLossyNearLosslessImageCompression: UID = UID {
    ident: "JPEGLSLossyNearLosslessImageCompression",
    uid: "1.2.840.10008.1.2.4.81",
    name: "JPEG-LS Lossy (Near-Lossless) Image Compression",
};

/// JPEG 2000 Image Compression (Lossless Only)
///
/// - **UID:** 1.2.840.10008.1.2.4.90
/// - **UID Type:** Transfer Syntax
pub static JPEG2000ImageCompressionLosslessOnly: UID = UID {
    ident: "JPEG2000ImageCompressionLosslessOnly",
    uid: "1.2.840.10008.1.2.4.90",
    name: "JPEG 2000 Image Compression (Lossless Only)",
};

/// JPEG 2000 Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.91
/// - **UID Type:** Transfer Syntax
pub static JPEG2000ImageCompression: UID = UID {
    ident: "JPEG2000ImageCompression",
    uid: "1.2.840.10008.1.2.4.91",
    name: "JPEG 2000 Image Compression",
};

/// JPEG 2000 Part 2 Multi-component Image Compression (Lossless Only)
///
/// - **UID:** 1.2.840.10008.1.2.4.92
/// - **UID Type:** Transfer Syntax
pub static JPEG2000Part2MulticomponentImageCompressionLosslessOnly: UID = UID {
    ident: "JPEG2000Part2MulticomponentImageCompressionLosslessOnly",
    uid: "1.2.840.10008.1.2.4.92",
    name: "JPEG 2000 Part 2 Multi-component Image Compression (Lossless Only)",
};

/// JPEG 2000 Part 2 Multi-component Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.93
/// - **UID Type:** Transfer Syntax
pub static JPEG2000Part2MulticomponentImageCompression: UID = UID {
    ident: "JPEG2000Part2MulticomponentImageCompression",
    uid: "1.2.840.10008.1.2.4.93",
    name: "JPEG 2000 Part 2 Multi-component Image Compression",
};

/// JPIP Referenced
///
/// - **UID:** 1.2.840.10008.1.2.4.94
/// - **UID Type:** Transfer Syntax
pub static JPIPReferenced: UID = UID {
    ident: "JPIPReferenced",
    uid: "1.2.840.10008.1.2.4.94",
    name: "JPIP Referenced",
};

/// JPIP Referenced Deflate
///
/// - **UID:** 1.2.840.10008.1.2.4.95
/// - **UID Type:** Transfer Syntax
pub static JPIPReferencedDeflate: UID = UID {
    ident: "JPIPReferencedDeflate",
    uid: "1.2.840.10008.1.2.4.95",
    name: "JPIP Referenced Deflate",
};

/// MPEG2 Main Profile / Main Level
///
/// - **UID:** 1.2.840.10008.1.2.4.100
/// - **UID Type:** Transfer Syntax
pub static MPEG2MainProfileMainLevel: UID = UID {
    ident: "MPEG2MainProfileMainLevel",
    uid: "1.2.840.10008.1.2.4.100",
    name: "MPEG2 Main Profile / Main Level",
};

/// Fragmentable MPEG2 Main Profile / Main Level
///
/// - **UID:** 1.2.840.10008.1.2.4.100.1
/// - **UID Type:** Transfer Syntax
pub static FragmentableMPEG2MainProfileMainLevel: UID = UID {
    ident: "FragmentableMPEG2MainProfileMainLevel",
    uid: "1.2.840.10008.1.2.4.100.1",
    name: "Fragmentable MPEG2 Main Profile / Main Level",
};

/// MPEG2 Main Profile / High Level
///
/// - **UID:** 1.2.840.10008.1.2.4.101
/// - **UID Type:** Transfer Syntax
pub static MPEG2MainProfileHighLevel: UID = UID {
    ident: "MPEG2MainProfileHighLevel",
    uid: "1.2.840.10008.1.2.4.101",
    name: "MPEG2 Main Profile / High Level",
};

/// Fragmentable MPEG2 Main Profile / High Level
///
/// - **UID:** 1.2.840.10008.1.2.4.101.1
/// - **UID Type:** Transfer Syntax
pub static FragmentableMPEG2MainProfileHighLevel: UID = UID {
    ident: "FragmentableMPEG2MainProfileHighLevel",
    uid: "1.2.840.10008.1.2.4.101.1",
    name: "Fragmentable MPEG2 Main Profile / High Level",
};

/// MPEG-4 AVC/H.264 High Profile / Level 4.1
///
/// - **UID:** 1.2.840.10008.1.2.4.102
/// - **UID Type:** Transfer Syntax
pub static MPEG4AVCH264HighProfileLevel41: UID = UID {
    ident: "MPEG4AVCH264HighProfileLevel41",
    uid: "1.2.840.10008.1.2.4.102",
    name: "MPEG-4 AVC/H.264 High Profile / Level 4.1",
};

/// Fragmentable MPEG-4 AVC/H.264 High Profile / Level 4.1
///
/// - **UID:** 1.2.840.10008.1.2.4.102.1
/// - **UID Type:** Transfer Syntax
pub static FragmentableMPEG4AVCH264HighProfileLevel41: UID = UID {
    ident: "FragmentableMPEG4AVCH264HighProfileLevel41",
    uid: "1.2.840.10008.1.2.4.102.1",
    name: "Fragmentable MPEG-4 AVC/H.264 High Profile / Level 4.1",
};

/// MPEG-4 AVC/H.264 BD-compatible High Profile / Level 4.1
///
/// - **UID:** 1.2.840.10008.1.2.4.103
/// - **UID Type:** Transfer Syntax
pub static MPEG4AVCH264BDcompatibleHighProfileLevel41: UID = UID {
    ident: "MPEG4AVCH264BDcompatibleHighProfileLevel41",
    uid: "1.2.840.10008.1.2.4.103",
    name: "MPEG-4 AVC/H.264 BD-compatible High Profile / Level 4.1",
};

/// Fragmentable MPEG-4 AVC/H.264 BD-compatible High Profile / Level 4.1
///
/// - **UID:** 1.2.840.10008.1.2.4.103.1
/// - **UID Type:** Transfer Syntax
pub static FragmentableMPEG4AVCH264BDcompatibleHighProfileLevel41: UID = UID {
    ident: "FragmentableMPEG4AVCH264BDcompatibleHighProfileLevel41",
    uid: "1.2.840.10008.1.2.4.103.1",
    name: "Fragmentable MPEG-4 AVC/H.264 BD-compatible High Profile / Level 4.1",
};

/// MPEG-4 AVC/H.264 High Profile / Level 4.2 For 2D Video
///
/// - **UID:** 1.2.840.10008.1.2.4.104
/// - **UID Type:** Transfer Syntax
pub static MPEG4AVCH264HighProfileLevel42For2DVideo: UID = UID {
    ident: "MPEG4AVCH264HighProfileLevel42For2DVideo",
    uid: "1.2.840.10008.1.2.4.104",
    name: "MPEG-4 AVC/H.264 High Profile / Level 4.2 For 2D Video",
};

/// Fragmentable MPEG-4 AVC/H.264 High Profile / Level 4.2 For 2D Video
///
/// - **UID:** 1.2.840.10008.1.2.4.104.1
/// - **UID Type:** Transfer Syntax
pub static FragmentableMPEG4AVCH264HighProfileLevel42For2DVideo: UID = UID {
    ident: "FragmentableMPEG4AVCH264HighProfileLevel42For2DVideo",
    uid: "1.2.840.10008.1.2.4.104.1",
    name: "Fragmentable MPEG-4 AVC/H.264 High Profile / Level 4.2 For 2D Video",
};

/// MPEG-4 AVC/H.264 High Profile / Level 4.2 For 3D Video
///
/// - **UID:** 1.2.840.10008.1.2.4.105
/// - **UID Type:** Transfer Syntax
pub static MPEG4AVCH264HighProfileLevel42For3DVideo: UID = UID {
    ident: "MPEG4AVCH264HighProfileLevel42For3DVideo",
    uid: "1.2.840.10008.1.2.4.105",
    name: "MPEG-4 AVC/H.264 High Profile / Level 4.2 For 3D Video",
};

/// Fragmentable MPEG-4 AVC/H.264 High Profile / Level 4.2 For 3D Video
///
/// - **UID:** 1.2.840.10008.1.2.4.105.1
/// - **UID Type:** Transfer Syntax
pub static FragmentableMPEG4AVCH264HighProfileLevel42For3DVideo: UID = UID {
    ident: "FragmentableMPEG4AVCH264HighProfileLevel42For3DVideo",
    uid: "1.2.840.10008.1.2.4.105.1",
    name: "Fragmentable MPEG-4 AVC/H.264 High Profile / Level 4.2 For 3D Video",
};

/// MPEG-4 AVC/H.264 Stereo High Profile / Level 4.2
///
/// - **UID:** 1.2.840.10008.1.2.4.106
/// - **UID Type:** Transfer Syntax
pub static MPEG4AVCH264StereoHighProfileLevel42: UID = UID {
    ident: "MPEG4AVCH264StereoHighProfileLevel42",
    uid: "1.2.840.10008.1.2.4.106",
    name: "MPEG-4 AVC/H.264 Stereo High Profile / Level 4.2",
};

/// Fragmentable MPEG-4 AVC/H.264 Stereo High Profile / Level 4.2
///
/// - **UID:** 1.2.840.10008.1.2.4.106.1
/// - **UID Type:** Transfer Syntax
pub static FragmentableMPEG4AVCH264StereoHighProfileLevel42: UID = UID {
    ident: "FragmentableMPEG4AVCH264StereoHighProfileLevel42",
    uid: "1.2.840.10008.1.2.4.106.1",
    name: "Fragmentable MPEG-4 AVC/H.264 Stereo High Profile / Level 4.2",
};

/// HEVC/H.265 Main Profile / Level 5.1
///
/// - **UID:** 1.2.840.10008.1.2.4.107
/// - **UID Type:** Transfer Syntax
pub static HEVCH265MainProfileLevel51: UID = UID {
    ident: "HEVCH265MainProfileLevel51",
    uid: "1.2.840.10008.1.2.4.107",
    name: "HEVC/H.265 Main Profile / Level 5.1",
};

/// HEVC/H.265 Main 10 Profile / Level 5.1
///
/// - **UID:** 1.2.840.10008.1.2.4.108
/// - **UID Type:** Transfer Syntax
pub static HEVCH265Main10ProfileLevel51: UID = UID {
    ident: "HEVCH265Main10ProfileLevel51",
    uid: "1.2.840.10008.1.2.4.108",
    name: "HEVC/H.265 Main 10 Profile / Level 5.1",
};

/// High-Throughput JPEG 2000 Image Compression (Lossless Only)
///
/// - **UID:** 1.2.840.10008.1.2.4.201
/// - **UID Type:** Transfer Syntax
pub static HighThroughputJPEG2000ImageCompressionLosslessOnly: UID = UID {
    ident: "HighThroughputJPEG2000ImageCompressionLosslessOnly",
    uid: "1.2.840.10008.1.2.4.201",
    name: "High-Throughput JPEG 2000 Image Compression (Lossless Only)",
};

/// High-Throughput JPEG 2000 with RPCL Options Image Compression (Lossless Only)
///
/// - **UID:** 1.2.840.10008.1.2.4.202
/// - **UID Type:** Transfer Syntax
pub static HighThroughputJPEG2000withRPCLOptionsImageCompressionLosslessOnly: UID = UID {
    ident: "HighThroughputJPEG2000withRPCLOptionsImageCompressionLosslessOnly",
    uid: "1.2.840.10008.1.2.4.202",
    name: "High-Throughput JPEG 2000 with RPCL Options Image Compression (Lossless Only)",
};

/// High-Throughput JPEG 2000 Image Compression
///
/// - **UID:** 1.2.840.10008.1.2.4.203
/// - **UID Type:** Transfer Syntax
pub static HighThroughputJPEG2000ImageCompression: UID = UID {
    ident: "HighThroughputJPEG2000ImageCompression",
    uid: "1.2.840.10008.1.2.4.203",
    name: "High-Throughput JPEG 2000 Image Compression",
};

/// JPIP HTJ2K Referenced
///
/// - **UID:** 1.2.840.10008.1.2.4.204
/// - **UID Type:** Transfer Syntax
pub static JPIPHTJ2KReferenced: UID = UID {
    ident: "JPIPHTJ2KReferenced",
    uid: "1.2.840.10008.1.2.4.204",
    name: "JPIP HTJ2K Referenced",
};

/// JPIP HTJ2K Referenced Deflate
///
/// - **UID:** 1.2.840.10008.1.2.4.205
/// - **UID Type:** Transfer Syntax
pub static JPIPHTJ2KReferencedDeflate: UID = UID {
    ident: "JPIPHTJ2KReferencedDeflate",
    uid: "1.2.840.10008.1.2.4.205",
    name: "JPIP HTJ2K Referenced Deflate",
};

/// RLE Lossless
///
/// - **UID:** 1.2.840.10008.1.2.5
/// - **UID Type:** Transfer Syntax
pub static RLELossless: UID = UID {
    ident: "RLELossless",
    uid: "1.2.840.10008.1.2.5",
    name: "RLE Lossless",
};

/// RFC 2557 MIME encapsulation (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.6.1
/// - **UID Type:** Transfer Syntax
pub static RFC2557MIMEencapsulation: UID = UID {
    ident: "RFC2557MIMEencapsulation",
    uid: "1.2.840.10008.1.2.6.1",
    name: "RFC 2557 MIME encapsulation (Retired)",
};

/// XML Encoding (Retired)
///
/// - **UID:** 1.2.840.10008.1.2.6.2
/// - **UID Type:** Transfer Syntax
pub static XMLEncoding: UID = UID {
    ident: "XMLEncoding",
    uid: "1.2.840.10008.1.2.6.2",
    name: "XML Encoding (Retired)",
};

/// SMPTE ST 2110-20 Uncompressed Progressive Active Video
///
/// - **UID:** 1.2.840.10008.1.2.7.1
/// - **UID Type:** Transfer Syntax
pub static SMPTEST211020UncompressedProgressiveActiveVideo: UID = UID {
    ident: "SMPTEST211020UncompressedProgressiveActiveVideo",
    uid: "1.2.840.10008.1.2.7.1",
    name: "SMPTE ST 2110-20 Uncompressed Progressive Active Video",
};

/// SMPTE ST 2110-20 Uncompressed Interlaced Active Video
///
/// - **UID:** 1.2.840.10008.1.2.7.2
/// - **UID Type:** Transfer Syntax
pub static SMPTEST211020UncompressedInterlacedActiveVideo: UID = UID {
    ident: "SMPTEST211020UncompressedInterlacedActiveVideo",
    uid: "1.2.840.10008.1.2.7.2",
    name: "SMPTE ST 2110-20 Uncompressed Interlaced Active Video",
};

/// SMPTE ST 2110-30 PCM Digital Audio
///
/// - **UID:** 1.2.840.10008.1.2.7.3
/// - **UID Type:** Transfer Syntax
pub static SMPTEST211030PCMDigitalAudio: UID = UID {
    ident: "SMPTEST211030PCMDigitalAudio",
    uid: "1.2.840.10008.1.2.7.3",
    name: "SMPTE ST 2110-30 PCM Digital Audio",
};

/// Media Storage Directory Storage
///
/// - **UID:** 1.2.840.10008.1.3.10
/// - **UID Type:** SOP Class
pub static MediaStorageDirectoryStorage: UID = UID {
    ident: "MediaStorageDirectoryStorage",
    uid: "1.2.840.10008.1.3.10",
    name: "Media Storage Directory Storage",
};

/// Hot Iron Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.1
/// - **UID Type:** Well-known SOP Instance
pub static HotIronColorPaletteSOPInstance: UID = UID {
    ident: "HotIronColorPaletteSOPInstance",
    uid: "1.2.840.10008.1.5.1",
    name: "Hot Iron Color Palette SOP Instance",
};

/// PET Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.2
/// - **UID Type:** Well-known SOP Instance
pub static PETColorPaletteSOPInstance: UID = UID {
    ident: "PETColorPaletteSOPInstance",
    uid: "1.2.840.10008.1.5.2",
    name: "PET Color Palette SOP Instance",
};

/// Hot Metal Blue Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.3
/// - **UID Type:** Well-known SOP Instance
pub static HotMetalBlueColorPaletteSOPInstance: UID = UID {
    ident: "HotMetalBlueColorPaletteSOPInstance",
    uid: "1.2.840.10008.1.5.3",
    name: "Hot Metal Blue Color Palette SOP Instance",
};

/// PET 20 Step Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.4
/// - **UID Type:** Well-known SOP Instance
pub static PET20StepColorPaletteSOPInstance: UID = UID {
    ident: "PET20StepColorPaletteSOPInstance",
    uid: "1.2.840.10008.1.5.4",
    name: "PET 20 Step Color Palette SOP Instance",
};

/// Spring Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.5
/// - **UID Type:** Well-known SOP Instance
pub static SpringColorPaletteSOPInstance: UID = UID {
    ident: "SpringColorPaletteSOPInstance",
    uid: "1.2.840.10008.1.5.5",
    name: "Spring Color Palette SOP Instance",
};

/// Summer Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.6
/// - **UID Type:** Well-known SOP Instance
pub static SummerColorPaletteSOPInstance: UID = UID {
    ident: "SummerColorPaletteSOPInstance",
    uid: "1.2.840.10008.1.5.6",
    name: "Summer Color Palette SOP Instance",
};

/// Fall Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.7
/// - **UID Type:** Well-known SOP Instance
pub static FallColorPaletteSOPInstance: UID = UID {
    ident: "FallColorPaletteSOPInstance",
    uid: "1.2.840.10008.1.5.7",
    name: "Fall Color Palette SOP Instance",
};

/// Winter Color Palette SOP Instance
///
/// - **UID:** 1.2.840.10008.1.5.8
/// - **UID Type:** Well-known SOP Instance
pub static WinterColorPaletteSOPInstance: UID = UID {
    ident: "WinterColorPaletteSOPInstance",
    uid: "1.2.840.10008.1.5.8",
    name: "Winter Color Palette SOP Instance",
};

/// Basic Study Content Notification SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.1.9
/// - **UID Type:** SOP Class
pub static BasicStudyContentNotificationSOPClass: UID = UID {
    ident: "BasicStudyContentNotificationSOPClass",
    uid: "1.2.840.10008.1.9",
    name: "Basic Study Content Notification SOP Class (Retired)",
};

/// Papyrus 3 Implicit VR Little Endian (Retired)
///
/// - **UID:** 1.2.840.10008.1.20
/// - **UID Type:** Transfer Syntax
pub static Papyrus3ImplicitVRLittleEndian: UID = UID {
    ident: "Papyrus3ImplicitVRLittleEndian",
    uid: "1.2.840.10008.1.20",
    name: "Papyrus 3 Implicit VR Little Endian (Retired)",
};

/// Storage Commitment Push Model SOP Class
///
/// - **UID:** 1.2.840.10008.1.20.1
/// - **UID Type:** SOP Class
pub static StorageCommitmentPushModelSOPClass: UID = UID {
    ident: "StorageCommitmentPushModelSOPClass",
    uid: "1.2.840.10008.1.20.1",
    name: "Storage Commitment Push Model SOP Class",
};

/// Storage Commitment Push Model SOP Instance
///
/// - **UID:** 1.2.840.10008.1.20.1.1
/// - **UID Type:** Well-known SOP Instance
pub static StorageCommitmentPushModelSOPInstance: UID = UID {
    ident: "StorageCommitmentPushModelSOPInstance",
    uid: "1.2.840.10008.1.20.1.1",
    name: "Storage Commitment Push Model SOP Instance",
};

/// Storage Commitment Pull Model SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.1.20.2
/// - **UID Type:** SOP Class
pub static StorageCommitmentPullModelSOPClass: UID = UID {
    ident: "StorageCommitmentPullModelSOPClass",
    uid: "1.2.840.10008.1.20.2",
    name: "Storage Commitment Pull Model SOP Class (Retired)",
};

/// Storage Commitment Pull Model SOP Instance (Retired)
///
/// - **UID:** 1.2.840.10008.1.20.2.1
/// - **UID Type:** Well-known SOP Instance
pub static StorageCommitmentPullModelSOPInstance: UID = UID {
    ident: "StorageCommitmentPullModelSOPInstance",
    uid: "1.2.840.10008.1.20.2.1",
    name: "Storage Commitment Pull Model SOP Instance (Retired)",
};

/// Procedural Event Logging SOP Class
///
/// - **UID:** 1.2.840.10008.1.40
/// - **UID Type:** SOP Class
pub static ProceduralEventLoggingSOPClass: UID = UID {
    ident: "ProceduralEventLoggingSOPClass",
    uid: "1.2.840.10008.1.40",
    name: "Procedural Event Logging SOP Class",
};

/// Procedural Event Logging SOP Instance
///
/// - **UID:** 1.2.840.10008.1.40.1
/// - **UID Type:** Well-known SOP Instance
pub static ProceduralEventLoggingSOPInstance: UID = UID {
    ident: "ProceduralEventLoggingSOPInstance",
    uid: "1.2.840.10008.1.40.1",
    name: "Procedural Event Logging SOP Instance",
};

/// Substance Administration Logging SOP Class
///
/// - **UID:** 1.2.840.10008.1.42
/// - **UID Type:** SOP Class
pub static SubstanceAdministrationLoggingSOPClass: UID = UID {
    ident: "SubstanceAdministrationLoggingSOPClass",
    uid: "1.2.840.10008.1.42",
    name: "Substance Administration Logging SOP Class",
};

/// Substance Administration Logging SOP Instance
///
/// - **UID:** 1.2.840.10008.1.42.1
/// - **UID Type:** Well-known SOP Instance
pub static SubstanceAdministrationLoggingSOPInstance: UID = UID {
    ident: "SubstanceAdministrationLoggingSOPInstance",
    uid: "1.2.840.10008.1.42.1",
    name: "Substance Administration Logging SOP Instance",
};

/// DICOM UID Registry
///
/// - **UID:** 1.2.840.10008.2.6.1
/// - **UID Type:** DICOM UIDs as a Coding Scheme
pub static DICOMUIDRegistry: UID = UID {
    ident: "DICOMUIDRegistry",
    uid: "1.2.840.10008.2.6.1",
    name: "DICOM UID Registry",
};

/// DICOM Controlled Terminology
///
/// - **UID:** 1.2.840.10008.2.16.4
/// - **UID Type:** Coding Scheme
pub static DICOMControlledTerminology: UID = UID {
    ident: "DICOMControlledTerminology",
    uid: "1.2.840.10008.2.16.4",
    name: "DICOM Controlled Terminology",
};

/// Adult Mouse Anatomy Ontology
///
/// - **UID:** 1.2.840.10008.2.16.5
/// - **UID Type:** Coding Scheme
pub static AdultMouseAnatomyOntology: UID = UID {
    ident: "AdultMouseAnatomyOntology",
    uid: "1.2.840.10008.2.16.5",
    name: "Adult Mouse Anatomy Ontology",
};

/// Uberon Ontology
///
/// - **UID:** 1.2.840.10008.2.16.6
/// - **UID Type:** Coding Scheme
pub static UberonOntology: UID = UID {
    ident: "UberonOntology",
    uid: "1.2.840.10008.2.16.6",
    name: "Uberon Ontology",
};

/// Integrated Taxonomic Information System (ITIS) Taxonomic Serial Number (TSN)
///
/// - **UID:** 1.2.840.10008.2.16.7
/// - **UID Type:** Coding Scheme
pub static IntegratedTaxonomicInformationSystemITISTaxonomicSerialNumberTSN: UID = UID {
    ident: "IntegratedTaxonomicInformationSystemITISTaxonomicSerialNumberTSN",
    uid: "1.2.840.10008.2.16.7",
    name: "Integrated Taxonomic Information System (ITIS) Taxonomic Serial Number (TSN)",
};

/// Mouse Genome Initiative (MGI)
///
/// - **UID:** 1.2.840.10008.2.16.8
/// - **UID Type:** Coding Scheme
pub static MouseGenomeInitiativeMGI: UID = UID {
    ident: "MouseGenomeInitiativeMGI",
    uid: "1.2.840.10008.2.16.8",
    name: "Mouse Genome Initiative (MGI)",
};

/// PubChem Compound CID
///
/// - **UID:** 1.2.840.10008.2.16.9
/// - **UID Type:** Coding Scheme
pub static PubChemCompoundCID: UID = UID {
    ident: "PubChemCompoundCID",
    uid: "1.2.840.10008.2.16.9",
    name: "PubChem Compound CID",
};

/// Dublin Core
///
/// - **UID:** 1.2.840.10008.2.16.10
/// - **UID Type:** Coding Scheme
pub static DublinCore: UID = UID {
    ident: "DublinCore",
    uid: "1.2.840.10008.2.16.10",
    name: "Dublin Core",
};

/// New York University Melanoma Clinical Cooperative Group
///
/// - **UID:** 1.2.840.10008.2.16.11
/// - **UID Type:** Coding Scheme
pub static NewYorkUniversityMelanomaClinicalCooperativeGroup: UID = UID {
    ident: "NewYorkUniversityMelanomaClinicalCooperativeGroup",
    uid: "1.2.840.10008.2.16.11",
    name: "New York University Melanoma Clinical Cooperative Group",
};

/// Mayo Clinic Non-radiological Images Specific Body Structure Anatomical Surface Region Guide
///
/// - **UID:** 1.2.840.10008.2.16.12
/// - **UID Type:** Coding Scheme
pub static MayoClinicNonradiologicalImagesSpecificBodyStructureAnatomicalSurfaceRegionGuide: UID = UID {
    ident: "MayoClinicNonradiologicalImagesSpecificBodyStructureAnatomicalSurfaceRegionGuide",
    uid: "1.2.840.10008.2.16.12",
    name: "Mayo Clinic Non-radiological Images Specific Body Structure Anatomical Surface Region Guide",
};

/// Image Biomarker Standardisation Initiative
///
/// - **UID:** 1.2.840.10008.2.16.13
/// - **UID Type:** Coding Scheme
pub static ImageBiomarkerStandardisationInitiative: UID = UID {
    ident: "ImageBiomarkerStandardisationInitiative",
    uid: "1.2.840.10008.2.16.13",
    name: "Image Biomarker Standardisation Initiative",
};

/// Radiomics Ontology
///
/// - **UID:** 1.2.840.10008.2.16.14
/// - **UID Type:** Coding Scheme
pub static RadiomicsOntology: UID = UID {
    ident: "RadiomicsOntology",
    uid: "1.2.840.10008.2.16.14",
    name: "Radiomics Ontology",
};

/// RadElement
///
/// - **UID:** 1.2.840.10008.2.16.15
/// - **UID Type:** Coding Scheme
pub static RadElement: UID = UID {
    ident: "RadElement",
    uid: "1.2.840.10008.2.16.15",
    name: "RadElement",
};

/// ICD-11
///
/// - **UID:** 1.2.840.10008.2.16.16
/// - **UID Type:** Coding Scheme
pub static ICD11: UID = UID {
    ident: "ICD11",
    uid: "1.2.840.10008.2.16.16",
    name: "ICD-11",
};

/// Unified numbering system (UNS) for metals and alloys
///
/// - **UID:** 1.2.840.10008.2.16.17
/// - **UID Type:** Coding Scheme
pub static UnifiednumberingsystemUNSformetalsandalloys: UID = UID {
    ident: "UnifiednumberingsystemUNSformetalsandalloys",
    uid: "1.2.840.10008.2.16.17",
    name: "Unified numbering system (UNS) for metals and alloys",
};

/// Research Resource Identification
///
/// - **UID:** 1.2.840.10008.2.16.18
/// - **UID Type:** Coding Scheme
pub static ResearchResourceIdentification: UID = UID {
    ident: "ResearchResourceIdentification",
    uid: "1.2.840.10008.2.16.18",
    name: "Research Resource Identification",
};

/// DICOM Application Context Name
///
/// - **UID:** 1.2.840.10008.3.1.1.1
/// - **UID Type:** Application Context Name
pub static DICOMApplicationContextName: UID = UID {
    ident: "DICOMApplicationContextName",
    uid: "1.2.840.10008.3.1.1.1",
    name: "DICOM Application Context Name",
};

/// Detached Patient Management SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.1.1
/// - **UID Type:** SOP Class
pub static DetachedPatientManagementSOPClass: UID = UID {
    ident: "DetachedPatientManagementSOPClass",
    uid: "1.2.840.10008.3.1.2.1.1",
    name: "Detached Patient Management SOP Class (Retired)",
};

/// Detached Patient Management Meta SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.1.4
/// - **UID Type:** Meta SOP Class
pub static DetachedPatientManagementMetaSOPClass: UID = UID {
    ident: "DetachedPatientManagementMetaSOPClass",
    uid: "1.2.840.10008.3.1.2.1.4",
    name: "Detached Patient Management Meta SOP Class (Retired)",
};

/// Detached Visit Management SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.2.1
/// - **UID Type:** SOP Class
pub static DetachedVisitManagementSOPClass: UID = UID {
    ident: "DetachedVisitManagementSOPClass",
    uid: "1.2.840.10008.3.1.2.2.1",
    name: "Detached Visit Management SOP Class (Retired)",
};

/// Detached Study Management SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.3.1
/// - **UID Type:** SOP Class
pub static DetachedStudyManagementSOPClass: UID = UID {
    ident: "DetachedStudyManagementSOPClass",
    uid: "1.2.840.10008.3.1.2.3.1",
    name: "Detached Study Management SOP Class (Retired)",
};

/// Study Component Management SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.3.2
/// - **UID Type:** SOP Class
pub static StudyComponentManagementSOPClass: UID = UID {
    ident: "StudyComponentManagementSOPClass",
    uid: "1.2.840.10008.3.1.2.3.2",
    name: "Study Component Management SOP Class (Retired)",
};

/// Modality Performed Procedure Step SOP Class
///
/// - **UID:** 1.2.840.10008.3.1.2.3.3
/// - **UID Type:** SOP Class
pub static ModalityPerformedProcedureStepSOPClass: UID = UID {
    ident: "ModalityPerformedProcedureStepSOPClass",
    uid: "1.2.840.10008.3.1.2.3.3",
    name: "Modality Performed Procedure Step SOP Class",
};

/// Modality Performed Procedure Step Retrieve SOP Class
///
/// - **UID:** 1.2.840.10008.3.1.2.3.4
/// - **UID Type:** SOP Class
pub static ModalityPerformedProcedureStepRetrieveSOPClass: UID = UID {
    ident: "ModalityPerformedProcedureStepRetrieveSOPClass",
    uid: "1.2.840.10008.3.1.2.3.4",
    name: "Modality Performed Procedure Step Retrieve SOP Class",
};

/// Modality Performed Procedure Step Notification SOP Class
///
/// - **UID:** 1.2.840.10008.3.1.2.3.5
/// - **UID Type:** SOP Class
pub static ModalityPerformedProcedureStepNotificationSOPClass: UID = UID {
    ident: "ModalityPerformedProcedureStepNotificationSOPClass",
    uid: "1.2.840.10008.3.1.2.3.5",
    name: "Modality Performed Procedure Step Notification SOP Class",
};

/// Detached Results Management SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.5.1
/// - **UID Type:** SOP Class
pub static DetachedResultsManagementSOPClass: UID = UID {
    ident: "DetachedResultsManagementSOPClass",
    uid: "1.2.840.10008.3.1.2.5.1",
    name: "Detached Results Management SOP Class (Retired)",
};

/// Detached Results Management Meta SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.5.4
/// - **UID Type:** Meta SOP Class
pub static DetachedResultsManagementMetaSOPClass: UID = UID {
    ident: "DetachedResultsManagementMetaSOPClass",
    uid: "1.2.840.10008.3.1.2.5.4",
    name: "Detached Results Management Meta SOP Class (Retired)",
};

/// Detached Study Management Meta SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.5.5
/// - **UID Type:** Meta SOP Class
pub static DetachedStudyManagementMetaSOPClass: UID = UID {
    ident: "DetachedStudyManagementMetaSOPClass",
    uid: "1.2.840.10008.3.1.2.5.5",
    name: "Detached Study Management Meta SOP Class (Retired)",
};

/// Detached Interpretation Management SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.3.1.2.6.1
/// - **UID Type:** SOP Class
pub static DetachedInterpretationManagementSOPClass: UID = UID {
    ident: "DetachedInterpretationManagementSOPClass",
    uid: "1.2.840.10008.3.1.2.6.1",
    name: "Detached Interpretation Management SOP Class (Retired)",
};

/// Storage Service Class
///
/// - **UID:** 1.2.840.10008.4.2
/// - **UID Type:** Service Class
pub static StorageServiceClass: UID = UID {
    ident: "StorageServiceClass",
    uid: "1.2.840.10008.4.2",
    name: "Storage Service Class",
};

/// Basic Film Session SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.1
/// - **UID Type:** SOP Class
pub static BasicFilmSessionSOPClass: UID = UID {
    ident: "BasicFilmSessionSOPClass",
    uid: "1.2.840.10008.5.1.1.1",
    name: "Basic Film Session SOP Class",
};

/// Basic Film Box SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.2
/// - **UID Type:** SOP Class
pub static BasicFilmBoxSOPClass: UID = UID {
    ident: "BasicFilmBoxSOPClass",
    uid: "1.2.840.10008.5.1.1.2",
    name: "Basic Film Box SOP Class",
};

/// Basic Grayscale Image Box SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.4
/// - **UID Type:** SOP Class
pub static BasicGrayscaleImageBoxSOPClass: UID = UID {
    ident: "BasicGrayscaleImageBoxSOPClass",
    uid: "1.2.840.10008.5.1.1.4",
    name: "Basic Grayscale Image Box SOP Class",
};

/// Basic Color Image Box SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.4.1
/// - **UID Type:** SOP Class
pub static BasicColorImageBoxSOPClass: UID = UID {
    ident: "BasicColorImageBoxSOPClass",
    uid: "1.2.840.10008.5.1.1.4.1",
    name: "Basic Color Image Box SOP Class",
};

/// Referenced Image Box SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.4.2
/// - **UID Type:** SOP Class
pub static ReferencedImageBoxSOPClass: UID = UID {
    ident: "ReferencedImageBoxSOPClass",
    uid: "1.2.840.10008.5.1.1.4.2",
    name: "Referenced Image Box SOP Class (Retired)",
};

/// Basic Grayscale Print Management Meta SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.9
/// - **UID Type:** Meta SOP Class
pub static BasicGrayscalePrintManagementMetaSOPClass: UID = UID {
    ident: "BasicGrayscalePrintManagementMetaSOPClass",
    uid: "1.2.840.10008.5.1.1.9",
    name: "Basic Grayscale Print Management Meta SOP Class",
};

/// Referenced Grayscale Print Management Meta SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.9.1
/// - **UID Type:** Meta SOP Class
pub static ReferencedGrayscalePrintManagementMetaSOPClass: UID = UID {
    ident: "ReferencedGrayscalePrintManagementMetaSOPClass",
    uid: "1.2.840.10008.5.1.1.9.1",
    name: "Referenced Grayscale Print Management Meta SOP Class (Retired)",
};

/// Print Job SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.14
/// - **UID Type:** SOP Class
pub static PrintJobSOPClass: UID = UID {
    ident: "PrintJobSOPClass",
    uid: "1.2.840.10008.5.1.1.14",
    name: "Print Job SOP Class",
};

/// Basic Annotation Box SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.15
/// - **UID Type:** SOP Class
pub static BasicAnnotationBoxSOPClass: UID = UID {
    ident: "BasicAnnotationBoxSOPClass",
    uid: "1.2.840.10008.5.1.1.15",
    name: "Basic Annotation Box SOP Class",
};

/// Printer SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.16
/// - **UID Type:** SOP Class
pub static PrinterSOPClass: UID = UID {
    ident: "PrinterSOPClass",
    uid: "1.2.840.10008.5.1.1.16",
    name: "Printer SOP Class",
};

/// Printer Configuration Retrieval SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.16.376
/// - **UID Type:** SOP Class
pub static PrinterConfigurationRetrievalSOPClass: UID = UID {
    ident: "PrinterConfigurationRetrievalSOPClass",
    uid: "1.2.840.10008.5.1.1.16.376",
    name: "Printer Configuration Retrieval SOP Class",
};

/// Printer SOP Instance
///
/// - **UID:** 1.2.840.10008.5.1.1.17
/// - **UID Type:** Well-known SOP Instance
pub static PrinterSOPInstance: UID = UID {
    ident: "PrinterSOPInstance",
    uid: "1.2.840.10008.5.1.1.17",
    name: "Printer SOP Instance",
};

/// Printer Configuration Retrieval SOP Instance
///
/// - **UID:** 1.2.840.10008.5.1.1.17.376
/// - **UID Type:** Well-known SOP Instance
pub static PrinterConfigurationRetrievalSOPInstance: UID = UID {
    ident: "PrinterConfigurationRetrievalSOPInstance",
    uid: "1.2.840.10008.5.1.1.17.376",
    name: "Printer Configuration Retrieval SOP Instance",
};

/// Basic Color Print Management Meta SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.18
/// - **UID Type:** Meta SOP Class
pub static BasicColorPrintManagementMetaSOPClass: UID = UID {
    ident: "BasicColorPrintManagementMetaSOPClass",
    uid: "1.2.840.10008.5.1.1.18",
    name: "Basic Color Print Management Meta SOP Class",
};

/// Referenced Color Print Management Meta SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.18.1
/// - **UID Type:** Meta SOP Class
pub static ReferencedColorPrintManagementMetaSOPClass: UID = UID {
    ident: "ReferencedColorPrintManagementMetaSOPClass",
    uid: "1.2.840.10008.5.1.1.18.1",
    name: "Referenced Color Print Management Meta SOP Class (Retired)",
};

/// VOI LUT Box SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.22
/// - **UID Type:** SOP Class
pub static VOILUTBoxSOPClass: UID = UID {
    ident: "VOILUTBoxSOPClass",
    uid: "1.2.840.10008.5.1.1.22",
    name: "VOI LUT Box SOP Class",
};

/// Presentation LUT SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.23
/// - **UID Type:** SOP Class
pub static PresentationLUTSOPClass: UID = UID {
    ident: "PresentationLUTSOPClass",
    uid: "1.2.840.10008.5.1.1.23",
    name: "Presentation LUT SOP Class",
};

/// Image Overlay Box SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.24
/// - **UID Type:** SOP Class
pub static ImageOverlayBoxSOPClass: UID = UID {
    ident: "ImageOverlayBoxSOPClass",
    uid: "1.2.840.10008.5.1.1.24",
    name: "Image Overlay Box SOP Class (Retired)",
};

/// Basic Print Image Overlay Box SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.24.1
/// - **UID Type:** SOP Class
pub static BasicPrintImageOverlayBoxSOPClass: UID = UID {
    ident: "BasicPrintImageOverlayBoxSOPClass",
    uid: "1.2.840.10008.5.1.1.24.1",
    name: "Basic Print Image Overlay Box SOP Class (Retired)",
};

/// Print Queue SOP Instance (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.25
/// - **UID Type:** Well-known SOP Instance
pub static PrintQueueSOPInstance: UID = UID {
    ident: "PrintQueueSOPInstance",
    uid: "1.2.840.10008.5.1.1.25",
    name: "Print Queue SOP Instance (Retired)",
};

/// Print Queue Management SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.26
/// - **UID Type:** SOP Class
pub static PrintQueueManagementSOPClass: UID = UID {
    ident: "PrintQueueManagementSOPClass",
    uid: "1.2.840.10008.5.1.1.26",
    name: "Print Queue Management SOP Class (Retired)",
};

/// Stored Print Storage SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.27
/// - **UID Type:** SOP Class
pub static StoredPrintStorageSOPClass: UID = UID {
    ident: "StoredPrintStorageSOPClass",
    uid: "1.2.840.10008.5.1.1.27",
    name: "Stored Print Storage SOP Class (Retired)",
};

/// Hardcopy Grayscale Image Storage SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.29
/// - **UID Type:** SOP Class
pub static HardcopyGrayscaleImageStorageSOPClass: UID = UID {
    ident: "HardcopyGrayscaleImageStorageSOPClass",
    uid: "1.2.840.10008.5.1.1.29",
    name: "Hardcopy Grayscale Image Storage SOP Class (Retired)",
};

/// Hardcopy Color Image Storage SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.30
/// - **UID Type:** SOP Class
pub static HardcopyColorImageStorageSOPClass: UID = UID {
    ident: "HardcopyColorImageStorageSOPClass",
    uid: "1.2.840.10008.5.1.1.30",
    name: "Hardcopy Color Image Storage SOP Class (Retired)",
};

/// Pull Print Request SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.31
/// - **UID Type:** SOP Class
pub static PullPrintRequestSOPClass: UID = UID {
    ident: "PullPrintRequestSOPClass",
    uid: "1.2.840.10008.5.1.1.31",
    name: "Pull Print Request SOP Class (Retired)",
};

/// Pull Stored Print Management Meta SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.1.32
/// - **UID Type:** Meta SOP Class
pub static PullStoredPrintManagementMetaSOPClass: UID = UID {
    ident: "PullStoredPrintManagementMetaSOPClass",
    uid: "1.2.840.10008.5.1.1.32",
    name: "Pull Stored Print Management Meta SOP Class (Retired)",
};

/// Media Creation Management SOP Class UID
///
/// - **UID:** 1.2.840.10008.5.1.1.33
/// - **UID Type:** SOP Class
pub static MediaCreationManagementSOPClassUID: UID = UID {
    ident: "MediaCreationManagementSOPClassUID",
    uid: "1.2.840.10008.5.1.1.33",
    name: "Media Creation Management SOP Class UID",
};

/// Display System SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.1.40
/// - **UID Type:** SOP Class
pub static DisplaySystemSOPClass: UID = UID {
    ident: "DisplaySystemSOPClass",
    uid: "1.2.840.10008.5.1.1.40",
    name: "Display System SOP Class",
};

/// Display System SOP Instance
///
/// - **UID:** 1.2.840.10008.5.1.1.40.1
/// - **UID Type:** Well-known SOP Instance
pub static DisplaySystemSOPInstance: UID = UID {
    ident: "DisplaySystemSOPInstance",
    uid: "1.2.840.10008.5.1.1.40.1",
    name: "Display System SOP Instance",
};

/// Computed Radiography Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.1
/// - **UID Type:** SOP Class
pub static ComputedRadiographyImageStorage: UID = UID {
    ident: "ComputedRadiographyImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.1",
    name: "Computed Radiography Image Storage",
};

/// Digital X-Ray Image Storage - For Presentation
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.1.1
/// - **UID Type:** SOP Class
pub static DigitalXRayImageStorageForPresentation: UID = UID {
    ident: "DigitalXRayImageStorageForPresentation",
    uid: "1.2.840.10008.5.1.4.1.1.1.1",
    name: "Digital X-Ray Image Storage - For Presentation",
};

/// Digital X-Ray Image Storage - For Processing
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.1.1.1
/// - **UID Type:** SOP Class
pub static DigitalXRayImageStorageForProcessing: UID = UID {
    ident: "DigitalXRayImageStorageForProcessing",
    uid: "1.2.840.10008.5.1.4.1.1.1.1.1",
    name: "Digital X-Ray Image Storage - For Processing",
};

/// Digital Mammography X-Ray Image Storage - For Presentation
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.1.2
/// - **UID Type:** SOP Class
pub static DigitalMammographyXRayImageStorageForPresentation: UID = UID {
    ident: "DigitalMammographyXRayImageStorageForPresentation",
    uid: "1.2.840.10008.5.1.4.1.1.1.2",
    name: "Digital Mammography X-Ray Image Storage - For Presentation",
};

/// Digital Mammography X-Ray Image Storage - For Processing
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.1.2.1
/// - **UID Type:** SOP Class
pub static DigitalMammographyXRayImageStorageForProcessing: UID = UID {
    ident: "DigitalMammographyXRayImageStorageForProcessing",
    uid: "1.2.840.10008.5.1.4.1.1.1.2.1",
    name: "Digital Mammography X-Ray Image Storage - For Processing",
};

/// Digital Intra-Oral X-Ray Image Storage - For Presentation
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.1.3
/// - **UID Type:** SOP Class
pub static DigitalIntraOralXRayImageStorageForPresentation: UID = UID {
    ident: "DigitalIntraOralXRayImageStorageForPresentation",
    uid: "1.2.840.10008.5.1.4.1.1.1.3",
    name: "Digital Intra-Oral X-Ray Image Storage - For Presentation",
};

/// Digital Intra-Oral X-Ray Image Storage - For Processing
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.1.3.1
/// - **UID Type:** SOP Class
pub static DigitalIntraOralXRayImageStorageForProcessing: UID = UID {
    ident: "DigitalIntraOralXRayImageStorageForProcessing",
    uid: "1.2.840.10008.5.1.4.1.1.1.3.1",
    name: "Digital Intra-Oral X-Ray Image Storage - For Processing",
};

/// CT Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.2
/// - **UID Type:** SOP Class
pub static CTImageStorage: UID = UID {
    ident: "CTImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.2",
    name: "CT Image Storage",
};

/// Enhanced CT Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.2.1
/// - **UID Type:** SOP Class
pub static EnhancedCTImageStorage: UID = UID {
    ident: "EnhancedCTImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.2.1",
    name: "Enhanced CT Image Storage",
};

/// Legacy Converted Enhanced CT Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.2.2
/// - **UID Type:** SOP Class
pub static LegacyConvertedEnhancedCTImageStorage: UID = UID {
    ident: "LegacyConvertedEnhancedCTImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.2.2",
    name: "Legacy Converted Enhanced CT Image Storage",
};

/// Ultrasound Multi-frame Image Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.3
/// - **UID Type:** SOP Class
pub static UltrasoundMultiframeImageStorage_Retired: UID = UID {
    ident: "UltrasoundMultiframeImageStorage_Retired",
    uid: "1.2.840.10008.5.1.4.1.1.3",
    name: "Ultrasound Multi-frame Image Storage (Retired)",
};

/// Ultrasound Multi-frame Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.3.1
/// - **UID Type:** SOP Class
pub static UltrasoundMultiframeImageStorage: UID = UID {
    ident: "UltrasoundMultiframeImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.3.1",
    name: "Ultrasound Multi-frame Image Storage",
};

/// MR Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.4
/// - **UID Type:** SOP Class
pub static MRImageStorage: UID = UID {
    ident: "MRImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.4",
    name: "MR Image Storage",
};

/// Enhanced MR Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.4.1
/// - **UID Type:** SOP Class
pub static EnhancedMRImageStorage: UID = UID {
    ident: "EnhancedMRImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.4.1",
    name: "Enhanced MR Image Storage",
};

/// MR Spectroscopy Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.4.2
/// - **UID Type:** SOP Class
pub static MRSpectroscopyStorage: UID = UID {
    ident: "MRSpectroscopyStorage",
    uid: "1.2.840.10008.5.1.4.1.1.4.2",
    name: "MR Spectroscopy Storage",
};

/// Enhanced MR Color Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.4.3
/// - **UID Type:** SOP Class
pub static EnhancedMRColorImageStorage: UID = UID {
    ident: "EnhancedMRColorImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.4.3",
    name: "Enhanced MR Color Image Storage",
};

/// Legacy Converted Enhanced MR Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.4.4
/// - **UID Type:** SOP Class
pub static LegacyConvertedEnhancedMRImageStorage: UID = UID {
    ident: "LegacyConvertedEnhancedMRImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.4.4",
    name: "Legacy Converted Enhanced MR Image Storage",
};

/// Nuclear Medicine Image Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.5
/// - **UID Type:** SOP Class
pub static NuclearMedicineImageStorage_Retired: UID = UID {
    ident: "NuclearMedicineImageStorage_Retired",
    uid: "1.2.840.10008.5.1.4.1.1.5",
    name: "Nuclear Medicine Image Storage (Retired)",
};

/// Ultrasound Image Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.6
/// - **UID Type:** SOP Class
pub static UltrasoundImageStorage_Retired: UID = UID {
    ident: "UltrasoundImageStorage_Retired",
    uid: "1.2.840.10008.5.1.4.1.1.6",
    name: "Ultrasound Image Storage (Retired)",
};

/// Ultrasound Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.6.1
/// - **UID Type:** SOP Class
pub static UltrasoundImageStorage: UID = UID {
    ident: "UltrasoundImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.6.1",
    name: "Ultrasound Image Storage",
};

/// Enhanced US Volume Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.6.2
/// - **UID Type:** SOP Class
pub static EnhancedUSVolumeStorage: UID = UID {
    ident: "EnhancedUSVolumeStorage",
    uid: "1.2.840.10008.5.1.4.1.1.6.2",
    name: "Enhanced US Volume Storage",
};

/// Photoacoustic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.6.3
/// - **UID Type:** SOP Class
pub static PhotoacousticImageStorage: UID = UID {
    ident: "PhotoacousticImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.6.3",
    name: "Photoacoustic Image Storage",
};

/// Secondary Capture Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.7
/// - **UID Type:** SOP Class
pub static SecondaryCaptureImageStorage: UID = UID {
    ident: "SecondaryCaptureImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.7",
    name: "Secondary Capture Image Storage",
};

/// Multi-frame Single Bit Secondary Capture Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.7.1
/// - **UID Type:** SOP Class
pub static MultiframeSingleBitSecondaryCaptureImageStorage: UID = UID {
    ident: "MultiframeSingleBitSecondaryCaptureImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.7.1",
    name: "Multi-frame Single Bit Secondary Capture Image Storage",
};

/// Multi-frame Grayscale Byte Secondary Capture Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.7.2
/// - **UID Type:** SOP Class
pub static MultiframeGrayscaleByteSecondaryCaptureImageStorage: UID = UID {
    ident: "MultiframeGrayscaleByteSecondaryCaptureImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.7.2",
    name: "Multi-frame Grayscale Byte Secondary Capture Image Storage",
};

/// Multi-frame Grayscale Word Secondary Capture Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.7.3
/// - **UID Type:** SOP Class
pub static MultiframeGrayscaleWordSecondaryCaptureImageStorage: UID = UID {
    ident: "MultiframeGrayscaleWordSecondaryCaptureImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.7.3",
    name: "Multi-frame Grayscale Word Secondary Capture Image Storage",
};

/// Multi-frame True Color Secondary Capture Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.7.4
/// - **UID Type:** SOP Class
pub static MultiframeTrueColorSecondaryCaptureImageStorage: UID = UID {
    ident: "MultiframeTrueColorSecondaryCaptureImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.7.4",
    name: "Multi-frame True Color Secondary Capture Image Storage",
};

/// Standalone Overlay Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.8
/// - **UID Type:** SOP Class
pub static StandaloneOverlayStorage: UID = UID {
    ident: "StandaloneOverlayStorage",
    uid: "1.2.840.10008.5.1.4.1.1.8",
    name: "Standalone Overlay Storage (Retired)",
};

/// Standalone Curve Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9
/// - **UID Type:** SOP Class
pub static StandaloneCurveStorage: UID = UID {
    ident: "StandaloneCurveStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9",
    name: "Standalone Curve Storage (Retired)",
};

/// Waveform Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.1
/// - **UID Type:** SOP Class
pub static WaveformStorageTrial: UID = UID {
    ident: "WaveformStorageTrial",
    uid: "1.2.840.10008.5.1.4.1.1.9.1",
    name: "Waveform Storage - Trial (Retired)",
};

/// 12-lead ECG Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.1.1
/// - **UID Type:** SOP Class
pub static Tag_12leadECGWaveformStorage: UID = UID {
    ident: "Tag_12leadECGWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.1.1",
    name: "12-lead ECG Waveform Storage",
};

/// General ECG Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.1.2
/// - **UID Type:** SOP Class
pub static GeneralECGWaveformStorage: UID = UID {
    ident: "GeneralECGWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.1.2",
    name: "General ECG Waveform Storage",
};

/// Ambulatory ECG Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.1.3
/// - **UID Type:** SOP Class
pub static AmbulatoryECGWaveformStorage: UID = UID {
    ident: "AmbulatoryECGWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.1.3",
    name: "Ambulatory ECG Waveform Storage",
};

/// General 32-bit ECG Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.1.4
/// - **UID Type:** SOP Class
pub static General32bitECGWaveformStorage: UID = UID {
    ident: "General32bitECGWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.1.4",
    name: "General 32-bit ECG Waveform Storage",
};

/// Hemodynamic Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.2.1
/// - **UID Type:** SOP Class
pub static HemodynamicWaveformStorage: UID = UID {
    ident: "HemodynamicWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.2.1",
    name: "Hemodynamic Waveform Storage",
};

/// Cardiac Electrophysiology Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.3.1
/// - **UID Type:** SOP Class
pub static CardiacElectrophysiologyWaveformStorage: UID = UID {
    ident: "CardiacElectrophysiologyWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.3.1",
    name: "Cardiac Electrophysiology Waveform Storage",
};

/// Basic Voice Audio Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.4.1
/// - **UID Type:** SOP Class
pub static BasicVoiceAudioWaveformStorage: UID = UID {
    ident: "BasicVoiceAudioWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.4.1",
    name: "Basic Voice Audio Waveform Storage",
};

/// General Audio Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.4.2
/// - **UID Type:** SOP Class
pub static GeneralAudioWaveformStorage: UID = UID {
    ident: "GeneralAudioWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.4.2",
    name: "General Audio Waveform Storage",
};

/// Arterial Pulse Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.5.1
/// - **UID Type:** SOP Class
pub static ArterialPulseWaveformStorage: UID = UID {
    ident: "ArterialPulseWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.5.1",
    name: "Arterial Pulse Waveform Storage",
};

/// Respiratory Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.6.1
/// - **UID Type:** SOP Class
pub static RespiratoryWaveformStorage: UID = UID {
    ident: "RespiratoryWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.6.1",
    name: "Respiratory Waveform Storage",
};

/// Multi-channel Respiratory Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.6.2
/// - **UID Type:** SOP Class
pub static MultichannelRespiratoryWaveformStorage: UID = UID {
    ident: "MultichannelRespiratoryWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.6.2",
    name: "Multi-channel Respiratory Waveform Storage",
};

/// Routine Scalp Electroencephalogram Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.7.1
/// - **UID Type:** SOP Class
pub static RoutineScalpElectroencephalogramWaveformStorage: UID = UID {
    ident: "RoutineScalpElectroencephalogramWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.7.1",
    name: "Routine Scalp Electroencephalogram Waveform Storage",
};

/// Electromyogram Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.7.2
/// - **UID Type:** SOP Class
pub static ElectromyogramWaveformStorage: UID = UID {
    ident: "ElectromyogramWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.7.2",
    name: "Electromyogram Waveform Storage",
};

/// Electrooculogram Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.7.3
/// - **UID Type:** SOP Class
pub static ElectrooculogramWaveformStorage: UID = UID {
    ident: "ElectrooculogramWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.7.3",
    name: "Electrooculogram Waveform Storage",
};

/// Sleep Electroencephalogram Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.7.4
/// - **UID Type:** SOP Class
pub static SleepElectroencephalogramWaveformStorage: UID = UID {
    ident: "SleepElectroencephalogramWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.7.4",
    name: "Sleep Electroencephalogram Waveform Storage",
};

/// Body Position Waveform Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.9.8.1
/// - **UID Type:** SOP Class
pub static BodyPositionWaveformStorage: UID = UID {
    ident: "BodyPositionWaveformStorage",
    uid: "1.2.840.10008.5.1.4.1.1.9.8.1",
    name: "Body Position Waveform Storage",
};

/// Standalone Modality LUT Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.10
/// - **UID Type:** SOP Class
pub static StandaloneModalityLUTStorage: UID = UID {
    ident: "StandaloneModalityLUTStorage",
    uid: "1.2.840.10008.5.1.4.1.1.10",
    name: "Standalone Modality LUT Storage (Retired)",
};

/// Standalone VOI LUT Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11
/// - **UID Type:** SOP Class
pub static StandaloneVOILUTStorage: UID = UID {
    ident: "StandaloneVOILUTStorage",
    uid: "1.2.840.10008.5.1.4.1.1.11",
    name: "Standalone VOI LUT Storage (Retired)",
};

/// Grayscale Softcopy Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.1
/// - **UID Type:** SOP Class
pub static GrayscaleSoftcopyPresentationStateStorage: UID = UID {
    ident: "GrayscaleSoftcopyPresentationStateStorage",
    uid: "1.2.840.10008.5.1.4.1.1.11.1",
    name: "Grayscale Softcopy Presentation State Storage",
};

/// Color Softcopy Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.2
/// - **UID Type:** SOP Class
pub static ColorSoftcopyPresentationStateStorage: UID = UID {
    ident: "ColorSoftcopyPresentationStateStorage",
    uid: "1.2.840.10008.5.1.4.1.1.11.2",
    name: "Color Softcopy Presentation State Storage",
};

/// Pseudo-Color Softcopy Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.3
/// - **UID Type:** SOP Class
pub static PseudoColorSoftcopyPresentationStateStorage: UID = UID {
    ident: "PseudoColorSoftcopyPresentationStateStorage",
    uid: "1.2.840.10008.5.1.4.1.1.11.3",
    name: "Pseudo-Color Softcopy Presentation State Storage",
};

/// Blending Softcopy Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.4
/// - **UID Type:** SOP Class
pub static BlendingSoftcopyPresentationStateStorage: UID = UID {
    ident: "BlendingSoftcopyPresentationStateStorage",
    uid: "1.2.840.10008.5.1.4.1.1.11.4",
    name: "Blending Softcopy Presentation State Storage",
};

/// XA/XRF Grayscale Softcopy Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.5
/// - **UID Type:** SOP Class
pub static XAXRFGrayscaleSoftcopyPresentationStateStorage: UID = UID {
    ident: "XAXRFGrayscaleSoftcopyPresentationStateStorage",
    uid: "1.2.840.10008.5.1.4.1.1.11.5",
    name: "XA/XRF Grayscale Softcopy Presentation State Storage",
};

/// Grayscale Planar MPR Volumetric Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.6
/// - **UID Type:** SOP Class
pub static GrayscalePlanarMPRVolumetricPresentationStateStorage: UID = UID {
    ident: "GrayscalePlanarMPRVolumetricPresentationStateStorage",
    uid: "1.2.840.10008.5.1.4.1.1.11.6",
    name: "Grayscale Planar MPR Volumetric Presentation State Storage",
};

/// Compositing Planar MPR Volumetric Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.7
/// - **UID Type:** SOP Class
pub static CompositingPlanarMPRVolumetricPresentationStateStorage: UID = UID {
    ident: "CompositingPlanarMPRVolumetricPresentationStateStorage",
    uid: "1.2.840.10008.5.1.4.1.1.11.7",
    name: "Compositing Planar MPR Volumetric Presentation State Storage",
};

/// Advanced Blending Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.8
/// - **UID Type:** SOP Class
pub static AdvancedBlendingPresentationStateStorage: UID = UID {
    ident: "AdvancedBlendingPresentationStateStorage",
    uid: "1.2.840.10008.5.1.4.1.1.11.8",
    name: "Advanced Blending Presentation State Storage",
};

/// Volume Rendering Volumetric Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.9
/// - **UID Type:** SOP Class
pub static VolumeRenderingVolumetricPresentationStateStorage: UID = UID {
    ident: "VolumeRenderingVolumetricPresentationStateStorage",
    uid: "1.2.840.10008.5.1.4.1.1.11.9",
    name: "Volume Rendering Volumetric Presentation State Storage",
};

/// Segmented Volume Rendering Volumetric Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.10
/// - **UID Type:** SOP Class
pub static SegmentedVolumeRenderingVolumetricPresentationStateStorage: UID = UID {
    ident: "SegmentedVolumeRenderingVolumetricPresentationStateStorage",
    uid: "1.2.840.10008.5.1.4.1.1.11.10",
    name: "Segmented Volume Rendering Volumetric Presentation State Storage",
};

/// Multiple Volume Rendering Volumetric Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.11
/// - **UID Type:** SOP Class
pub static MultipleVolumeRenderingVolumetricPresentationStateStorage: UID = UID {
    ident: "MultipleVolumeRenderingVolumetricPresentationStateStorage",
    uid: "1.2.840.10008.5.1.4.1.1.11.11",
    name: "Multiple Volume Rendering Volumetric Presentation State Storage",
};

/// Variable Modality LUT Softcopy Presentation State Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.11.12
/// - **UID Type:** SOP Class
pub static VariableModalityLUTSoftcopyPresentationStateStorage: UID = UID {
    ident: "VariableModalityLUTSoftcopyPresentationStateStorage",
    uid: "1.2.840.10008.5.1.4.1.1.11.12",
    name: "Variable Modality LUT Softcopy Presentation State Storage",
};

/// X-Ray Angiographic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.12.1
/// - **UID Type:** SOP Class
pub static XRayAngiographicImageStorage: UID = UID {
    ident: "XRayAngiographicImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.12.1",
    name: "X-Ray Angiographic Image Storage",
};

/// Enhanced XA Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.12.1.1
/// - **UID Type:** SOP Class
pub static EnhancedXAImageStorage: UID = UID {
    ident: "EnhancedXAImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.12.1.1",
    name: "Enhanced XA Image Storage",
};

/// X-Ray Radiofluoroscopic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.12.2
/// - **UID Type:** SOP Class
pub static XRayRadiofluoroscopicImageStorage: UID = UID {
    ident: "XRayRadiofluoroscopicImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.12.2",
    name: "X-Ray Radiofluoroscopic Image Storage",
};

/// Enhanced XRF Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.12.2.1
/// - **UID Type:** SOP Class
pub static EnhancedXRFImageStorage: UID = UID {
    ident: "EnhancedXRFImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.12.2.1",
    name: "Enhanced XRF Image Storage",
};

/// X-Ray Angiographic Bi-Plane Image Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.12.3
/// - **UID Type:** SOP Class
pub static XRayAngiographicBiPlaneImageStorage: UID = UID {
    ident: "XRayAngiographicBiPlaneImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.12.3",
    name: "X-Ray Angiographic Bi-Plane Image Storage (Retired)",
};

/// X-Ray 3D Angiographic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.13.1.1
/// - **UID Type:** SOP Class
pub static XRay3DAngiographicImageStorage: UID = UID {
    ident: "XRay3DAngiographicImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.13.1.1",
    name: "X-Ray 3D Angiographic Image Storage",
};

/// X-Ray 3D Craniofacial Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.13.1.2
/// - **UID Type:** SOP Class
pub static XRay3DCraniofacialImageStorage: UID = UID {
    ident: "XRay3DCraniofacialImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.13.1.2",
    name: "X-Ray 3D Craniofacial Image Storage",
};

/// Breast Tomosynthesis Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.13.1.3
/// - **UID Type:** SOP Class
pub static BreastTomosynthesisImageStorage: UID = UID {
    ident: "BreastTomosynthesisImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.13.1.3",
    name: "Breast Tomosynthesis Image Storage",
};

/// Breast Projection X-Ray Image Storage - For Presentation
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.13.1.4
/// - **UID Type:** SOP Class
pub static BreastProjectionXRayImageStorageForPresentation: UID = UID {
    ident: "BreastProjectionXRayImageStorageForPresentation",
    uid: "1.2.840.10008.5.1.4.1.1.13.1.4",
    name: "Breast Projection X-Ray Image Storage - For Presentation",
};

/// Breast Projection X-Ray Image Storage - For Processing
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.13.1.5
/// - **UID Type:** SOP Class
pub static BreastProjectionXRayImageStorageForProcessing: UID = UID {
    ident: "BreastProjectionXRayImageStorageForProcessing",
    uid: "1.2.840.10008.5.1.4.1.1.13.1.5",
    name: "Breast Projection X-Ray Image Storage - For Processing",
};

/// Intravascular Optical Coherence Tomography Image Storage - For Presentation
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.14.1
/// - **UID Type:** SOP Class
pub static IntravascularOpticalCoherenceTomographyImageStorageForPresentation: UID = UID {
    ident: "IntravascularOpticalCoherenceTomographyImageStorageForPresentation",
    uid: "1.2.840.10008.5.1.4.1.1.14.1",
    name: "Intravascular Optical Coherence Tomography Image Storage - For Presentation",
};

/// Intravascular Optical Coherence Tomography Image Storage - For Processing
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.14.2
/// - **UID Type:** SOP Class
pub static IntravascularOpticalCoherenceTomographyImageStorageForProcessing: UID = UID {
    ident: "IntravascularOpticalCoherenceTomographyImageStorageForProcessing",
    uid: "1.2.840.10008.5.1.4.1.1.14.2",
    name: "Intravascular Optical Coherence Tomography Image Storage - For Processing",
};

/// Nuclear Medicine Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.20
/// - **UID Type:** SOP Class
pub static NuclearMedicineImageStorage: UID = UID {
    ident: "NuclearMedicineImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.20",
    name: "Nuclear Medicine Image Storage",
};

/// Parametric Map Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.30
/// - **UID Type:** SOP Class
pub static ParametricMapStorage: UID = UID {
    ident: "ParametricMapStorage",
    uid: "1.2.840.10008.5.1.4.1.1.30",
    name: "Parametric Map Storage",
};

/// Raw Data Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.66
/// - **UID Type:** SOP Class
pub static RawDataStorage: UID = UID {
    ident: "RawDataStorage",
    uid: "1.2.840.10008.5.1.4.1.1.66",
    name: "Raw Data Storage",
};

/// Spatial Registration Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.66.1
/// - **UID Type:** SOP Class
pub static SpatialRegistrationStorage: UID = UID {
    ident: "SpatialRegistrationStorage",
    uid: "1.2.840.10008.5.1.4.1.1.66.1",
    name: "Spatial Registration Storage",
};

/// Spatial Fiducials Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.66.2
/// - **UID Type:** SOP Class
pub static SpatialFiducialsStorage: UID = UID {
    ident: "SpatialFiducialsStorage",
    uid: "1.2.840.10008.5.1.4.1.1.66.2",
    name: "Spatial Fiducials Storage",
};

/// Deformable Spatial Registration Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.66.3
/// - **UID Type:** SOP Class
pub static DeformableSpatialRegistrationStorage: UID = UID {
    ident: "DeformableSpatialRegistrationStorage",
    uid: "1.2.840.10008.5.1.4.1.1.66.3",
    name: "Deformable Spatial Registration Storage",
};

/// Segmentation Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.66.4
/// - **UID Type:** SOP Class
pub static SegmentationStorage: UID = UID {
    ident: "SegmentationStorage",
    uid: "1.2.840.10008.5.1.4.1.1.66.4",
    name: "Segmentation Storage",
};

/// Surface Segmentation Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.66.5
/// - **UID Type:** SOP Class
pub static SurfaceSegmentationStorage: UID = UID {
    ident: "SurfaceSegmentationStorage",
    uid: "1.2.840.10008.5.1.4.1.1.66.5",
    name: "Surface Segmentation Storage",
};

/// Tractography Results Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.66.6
/// - **UID Type:** SOP Class
pub static TractographyResultsStorage: UID = UID {
    ident: "TractographyResultsStorage",
    uid: "1.2.840.10008.5.1.4.1.1.66.6",
    name: "Tractography Results Storage",
};

/// Real World Value Mapping Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.67
/// - **UID Type:** SOP Class
pub static RealWorldValueMappingStorage: UID = UID {
    ident: "RealWorldValueMappingStorage",
    uid: "1.2.840.10008.5.1.4.1.1.67",
    name: "Real World Value Mapping Storage",
};

/// Surface Scan Mesh Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.68.1
/// - **UID Type:** SOP Class
pub static SurfaceScanMeshStorage: UID = UID {
    ident: "SurfaceScanMeshStorage",
    uid: "1.2.840.10008.5.1.4.1.1.68.1",
    name: "Surface Scan Mesh Storage",
};

/// Surface Scan Point Cloud Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.68.2
/// - **UID Type:** SOP Class
pub static SurfaceScanPointCloudStorage: UID = UID {
    ident: "SurfaceScanPointCloudStorage",
    uid: "1.2.840.10008.5.1.4.1.1.68.2",
    name: "Surface Scan Point Cloud Storage",
};

/// VL Image Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1
/// - **UID Type:** SOP Class
pub static VLImageStorageTrial: UID = UID {
    ident: "VLImageStorageTrial",
    uid: "1.2.840.10008.5.1.4.1.1.77.1",
    name: "VL Image Storage - Trial (Retired)",
};

/// VL Multi-frame Image Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.2
/// - **UID Type:** SOP Class
pub static VLMultiframeImageStorageTrial: UID = UID {
    ident: "VLMultiframeImageStorageTrial",
    uid: "1.2.840.10008.5.1.4.1.1.77.2",
    name: "VL Multi-frame Image Storage - Trial (Retired)",
};

/// VL Endoscopic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.1
/// - **UID Type:** SOP Class
pub static VLEndoscopicImageStorage: UID = UID {
    ident: "VLEndoscopicImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.1",
    name: "VL Endoscopic Image Storage",
};

/// Video Endoscopic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.1.1
/// - **UID Type:** SOP Class
pub static VideoEndoscopicImageStorage: UID = UID {
    ident: "VideoEndoscopicImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.1.1",
    name: "Video Endoscopic Image Storage",
};

/// VL Microscopic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.2
/// - **UID Type:** SOP Class
pub static VLMicroscopicImageStorage: UID = UID {
    ident: "VLMicroscopicImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.2",
    name: "VL Microscopic Image Storage",
};

/// Video Microscopic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.2.1
/// - **UID Type:** SOP Class
pub static VideoMicroscopicImageStorage: UID = UID {
    ident: "VideoMicroscopicImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.2.1",
    name: "Video Microscopic Image Storage",
};

/// VL Slide-Coordinates Microscopic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.3
/// - **UID Type:** SOP Class
pub static VLSlideCoordinatesMicroscopicImageStorage: UID = UID {
    ident: "VLSlideCoordinatesMicroscopicImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.3",
    name: "VL Slide-Coordinates Microscopic Image Storage",
};

/// VL Photographic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.4
/// - **UID Type:** SOP Class
pub static VLPhotographicImageStorage: UID = UID {
    ident: "VLPhotographicImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.4",
    name: "VL Photographic Image Storage",
};

/// Video Photographic Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.4.1
/// - **UID Type:** SOP Class
pub static VideoPhotographicImageStorage: UID = UID {
    ident: "VideoPhotographicImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.4.1",
    name: "Video Photographic Image Storage",
};

/// Ophthalmic Photography 8 Bit Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.1
/// - **UID Type:** SOP Class
pub static OphthalmicPhotography8BitImageStorage: UID = UID {
    ident: "OphthalmicPhotography8BitImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.1",
    name: "Ophthalmic Photography 8 Bit Image Storage",
};

/// Ophthalmic Photography 16 Bit Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.2
/// - **UID Type:** SOP Class
pub static OphthalmicPhotography16BitImageStorage: UID = UID {
    ident: "OphthalmicPhotography16BitImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.2",
    name: "Ophthalmic Photography 16 Bit Image Storage",
};

/// Stereometric Relationship Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.3
/// - **UID Type:** SOP Class
pub static StereometricRelationshipStorage: UID = UID {
    ident: "StereometricRelationshipStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.3",
    name: "Stereometric Relationship Storage",
};

/// Ophthalmic Tomography Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.4
/// - **UID Type:** SOP Class
pub static OphthalmicTomographyImageStorage: UID = UID {
    ident: "OphthalmicTomographyImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.4",
    name: "Ophthalmic Tomography Image Storage",
};

/// Wide Field Ophthalmic Photography Stereographic Projection Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.5
/// - **UID Type:** SOP Class
pub static WideFieldOphthalmicPhotographyStereographicProjectionImageStorage: UID = UID {
    ident: "WideFieldOphthalmicPhotographyStereographicProjectionImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.5",
    name: "Wide Field Ophthalmic Photography Stereographic Projection Image Storage",
};

/// Wide Field Ophthalmic Photography 3D Coordinates Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.6
/// - **UID Type:** SOP Class
pub static WideFieldOphthalmicPhotography3DCoordinatesImageStorage: UID = UID {
    ident: "WideFieldOphthalmicPhotography3DCoordinatesImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.6",
    name: "Wide Field Ophthalmic Photography 3D Coordinates Image Storage",
};

/// Ophthalmic Optical Coherence Tomography En Face Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.7
/// - **UID Type:** SOP Class
pub static OphthalmicOpticalCoherenceTomographyEnFaceImageStorage: UID = UID {
    ident: "OphthalmicOpticalCoherenceTomographyEnFaceImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.7",
    name: "Ophthalmic Optical Coherence Tomography En Face Image Storage",
};

/// Ophthalmic Optical Coherence Tomography B-scan Volume Analysis Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.5.8
/// - **UID Type:** SOP Class
pub static OphthalmicOpticalCoherenceTomographyBscanVolumeAnalysisStorage: UID = UID {
    ident: "OphthalmicOpticalCoherenceTomographyBscanVolumeAnalysisStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.5.8",
    name: "Ophthalmic Optical Coherence Tomography B-scan Volume Analysis Storage",
};

/// VL Whole Slide Microscopy Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.6
/// - **UID Type:** SOP Class
pub static VLWholeSlideMicroscopyImageStorage: UID = UID {
    ident: "VLWholeSlideMicroscopyImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.6",
    name: "VL Whole Slide Microscopy Image Storage",
};

/// Dermoscopic Photography Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.7
/// - **UID Type:** SOP Class
pub static DermoscopicPhotographyImageStorage: UID = UID {
    ident: "DermoscopicPhotographyImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.7",
    name: "Dermoscopic Photography Image Storage",
};

/// Confocal Microscopy Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.8
/// - **UID Type:** SOP Class
pub static ConfocalMicroscopyImageStorage: UID = UID {
    ident: "ConfocalMicroscopyImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.8",
    name: "Confocal Microscopy Image Storage",
};

/// Confocal Microscopy Tiled Pyramidal Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.77.1.9
/// - **UID Type:** SOP Class
pub static ConfocalMicroscopyTiledPyramidalImageStorage: UID = UID {
    ident: "ConfocalMicroscopyTiledPyramidalImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.77.1.9",
    name: "Confocal Microscopy Tiled Pyramidal Image Storage",
};

/// Lensometry Measurements Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.1
/// - **UID Type:** SOP Class
pub static LensometryMeasurementsStorage: UID = UID {
    ident: "LensometryMeasurementsStorage",
    uid: "1.2.840.10008.5.1.4.1.1.78.1",
    name: "Lensometry Measurements Storage",
};

/// Autorefraction Measurements Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.2
/// - **UID Type:** SOP Class
pub static AutorefractionMeasurementsStorage: UID = UID {
    ident: "AutorefractionMeasurementsStorage",
    uid: "1.2.840.10008.5.1.4.1.1.78.2",
    name: "Autorefraction Measurements Storage",
};

/// Keratometry Measurements Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.3
/// - **UID Type:** SOP Class
pub static KeratometryMeasurementsStorage: UID = UID {
    ident: "KeratometryMeasurementsStorage",
    uid: "1.2.840.10008.5.1.4.1.1.78.3",
    name: "Keratometry Measurements Storage",
};

/// Subjective Refraction Measurements Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.4
/// - **UID Type:** SOP Class
pub static SubjectiveRefractionMeasurementsStorage: UID = UID {
    ident: "SubjectiveRefractionMeasurementsStorage",
    uid: "1.2.840.10008.5.1.4.1.1.78.4",
    name: "Subjective Refraction Measurements Storage",
};

/// Visual Acuity Measurements Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.5
/// - **UID Type:** SOP Class
pub static VisualAcuityMeasurementsStorage: UID = UID {
    ident: "VisualAcuityMeasurementsStorage",
    uid: "1.2.840.10008.5.1.4.1.1.78.5",
    name: "Visual Acuity Measurements Storage",
};

/// Spectacle Prescription Report Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.6
/// - **UID Type:** SOP Class
pub static SpectaclePrescriptionReportStorage: UID = UID {
    ident: "SpectaclePrescriptionReportStorage",
    uid: "1.2.840.10008.5.1.4.1.1.78.6",
    name: "Spectacle Prescription Report Storage",
};

/// Ophthalmic Axial Measurements Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.7
/// - **UID Type:** SOP Class
pub static OphthalmicAxialMeasurementsStorage: UID = UID {
    ident: "OphthalmicAxialMeasurementsStorage",
    uid: "1.2.840.10008.5.1.4.1.1.78.7",
    name: "Ophthalmic Axial Measurements Storage",
};

/// Intraocular Lens Calculations Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.78.8
/// - **UID Type:** SOP Class
pub static IntraocularLensCalculationsStorage: UID = UID {
    ident: "IntraocularLensCalculationsStorage",
    uid: "1.2.840.10008.5.1.4.1.1.78.8",
    name: "Intraocular Lens Calculations Storage",
};

/// Macular Grid Thickness and Volume Report Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.79.1
/// - **UID Type:** SOP Class
pub static MacularGridThicknessandVolumeReportStorage: UID = UID {
    ident: "MacularGridThicknessandVolumeReportStorage",
    uid: "1.2.840.10008.5.1.4.1.1.79.1",
    name: "Macular Grid Thickness and Volume Report Storage",
};

/// Ophthalmic Visual Field Static Perimetry Measurements Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.80.1
/// - **UID Type:** SOP Class
pub static OphthalmicVisualFieldStaticPerimetryMeasurementsStorage: UID = UID {
    ident: "OphthalmicVisualFieldStaticPerimetryMeasurementsStorage",
    uid: "1.2.840.10008.5.1.4.1.1.80.1",
    name: "Ophthalmic Visual Field Static Perimetry Measurements Storage",
};

/// Ophthalmic Thickness Map Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.81.1
/// - **UID Type:** SOP Class
pub static OphthalmicThicknessMapStorage: UID = UID {
    ident: "OphthalmicThicknessMapStorage",
    uid: "1.2.840.10008.5.1.4.1.1.81.1",
    name: "Ophthalmic Thickness Map Storage",
};

/// Corneal Topography Map Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.82.1
/// - **UID Type:** SOP Class
pub static CornealTopographyMapStorage: UID = UID {
    ident: "CornealTopographyMapStorage",
    uid: "1.2.840.10008.5.1.4.1.1.82.1",
    name: "Corneal Topography Map Storage",
};

/// Text SR Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.1
/// - **UID Type:** SOP Class
pub static TextSRStorageTrial: UID = UID {
    ident: "TextSRStorageTrial",
    uid: "1.2.840.10008.5.1.4.1.1.88.1",
    name: "Text SR Storage - Trial (Retired)",
};

/// Audio SR Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.2
/// - **UID Type:** SOP Class
pub static AudioSRStorageTrial: UID = UID {
    ident: "AudioSRStorageTrial",
    uid: "1.2.840.10008.5.1.4.1.1.88.2",
    name: "Audio SR Storage - Trial (Retired)",
};

/// Detail SR Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.3
/// - **UID Type:** SOP Class
pub static DetailSRStorageTrial: UID = UID {
    ident: "DetailSRStorageTrial",
    uid: "1.2.840.10008.5.1.4.1.1.88.3",
    name: "Detail SR Storage - Trial (Retired)",
};

/// Comprehensive SR Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.4
/// - **UID Type:** SOP Class
pub static ComprehensiveSRStorageTrial: UID = UID {
    ident: "ComprehensiveSRStorageTrial",
    uid: "1.2.840.10008.5.1.4.1.1.88.4",
    name: "Comprehensive SR Storage - Trial (Retired)",
};

/// Basic Text SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.11
/// - **UID Type:** SOP Class
pub static BasicTextSRStorage: UID = UID {
    ident: "BasicTextSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.11",
    name: "Basic Text SR Storage",
};

/// Enhanced SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.22
/// - **UID Type:** SOP Class
pub static EnhancedSRStorage: UID = UID {
    ident: "EnhancedSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.22",
    name: "Enhanced SR Storage",
};

/// Comprehensive SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.33
/// - **UID Type:** SOP Class
pub static ComprehensiveSRStorage: UID = UID {
    ident: "ComprehensiveSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.33",
    name: "Comprehensive SR Storage",
};

/// Comprehensive 3D SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.34
/// - **UID Type:** SOP Class
pub static Comprehensive3DSRStorage: UID = UID {
    ident: "Comprehensive3DSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.34",
    name: "Comprehensive 3D SR Storage",
};

/// Extensible SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.35
/// - **UID Type:** SOP Class
pub static ExtensibleSRStorage: UID = UID {
    ident: "ExtensibleSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.35",
    name: "Extensible SR Storage",
};

/// Procedure Log Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.40
/// - **UID Type:** SOP Class
pub static ProcedureLogStorage: UID = UID {
    ident: "ProcedureLogStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.40",
    name: "Procedure Log Storage",
};

/// Mammography CAD SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.50
/// - **UID Type:** SOP Class
pub static MammographyCADSRStorage: UID = UID {
    ident: "MammographyCADSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.50",
    name: "Mammography CAD SR Storage",
};

/// Key Object Selection Document Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.59
/// - **UID Type:** SOP Class
pub static KeyObjectSelectionDocumentStorage: UID = UID {
    ident: "KeyObjectSelectionDocumentStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.59",
    name: "Key Object Selection Document Storage",
};

/// Chest CAD SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.65
/// - **UID Type:** SOP Class
pub static ChestCADSRStorage: UID = UID {
    ident: "ChestCADSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.65",
    name: "Chest CAD SR Storage",
};

/// X-Ray Radiation Dose SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.67
/// - **UID Type:** SOP Class
pub static XRayRadiationDoseSRStorage: UID = UID {
    ident: "XRayRadiationDoseSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.67",
    name: "X-Ray Radiation Dose SR Storage",
};

/// Radiopharmaceutical Radiation Dose SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.68
/// - **UID Type:** SOP Class
pub static RadiopharmaceuticalRadiationDoseSRStorage: UID = UID {
    ident: "RadiopharmaceuticalRadiationDoseSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.68",
    name: "Radiopharmaceutical Radiation Dose SR Storage",
};

/// Colon CAD SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.69
/// - **UID Type:** SOP Class
pub static ColonCADSRStorage: UID = UID {
    ident: "ColonCADSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.69",
    name: "Colon CAD SR Storage",
};

/// Implantation Plan SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.70
/// - **UID Type:** SOP Class
pub static ImplantationPlanSRStorage: UID = UID {
    ident: "ImplantationPlanSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.70",
    name: "Implantation Plan SR Storage",
};

/// Acquisition Context SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.71
/// - **UID Type:** SOP Class
pub static AcquisitionContextSRStorage: UID = UID {
    ident: "AcquisitionContextSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.71",
    name: "Acquisition Context SR Storage",
};

/// Simplified Adult Echo SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.72
/// - **UID Type:** SOP Class
pub static SimplifiedAdultEchoSRStorage: UID = UID {
    ident: "SimplifiedAdultEchoSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.72",
    name: "Simplified Adult Echo SR Storage",
};

/// Patient Radiation Dose SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.73
/// - **UID Type:** SOP Class
pub static PatientRadiationDoseSRStorage: UID = UID {
    ident: "PatientRadiationDoseSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.73",
    name: "Patient Radiation Dose SR Storage",
};

/// Planned Imaging Agent Administration SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.74
/// - **UID Type:** SOP Class
pub static PlannedImagingAgentAdministrationSRStorage: UID = UID {
    ident: "PlannedImagingAgentAdministrationSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.74",
    name: "Planned Imaging Agent Administration SR Storage",
};

/// Performed Imaging Agent Administration SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.75
/// - **UID Type:** SOP Class
pub static PerformedImagingAgentAdministrationSRStorage: UID = UID {
    ident: "PerformedImagingAgentAdministrationSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.75",
    name: "Performed Imaging Agent Administration SR Storage",
};

/// Enhanced X-Ray Radiation Dose SR Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.88.76
/// - **UID Type:** SOP Class
pub static EnhancedXRayRadiationDoseSRStorage: UID = UID {
    ident: "EnhancedXRayRadiationDoseSRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.88.76",
    name: "Enhanced X-Ray Radiation Dose SR Storage",
};

/// Content Assessment Results Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.90.1
/// - **UID Type:** SOP Class
pub static ContentAssessmentResultsStorage: UID = UID {
    ident: "ContentAssessmentResultsStorage",
    uid: "1.2.840.10008.5.1.4.1.1.90.1",
    name: "Content Assessment Results Storage",
};

/// Microscopy Bulk Simple Annotations Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.91.1
/// - **UID Type:** SOP Class
pub static MicroscopyBulkSimpleAnnotationsStorage: UID = UID {
    ident: "MicroscopyBulkSimpleAnnotationsStorage",
    uid: "1.2.840.10008.5.1.4.1.1.91.1",
    name: "Microscopy Bulk Simple Annotations Storage",
};

/// Encapsulated PDF Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.104.1
/// - **UID Type:** SOP Class
pub static EncapsulatedPDFStorage: UID = UID {
    ident: "EncapsulatedPDFStorage",
    uid: "1.2.840.10008.5.1.4.1.1.104.1",
    name: "Encapsulated PDF Storage",
};

/// Encapsulated CDA Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.104.2
/// - **UID Type:** SOP Class
pub static EncapsulatedCDAStorage: UID = UID {
    ident: "EncapsulatedCDAStorage",
    uid: "1.2.840.10008.5.1.4.1.1.104.2",
    name: "Encapsulated CDA Storage",
};

/// Encapsulated STL Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.104.3
/// - **UID Type:** SOP Class
pub static EncapsulatedSTLStorage: UID = UID {
    ident: "EncapsulatedSTLStorage",
    uid: "1.2.840.10008.5.1.4.1.1.104.3",
    name: "Encapsulated STL Storage",
};

/// Encapsulated OBJ Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.104.4
/// - **UID Type:** SOP Class
pub static EncapsulatedOBJStorage: UID = UID {
    ident: "EncapsulatedOBJStorage",
    uid: "1.2.840.10008.5.1.4.1.1.104.4",
    name: "Encapsulated OBJ Storage",
};

/// Encapsulated MTL Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.104.5
/// - **UID Type:** SOP Class
pub static EncapsulatedMTLStorage: UID = UID {
    ident: "EncapsulatedMTLStorage",
    uid: "1.2.840.10008.5.1.4.1.1.104.5",
    name: "Encapsulated MTL Storage",
};

/// Positron Emission Tomography Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.128
/// - **UID Type:** SOP Class
pub static PositronEmissionTomographyImageStorage: UID = UID {
    ident: "PositronEmissionTomographyImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.128",
    name: "Positron Emission Tomography Image Storage",
};

/// Legacy Converted Enhanced PET Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.128.1
/// - **UID Type:** SOP Class
pub static LegacyConvertedEnhancedPETImageStorage: UID = UID {
    ident: "LegacyConvertedEnhancedPETImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.128.1",
    name: "Legacy Converted Enhanced PET Image Storage",
};

/// Standalone PET Curve Storage (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.129
/// - **UID Type:** SOP Class
pub static StandalonePETCurveStorage: UID = UID {
    ident: "StandalonePETCurveStorage",
    uid: "1.2.840.10008.5.1.4.1.1.129",
    name: "Standalone PET Curve Storage (Retired)",
};

/// Enhanced PET Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.130
/// - **UID Type:** SOP Class
pub static EnhancedPETImageStorage: UID = UID {
    ident: "EnhancedPETImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.130",
    name: "Enhanced PET Image Storage",
};

/// Basic Structured Display Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.131
/// - **UID Type:** SOP Class
pub static BasicStructuredDisplayStorage: UID = UID {
    ident: "BasicStructuredDisplayStorage",
    uid: "1.2.840.10008.5.1.4.1.1.131",
    name: "Basic Structured Display Storage",
};

/// CT Defined Procedure Protocol Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.1
/// - **UID Type:** SOP Class
pub static CTDefinedProcedureProtocolStorage: UID = UID {
    ident: "CTDefinedProcedureProtocolStorage",
    uid: "1.2.840.10008.5.1.4.1.1.200.1",
    name: "CT Defined Procedure Protocol Storage",
};

/// CT Performed Procedure Protocol Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.2
/// - **UID Type:** SOP Class
pub static CTPerformedProcedureProtocolStorage: UID = UID {
    ident: "CTPerformedProcedureProtocolStorage",
    uid: "1.2.840.10008.5.1.4.1.1.200.2",
    name: "CT Performed Procedure Protocol Storage",
};

/// Protocol Approval Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.3
/// - **UID Type:** SOP Class
pub static ProtocolApprovalStorage: UID = UID {
    ident: "ProtocolApprovalStorage",
    uid: "1.2.840.10008.5.1.4.1.1.200.3",
    name: "Protocol Approval Storage",
};

/// Protocol Approval Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.4
/// - **UID Type:** SOP Class
pub static ProtocolApprovalInformationModelFIND: UID = UID {
    ident: "ProtocolApprovalInformationModelFIND",
    uid: "1.2.840.10008.5.1.4.1.1.200.4",
    name: "Protocol Approval Information Model - FIND",
};

/// Protocol Approval Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.5
/// - **UID Type:** SOP Class
pub static ProtocolApprovalInformationModelMOVE: UID = UID {
    ident: "ProtocolApprovalInformationModelMOVE",
    uid: "1.2.840.10008.5.1.4.1.1.200.5",
    name: "Protocol Approval Information Model - MOVE",
};

/// Protocol Approval Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.6
/// - **UID Type:** SOP Class
pub static ProtocolApprovalInformationModelGET: UID = UID {
    ident: "ProtocolApprovalInformationModelGET",
    uid: "1.2.840.10008.5.1.4.1.1.200.6",
    name: "Protocol Approval Information Model - GET",
};

/// XA Defined Procedure Protocol Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.7
/// - **UID Type:** SOP Class
pub static XADefinedProcedureProtocolStorage: UID = UID {
    ident: "XADefinedProcedureProtocolStorage",
    uid: "1.2.840.10008.5.1.4.1.1.200.7",
    name: "XA Defined Procedure Protocol Storage",
};

/// XA Performed Procedure Protocol Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.200.8
/// - **UID Type:** SOP Class
pub static XAPerformedProcedureProtocolStorage: UID = UID {
    ident: "XAPerformedProcedureProtocolStorage",
    uid: "1.2.840.10008.5.1.4.1.1.200.8",
    name: "XA Performed Procedure Protocol Storage",
};

/// Inventory Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.201.1
/// - **UID Type:** SOP Class
pub static InventoryStorage: UID = UID {
    ident: "InventoryStorage",
    uid: "1.2.840.10008.5.1.4.1.1.201.1",
    name: "Inventory Storage",
};

/// Inventory - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.201.2
/// - **UID Type:** SOP Class
pub static InventoryFIND: UID = UID {
    ident: "InventoryFIND",
    uid: "1.2.840.10008.5.1.4.1.1.201.2",
    name: "Inventory - FIND",
};

/// Inventory - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.201.3
/// - **UID Type:** SOP Class
pub static InventoryMOVE: UID = UID {
    ident: "InventoryMOVE",
    uid: "1.2.840.10008.5.1.4.1.1.201.3",
    name: "Inventory - MOVE",
};

/// Inventory - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.201.4
/// - **UID Type:** SOP Class
pub static InventoryGET: UID = UID {
    ident: "InventoryGET",
    uid: "1.2.840.10008.5.1.4.1.1.201.4",
    name: "Inventory - GET",
};

/// Inventory Creation
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.201.5
/// - **UID Type:** SOP Class
pub static InventoryCreation: UID = UID {
    ident: "InventoryCreation",
    uid: "1.2.840.10008.5.1.4.1.1.201.5",
    name: "Inventory Creation",
};

/// Repository Query
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.201.6
/// - **UID Type:** SOP Class
pub static RepositoryQuery: UID = UID {
    ident: "RepositoryQuery",
    uid: "1.2.840.10008.5.1.4.1.1.201.6",
    name: "Repository Query",
};

/// Storage Management SOP Instance
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.201.1.1
/// - **UID Type:** Well-known SOP Instance
pub static StorageManagementSOPInstance: UID = UID {
    ident: "StorageManagementSOPInstance",
    uid: "1.2.840.10008.5.1.4.1.1.201.1.1",
    name: "Storage Management SOP Instance",
};

/// RT Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.1
/// - **UID Type:** SOP Class
pub static RTImageStorage: UID = UID {
    ident: "RTImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.1",
    name: "RT Image Storage",
};

/// RT Dose Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.2
/// - **UID Type:** SOP Class
pub static RTDoseStorage: UID = UID {
    ident: "RTDoseStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.2",
    name: "RT Dose Storage",
};

/// RT Structure Set Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.3
/// - **UID Type:** SOP Class
pub static RTStructureSetStorage: UID = UID {
    ident: "RTStructureSetStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.3",
    name: "RT Structure Set Storage",
};

/// RT Beams Treatment Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.4
/// - **UID Type:** SOP Class
pub static RTBeamsTreatmentRecordStorage: UID = UID {
    ident: "RTBeamsTreatmentRecordStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.4",
    name: "RT Beams Treatment Record Storage",
};

/// RT Plan Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.5
/// - **UID Type:** SOP Class
pub static RTPlanStorage: UID = UID {
    ident: "RTPlanStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.5",
    name: "RT Plan Storage",
};

/// RT Brachy Treatment Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.6
/// - **UID Type:** SOP Class
pub static RTBrachyTreatmentRecordStorage: UID = UID {
    ident: "RTBrachyTreatmentRecordStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.6",
    name: "RT Brachy Treatment Record Storage",
};

/// RT Treatment Summary Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.7
/// - **UID Type:** SOP Class
pub static RTTreatmentSummaryRecordStorage: UID = UID {
    ident: "RTTreatmentSummaryRecordStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.7",
    name: "RT Treatment Summary Record Storage",
};

/// RT Ion Plan Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.8
/// - **UID Type:** SOP Class
pub static RTIonPlanStorage: UID = UID {
    ident: "RTIonPlanStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.8",
    name: "RT Ion Plan Storage",
};

/// RT Ion Beams Treatment Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.9
/// - **UID Type:** SOP Class
pub static RTIonBeamsTreatmentRecordStorage: UID = UID {
    ident: "RTIonBeamsTreatmentRecordStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.9",
    name: "RT Ion Beams Treatment Record Storage",
};

/// RT Physician Intent Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.10
/// - **UID Type:** SOP Class
pub static RTPhysicianIntentStorage: UID = UID {
    ident: "RTPhysicianIntentStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.10",
    name: "RT Physician Intent Storage",
};

/// RT Segment Annotation Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.11
/// - **UID Type:** SOP Class
pub static RTSegmentAnnotationStorage: UID = UID {
    ident: "RTSegmentAnnotationStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.11",
    name: "RT Segment Annotation Storage",
};

/// RT Radiation Set Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.12
/// - **UID Type:** SOP Class
pub static RTRadiationSetStorage: UID = UID {
    ident: "RTRadiationSetStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.12",
    name: "RT Radiation Set Storage",
};

/// C-Arm Photon-Electron Radiation Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.13
/// - **UID Type:** SOP Class
pub static CArmPhotonElectronRadiationStorage: UID = UID {
    ident: "CArmPhotonElectronRadiationStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.13",
    name: "C-Arm Photon-Electron Radiation Storage",
};

/// Tomotherapeutic Radiation Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.14
/// - **UID Type:** SOP Class
pub static TomotherapeuticRadiationStorage: UID = UID {
    ident: "TomotherapeuticRadiationStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.14",
    name: "Tomotherapeutic Radiation Storage",
};

/// Robotic-Arm Radiation Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.15
/// - **UID Type:** SOP Class
pub static RoboticArmRadiationStorage: UID = UID {
    ident: "RoboticArmRadiationStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.15",
    name: "Robotic-Arm Radiation Storage",
};

/// RT Radiation Record Set Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.16
/// - **UID Type:** SOP Class
pub static RTRadiationRecordSetStorage: UID = UID {
    ident: "RTRadiationRecordSetStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.16",
    name: "RT Radiation Record Set Storage",
};

/// RT Radiation Salvage Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.17
/// - **UID Type:** SOP Class
pub static RTRadiationSalvageRecordStorage: UID = UID {
    ident: "RTRadiationSalvageRecordStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.17",
    name: "RT Radiation Salvage Record Storage",
};

/// Tomotherapeutic Radiation Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.18
/// - **UID Type:** SOP Class
pub static TomotherapeuticRadiationRecordStorage: UID = UID {
    ident: "TomotherapeuticRadiationRecordStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.18",
    name: "Tomotherapeutic Radiation Record Storage",
};

/// C-Arm Photon-Electron Radiation Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.19
/// - **UID Type:** SOP Class
pub static CArmPhotonElectronRadiationRecordStorage: UID = UID {
    ident: "CArmPhotonElectronRadiationRecordStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.19",
    name: "C-Arm Photon-Electron Radiation Record Storage",
};

/// Robotic Radiation Record Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.20
/// - **UID Type:** SOP Class
pub static RoboticRadiationRecordStorage: UID = UID {
    ident: "RoboticRadiationRecordStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.20",
    name: "Robotic Radiation Record Storage",
};

/// RT Radiation Set Delivery Instruction Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.21
/// - **UID Type:** SOP Class
pub static RTRadiationSetDeliveryInstructionStorage: UID = UID {
    ident: "RTRadiationSetDeliveryInstructionStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.21",
    name: "RT Radiation Set Delivery Instruction Storage",
};

/// RT Treatment Preparation Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.22
/// - **UID Type:** SOP Class
pub static RTTreatmentPreparationStorage: UID = UID {
    ident: "RTTreatmentPreparationStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.22",
    name: "RT Treatment Preparation Storage",
};

/// Enhanced RT Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.23
/// - **UID Type:** SOP Class
pub static EnhancedRTImageStorage: UID = UID {
    ident: "EnhancedRTImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.23",
    name: "Enhanced RT Image Storage",
};

/// Enhanced Continuous RT Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.24
/// - **UID Type:** SOP Class
pub static EnhancedContinuousRTImageStorage: UID = UID {
    ident: "EnhancedContinuousRTImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.24",
    name: "Enhanced Continuous RT Image Storage",
};

/// RT Patient Position Acquisition Instruction Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.481.25
/// - **UID Type:** SOP Class
pub static RTPatientPositionAcquisitionInstructionStorage: UID = UID {
    ident: "RTPatientPositionAcquisitionInstructionStorage",
    uid: "1.2.840.10008.5.1.4.1.1.481.25",
    name: "RT Patient Position Acquisition Instruction Storage",
};

/// DICOS CT Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.501.1
/// - **UID Type:** SOP Class
pub static DICOSCTImageStorage: UID = UID {
    ident: "DICOSCTImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.501.1",
    name: "DICOS CT Image Storage",
};

/// DICOS Digital X-Ray Image Storage - For Presentation
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.501.2.1
/// - **UID Type:** SOP Class
pub static DICOSDigitalXRayImageStorageForPresentation: UID = UID {
    ident: "DICOSDigitalXRayImageStorageForPresentation",
    uid: "1.2.840.10008.5.1.4.1.1.501.2.1",
    name: "DICOS Digital X-Ray Image Storage - For Presentation",
};

/// DICOS Digital X-Ray Image Storage - For Processing
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.501.2.2
/// - **UID Type:** SOP Class
pub static DICOSDigitalXRayImageStorageForProcessing: UID = UID {
    ident: "DICOSDigitalXRayImageStorageForProcessing",
    uid: "1.2.840.10008.5.1.4.1.1.501.2.2",
    name: "DICOS Digital X-Ray Image Storage - For Processing",
};

/// DICOS Threat Detection Report Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.501.3
/// - **UID Type:** SOP Class
pub static DICOSThreatDetectionReportStorage: UID = UID {
    ident: "DICOSThreatDetectionReportStorage",
    uid: "1.2.840.10008.5.1.4.1.1.501.3",
    name: "DICOS Threat Detection Report Storage",
};

/// DICOS 2D AIT Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.501.4
/// - **UID Type:** SOP Class
pub static DICOS2DAITStorage: UID = UID {
    ident: "DICOS2DAITStorage",
    uid: "1.2.840.10008.5.1.4.1.1.501.4",
    name: "DICOS 2D AIT Storage",
};

/// DICOS 3D AIT Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.501.5
/// - **UID Type:** SOP Class
pub static DICOS3DAITStorage: UID = UID {
    ident: "DICOS3DAITStorage",
    uid: "1.2.840.10008.5.1.4.1.1.501.5",
    name: "DICOS 3D AIT Storage",
};

/// DICOS Quadrupole Resonance (QR) Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.501.6
/// - **UID Type:** SOP Class
pub static DICOSQuadrupoleResonanceQRStorage: UID = UID {
    ident: "DICOSQuadrupoleResonanceQRStorage",
    uid: "1.2.840.10008.5.1.4.1.1.501.6",
    name: "DICOS Quadrupole Resonance (QR) Storage",
};

/// Eddy Current Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.601.1
/// - **UID Type:** SOP Class
pub static EddyCurrentImageStorage: UID = UID {
    ident: "EddyCurrentImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.601.1",
    name: "Eddy Current Image Storage",
};

/// Eddy Current Multi-frame Image Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.1.1.601.2
/// - **UID Type:** SOP Class
pub static EddyCurrentMultiframeImageStorage: UID = UID {
    ident: "EddyCurrentMultiframeImageStorage",
    uid: "1.2.840.10008.5.1.4.1.1.601.2",
    name: "Eddy Current Multi-frame Image Storage",
};

/// Patient Root Query/Retrieve Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.1.1
/// - **UID Type:** SOP Class
pub static PatientRootQueryRetrieveInformationModelFIND: UID = UID {
    ident: "PatientRootQueryRetrieveInformationModelFIND",
    uid: "1.2.840.10008.5.1.4.1.2.1.1",
    name: "Patient Root Query/Retrieve Information Model - FIND",
};

/// Patient Root Query/Retrieve Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.1.2
/// - **UID Type:** SOP Class
pub static PatientRootQueryRetrieveInformationModelMOVE: UID = UID {
    ident: "PatientRootQueryRetrieveInformationModelMOVE",
    uid: "1.2.840.10008.5.1.4.1.2.1.2",
    name: "Patient Root Query/Retrieve Information Model - MOVE",
};

/// Patient Root Query/Retrieve Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.1.3
/// - **UID Type:** SOP Class
pub static PatientRootQueryRetrieveInformationModelGET: UID = UID {
    ident: "PatientRootQueryRetrieveInformationModelGET",
    uid: "1.2.840.10008.5.1.4.1.2.1.3",
    name: "Patient Root Query/Retrieve Information Model - GET",
};

/// Study Root Query/Retrieve Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.2.1
/// - **UID Type:** SOP Class
pub static StudyRootQueryRetrieveInformationModelFIND: UID = UID {
    ident: "StudyRootQueryRetrieveInformationModelFIND",
    uid: "1.2.840.10008.5.1.4.1.2.2.1",
    name: "Study Root Query/Retrieve Information Model - FIND",
};

/// Study Root Query/Retrieve Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.2.2
/// - **UID Type:** SOP Class
pub static StudyRootQueryRetrieveInformationModelMOVE: UID = UID {
    ident: "StudyRootQueryRetrieveInformationModelMOVE",
    uid: "1.2.840.10008.5.1.4.1.2.2.2",
    name: "Study Root Query/Retrieve Information Model - MOVE",
};

/// Study Root Query/Retrieve Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.2.3
/// - **UID Type:** SOP Class
pub static StudyRootQueryRetrieveInformationModelGET: UID = UID {
    ident: "StudyRootQueryRetrieveInformationModelGET",
    uid: "1.2.840.10008.5.1.4.1.2.2.3",
    name: "Study Root Query/Retrieve Information Model - GET",
};

/// Patient/Study Only Query/Retrieve Information Model - FIND (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.3.1
/// - **UID Type:** SOP Class
pub static PatientStudyOnlyQueryRetrieveInformationModelFIND: UID = UID {
    ident: "PatientStudyOnlyQueryRetrieveInformationModelFIND",
    uid: "1.2.840.10008.5.1.4.1.2.3.1",
    name: "Patient/Study Only Query/Retrieve Information Model - FIND (Retired)",
};

/// Patient/Study Only Query/Retrieve Information Model - MOVE (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.3.2
/// - **UID Type:** SOP Class
pub static PatientStudyOnlyQueryRetrieveInformationModelMOVE: UID = UID {
    ident: "PatientStudyOnlyQueryRetrieveInformationModelMOVE",
    uid: "1.2.840.10008.5.1.4.1.2.3.2",
    name: "Patient/Study Only Query/Retrieve Information Model - MOVE (Retired)",
};

/// Patient/Study Only Query/Retrieve Information Model - GET (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.3.3
/// - **UID Type:** SOP Class
pub static PatientStudyOnlyQueryRetrieveInformationModelGET: UID = UID {
    ident: "PatientStudyOnlyQueryRetrieveInformationModelGET",
    uid: "1.2.840.10008.5.1.4.1.2.3.3",
    name: "Patient/Study Only Query/Retrieve Information Model - GET (Retired)",
};

/// Composite Instance Root Retrieve - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.4.2
/// - **UID Type:** SOP Class
pub static CompositeInstanceRootRetrieveMOVE: UID = UID {
    ident: "CompositeInstanceRootRetrieveMOVE",
    uid: "1.2.840.10008.5.1.4.1.2.4.2",
    name: "Composite Instance Root Retrieve - MOVE",
};

/// Composite Instance Root Retrieve - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.4.3
/// - **UID Type:** SOP Class
pub static CompositeInstanceRootRetrieveGET: UID = UID {
    ident: "CompositeInstanceRootRetrieveGET",
    uid: "1.2.840.10008.5.1.4.1.2.4.3",
    name: "Composite Instance Root Retrieve - GET",
};

/// Composite Instance Retrieve Without Bulk Data - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.1.2.5.3
/// - **UID Type:** SOP Class
pub static CompositeInstanceRetrieveWithoutBulkDataGET: UID = UID {
    ident: "CompositeInstanceRetrieveWithoutBulkDataGET",
    uid: "1.2.840.10008.5.1.4.1.2.5.3",
    name: "Composite Instance Retrieve Without Bulk Data - GET",
};

/// Defined Procedure Protocol Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.20.1
/// - **UID Type:** SOP Class
pub static DefinedProcedureProtocolInformationModelFIND: UID = UID {
    ident: "DefinedProcedureProtocolInformationModelFIND",
    uid: "1.2.840.10008.5.1.4.20.1",
    name: "Defined Procedure Protocol Information Model - FIND",
};

/// Defined Procedure Protocol Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.20.2
/// - **UID Type:** SOP Class
pub static DefinedProcedureProtocolInformationModelMOVE: UID = UID {
    ident: "DefinedProcedureProtocolInformationModelMOVE",
    uid: "1.2.840.10008.5.1.4.20.2",
    name: "Defined Procedure Protocol Information Model - MOVE",
};

/// Defined Procedure Protocol Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.20.3
/// - **UID Type:** SOP Class
pub static DefinedProcedureProtocolInformationModelGET: UID = UID {
    ident: "DefinedProcedureProtocolInformationModelGET",
    uid: "1.2.840.10008.5.1.4.20.3",
    name: "Defined Procedure Protocol Information Model - GET",
};

/// Modality Worklist Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.31
/// - **UID Type:** SOP Class
pub static ModalityWorklistInformationModelFIND: UID = UID {
    ident: "ModalityWorklistInformationModelFIND",
    uid: "1.2.840.10008.5.1.4.31",
    name: "Modality Worklist Information Model - FIND",
};

/// General Purpose Worklist Management Meta SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.32
/// - **UID Type:** Meta SOP Class
pub static GeneralPurposeWorklistManagementMetaSOPClass: UID = UID {
    ident: "GeneralPurposeWorklistManagementMetaSOPClass",
    uid: "1.2.840.10008.5.1.4.32",
    name: "General Purpose Worklist Management Meta SOP Class (Retired)",
};

/// General Purpose Worklist Information Model - FIND (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.32.1
/// - **UID Type:** SOP Class
pub static GeneralPurposeWorklistInformationModelFIND: UID = UID {
    ident: "GeneralPurposeWorklistInformationModelFIND",
    uid: "1.2.840.10008.5.1.4.32.1",
    name: "General Purpose Worklist Information Model - FIND (Retired)",
};

/// General Purpose Scheduled Procedure Step SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.32.2
/// - **UID Type:** SOP Class
pub static GeneralPurposeScheduledProcedureStepSOPClass: UID = UID {
    ident: "GeneralPurposeScheduledProcedureStepSOPClass",
    uid: "1.2.840.10008.5.1.4.32.2",
    name: "General Purpose Scheduled Procedure Step SOP Class (Retired)",
};

/// General Purpose Performed Procedure Step SOP Class (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.32.3
/// - **UID Type:** SOP Class
pub static GeneralPurposePerformedProcedureStepSOPClass: UID = UID {
    ident: "GeneralPurposePerformedProcedureStepSOPClass",
    uid: "1.2.840.10008.5.1.4.32.3",
    name: "General Purpose Performed Procedure Step SOP Class (Retired)",
};

/// Instance Availability Notification SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.33
/// - **UID Type:** SOP Class
pub static InstanceAvailabilityNotificationSOPClass: UID = UID {
    ident: "InstanceAvailabilityNotificationSOPClass",
    uid: "1.2.840.10008.5.1.4.33",
    name: "Instance Availability Notification SOP Class",
};

/// RT Beams Delivery Instruction Storage - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.1
/// - **UID Type:** SOP Class
pub static RTBeamsDeliveryInstructionStorageTrial: UID = UID {
    ident: "RTBeamsDeliveryInstructionStorageTrial",
    uid: "1.2.840.10008.5.1.4.34.1",
    name: "RT Beams Delivery Instruction Storage - Trial (Retired)",
};

/// RT Conventional Machine Verification - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.2
/// - **UID Type:** SOP Class
pub static RTConventionalMachineVerificationTrial: UID = UID {
    ident: "RTConventionalMachineVerificationTrial",
    uid: "1.2.840.10008.5.1.4.34.2",
    name: "RT Conventional Machine Verification - Trial (Retired)",
};

/// RT Ion Machine Verification - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.3
/// - **UID Type:** SOP Class
pub static RTIonMachineVerificationTrial: UID = UID {
    ident: "RTIonMachineVerificationTrial",
    uid: "1.2.840.10008.5.1.4.34.3",
    name: "RT Ion Machine Verification - Trial (Retired)",
};

/// Unified Worklist and Procedure Step Service Class - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.4
/// - **UID Type:** Service Class
pub static UnifiedWorklistandProcedureStepServiceClassTrial: UID = UID {
    ident: "UnifiedWorklistandProcedureStepServiceClassTrial",
    uid: "1.2.840.10008.5.1.4.34.4",
    name: "Unified Worklist and Procedure Step Service Class - Trial (Retired)",
};

/// Unified Procedure Step - Push SOP Class - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.4.1
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepPushSOPClassTrial: UID = UID {
    ident: "UnifiedProcedureStepPushSOPClassTrial",
    uid: "1.2.840.10008.5.1.4.34.4.1",
    name: "Unified Procedure Step - Push SOP Class - Trial (Retired)",
};

/// Unified Procedure Step - Watch SOP Class - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.4.2
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepWatchSOPClassTrial: UID = UID {
    ident: "UnifiedProcedureStepWatchSOPClassTrial",
    uid: "1.2.840.10008.5.1.4.34.4.2",
    name: "Unified Procedure Step - Watch SOP Class - Trial (Retired)",
};

/// Unified Procedure Step - Pull SOP Class - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.4.3
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepPullSOPClassTrial: UID = UID {
    ident: "UnifiedProcedureStepPullSOPClassTrial",
    uid: "1.2.840.10008.5.1.4.34.4.3",
    name: "Unified Procedure Step - Pull SOP Class - Trial (Retired)",
};

/// Unified Procedure Step - Event SOP Class - Trial (Retired)
///
/// - **UID:** 1.2.840.10008.5.1.4.34.4.4
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepEventSOPClassTrial: UID = UID {
    ident: "UnifiedProcedureStepEventSOPClassTrial",
    uid: "1.2.840.10008.5.1.4.34.4.4",
    name: "Unified Procedure Step - Event SOP Class - Trial (Retired)",
};

/// UPS Global Subscription SOP Instance
///
/// - **UID:** 1.2.840.10008.5.1.4.34.5
/// - **UID Type:** Well-known SOP Instance
pub static UPSGlobalSubscriptionSOPInstance: UID = UID {
    ident: "UPSGlobalSubscriptionSOPInstance",
    uid: "1.2.840.10008.5.1.4.34.5",
    name: "UPS Global Subscription SOP Instance",
};

/// UPS Filtered Global Subscription SOP Instance
///
/// - **UID:** 1.2.840.10008.5.1.4.34.5.1
/// - **UID Type:** Well-known SOP Instance
pub static UPSFilteredGlobalSubscriptionSOPInstance: UID = UID {
    ident: "UPSFilteredGlobalSubscriptionSOPInstance",
    uid: "1.2.840.10008.5.1.4.34.5.1",
    name: "UPS Filtered Global Subscription SOP Instance",
};

/// Unified Worklist and Procedure Step Service Class
///
/// - **UID:** 1.2.840.10008.5.1.4.34.6
/// - **UID Type:** Service Class
pub static UnifiedWorklistandProcedureStepServiceClass: UID = UID {
    ident: "UnifiedWorklistandProcedureStepServiceClass",
    uid: "1.2.840.10008.5.1.4.34.6",
    name: "Unified Worklist and Procedure Step Service Class",
};

/// Unified Procedure Step - Push SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.34.6.1
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepPushSOPClass: UID = UID {
    ident: "UnifiedProcedureStepPushSOPClass",
    uid: "1.2.840.10008.5.1.4.34.6.1",
    name: "Unified Procedure Step - Push SOP Class",
};

/// Unified Procedure Step - Watch SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.34.6.2
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepWatchSOPClass: UID = UID {
    ident: "UnifiedProcedureStepWatchSOPClass",
    uid: "1.2.840.10008.5.1.4.34.6.2",
    name: "Unified Procedure Step - Watch SOP Class",
};

/// Unified Procedure Step - Pull SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.34.6.3
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepPullSOPClass: UID = UID {
    ident: "UnifiedProcedureStepPullSOPClass",
    uid: "1.2.840.10008.5.1.4.34.6.3",
    name: "Unified Procedure Step - Pull SOP Class",
};

/// Unified Procedure Step - Event SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.34.6.4
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepEventSOPClass: UID = UID {
    ident: "UnifiedProcedureStepEventSOPClass",
    uid: "1.2.840.10008.5.1.4.34.6.4",
    name: "Unified Procedure Step - Event SOP Class",
};

/// Unified Procedure Step - Query SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.34.6.5
/// - **UID Type:** SOP Class
pub static UnifiedProcedureStepQuerySOPClass: UID = UID {
    ident: "UnifiedProcedureStepQuerySOPClass",
    uid: "1.2.840.10008.5.1.4.34.6.5",
    name: "Unified Procedure Step - Query SOP Class",
};

/// RT Beams Delivery Instruction Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.34.7
/// - **UID Type:** SOP Class
pub static RTBeamsDeliveryInstructionStorage: UID = UID {
    ident: "RTBeamsDeliveryInstructionStorage",
    uid: "1.2.840.10008.5.1.4.34.7",
    name: "RT Beams Delivery Instruction Storage",
};

/// RT Conventional Machine Verification
///
/// - **UID:** 1.2.840.10008.5.1.4.34.8
/// - **UID Type:** SOP Class
pub static RTConventionalMachineVerification: UID = UID {
    ident: "RTConventionalMachineVerification",
    uid: "1.2.840.10008.5.1.4.34.8",
    name: "RT Conventional Machine Verification",
};

/// RT Ion Machine Verification
///
/// - **UID:** 1.2.840.10008.5.1.4.34.9
/// - **UID Type:** SOP Class
pub static RTIonMachineVerification: UID = UID {
    ident: "RTIonMachineVerification",
    uid: "1.2.840.10008.5.1.4.34.9",
    name: "RT Ion Machine Verification",
};

/// RT Brachy Application Setup Delivery Instruction Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.34.10
/// - **UID Type:** SOP Class
pub static RTBrachyApplicationSetupDeliveryInstructionStorage: UID = UID {
    ident: "RTBrachyApplicationSetupDeliveryInstructionStorage",
    uid: "1.2.840.10008.5.1.4.34.10",
    name: "RT Brachy Application Setup Delivery Instruction Storage",
};

/// General Relevant Patient Information Query
///
/// - **UID:** 1.2.840.10008.5.1.4.37.1
/// - **UID Type:** SOP Class
pub static GeneralRelevantPatientInformationQuery: UID = UID {
    ident: "GeneralRelevantPatientInformationQuery",
    uid: "1.2.840.10008.5.1.4.37.1",
    name: "General Relevant Patient Information Query",
};

/// Breast Imaging Relevant Patient Information Query
///
/// - **UID:** 1.2.840.10008.5.1.4.37.2
/// - **UID Type:** SOP Class
pub static BreastImagingRelevantPatientInformationQuery: UID = UID {
    ident: "BreastImagingRelevantPatientInformationQuery",
    uid: "1.2.840.10008.5.1.4.37.2",
    name: "Breast Imaging Relevant Patient Information Query",
};

/// Cardiac Relevant Patient Information Query
///
/// - **UID:** 1.2.840.10008.5.1.4.37.3
/// - **UID Type:** SOP Class
pub static CardiacRelevantPatientInformationQuery: UID = UID {
    ident: "CardiacRelevantPatientInformationQuery",
    uid: "1.2.840.10008.5.1.4.37.3",
    name: "Cardiac Relevant Patient Information Query",
};

/// Hanging Protocol Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.38.1
/// - **UID Type:** SOP Class
pub static HangingProtocolStorage: UID = UID {
    ident: "HangingProtocolStorage",
    uid: "1.2.840.10008.5.1.4.38.1",
    name: "Hanging Protocol Storage",
};

/// Hanging Protocol Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.38.2
/// - **UID Type:** SOP Class
pub static HangingProtocolInformationModelFIND: UID = UID {
    ident: "HangingProtocolInformationModelFIND",
    uid: "1.2.840.10008.5.1.4.38.2",
    name: "Hanging Protocol Information Model - FIND",
};

/// Hanging Protocol Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.38.3
/// - **UID Type:** SOP Class
pub static HangingProtocolInformationModelMOVE: UID = UID {
    ident: "HangingProtocolInformationModelMOVE",
    uid: "1.2.840.10008.5.1.4.38.3",
    name: "Hanging Protocol Information Model - MOVE",
};

/// Hanging Protocol Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.38.4
/// - **UID Type:** SOP Class
pub static HangingProtocolInformationModelGET: UID = UID {
    ident: "HangingProtocolInformationModelGET",
    uid: "1.2.840.10008.5.1.4.38.4",
    name: "Hanging Protocol Information Model - GET",
};

/// Color Palette Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.39.1
/// - **UID Type:** SOP Class
pub static ColorPaletteStorage: UID = UID {
    ident: "ColorPaletteStorage",
    uid: "1.2.840.10008.5.1.4.39.1",
    name: "Color Palette Storage",
};

/// Color Palette Query/Retrieve Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.39.2
/// - **UID Type:** SOP Class
pub static ColorPaletteQueryRetrieveInformationModelFIND: UID = UID {
    ident: "ColorPaletteQueryRetrieveInformationModelFIND",
    uid: "1.2.840.10008.5.1.4.39.2",
    name: "Color Palette Query/Retrieve Information Model - FIND",
};

/// Color Palette Query/Retrieve Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.39.3
/// - **UID Type:** SOP Class
pub static ColorPaletteQueryRetrieveInformationModelMOVE: UID = UID {
    ident: "ColorPaletteQueryRetrieveInformationModelMOVE",
    uid: "1.2.840.10008.5.1.4.39.3",
    name: "Color Palette Query/Retrieve Information Model - MOVE",
};

/// Color Palette Query/Retrieve Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.39.4
/// - **UID Type:** SOP Class
pub static ColorPaletteQueryRetrieveInformationModelGET: UID = UID {
    ident: "ColorPaletteQueryRetrieveInformationModelGET",
    uid: "1.2.840.10008.5.1.4.39.4",
    name: "Color Palette Query/Retrieve Information Model - GET",
};

/// Product Characteristics Query SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.41
/// - **UID Type:** SOP Class
pub static ProductCharacteristicsQuerySOPClass: UID = UID {
    ident: "ProductCharacteristicsQuerySOPClass",
    uid: "1.2.840.10008.5.1.4.41",
    name: "Product Characteristics Query SOP Class",
};

/// Substance Approval Query SOP Class
///
/// - **UID:** 1.2.840.10008.5.1.4.42
/// - **UID Type:** SOP Class
pub static SubstanceApprovalQuerySOPClass: UID = UID {
    ident: "SubstanceApprovalQuerySOPClass",
    uid: "1.2.840.10008.5.1.4.42",
    name: "Substance Approval Query SOP Class",
};

/// Generic Implant Template Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.43.1
/// - **UID Type:** SOP Class
pub static GenericImplantTemplateStorage: UID = UID {
    ident: "GenericImplantTemplateStorage",
    uid: "1.2.840.10008.5.1.4.43.1",
    name: "Generic Implant Template Storage",
};

/// Generic Implant Template Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.43.2
/// - **UID Type:** SOP Class
pub static GenericImplantTemplateInformationModelFIND: UID = UID {
    ident: "GenericImplantTemplateInformationModelFIND",
    uid: "1.2.840.10008.5.1.4.43.2",
    name: "Generic Implant Template Information Model - FIND",
};

/// Generic Implant Template Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.43.3
/// - **UID Type:** SOP Class
pub static GenericImplantTemplateInformationModelMOVE: UID = UID {
    ident: "GenericImplantTemplateInformationModelMOVE",
    uid: "1.2.840.10008.5.1.4.43.3",
    name: "Generic Implant Template Information Model - MOVE",
};

/// Generic Implant Template Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.43.4
/// - **UID Type:** SOP Class
pub static GenericImplantTemplateInformationModelGET: UID = UID {
    ident: "GenericImplantTemplateInformationModelGET",
    uid: "1.2.840.10008.5.1.4.43.4",
    name: "Generic Implant Template Information Model - GET",
};

/// Implant Assembly Template Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.44.1
/// - **UID Type:** SOP Class
pub static ImplantAssemblyTemplateStorage: UID = UID {
    ident: "ImplantAssemblyTemplateStorage",
    uid: "1.2.840.10008.5.1.4.44.1",
    name: "Implant Assembly Template Storage",
};

/// Implant Assembly Template Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.44.2
/// - **UID Type:** SOP Class
pub static ImplantAssemblyTemplateInformationModelFIND: UID = UID {
    ident: "ImplantAssemblyTemplateInformationModelFIND",
    uid: "1.2.840.10008.5.1.4.44.2",
    name: "Implant Assembly Template Information Model - FIND",
};

/// Implant Assembly Template Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.44.3
/// - **UID Type:** SOP Class
pub static ImplantAssemblyTemplateInformationModelMOVE: UID = UID {
    ident: "ImplantAssemblyTemplateInformationModelMOVE",
    uid: "1.2.840.10008.5.1.4.44.3",
    name: "Implant Assembly Template Information Model - MOVE",
};

/// Implant Assembly Template Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.44.4
/// - **UID Type:** SOP Class
pub static ImplantAssemblyTemplateInformationModelGET: UID = UID {
    ident: "ImplantAssemblyTemplateInformationModelGET",
    uid: "1.2.840.10008.5.1.4.44.4",
    name: "Implant Assembly Template Information Model - GET",
};

/// Implant Template Group Storage
///
/// - **UID:** 1.2.840.10008.5.1.4.45.1
/// - **UID Type:** SOP Class
pub static ImplantTemplateGroupStorage: UID = UID {
    ident: "ImplantTemplateGroupStorage",
    uid: "1.2.840.10008.5.1.4.45.1",
    name: "Implant Template Group Storage",
};

/// Implant Template Group Information Model - FIND
///
/// - **UID:** 1.2.840.10008.5.1.4.45.2
/// - **UID Type:** SOP Class
pub static ImplantTemplateGroupInformationModelFIND: UID = UID {
    ident: "ImplantTemplateGroupInformationModelFIND",
    uid: "1.2.840.10008.5.1.4.45.2",
    name: "Implant Template Group Information Model - FIND",
};

/// Implant Template Group Information Model - MOVE
///
/// - **UID:** 1.2.840.10008.5.1.4.45.3
/// - **UID Type:** SOP Class
pub static ImplantTemplateGroupInformationModelMOVE: UID = UID {
    ident: "ImplantTemplateGroupInformationModelMOVE",
    uid: "1.2.840.10008.5.1.4.45.3",
    name: "Implant Template Group Information Model - MOVE",
};

/// Implant Template Group Information Model - GET
///
/// - **UID:** 1.2.840.10008.5.1.4.45.4
/// - **UID Type:** SOP Class
pub static ImplantTemplateGroupInformationModelGET: UID = UID {
    ident: "ImplantTemplateGroupInformationModelGET",
    uid: "1.2.840.10008.5.1.4.45.4",
    name: "Implant Template Group Information Model - GET",
};

/// Native DICOM Model
///
/// - **UID:** 1.2.840.10008.7.1.1
/// - **UID Type:** Application Hosting Model
pub static NativeDICOMModel: UID = UID {
    ident: "NativeDICOMModel",
    uid: "1.2.840.10008.7.1.1",
    name: "Native DICOM Model",
};

/// Abstract Multi-Dimensional Image Model
///
/// - **UID:** 1.2.840.10008.7.1.2
/// - **UID Type:** Application Hosting Model
pub static AbstractMultiDimensionalImageModel: UID = UID {
    ident: "AbstractMultiDimensionalImageModel",
    uid: "1.2.840.10008.7.1.2",
    name: "Abstract Multi-Dimensional Image Model",
};

/// DICOM Content Mapping Resource
///
/// - **UID:** 1.2.840.10008.8.1.1
/// - **UID Type:** Mapping Resource
pub static DICOMContentMappingResource: UID = UID {
    ident: "DICOMContentMappingResource",
    uid: "1.2.840.10008.8.1.1",
    name: "DICOM Content Mapping Resource",
};

/// Video Endoscopic Image Real-Time Communication
///
/// - **UID:** 1.2.840.10008.10.1
/// - **UID Type:** SOP Class
pub static VideoEndoscopicImageRealTimeCommunication: UID = UID {
    ident: "VideoEndoscopicImageRealTimeCommunication",
    uid: "1.2.840.10008.10.1",
    name: "Video Endoscopic Image Real-Time Communication",
};

/// Video Photographic Image Real-Time Communication
///
/// - **UID:** 1.2.840.10008.10.2
/// - **UID Type:** SOP Class
pub static VideoPhotographicImageRealTimeCommunication: UID = UID {
    ident: "VideoPhotographicImageRealTimeCommunication",
    uid: "1.2.840.10008.10.2",
    name: "Video Photographic Image Real-Time Communication",
};

/// Audio Waveform Real-Time Communication
///
/// - **UID:** 1.2.840.10008.10.3
/// - **UID Type:** SOP Class
pub static AudioWaveformRealTimeCommunication: UID = UID {
    ident: "AudioWaveformRealTimeCommunication",
    uid: "1.2.840.10008.10.3",
    name: "Audio Waveform Real-Time Communication",
};

/// Rendition Selection Document Real-Time Communication
///
/// - **UID:** 1.2.840.10008.10.4
/// - **UID Type:** SOP Class
pub static RenditionSelectionDocumentRealTimeCommunication: UID = UID {
    ident: "RenditionSelectionDocumentRealTimeCommunication",
    uid: "1.2.840.10008.10.4",
    name: "Rendition Selection Document Real-Time Communication",
};

/// dicomDeviceName
///
/// - **UID:** 1.2.840.10008.15.0.3.1
/// - **UID Type:** LDAP OID
pub static DicomDeviceName: UID = UID {
    ident: "DicomDeviceName",
    uid: "1.2.840.10008.15.0.3.1",
    name: "dicomDeviceName",
};

/// dicomDescription
///
/// - **UID:** 1.2.840.10008.15.0.3.2
/// - **UID Type:** LDAP OID
pub static DicomDescription: UID = UID {
    ident: "DicomDescription",
    uid: "1.2.840.10008.15.0.3.2",
    name: "dicomDescription",
};

/// dicomManufacturer
///
/// - **UID:** 1.2.840.10008.15.0.3.3
/// - **UID Type:** LDAP OID
pub static DicomManufacturer: UID = UID {
    ident: "DicomManufacturer",
    uid: "1.2.840.10008.15.0.3.3",
    name: "dicomManufacturer",
};

/// dicomManufacturerModelName
///
/// - **UID:** 1.2.840.10008.15.0.3.4
/// - **UID Type:** LDAP OID
pub static DicomManufacturerModelName: UID = UID {
    ident: "DicomManufacturerModelName",
    uid: "1.2.840.10008.15.0.3.4",
    name: "dicomManufacturerModelName",
};

/// dicomSoftwareVersion
///
/// - **UID:** 1.2.840.10008.15.0.3.5
/// - **UID Type:** LDAP OID
pub static DicomSoftwareVersion: UID = UID {
    ident: "DicomSoftwareVersion",
    uid: "1.2.840.10008.15.0.3.5",
    name: "dicomSoftwareVersion",
};

/// dicomVendorData
///
/// - **UID:** 1.2.840.10008.15.0.3.6
/// - **UID Type:** LDAP OID
pub static DicomVendorData: UID = UID {
    ident: "DicomVendorData",
    uid: "1.2.840.10008.15.0.3.6",
    name: "dicomVendorData",
};

/// dicomAETitle
///
/// - **UID:** 1.2.840.10008.15.0.3.7
/// - **UID Type:** LDAP OID
pub static DicomAETitle: UID = UID {
    ident: "DicomAETitle",
    uid: "1.2.840.10008.15.0.3.7",
    name: "dicomAETitle",
};

/// dicomNetworkConnectionReference
///
/// - **UID:** 1.2.840.10008.15.0.3.8
/// - **UID Type:** LDAP OID
pub static DicomNetworkConnectionReference: UID = UID {
    ident: "DicomNetworkConnectionReference",
    uid: "1.2.840.10008.15.0.3.8",
    name: "dicomNetworkConnectionReference",
};

/// dicomApplicationCluster
///
/// - **UID:** 1.2.840.10008.15.0.3.9
/// - **UID Type:** LDAP OID
pub static DicomApplicationCluster: UID = UID {
    ident: "DicomApplicationCluster",
    uid: "1.2.840.10008.15.0.3.9",
    name: "dicomApplicationCluster",
};

/// dicomAssociationInitiator
///
/// - **UID:** 1.2.840.10008.15.0.3.10
/// - **UID Type:** LDAP OID
pub static DicomAssociationInitiator: UID = UID {
    ident: "DicomAssociationInitiator",
    uid: "1.2.840.10008.15.0.3.10",
    name: "dicomAssociationInitiator",
};

/// dicomAssociationAcceptor
///
/// - **UID:** 1.2.840.10008.15.0.3.11
/// - **UID Type:** LDAP OID
pub static DicomAssociationAcceptor: UID = UID {
    ident: "DicomAssociationAcceptor",
    uid: "1.2.840.10008.15.0.3.11",
    name: "dicomAssociationAcceptor",
};

/// dicomHostname
///
/// - **UID:** 1.2.840.10008.15.0.3.12
/// - **UID Type:** LDAP OID
pub static DicomHostname: UID = UID {
    ident: "DicomHostname",
    uid: "1.2.840.10008.15.0.3.12",
    name: "dicomHostname",
};

/// dicomPort
///
/// - **UID:** 1.2.840.10008.15.0.3.13
/// - **UID Type:** LDAP OID
pub static DicomPort: UID = UID {
    ident: "DicomPort",
    uid: "1.2.840.10008.15.0.3.13",
    name: "dicomPort",
};

/// dicomSOPClass
///
/// - **UID:** 1.2.840.10008.15.0.3.14
/// - **UID Type:** LDAP OID
pub static DicomSOPClass: UID = UID {
    ident: "DicomSOPClass",
    uid: "1.2.840.10008.15.0.3.14",
    name: "dicomSOPClass",
};

/// dicomTransferRole
///
/// - **UID:** 1.2.840.10008.15.0.3.15
/// - **UID Type:** LDAP OID
pub static DicomTransferRole: UID = UID {
    ident: "DicomTransferRole",
    uid: "1.2.840.10008.15.0.3.15",
    name: "dicomTransferRole",
};

/// dicomTransferSyntax
///
/// - **UID:** 1.2.840.10008.15.0.3.16
/// - **UID Type:** LDAP OID
pub static DicomTransferSyntax: UID = UID {
    ident: "DicomTransferSyntax",
    uid: "1.2.840.10008.15.0.3.16",
    name: "dicomTransferSyntax",
};

/// dicomPrimaryDeviceType
///
/// - **UID:** 1.2.840.10008.15.0.3.17
/// - **UID Type:** LDAP OID
pub static DicomPrimaryDeviceType: UID = UID {
    ident: "DicomPrimaryDeviceType",
    uid: "1.2.840.10008.15.0.3.17",
    name: "dicomPrimaryDeviceType",
};

/// dicomRelatedDeviceReference
///
/// - **UID:** 1.2.840.10008.15.0.3.18
/// - **UID Type:** LDAP OID
pub static DicomRelatedDeviceReference: UID = UID {
    ident: "DicomRelatedDeviceReference",
    uid: "1.2.840.10008.15.0.3.18",
    name: "dicomRelatedDeviceReference",
};

/// dicomPreferredCalledAETitle
///
/// - **UID:** 1.2.840.10008.15.0.3.19
/// - **UID Type:** LDAP OID
pub static DicomPreferredCalledAETitle: UID = UID {
    ident: "DicomPreferredCalledAETitle",
    uid: "1.2.840.10008.15.0.3.19",
    name: "dicomPreferredCalledAETitle",
};

/// dicomTLSCyphersuite
///
/// - **UID:** 1.2.840.10008.15.0.3.20
/// - **UID Type:** LDAP OID
pub static DicomTLSCyphersuite: UID = UID {
    ident: "DicomTLSCyphersuite",
    uid: "1.2.840.10008.15.0.3.20",
    name: "dicomTLSCyphersuite",
};

/// dicomAuthorizedNodeCertificateReference
///
/// - **UID:** 1.2.840.10008.15.0.3.21
/// - **UID Type:** LDAP OID
pub static DicomAuthorizedNodeCertificateReference: UID = UID {
    ident: "DicomAuthorizedNodeCertificateReference",
    uid: "1.2.840.10008.15.0.3.21",
    name: "dicomAuthorizedNodeCertificateReference",
};

/// dicomThisNodeCertificateReference
///
/// - **UID:** 1.2.840.10008.15.0.3.22
/// - **UID Type:** LDAP OID
pub static DicomThisNodeCertificateReference: UID = UID {
    ident: "DicomThisNodeCertificateReference",
    uid: "1.2.840.10008.15.0.3.22",
    name: "dicomThisNodeCertificateReference",
};

/// dicomInstalled
///
/// - **UID:** 1.2.840.10008.15.0.3.23
/// - **UID Type:** LDAP OID
pub static DicomInstalled: UID = UID {
    ident: "DicomInstalled",
    uid: "1.2.840.10008.15.0.3.23",
    name: "dicomInstalled",
};

/// dicomStationName
///
/// - **UID:** 1.2.840.10008.15.0.3.24
/// - **UID Type:** LDAP OID
pub static DicomStationName: UID = UID {
    ident: "DicomStationName",
    uid: "1.2.840.10008.15.0.3.24",
    name: "dicomStationName",
};

/// dicomDeviceSerialNumber
///
/// - **UID:** 1.2.840.10008.15.0.3.25
/// - **UID Type:** LDAP OID
pub static DicomDeviceSerialNumber: UID = UID {
    ident: "DicomDeviceSerialNumber",
    uid: "1.2.840.10008.15.0.3.25",
    name: "dicomDeviceSerialNumber",
};

/// dicomInstitutionName
///
/// - **UID:** 1.2.840.10008.15.0.3.26
/// - **UID Type:** LDAP OID
pub static DicomInstitutionName: UID = UID {
    ident: "DicomInstitutionName",
    uid: "1.2.840.10008.15.0.3.26",
    name: "dicomInstitutionName",
};

/// dicomInstitutionAddress
///
/// - **UID:** 1.2.840.10008.15.0.3.27
/// - **UID Type:** LDAP OID
pub static DicomInstitutionAddress: UID = UID {
    ident: "DicomInstitutionAddress",
    uid: "1.2.840.10008.15.0.3.27",
    name: "dicomInstitutionAddress",
};

/// dicomInstitutionDepartmentName
///
/// - **UID:** 1.2.840.10008.15.0.3.28
/// - **UID Type:** LDAP OID
pub static DicomInstitutionDepartmentName: UID = UID {
    ident: "DicomInstitutionDepartmentName",
    uid: "1.2.840.10008.15.0.3.28",
    name: "dicomInstitutionDepartmentName",
};

/// dicomIssuerOfPatientID
///
/// - **UID:** 1.2.840.10008.15.0.3.29
/// - **UID Type:** LDAP OID
pub static DicomIssuerOfPatientID: UID = UID {
    ident: "DicomIssuerOfPatientID",
    uid: "1.2.840.10008.15.0.3.29",
    name: "dicomIssuerOfPatientID",
};

/// dicomPreferredCallingAETitle
///
/// - **UID:** 1.2.840.10008.15.0.3.30
/// - **UID Type:** LDAP OID
pub static DicomPreferredCallingAETitle: UID = UID {
    ident: "DicomPreferredCallingAETitle",
    uid: "1.2.840.10008.15.0.3.30",
    name: "dicomPreferredCallingAETitle",
};

/// dicomSupportedCharacterSet
///
/// - **UID:** 1.2.840.10008.15.0.3.31
/// - **UID Type:** LDAP OID
pub static DicomSupportedCharacterSet: UID = UID {
    ident: "DicomSupportedCharacterSet",
    uid: "1.2.840.10008.15.0.3.31",
    name: "dicomSupportedCharacterSet",
};

/// dicomConfigurationRoot
///
/// - **UID:** 1.2.840.10008.15.0.4.1
/// - **UID Type:** LDAP OID
pub static DicomConfigurationRoot: UID = UID {
    ident: "DicomConfigurationRoot",
    uid: "1.2.840.10008.15.0.4.1",
    name: "dicomConfigurationRoot",
};

/// dicomDevicesRoot
///
/// - **UID:** 1.2.840.10008.15.0.4.2
/// - **UID Type:** LDAP OID
pub static DicomDevicesRoot: UID = UID {
    ident: "DicomDevicesRoot",
    uid: "1.2.840.10008.15.0.4.2",
    name: "dicomDevicesRoot",
};

/// dicomUniqueAETitlesRegistryRoot
///
/// - **UID:** 1.2.840.10008.15.0.4.3
/// - **UID Type:** LDAP OID
pub static DicomUniqueAETitlesRegistryRoot: UID = UID {
    ident: "DicomUniqueAETitlesRegistryRoot",
    uid: "1.2.840.10008.15.0.4.3",
    name: "dicomUniqueAETitlesRegistryRoot",
};

/// dicomDevice
///
/// - **UID:** 1.2.840.10008.15.0.4.4
/// - **UID Type:** LDAP OID
pub static DicomDevice: UID = UID {
    ident: "DicomDevice",
    uid: "1.2.840.10008.15.0.4.4",
    name: "dicomDevice",
};

/// dicomNetworkAE
///
/// - **UID:** 1.2.840.10008.15.0.4.5
/// - **UID Type:** LDAP OID
pub static DicomNetworkAE: UID = UID {
    ident: "DicomNetworkAE",
    uid: "1.2.840.10008.15.0.4.5",
    name: "dicomNetworkAE",
};

/// dicomNetworkConnection
///
/// - **UID:** 1.2.840.10008.15.0.4.6
/// - **UID Type:** LDAP OID
pub static DicomNetworkConnection: UID = UID {
    ident: "DicomNetworkConnection",
    uid: "1.2.840.10008.15.0.4.6",
    name: "dicomNetworkConnection",
};

/// dicomUniqueAETitle
///
/// - **UID:** 1.2.840.10008.15.0.4.7
/// - **UID Type:** LDAP OID
pub static DicomUniqueAETitle: UID = UID {
    ident: "DicomUniqueAETitle",
    uid: "1.2.840.10008.15.0.4.7",
    name: "dicomUniqueAETitle",
};

/// dicomTransferCapability
///
/// - **UID:** 1.2.840.10008.15.0.4.8
/// - **UID Type:** LDAP OID
pub static DicomTransferCapability: UID = UID {
    ident: "DicomTransferCapability",
    uid: "1.2.840.10008.15.0.4.8",
    name: "dicomTransferCapability",
};

/// Universal Coordinated Time
///
/// - **UID:** 1.2.840.10008.15.1.1
/// - **UID Type:** Synchronization Frame of Reference
pub static UniversalCoordinatedTime: UID = UID {
    ident: "UniversalCoordinatedTime",
    uid: "1.2.840.10008.15.1.1",
    name: "Universal Coordinated Time",
};
