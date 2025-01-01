/*
   Copyright 2024-2025 Christopher Speck

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

#[cfg(feature = "stddicom")]
mod dict_tests {
    use medicom::{
        self,
        core::defn::{tag::Tag, ts::TransferSyntax, uid::UIDRef},
        dict::{
            lookup::{
                TAG_BY_IDENT, TAG_BY_VALUE, TS_BY_IDENT, TS_BY_UID, UID_BY_IDENT, UID_BY_UID,
            },
            tags::{FilesetID, PixelData, TransferSyntaxUID},
            transfer_syntaxes::{ExplicitVRLittleEndian, ImplicitVRLittleEndian},
            uids::{CTImageStorage, MRImageStorage},
        },
    };

    #[test]
    pub fn test_equality() {
        assert_eq!(MRImageStorage.ident(), "MRImageStorage");
        assert_eq!(MRImageStorage.uid(), "1.2.840.10008.5.1.4.1.1.4");
        assert_eq!(MRImageStorage.name(), "MR Image Storage");
    }

    /// These tests are using the lookup maps directly rather than using a `DicomDictionary`, and must
    /// account for the maps using lowercased keys.
    fn lookup_key(key: &str) -> String {
        key.to_lowercase()
    }

    #[test]
    pub fn test_tags_lookup() {
        // Lookup a DICOM Element by identity/value from the TAG maps
        let pd_by_ident: &Tag = TAG_BY_IDENT
            .get(&lookup_key("PixelData"))
            .expect("Tag PixelData not found");
        assert_eq!(&PixelData, pd_by_ident);

        let pd_by_tag: &Tag = TAG_BY_VALUE
            .get(&0x7FE0_0010)
            .expect("Tag 0x7FE00010 not found");
        assert_eq!(&PixelData, pd_by_tag);

        // Lookup a Directory Structure Element by identity/value from the TAG maps
        let fsid_by_ident: &Tag = TAG_BY_IDENT
            .get(&lookup_key("FilesetID"))
            .expect("Tag FilesetID not found");
        assert_eq!(&FilesetID, fsid_by_ident);

        let fsid_by_tag: &Tag = TAG_BY_VALUE
            .get(&0x0004_1130)
            .expect("Tag 0x00041130 not found");
        assert_eq!(&FilesetID, fsid_by_tag);

        // Lookup a File Meta Element by identity/value from the TAG maps
        let tsuid_by_ident: &Tag = TAG_BY_IDENT
            .get(&lookup_key("TransferSyntaxUID"))
            .expect("Tag TransferSyntaxUID not found");
        assert_eq!(&TransferSyntaxUID, tsuid_by_ident);

        let tsuid_by_tag: &Tag = TAG_BY_VALUE
            .get(&0x0002_0010)
            .expect("Tag 0x00020010 not found");
        assert_eq!(&TransferSyntaxUID, tsuid_by_tag);
    }

    #[test]
    pub fn test_transfer_syntaxes_lookup() {
        let ivrle_by_ident: &TransferSyntax = TS_BY_IDENT
            .get(&lookup_key("ImplicitVRLittleEndian"))
            .expect("TransferSyntax ImplicitVRLittleEndian not found");
        assert_eq!(&ImplicitVRLittleEndian, ivrle_by_ident);

        let ivrle_by_id: &TransferSyntax = TS_BY_UID
            .get("1.2.840.10008.1.2")
            .expect("TransferSyntax 1.2.840.10008.1.2 not found");
        assert_eq!(&ImplicitVRLittleEndian, ivrle_by_id);

        let ivrle_by_uid: &TransferSyntax = TS_BY_UID
            .get(ImplicitVRLittleEndian.uid().uid())
            .unwrap_or_else(|| {
                panic!(
                    "TransferSyntax {}, not found",
                    ImplicitVRLittleEndian.uid().uid()
                )
            });
        assert_eq!(&ImplicitVRLittleEndian, ivrle_by_uid);

        let evrle_by_ident: &TransferSyntax = TS_BY_IDENT
            .get(&lookup_key("ExplicitVRLittleEndian"))
            .expect("TransferSyntax ExplicitVRLittleEndian not found");
        assert_eq!(&ExplicitVRLittleEndian, evrle_by_ident);

        let evrle_by_id: &TransferSyntax = TS_BY_UID
            .get("1.2.840.10008.1.2.1")
            .expect("TransferSyntax 1.2.840.10008.1.2.1 not found");
        assert_eq!(&ExplicitVRLittleEndian, evrle_by_id);

        let evrle_by_uid: &TransferSyntax = TS_BY_UID
            .get(ExplicitVRLittleEndian.uid().uid())
            .unwrap_or_else(|| {
                panic!(
                    "TransferSyntax {}, not found",
                    ExplicitVRLittleEndian.uid().uid()
                )
            });
        assert_eq!(&ExplicitVRLittleEndian, evrle_by_uid);
    }

    #[test]
    pub fn test_uids_lookup() {
        let ctis_by_ident: UIDRef = UID_BY_IDENT
            .get(&lookup_key("CTImageStorage"))
            .expect("UID CTImageStorage not found");
        assert_eq!(&CTImageStorage, ctis_by_ident);

        let ctis_by_id: UIDRef = UID_BY_UID
            .get("1.2.840.10008.5.1.4.1.1.2")
            .expect("UID 1.2.840.10008.5.1.4.1.1.2 not found");
        assert_eq!(&CTImageStorage, ctis_by_id);
    }

    /// Sanity-check of the pre-defined `TransferSyntax`'s to ensure
    /// that their defined properties reflect the UID's name.
    /// May catch issues with improperly copying over values from definitions.
    #[test]
    fn test_ts_name_vs_properties() {
        for (_, ts) in TS_BY_IDENT.entries() {
            let contains_little: bool = ts.uid().ident().contains("LittleEndian");
            let contains_big: bool = ts.uid().ident().contains("BigEndian");
            let contains_explicit: bool = ts.uid().ident().contains("ExplicitVR");
            let contains_implicit: bool = ts.uid().ident().contains("ImplicitVR");
            let contains_deflate: bool = ts.uid().ident().contains("Deflate");
            let contains_encapsulated: bool = ts.uid().ident().contains("Encapsulated");

            if contains_little {
                assert!(
                    !ts.big_endian(),
                    "Name contains \"LittleEndian\" but is big_endian: {:?}",
                    ts.uid()
                );
            } else if contains_big {
                assert!(
                    ts.big_endian(),
                    "Name contains \"BigEndian\" but is not big_endian: {:?}",
                    ts.uid()
                );
            } else {
                // Defined/known TS's without "big/little" in the name are assumed little endian
                assert!(
                    !ts.big_endian(),
                    "Name contains no endian but is not big_endian: {:?}",
                    ts.uid()
                );
            }

            if contains_explicit {
                assert!(
                    ts.explicit_vr(),
                    "Name contains \"ExplicitVR\" but is not explicit_vr: {:?}",
                    ts.uid()
                );
            } else if contains_implicit {
                assert!(
                    !ts.explicit_vr(),
                    "Name contains \"ImplicitVR\" but is explicit_vr: {:?}",
                    ts.uid()
                );
            } else {
                // Transfer syntaxes without "implicit/explicit" in name assumed to be explicit vr
                assert!(
                    ts.explicit_vr(),
                    "Name contains no vr but is not explicit_vr: {:?}",
                    ts.uid()
                );
            }

            if contains_deflate {
                assert!(
                    ts.deflated(),
                    "Name contains \"Deflate\" but is not deflated: {:?}",
                    ts.uid()
                );
            }

            if contains_encapsulated {
                assert!(
                    ts.encapsulated(),
                    "Name contains \"Encapsulated\" but is not encapsulated: {:?}",
                    ts.uid()
                );
            }
        }
    }
}
