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

use std::{
    collections::HashMap,
    io::{Read, Write},
    path::PathBuf,
};

use dcmpipe_lib::{
    dict::tags::{AffectedSOPClassUID, MessageID, MoveDestination},
    dimse::{commands::messages::CommandMessage, error::AssocError},
};

use crate::app::{indexapp::DicomDoc, scpapp::AssociationDevice};

impl<R: Read, W: Write> AssociationDevice<R, W> {
    #[allow(unused_variables)] // This is in development
    pub(crate) fn handle_c_move_req(&mut self, cmd: &CommandMessage) -> Result<(), AssocError> {
        let ctx_id = cmd.ctx_id();
        let msg_id = cmd.get_ushort(&MessageID).map_err(AssocError::ab_failure)?;
        let aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;
        let dest = cmd
            .get_string(&MoveDestination)
            .map_err(AssocError::ab_failure)?;

        let (pres_ctx, ts) = self.assoc.common().get_pres_ctx_and_ts(ctx_id)?;

        let dcm_query =
            self.assoc
                .common()
                .read_dataset_in_mem(&mut self.reader, &mut self.writer, ts)?;

        let query_results = self.query_database(&dcm_query)?;

        let path_map = Self::resolve_to_files(query_results.group_map);

        Ok(())
    }

    fn resolve_to_files(
        group_map: HashMap<String, Vec<DicomDoc>>,
    ) -> HashMap<String, Vec<PathBuf>> {
        let mut path_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
        for (group_id, docs) in group_map {
            let paths = docs
                .iter()
                .filter_map(|d| {
                    d.doc()
                        .get_document("metadata")
                        .and_then(|m| m.get_array("files"))
                        .ok()
                })
                .flatten()
                .filter_map(|b| b.as_str())
                .map(PathBuf::from)
                .collect::<Vec<PathBuf>>();
            path_map.insert(group_id, paths);
        }
        path_map
    }
}
