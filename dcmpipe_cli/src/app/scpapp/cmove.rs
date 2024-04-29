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
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    net::TcpStream,
    path::PathBuf,
};

use dcmpipe_lib::{
    core::read::ParserBuilder,
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::{AffectedSOPClassUID, MessageID, MoveDestination},
    },
    dimse::{
        assoc::{scu::UserAssocBuilder, DimseMsg},
        commands::{messages::CommandMessage, CommandStatus},
        error::{AssocError, DimseError},
    },
};

use crate::app::{indexapp::DicomDoc, scpapp::AssociationDevice};

#[allow(unused_variables)]
impl<R: Read, W: Write> AssociationDevice<R, W> {
    pub(crate) fn handle_c_move_req(&mut self, cmd: &CommandMessage) -> Result<(), AssocError> {
        let ctx_id = cmd.ctx_id();
        let msg_id = cmd.get_ushort(&MessageID).map_err(AssocError::ab_failure)?;
        let aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;
        let dest = cmd
            .get_string(&MoveDestination)
            .map_err(AssocError::ab_failure)?;

        let Some(aet_host) = self.assoc.aet_host(&dest).cloned() else {
            let failure_status = CommandMessage::c_move_rsp(
                ctx_id,
                msg_id,
                &aff_sop_class,
                &CommandStatus::Failure(0xA801),
                0,
                0,
                0,
                0,
            );

            // Report failed progress to C-MOVE requestor.
            self.assoc
                .common()
                .write_command(&failure_status, &mut self.writer)?;
            return Ok(());
        };

        let (_pres_ctx, ts) = self.assoc.common().get_pres_ctx_and_ts(ctx_id)?;

        let dcm_query =
            self.assoc
                .common()
                .read_dataset_in_mem(&mut self.reader, &mut self.writer, ts)?;

        let query_results = self.query_database(&dcm_query)?;

        let path_map = Self::resolve_to_files(query_results.group_map);

        let sop_count = path_map
            .values()
            .map(|v| v.len())
            .sum::<usize>();
        let sop_count = u16::try_from(sop_count).unwrap_or_default();
        let move_status = CommandMessage::c_move_rsp(
            ctx_id,
            msg_id,
            &aff_sop_class,
            &CommandStatus::pending(),
            sop_count,
            0,
            0,
            0,
        );

        self.assoc
            .common()
            .write_command(&move_status, &mut self.writer)?;

        let stream = TcpStream::connect(aet_host)
            .map_err(|e| AssocError::ab_failure(DimseError::IOError(e)))?;

        let mut scu_assoc = UserAssocBuilder::new()
            .my_ae(self.assoc.common().this_ae().clone())
            .service_ae(dest.clone())
            .supported_abs(self.assoc.common().supported_abs().clone())
            .supported_ts(self.assoc.common().supported_ts().clone())
            .pdu_rcv_max_len(
                u32::try_from(self.assoc.common().get_pdu_max_rcv_size()).unwrap_or_default(),
            )
            .build();

        let mut dest_reader = BufReader::new(&stream);
        let mut dest_writer = BufWriter::new(&stream);
        if let Some(msg) = scu_assoc.request_association(&mut dest_reader, &mut dest_writer)? {
            // TODO: Add error comment to message.
            let failure_status = CommandMessage::c_move_rsp(
                ctx_id,
                msg_id,
                &aff_sop_class,
                &CommandStatus::Failure(0xC000),
                0,
                0,
                sop_count, // All failed
                0,
            );

            // Report failed progress to C-MOVE requestor.
            self.assoc
                .common()
                .write_command(&move_status, &mut self.writer)?;

            return Ok(());
        }

        let mut successful: u16 = 0;
        let mut remaining: u16 = sop_count;
        for (_key, paths) in path_map {
            for path in paths {
                let file = match File::open(path)
                    .map_err(|e| AssocError::ab_failure(DimseError::IOError(e)))
                {
                    Ok(file) => file,
                    Err(err) => {
                        // Abort the C-STORE association with the destination AE.
                        err.write(&mut dest_writer)
                            .map_err(AssocError::ab_failure)?;

                        let failure_status = CommandMessage::c_move_rsp(
                            ctx_id,
                            msg_id,
                            &aff_sop_class,
                            &CommandStatus::Failure(0xC000),
                            0, // No more remaining,
                            successful,
                            remaining, // The rest are not being attempted, all fail.
                            0,
                        );
                        self.assoc
                            .common()
                            .write_command(&failure_status, &mut self.writer)?;

                        // The failure here does not need to abort the association from the C-STORE
                        // requestor.
                        return Ok(());
                    }
                };

                let parser = ParserBuilder::default().build(file, &STANDARD_DICOM_DICTIONARY);
                let store_rsp = scu_assoc.c_store_req(
                    &mut dest_reader,
                    &mut dest_writer,
                    parser,
                    self.assoc.common().this_ae(),
                    msg_id,
                );

                let Ok(Some(DimseMsg::Cmd(cmd_rsp))) = store_rsp else {
                    let failure_status = CommandMessage::c_move_rsp(
                        ctx_id,
                        msg_id,
                        &aff_sop_class,
                        &CommandStatus::Failure(0xC000),
                        0, // No more remaining,
                        successful,
                        remaining, // The rest are not being attempted, all fail.
                        0,
                    );
                    self.assoc
                        .common()
                        .write_command(&failure_status, &mut self.writer)?;

                    return Ok(());
                };

                if !cmd_rsp.status().is_success() {
                    let failure_status = CommandMessage::c_move_rsp(
                        ctx_id,
                        msg_id,
                        &aff_sop_class,
                        &CommandStatus::Failure(0xC000),
                        0, // No more remaining,
                        successful,
                        remaining, // The rest are not being attempted, all fail.
                        0,
                    );
                    self.assoc
                        .common()
                        .write_command(&failure_status, &mut self.writer)?;

                    return Ok(());
                }

                successful += 1;
                remaining -= 1;

                // Report progress.
                let progress_status = CommandMessage::c_move_rsp(
                    ctx_id,
                    msg_id,
                    &aff_sop_class,
                    &CommandStatus::pending(),
                    remaining,
                    successful,
                    0,
                    0,
                );
                self.assoc
                    .common()
                    .write_command(&progress_status, &mut self.writer)?;
            }
        }

        let _ = scu_assoc.release_association(&mut dest_reader, &mut dest_writer);

        // Report success.
        let success_status = CommandMessage::c_move_rsp(
            ctx_id,
            msg_id,
            &aff_sop_class,
            &CommandStatus::success(),
            remaining,
            successful,
            0,
            0,
        );
        self.assoc
            .common()
            .write_command(&move_status, &mut self.writer)?;

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
