//! Unique Identifiers

use std::hash::{Hash, Hasher};

pub type UIDRef = &'static UID;

/// Unique Identifiers
#[derive(Debug, Eq)]
pub struct UID {
    /// Some identifier or name, useful for lookup (no spaces - matches definition/name in code)
    pub ident: &'static str,
    /// The UID string code
    pub uid: &'static str,
    /// Some descriptive name
    pub name: &'static str,
}

impl UID {
    pub fn new(uid: &'static str, ident: &'static str, name: &'static str) -> UID {
        UID { uid, ident, name }
    }

    pub fn get_ident(&self) -> &'static str {
        self.ident
    }

    pub fn get_uid(&self) -> &'static str {
        self.uid
    }

    pub fn get_name(&self) -> &'static str {
        self.name
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
