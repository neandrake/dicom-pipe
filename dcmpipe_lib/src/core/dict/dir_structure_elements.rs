//! This is an auto-generated file. Do not make modifications here.

#![allow(non_upper_case_globals)]

use crate::core::tag::Tag;
use crate::core::vm::VM;
use crate::core::vr;

/// File-set ID
///
/// - **Tag:** (0004,1130)
/// - **VR:** CS
/// - **VM:** 1
pub static FilesetID: Tag = Tag {
    ident: "FilesetID",
    tag: 0x0004_1130,
    implicit_vr: Some(&vr::CS),
    vm: &VM::Distinct(1),
    desc: "File-set ID",
};

/// File-set Descriptor File ID
///
/// - **Tag:** (0004,1141)
/// - **VR:** CS
/// - **VM:** 1-8
pub static FilesetDescriptorFileID: Tag = Tag {
    ident: "FilesetDescriptorFileID",
    tag: 0x0004_1141,
    implicit_vr: Some(&vr::CS),
    vm: &VM::AtMost(8),
    desc: "File-set Descriptor File ID",
};

/// Specific Character Set of File-set Descriptor File
///
/// - **Tag:** (0004,1142)
/// - **VR:** CS
/// - **VM:** 1
pub static SpecificCharacterSetofFilesetDescriptorFile: Tag = Tag {
    ident: "SpecificCharacterSetofFilesetDescriptorFile",
    tag: 0x0004_1142,
    implicit_vr: Some(&vr::CS),
    vm: &VM::Distinct(1),
    desc: "Specific Character Set of File-set Descriptor File",
};

/// Offset of the First Directory Record of the Root Directory Entity
///
/// - **Tag:** (0004,1200)
/// - **VR:** UL
/// - **VM:** 1
pub static OffsetoftheFirstDirectoryRecordoftheRootDirectoryEntity: Tag = Tag {
    ident: "OffsetoftheFirstDirectoryRecordoftheRootDirectoryEntity",
    tag: 0x0004_1200,
    implicit_vr: Some(&vr::UL),
    vm: &VM::Distinct(1),
    desc: "Offset of the First Directory Record of the Root Directory Entity",
};

/// Offset of the Last Directory Record of the Root Directory Entity
///
/// - **Tag:** (0004,1202)
/// - **VR:** UL
/// - **VM:** 1
pub static OffsetoftheLastDirectoryRecordoftheRootDirectoryEntity: Tag = Tag {
    ident: "OffsetoftheLastDirectoryRecordoftheRootDirectoryEntity",
    tag: 0x0004_1202,
    implicit_vr: Some(&vr::UL),
    vm: &VM::Distinct(1),
    desc: "Offset of the Last Directory Record of the Root Directory Entity",
};

/// File-set Consistency Flag
///
/// - **Tag:** (0004,1212)
/// - **VR:** US
/// - **VM:** 1
pub static FilesetConsistencyFlag: Tag = Tag {
    ident: "FilesetConsistencyFlag",
    tag: 0x0004_1212,
    implicit_vr: Some(&vr::US),
    vm: &VM::Distinct(1),
    desc: "File-set Consistency Flag",
};

/// Directory Record Sequence
///
/// - **Tag:** (0004,1220)
/// - **VR:** SQ
/// - **VM:** 1
pub static DirectoryRecordSequence: Tag = Tag {
    ident: "DirectoryRecordSequence",
    tag: 0x0004_1220,
    implicit_vr: Some(&vr::SQ),
    vm: &VM::Distinct(1),
    desc: "Directory Record Sequence",
};

/// Offset of the Next Directory Record
///
/// - **Tag:** (0004,1400)
/// - **VR:** UL
/// - **VM:** 1
pub static OffsetoftheNextDirectoryRecord: Tag = Tag {
    ident: "OffsetoftheNextDirectoryRecord",
    tag: 0x0004_1400,
    implicit_vr: Some(&vr::UL),
    vm: &VM::Distinct(1),
    desc: "Offset of the Next Directory Record",
};

/// Record In-use Flag
///
/// - **Tag:** (0004,1410)
/// - **VR:** US
/// - **VM:** 1
pub static RecordInuseFlag: Tag = Tag {
    ident: "RecordInuseFlag",
    tag: 0x0004_1410,
    implicit_vr: Some(&vr::US),
    vm: &VM::Distinct(1),
    desc: "Record In-use Flag",
};

/// Offset of Referenced Lower-Level Directory Entity
///
/// - **Tag:** (0004,1420)
/// - **VR:** UL
/// - **VM:** 1
pub static OffsetofReferencedLowerLevelDirectoryEntity: Tag = Tag {
    ident: "OffsetofReferencedLowerLevelDirectoryEntity",
    tag: 0x0004_1420,
    implicit_vr: Some(&vr::UL),
    vm: &VM::Distinct(1),
    desc: "Offset of Referenced Lower-Level Directory Entity",
};

/// Directory Record Type
///
/// - **Tag:** (0004,1430)
/// - **VR:** CS
/// - **VM:** 1
pub static DirectoryRecordType: Tag = Tag {
    ident: "DirectoryRecordType",
    tag: 0x0004_1430,
    implicit_vr: Some(&vr::CS),
    vm: &VM::Distinct(1),
    desc: "Directory Record Type",
};

/// Private Record UID
///
/// - **Tag:** (0004,1432)
/// - **VR:** UI
/// - **VM:** 1
pub static PrivateRecordUID: Tag = Tag {
    ident: "PrivateRecordUID",
    tag: 0x0004_1432,
    implicit_vr: Some(&vr::UI),
    vm: &VM::Distinct(1),
    desc: "Private Record UID",
};

/// Referenced File ID
///
/// - **Tag:** (0004,1500)
/// - **VR:** CS
/// - **VM:** 1-8
pub static ReferencedFileID: Tag = Tag {
    ident: "ReferencedFileID",
    tag: 0x0004_1500,
    implicit_vr: Some(&vr::CS),
    vm: &VM::AtMost(8),
    desc: "Referenced File ID",
};

/// MRDR Directory Record Offset
///
/// - **Tag:** (0004,1504)
/// - **VR:** UL
/// - **VM:** 1
pub static MRDRDirectoryRecordOffset: Tag = Tag {
    ident: "MRDRDirectoryRecordOffset",
    tag: 0x0004_1504,
    implicit_vr: Some(&vr::UL),
    vm: &VM::Distinct(1),
    desc: "MRDR Directory Record Offset",
};

/// Referenced SOP Class UID in File
///
/// - **Tag:** (0004,1510)
/// - **VR:** UI
/// - **VM:** 1
pub static ReferencedSOPClassUIDinFile: Tag = Tag {
    ident: "ReferencedSOPClassUIDinFile",
    tag: 0x0004_1510,
    implicit_vr: Some(&vr::UI),
    vm: &VM::Distinct(1),
    desc: "Referenced SOP Class UID in File",
};

/// Referenced SOP Instance UID in File
///
/// - **Tag:** (0004,1511)
/// - **VR:** UI
/// - **VM:** 1
pub static ReferencedSOPInstanceUIDinFile: Tag = Tag {
    ident: "ReferencedSOPInstanceUIDinFile",
    tag: 0x0004_1511,
    implicit_vr: Some(&vr::UI),
    vm: &VM::Distinct(1),
    desc: "Referenced SOP Instance UID in File",
};

/// Referenced Transfer Syntax UID in File
///
/// - **Tag:** (0004,1512)
/// - **VR:** UI
/// - **VM:** 1
pub static ReferencedTransferSyntaxUIDinFile: Tag = Tag {
    ident: "ReferencedTransferSyntaxUIDinFile",
    tag: 0x0004_1512,
    implicit_vr: Some(&vr::UI),
    vm: &VM::Distinct(1),
    desc: "Referenced Transfer Syntax UID in File",
};

/// Referenced Related General SOP Class UID in File
///
/// - **Tag:** (0004,151A)
/// - **VR:** UI
/// - **VM:** 1-n
pub static ReferencedRelatedGeneralSOPClassUIDinFile: Tag = Tag {
    ident: "ReferencedRelatedGeneralSOPClassUIDinFile",
    tag: 0x0004_151A,
    implicit_vr: Some(&vr::UI),
    vm: &VM::AtLeast(1),
    desc: "Referenced Related General SOP Class UID in File",
};

/// Number of References
///
/// - **Tag:** (0004,1600)
/// - **VR:** UL
/// - **VM:** 1
pub static NumberofReferences: Tag = Tag {
    ident: "NumberofReferences",
    tag: 0x0004_1600,
    implicit_vr: Some(&vr::UL),
    vm: &VM::Distinct(1),
    desc: "Number of References",
};

