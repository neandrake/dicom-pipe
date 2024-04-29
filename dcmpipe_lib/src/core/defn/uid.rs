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
    #[must_use]
    pub fn new(uid: &'static str, ident: &'static str, name: &'static str) -> UID {
        UID { ident, uid, name }
    }

    /// Get the identifier or name for this UID.
    #[must_use]
    pub fn ident(&self) -> &'static str {
        self.ident
    }

    /// Get the string representation of the UID.
    #[must_use]
    pub fn uid(&self) -> &'static str {
        self.uid
    }

    /// Get the longer name or description of this UID.
    #[must_use]
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
