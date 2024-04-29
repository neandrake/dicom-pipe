//! Unique Identifier

use std::hash::{Hash, Hasher};

pub type UIDRef = &'static UID;

/// Unique Identifier Definition
#[derive(Debug, Eq)]
pub struct UID {
    /// Identifier or name which can be used with a `DicomDictionary`.
    pub ident: &'static str,

    /// The string representation of the UID.
    pub uid: &'static str,

    /// A longer name or description of the UID.
    pub name: &'static str,
}

impl UID {
    pub fn new(uid: &'static str, ident: &'static str, name: &'static str) -> UID {
        UID { ident, uid, name }
    }

    /// Get the identifier or name for this UID.
    pub fn ident(&self) -> &'static str {
        self.ident
    }

    /// Get the string representation of the UID.
    pub fn uid(&self) -> &'static str {
        self.uid
    }

    /// Get the longer name or description of this UID.
    pub fn name(&self) -> &'static str {
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
