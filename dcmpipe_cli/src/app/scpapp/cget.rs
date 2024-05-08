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
    fs::File,
    io::{BufReader, Read, Write},
};

use dcmpipe_lib::{
    core::read::ParserBuilder,
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::{AffectedSOPClassUID, MessageID},
    },
    dimse::{
        commands::messages::CommandMessage,
        error::{AssocError, DimseError},
    },
};

use crate::app::scpapp::{fail, prog, AssociationDevice, Stat, StatusMsgBuilder};

impl<R: Read, W: Write> AssociationDevice<R, W> {
    pub(crate) fn handle_c_get_req(&mut self, cmd: &CommandMessage) -> Result<(), AssocError> {
        let ctx_id = cmd.ctx_id();
        let msg_id = cmd.get_ushort(&MessageID).map_err(AssocError::ab_failure)?;
        let aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;

        let statter = StatusMsgBuilder::new(false, ctx_id, msg_id, aff_sop_class);

        let (_pres_ctx, ts) = self.assoc.common().get_pres_ctx_and_ts(ctx_id)?;

        let dcm_query =
            self.assoc
                .common()
                .read_dataset_in_mem(&mut self.reader, &mut self.writer, ts)?;

        let query_results = self.query_database(&dcm_query)?;
        let path_map = Self::resolve_to_paths(query_results.group_map);

        let sop_count = path_map.values().map(Vec::len).sum::<usize>();
        let sop_count = u16::try_from(sop_count).unwrap_or_default();

        let mut successful: u16 = 0;
        let mut remaining: u16 = sop_count;
        let mut store_msg_id = msg_id + 1;
        for (_key, paths) in path_map {
            for path in paths {
                let file = match File::open(&path)
                    .map_err(|e| AssocError::ab_failure(DimseError::IOError(e)))
                {
                    Ok(file) => file,
                    Err(err) => {
                        // Abort the C-STORE association with the destination AE.
                        let _ = err.write(&mut self.writer);

                        // For now, if one fails then do not attempt the rest.
                        self.assoc.common().write_command(
                            &statter.msg(&Stat::fail(), &prog(0, successful, remaining, 0)),
                            &mut self.writer,
                        )?;
                        return fail(&format!("Failed resolving {path:?}"));
                    }
                };

                let input = BufReader::with_capacity(1024 * 1024, file);
                let parser = ParserBuilder::default().build(input, &STANDARD_DICOM_DICTIONARY);
                let store_rsp = self.assoc.common().c_store_req(
                    &mut self.reader,
                    &mut self.writer,
                    parser,
                    store_msg_id,
                    self.assoc.common().this_ae(),
                    msg_id,
                );
                self.interpret_cstore_rsp(store_rsp, &statter, &prog(0, successful, remaining, 0))?;

                store_msg_id += 1;
                successful += 1;
                remaining -= 1;

                self.assoc.common().write_command(
                    &statter.msg(&Stat::pending(), &prog(remaining, successful, 0, 0)),
                    &mut self.writer,
                )?;
            }
        }

        self.assoc.common().write_command(
            &statter.msg(&Stat::success(), &prog(0, successful, 0, 0)),
            &mut self.writer,
        )?;

        Ok(())
    }
}
