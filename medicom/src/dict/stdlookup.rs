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

use crate::{
    core::defn::{dcmdict::DicomDictionary, tag::TagRef, ts::TSRef, uid::UIDRef},
    dict::lookup::{TAG_BY_IDENT, TAG_BY_VALUE, TS_BY_IDENT, TS_BY_UID, UID_BY_IDENT, UID_BY_UID},
};

/// The Standard DICOM Dictionary
pub static STANDARD_DICOM_DICTIONARY: StandardDicomDictionary = StandardDicomDictionary {};

/// The Standard DICOM Dictionary
pub struct StandardDicomDictionary {}

impl DicomDictionary for StandardDicomDictionary {
    fn get_ts_by_uid(&self, uid: &str) -> Option<TSRef> {
        TS_BY_UID.get(uid).copied()
    }

    fn get_ts_by_name(&self, name: &str) -> Option<TSRef> {
        TS_BY_IDENT.get(&name.to_lowercase()).copied()
    }

    fn get_tag_by_number(&self, number: u32) -> Option<TagRef> {
        TAG_BY_VALUE.get(&number).copied()
    }

    fn get_tag_by_name(&self, name: &str) -> Option<TagRef> {
        TAG_BY_IDENT.get(&name.to_lowercase()).copied()
    }

    fn get_uid_by_uid(&self, uid: &str) -> Option<UIDRef> {
        UID_BY_UID.get(uid).copied()
    }

    fn get_uid_by_name(&self, name: &str) -> Option<UIDRef> {
        UID_BY_IDENT.get(&name.to_lowercase()).copied()
    }
}
