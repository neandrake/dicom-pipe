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

    pub fn tag_by_tag(&self, tag: &u32) -> Option<&'static Tag> {
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