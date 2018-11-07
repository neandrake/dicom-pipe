#![cfg(test)]

use core::dict::dicom_elements as tags;
use core::dict::dir_structure_elements as dse;
use core::dict::file_meta_elements as fme;
use core::dict::lookup::{TAG_BY_IDENT, TAG_BY_VALUE, TS_BY_ID, TS_BY_IDENT, UID_BY_ID, UID_BY_IDENT};
use core::dict::transfer_syntaxes as ts;
use core::dict::uids;
use core::tag::Tag;
use core::ts::TransferSyntax;
use core::uid::UIDRef;

#[test]
pub fn test_tags_lookup() {
	let pd_by_ident: &Tag = TAG_BY_IDENT.get("PixelData").expect("Tag not found");
	assert_eq!(pd_by_ident, &tags::PixelData);

	let pd_by_tag: &Tag = TAG_BY_VALUE.get(&0x7FE00010).expect("Tag not found");
	assert_eq!(pd_by_tag, &tags::PixelData);


	let fsid_by_ident: &Tag = TAG_BY_IDENT.get("FilesetID").expect("Tag not found");
	assert_eq!(fsid_by_ident, &dse::FilesetID);

	let fsid_by_tag: &Tag = TAG_BY_VALUE.get(&0x00041130).expect("Tag not found");
	assert_eq!(fsid_by_tag, &dse::FilesetID);


	let tsuid_by_ident: &Tag = TAG_BY_IDENT.get("TransferSyntaxUID").expect("Tag not found");
	assert_eq!(tsuid_by_ident, &fme::TransferSyntaxUID);

	let tsuid_by_tag: &Tag = TAG_BY_VALUE.get(&0x00020010).expect("Tag not found");
	assert_eq!(tsuid_by_tag, &fme::TransferSyntaxUID);
}

#[test]
pub fn test_transfer_syntaxes_lookup() {
	let ivrle_by_ident: &TransferSyntax = TS_BY_IDENT.get("ImplicitVRLittleEndian").expect("TransferSyntax not found");
	assert_eq!(ivrle_by_ident, &ts::ImplicitVRLittleEndian);

	let ivrle_by_id: &TransferSyntax = TS_BY_ID.get("1.2.840.10008.1.2").expect("TransferSyntax not found");
	assert_eq!(ivrle_by_id, &ts::ImplicitVRLittleEndian);

	let ivrle_by_uid: &TransferSyntax = TS_BY_ID.get(&uids::ImplicitVRLittleEndian.uid).expect("TransferSyntax not found");
	assert_eq!(ivrle_by_uid, &ts::ImplicitVRLittleEndian);


	let evrle_by_ident: &TransferSyntax = TS_BY_IDENT.get("ExplicitVRLittleEndian").expect("TransferSyntax not found");
	assert_eq!(evrle_by_ident, &ts::ExplicitVRLittleEndian);

	let evrle_by_id: &TransferSyntax = TS_BY_ID.get("1.2.840.10008.1.2.1").expect("TransferSyntax not found");
	assert_eq!(evrle_by_id, &ts::ExplicitVRLittleEndian);

	let evrle_by_uid: &TransferSyntax = TS_BY_ID.get(&uids::ExplicitVRLittleEndian.uid).expect("TransferSyntax not found");
	assert_eq!(evrle_by_uid, &ts::ExplicitVRLittleEndian);
}

#[test]
pub fn test_uids_lookup() {
	let ctis_by_ident: UIDRef = UID_BY_IDENT.get("CTImageStorage").expect("UID not found");
	assert_eq!(ctis_by_ident, &uids::CTImageStorage);

	let ctis_by_id: UIDRef = UID_BY_ID.get("1.2.840.10008.5.1.4.1.1.2").expect("UID not found");
	assert_eq!(ctis_by_id, &uids::CTImageStorage);
}

/// Sanity-check of the pre-defined TransferSyntax's to ensure
/// that their defined properties reflect the UID's name.
/// May catch issues with improperly copying over values from definitions.
#[test]
fn test_ts_name_vs_properties() {
	for (_, ts) in TS_BY_IDENT.entries() {
		let contains_little: bool = ts.uid.get_ident().contains("LittleEndian");
		let contains_big: bool = ts.uid.get_ident().contains("BigEndian");
		let contains_explicit: bool = ts.uid.get_ident().contains("ExplicitVR");
		let contains_implicit: bool = ts.uid.get_ident().contains("ImplicitVR");
		let contains_deflate: bool = ts.uid.get_ident().contains("Deflate");
		let contains_encapsulated: bool = ts.uid.get_ident().contains("Encapsulated");

		if contains_little {
			assert!(!ts.big_endian, "Name contains \"LittleEndian\" but is big_endian: {:?}", ts.uid);
		} else if contains_big {
			assert!(ts.big_endian, "Name contains \"BigEndian\" but is not big_endian: {:?}", ts.uid);
		} else {
			// Currently the defined/known TS's which don't have Big/Little in the name are LittleEndian
			assert!(!ts.big_endian, "Name contains no endian but is not big_endian: {:?}", ts.uid);
		}

		if contains_explicit {
			assert!(ts.explicit_vr, "Name contains \"ExplicitVR\" but is not explicit_vr: {:?}", ts.uid);
		} else if contains_implicit {
			assert!(!ts.explicit_vr, "Name contains \"ImplicitVR\" but is explicit_vr: {:?}", ts.uid);
		} else {
			// Currently the defined/known TS's which don't have Implicit/Explicit in the name are Implicit
			assert!(!ts.explicit_vr, "Name contains no vr but is not explicit_vr: {:?}", ts.uid);
		}

		assert_eq!(contains_deflate, ts.deflated, "Name contains \"Deflate\" but is not deflated: {:?}", ts.uid);
		assert_eq!(contains_encapsulated, ts.encapsulated, "Name contains \"Encapsulated\" but is not encapsulated: {:?}", ts.uid);
	}
}