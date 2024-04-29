//! This is an auto-generated file. Do not make modifications here.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::tag::Tag;
use core::vm::VM;
use core::vr;

/// File Meta Information Group Length
/// 
/// - **Tag:** (0002,0000)
/// - **VR:** UL
/// - **VM:** 1
pub static FileMetaInformationGroupLength: Tag = Tag {
    ident: "FileMetaInformationGroupLength",
    tag: 0x00020000,
    implicit_vr: Some(&vr::UL),
    vm: &VM::Distinct(1),
    desc: "File Meta Information Group Length",
};

/// File Meta Information Version
/// 
/// - **Tag:** (0002,0001)
/// - **VR:** OB
/// - **VM:** 1
pub static FileMetaInformationVersion: Tag = Tag {
    ident: "FileMetaInformationVersion",
    tag: 0x00020001,
    implicit_vr: Some(&vr::OB),
    vm: &VM::Distinct(1),
    desc: "File Meta Information Version",
};

/// Media Storage SOP Class UID
/// 
/// - **Tag:** (0002,0002)
/// - **VR:** UI
/// - **VM:** 1
pub static MediaStorageSOPClassUID: Tag = Tag {
    ident: "MediaStorageSOPClassUID",
    tag: 0x00020002,
    implicit_vr: Some(&vr::UI),
    vm: &VM::Distinct(1),
    desc: "Media Storage SOP Class UID",
};

/// Media Storage SOP Instance UID
/// 
/// - **Tag:** (0002,0003)
/// - **VR:** UI
/// - **VM:** 1
pub static MediaStorageSOPInstanceUID: Tag = Tag {
    ident: "MediaStorageSOPInstanceUID",
    tag: 0x00020003,
    implicit_vr: Some(&vr::UI),
    vm: &VM::Distinct(1),
    desc: "Media Storage SOP Instance UID",
};

/// Transfer Syntax UID
/// 
/// - **Tag:** (0002,0010)
/// - **VR:** UI
/// - **VM:** 1
pub static TransferSyntaxUID: Tag = Tag {
    ident: "TransferSyntaxUID",
    tag: 0x00020010,
    implicit_vr: Some(&vr::UI),
    vm: &VM::Distinct(1),
    desc: "Transfer Syntax UID",
};

/// Implementation Class UID
/// 
/// - **Tag:** (0002,0012)
/// - **VR:** UI
/// - **VM:** 1
pub static ImplementationClassUID: Tag = Tag {
    ident: "ImplementationClassUID",
    tag: 0x00020012,
    implicit_vr: Some(&vr::UI),
    vm: &VM::Distinct(1),
    desc: "Implementation Class UID",
};

/// Implementation Version Name
/// 
/// - **Tag:** (0002,0013)
/// - **VR:** SH
/// - **VM:** 1
pub static ImplementationVersionName: Tag = Tag {
    ident: "ImplementationVersionName",
    tag: 0x00020013,
    implicit_vr: Some(&vr::SH),
    vm: &VM::Distinct(1),
    desc: "Implementation Version Name",
};

/// Source Application Entity Title
/// 
/// - **Tag:** (0002,0016)
/// - **VR:** AE
/// - **VM:** 1
pub static SourceApplicationEntityTitle: Tag = Tag {
    ident: "SourceApplicationEntityTitle",
    tag: 0x00020016,
    implicit_vr: Some(&vr::AE),
    vm: &VM::Distinct(1),
    desc: "Source Application Entity Title",
};

/// Sending Application Entity Title
/// 
/// - **Tag:** (0002,0017)
/// - **VR:** AE
/// - **VM:** 1
pub static SendingApplicationEntityTitle: Tag = Tag {
    ident: "SendingApplicationEntityTitle",
    tag: 0x00020017,
    implicit_vr: Some(&vr::AE),
    vm: &VM::Distinct(1),
    desc: "Sending Application Entity Title",
};

/// Receiving Application Entity Title
/// 
/// - **Tag:** (0002,0018)
/// - **VR:** AE
/// - **VM:** 1
pub static ReceivingApplicationEntityTitle: Tag = Tag {
    ident: "ReceivingApplicationEntityTitle",
    tag: 0x00020018,
    implicit_vr: Some(&vr::AE),
    vm: &VM::Distinct(1),
    desc: "Receiving Application Entity Title",
};

/// Private Information Creator UID
/// 
/// - **Tag:** (0002,0100)
/// - **VR:** UI
/// - **VM:** 1
pub static PrivateInformationCreatorUID: Tag = Tag {
    ident: "PrivateInformationCreatorUID",
    tag: 0x00020100,
    implicit_vr: Some(&vr::UI),
    vm: &VM::Distinct(1),
    desc: "Private Information Creator UID",
};

/// Private Information
/// 
/// - **Tag:** (0002,0102)
/// - **VR:** OB
/// - **VM:** 1
pub static PrivateInformation: Tag = Tag {
    ident: "PrivateInformation",
    tag: 0x00020102,
    implicit_vr: Some(&vr::OB),
    vm: &VM::Distinct(1),
    desc: "Private Information",
};

