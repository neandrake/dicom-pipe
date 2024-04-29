use core::dict::tag_lookup;
use core::dict::ts_lookup;
use core::dict::uid_lookup;

use core::tag::Tag;
use core::ts::TransferSyntax;
use core::uid::UID;

pub struct Lookup {
    tags: tag_lookup::TagLookup,
    ts: ts_lookup::TransferSyntaxLookup,
    uids: uid_lookup::UidLookup,
}

impl Lookup {
    pub fn new() -> Lookup {
        Lookup {
            tags: tag_lookup::TagLookup::new(),
            ts: ts_lookup::TransferSyntaxLookup::new(),
            uids: uid_lookup::UidLookup::new(),
        }
    }

    pub fn tag_by_ident(&self, ident: &str) -> Option<&'static Tag> {
        self.tags.by_ident(ident)
    }

    pub fn tag_by_tag(&self, tag: u32) -> Option<&'static Tag> {
        self.tags.by_tag(tag)
    }

    pub fn ts_by_ident(&self, ident: &str) -> Option<&'static TransferSyntax> {
        self.ts.by_ident(ident)
    }

    pub fn ts_by_id(&self, id: &str) -> Option<&'static TransferSyntax> {
        self.ts.by_id(id)
    }

    pub fn ts_by_uid(&self, uid: &UID) -> Option<&'static TransferSyntax> {
        self.ts.by_uid(uid)
    }

    pub fn uid_by_ident(&self, ident: &str) -> Option<&'static UID> {
        self.uids.by_ident(ident)
    }

    pub fn uid_by_id(&self, id: &str) -> Option<&'static UID> {
        self.uids.by_id(id)
    }
}

#[cfg(test)]
mod tests {
    use core::dict::dicom_elements as tags;
    use core::dict::dir_structure_elements as dse;
    use core::dict::file_meta_elements as fme;
    use core::dict::transfer_syntaxes as ts;
    use core::dict::ts_lookup::TransferSyntaxLookup;
    use core::dict::uids;
    use core::lookup::Lookup;
    use core::tag::Tag;
    use core::ts::TransferSyntax;
    use core::uid::UID;

    #[test]
    pub fn test_tags() {
        let lookup: Lookup = Lookup::new();

        let pd_by_ident: &Tag = lookup.tag_by_ident("PixelData").expect("Tag not found");
        assert_eq!(pd_by_ident, &tags::PixelData);

        let pd_by_tag: &Tag = lookup.tag_by_tag(0x7FE00010).expect("Tag not found");
        assert_eq!(pd_by_tag, &tags::PixelData);


        let fsid_by_ident: &Tag = lookup.tag_by_ident("FileSetID").expect("Tag not found");
        assert_eq!(fsid_by_ident, &dse::FileSetID);

        let fsid_by_tag: &Tag = lookup.tag_by_tag(0x00041130 as u32).expect("Tag not found");
        assert_eq!(fsid_by_tag, &dse::FileSetID);


        let tsuid_by_ident: &Tag = lookup.tag_by_ident("TransferSyntaxUID").expect("Tag not found");
        assert_eq!(tsuid_by_ident, &fme::TransferSyntaxUID);

        let tsuid_by_tag: &Tag = lookup.tag_by_tag(0x00020010 as u32).expect("Tag not found");
        assert_eq!(tsuid_by_tag, &fme::TransferSyntaxUID);
    }

    #[test]
    pub fn test_transfer_syntaxes() {
        let lookup: Lookup = Lookup::new();

        let ivrle_by_ident: &TransferSyntax = lookup.ts_by_ident("ImplicitVRLittleEndian").expect("TransferSyntax not found");
        assert_eq!(ivrle_by_ident, &ts::ImplicitVRLittleEndian);

        let ivrle_by_id: &TransferSyntax = lookup.ts_by_id("1.2.840.10008.1.2").expect("TransferSyntax not found");
        assert_eq!(ivrle_by_id, &ts::ImplicitVRLittleEndian);

        let ivrle_by_uid: &TransferSyntax = lookup.ts_by_uid(&uids::ImplicitVRLittleEndian).expect("TransferSyntax not found");
        assert_eq!(ivrle_by_uid, &ts::ImplicitVRLittleEndian);


        let evrle_by_ident: &TransferSyntax = lookup.ts_by_ident("ExplicitVRLittleEndian").expect("TransferSyntax not found");
        assert_eq!(evrle_by_ident, &ts::ExplicitVRLittleEndian);

        let evrle_by_id: &TransferSyntax = lookup.ts_by_id("1.2.840.10008.1.2.1").expect("TransferSyntax not found");
        assert_eq!(evrle_by_id, &ts::ExplicitVRLittleEndian);

        let evrle_by_uid: &TransferSyntax = lookup.ts_by_uid(&uids::ExplicitVRLittleEndian).expect("TransferSyntax not found");
        assert_eq!(evrle_by_uid, &ts::ExplicitVRLittleEndian);
    }

    #[test]
    pub fn test_uids() {
        let lookup: Lookup = Lookup::new();

        let ctis_by_ident: &UID = lookup.uid_by_ident("CTImageStorage").expect("UID not found");
        assert_eq!(ctis_by_ident, &uids::CTImageStorage);

        let ctis_by_id: &UID = lookup.uid_by_id("1.2.840.10008.5.1.4.1.1.2").expect("UID not found");
        assert_eq!(ctis_by_id, &uids::CTImageStorage);
    }

    /// Sanity-check of the pre-defined TransferSyntax's to ensure
    /// that their defined properties reflect the UID's name.
    /// May catch issues with improperly copying over values from definitions.
    #[test]
    fn test_ts_name_vs_properties() {
        let lookup: TransferSyntaxLookup = TransferSyntaxLookup::new();
        for (_, ts) in lookup.uid_to_ts.iter() {
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
}