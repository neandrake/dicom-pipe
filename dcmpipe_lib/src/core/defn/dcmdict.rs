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

use super::{tag::TagRef, ts::TSRef, uid::UIDRef};

/// A DICOM dictionary enables looking up transer syntaxes, tags, and uids by
/// their name or number (as defined in the standard), or by their UID.
pub trait DicomDictionary {
    /// Look up a `TransferSyntax` definition by its name, case-insensitive.
    fn get_ts_by_name(&self, name: &str) -> Option<TSRef>;
    /// Look up a `TransferSyntax` definition by its UID.
    fn get_ts_by_uid(&self, uid: &str) -> Option<TSRef>;

    /// Look up a `Tag` definition by its name, case-insensitive.
    fn get_tag_by_name(&self, name: &str) -> Option<TagRef>;
    /// Look up a `Tag` definition by its tag number.
    fn get_tag_by_number(&self, number: u32) -> Option<TagRef>;

    /// Look up a `UID` definition by its name, case-insensitive.
    fn get_uid_by_name(&self, name: &str) -> Option<UIDRef>;
    /// Look up a `UID` definition by its UID.
    fn get_uid_by_uid(&self, uid: &str) -> Option<UIDRef>;
}

impl core::fmt::Debug for dyn DicomDictionary + '_ {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "<DicomDictionary>")
    }
}

/// Wraps a list of DICOM dictionaries to be used a single dictionary. The order of the
/// dictionaries affects the resulting lookup if the search key exists in multiple dictionaries.
pub struct MultiDicomDictionary<'d> {
    dicts: Vec<&'d dyn DicomDictionary>,
}

impl<'d> MultiDicomDictionary<'d> {
    #[must_use]
    pub fn new(dicts: Vec<&'d dyn DicomDictionary>) -> Self {
        Self { dicts }
    }
}

impl<'d> DicomDictionary for MultiDicomDictionary<'d> {
    fn get_ts_by_name(&self, name: &str) -> Option<TSRef> {
        self.dicts.iter().find_map(|d| d.get_ts_by_name(name))
    }

    fn get_ts_by_uid(&self, uid: &str) -> Option<TSRef> {
        self.dicts.iter().find_map(|d| d.get_ts_by_uid(uid))
    }

    fn get_tag_by_name(&self, name: &str) -> Option<TagRef> {
        self.dicts.iter().find_map(|d| d.get_tag_by_name(name))
    }

    fn get_tag_by_number(&self, number: u32) -> Option<TagRef> {
        self.dicts.iter().find_map(|d| d.get_tag_by_number(number))
    }

    fn get_uid_by_name(&self, name: &str) -> Option<UIDRef> {
        self.dicts.iter().find_map(|d| d.get_uid_by_name(name))
    }

    fn get_uid_by_uid(&self, uid: &str) -> Option<UIDRef> {
        self.dicts.iter().find_map(|d| d.get_uid_by_uid(uid))
    }
}
