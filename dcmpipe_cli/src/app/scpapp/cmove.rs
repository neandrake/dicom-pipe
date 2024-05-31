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
    io::{BufReader, BufWriter, Read, Write},
    net::TcpStream,
};

use dcmpipe_lib::{
    core::read::ParserBuilder,
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::{AffectedSOPClassUID, MessageID, MoveDestination},
    },
    dimse::{
        assoc::{
            scu::{UserAssoc, UserAssocBuilder},
            CommonAssoc,
        },
        commands::messages::CommandMessage,
        error::{AssocError, DimseError},
    },
};

use crate::app::scpapp::{fail, prog, AssociationDevice, Stat, StatusMsgBuilder};

impl<R: Read, W: Write> AssociationDevice<R, W> {
    pub(crate) fn handle_c_move_req(&mut self, cmd: &CommandMessage) -> Result<(), AssocError> {
        let ctx_id = cmd.ctx_id();
        let msg_id = cmd.get_ushort(&MessageID).map_err(AssocError::ab_failure)?;
        let aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;

        let statter = StatusMsgBuilder::new(true, ctx_id, msg_id, aff_sop_class);

        let dest = cmd
            .get_string(&MoveDestination)
            .map_err(AssocError::ab_failure)?;
        let Some(aet_host) = self.assoc.aet_host(&dest).cloned() else {
            // Report failed progress to C-MOVE requestor. The query for matching SOPs happens
            // after validating the AE title so we can't report any numbers for progress.
            self.assoc.common().write_command(
                &statter.msg(&Stat::fail_unknown_dest(), &prog(0, 0, 0, 0)),
                &mut self.writer,
            )?;
            return Err(AssocError::ab_failure(DimseError::InvalidAeTitle(dest)));
        };

        let (_pres_ctx, ts) = self.assoc.common().get_pres_ctx_and_ts(ctx_id)?;
        let dcm_query =
            self.assoc
                .common()
                .read_dataset_in_mem(&mut self.reader, &mut self.writer, ts)?;

        let query_results = self.query_database(&dcm_query)?;
        let path_map = Self::resolve_to_paths(query_results.group_map);

        let sop_count = path_map.values().map(Vec::len).sum::<usize>();
        let sop_count = u16::try_from(sop_count).unwrap_or_default();

        self.assoc.common().write_command(
            &statter.msg(&Stat::pending(), &prog(sop_count, 0, 0, 0)),
            &mut self.writer,
        )?;

        let mut scu_assoc = self.create_sub_assoc(&dest);

        let stream = TcpStream::connect(aet_host)
            .map_err(|e| AssocError::ab_failure(DimseError::IOError(e)))?;
        let mut dest_reader = BufReader::new(&stream);
        let mut dest_writer = BufWriter::new(&stream);
        if let Some(msg) = scu_assoc.request_association(&mut dest_reader, &mut dest_writer)? {
            self.assoc.common().write_command(
                &statter.msg(&Stat::fail(), &prog(0, 0, sop_count, 0)),
                &mut self.writer,
            )?;
            return fail(&format!(
                "Association to {dest} failed with response: {msg:?}"
            ));
        }

        let mut successful: u16 = 0;
        let mut remaining: u16 = sop_count;
        for (_key, paths) in path_map {
            for path in paths {
                let file = match File::open(&path)
                    .map_err(|e| AssocError::ab_failure(DimseError::IOError(e)))
                {
                    Ok(file) => file,
                    Err(err) => {
                        // Abort the C-STORE association with the destination AE.
                        let _ = err.write(&mut dest_writer);

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
                let store_msg_id = scu_assoc.next_msg_id();
                scu_assoc.common_mut().c_store_req(
                    &mut dest_reader,
                    &mut dest_writer,
                    parser,
                    store_msg_id,
                    self.assoc.common().this_ae(),
                    msg_id,
                )?;

                let store_rsp = CommonAssoc::next_msg(
                    &mut dest_reader,
                    &mut dest_writer,
                    self.assoc.common().get_pdu_max_rcv_size(),
                );

                if let Err(e) = self.interpret_cstore_rsp(
                    store_rsp,
                    &statter,
                    &prog(0, successful, remaining, 0),
                ) {
                    let _ = e.write(&mut dest_writer);
                    return Err(e);
                }

                successful += 1;
                remaining -= 1;

                if let Err(e) = self.assoc.common().write_command(
                    &statter.msg(&Stat::pending(), &prog(remaining, successful, 0, 0)),
                    &mut self.writer,
                ) {
                    // Create error to abort the sub-association but return the original error.
                    let _ =
                        AssocError::ab_failure(DimseError::ApplicationError("unused error".into()))
                            .write(&mut dest_writer);
                    return Err(e);
                }
            }
        }

        // If there's an error releasing the association don't propagate as an error.
        let _ = scu_assoc.release_association(&mut dest_reader, &mut dest_writer);

        self.assoc.common().write_command(
            &statter.msg(&Stat::success(), &prog(remaining, successful, 0, 0)),
            &mut self.writer,
        )?;
        Ok(())
    }

    /// Creates a new association for communicating with the destination AE.
    fn create_sub_assoc(&self, dest_ae: &str) -> UserAssoc {
        UserAssocBuilder::new()
            .my_ae(self.assoc.common().this_ae().clone())
            .service_ae(dest_ae.to_owned())
            .supported_abs(self.assoc.common().supported_abs().clone())
            .supported_ts(self.assoc.common().supported_ts().clone())
            .pdu_rcv_max_len(
                u32::try_from(self.assoc.common().get_pdu_max_rcv_size()).unwrap_or_default(),
            )
            .build()
    }
}
