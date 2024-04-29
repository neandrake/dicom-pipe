#![allow(dead_code)]
#![allow(non_camel_case_types)]

//! Unique Identifiers

use std::hash::{Hash, Hasher};

#[derive(Debug, Eq)]
pub struct UID {
    pub ident: &'static str,
    pub uid: &'static str,
    pub name: &'static str,
}

impl UID {
    pub fn new(uid: &'static str, ident: &'static str, name: &'static str) -> UID {
        UID {
            uid: uid,
            ident: ident,
            name: name,
        }
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
