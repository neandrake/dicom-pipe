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

use std::io::{Read, Write};

use crate::{
    core::dcmobject::DicomRoot,
    dict::tags::{AffectedSOPClassUID, MoveDestination},
    dimse::{
        assoc::CommonAssoc,
        commands::{messages::CommandMessage, CommandStatus, SubOpProgress},
        error::AssocError,
    },
};

pub enum AssocSvcOp {
    Echo(EchoSvcOp),
    Find(FindSvcOp),
    Get(GetSvcOp),
    Move(MoveSvcOp),
    Store(StoreSvcOp),
    Cancel(CancelSvcOp),
}

pub struct EchoSvcOp {
    msg_id: u16,
    is_complete: bool,
}

impl EchoSvcOp {
    #[must_use]
    pub fn new(msg_id: u16) -> Self {
        Self {
            msg_id,
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn process_req(&mut self, cmd: &CommandMessage) -> Result<CommandMessage, AssocError> {
        let aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;

        let cmd = CommandMessage::c_echo_rsp(
            cmd.ctx_id(),
            cmd.msg_id(),
            &aff_sop_class,
            &CommandStatus::success(),
        );

        Ok(cmd)
    }

    pub fn end_response<W: Write>(
        &mut self,
        rsp: &CommandMessage,
        writer: &mut W,
        pdu_max_snd_size: usize,
    ) -> Result<(), AssocError> {
        CommonAssoc::write_command(rsp, writer, pdu_max_snd_size)?;
        self.is_complete = true;
        Ok(())
    }
}

pub struct FindSvcOp {
    msg_id: u16,
    ctx_id: u8,
    aff_sop_class: String,
    is_complete: bool,
}

impl FindSvcOp {
    #[must_use]
    pub fn new(msg_id: u16) -> Self {
        Self {
            msg_id,
            ctx_id: 0,
            aff_sop_class: String::new(),
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn ctx_id(&self) -> u8 {
        self.ctx_id
    }

    #[must_use]
    pub fn aff_sop_class(&self) -> &str {
        &self.aff_sop_class
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn process_req<R: Read, W: Write>(
        &mut self,
        cmd: &CommandMessage,
        assoc: &CommonAssoc,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<DicomRoot, AssocError> {
        self.ctx_id = cmd.ctx_id();
        self.aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;
        let (_pres_ctx, ts) = assoc.get_pres_ctx_and_ts(self.ctx_id)?;
        let dcm_query =
            CommonAssoc::read_dataset_in_mem(reader, writer, assoc.get_pdu_max_rcv_size(), ts)?;
        Ok(dcm_query)
    }

    pub fn write_response<W: Write>(
        &mut self,
        mut writer: &mut W,
        pdu_max_snd_size: usize,
        dcm_result: &DicomRoot,
        status: &CommandStatus,
    ) -> Result<(), AssocError> {
        let cmd = CommandMessage::c_find_rsp(self.ctx_id, self.msg_id, &self.aff_sop_class, status);
        CommonAssoc::write_command(&cmd, &mut writer, pdu_max_snd_size)?;
        CommonAssoc::write_dataset(self.ctx_id, dcm_result, &mut writer, pdu_max_snd_size)?;

        Ok(())
    }

    pub fn end_response<W: Write>(
        &mut self,
        mut writer: &mut W,
        pdu_max_snd_size: usize,
        status: &CommandStatus,
    ) -> Result<(), AssocError> {
        let cmd = CommandMessage::c_find_rsp(self.ctx_id, self.msg_id, &self.aff_sop_class, status);
        CommonAssoc::write_command(&cmd, &mut writer, pdu_max_snd_size)?;
        self.is_complete = !status.is_pending();
        Ok(())
    }
}

pub struct GetSvcOp {
    msg_id: u16,
    ctx_id: u8,
    this_ae: String,
    aff_sop_class: String,
    is_complete: bool,
}

impl GetSvcOp {
    #[must_use]
    pub fn new(msg_id: u16) -> Self {
        Self {
            msg_id,
            ctx_id: 0,
            this_ae: String::new(),
            aff_sop_class: String::new(),
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn ctx_id(&self) -> u8 {
        self.ctx_id
    }

    #[must_use]
    pub fn aff_sop_class(&self) -> &str {
        &self.aff_sop_class
    }

    #[must_use]
    pub fn this_ae(&self) -> &str {
        &self.this_ae
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn process_req<R: Read, W: Write>(
        &mut self,
        cmd: &CommandMessage,
        assoc: &CommonAssoc,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<DicomRoot, AssocError> {
        self.ctx_id = cmd.ctx_id();
        self.this_ae.clone_from(assoc.this_ae());
        self.aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;

        let (_pres_ctx, ts) = assoc.get_pres_ctx_and_ts(self.ctx_id)?;
        let dcm_query =
            CommonAssoc::read_dataset_in_mem(reader, writer, assoc.get_pdu_max_rcv_size(), ts)?;
        Ok(dcm_query)
    }

    pub fn write_response<W: Write>(
        &mut self,
        mut writer: &mut W,
        pdu_max_snd_size: usize,
        status: &CommandStatus,
        progress: &SubOpProgress,
    ) -> Result<(), AssocError> {
        let cmd = CommandMessage::c_get_rsp(
            self.ctx_id,
            self.msg_id,
            &self.aff_sop_class,
            status,
            progress,
        );
        CommonAssoc::write_command(&cmd, &mut writer, pdu_max_snd_size)?;

        self.is_complete = !status.is_pending();

        Ok(())
    }
}

pub struct MoveSvcOp {
    msg_id: u16,
    ctx_id: u8,
    aff_sop_class: String,
    aet_dest: String,
    is_complete: bool,
}

impl MoveSvcOp {
    #[must_use]
    pub fn new(msg_id: u16) -> Self {
        Self {
            msg_id,
            ctx_id: 0,
            aff_sop_class: String::new(),
            aet_dest: String::new(),
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn ctx_id(&self) -> u8 {
        self.ctx_id
    }

    #[must_use]
    pub fn aff_sop_class(&self) -> &str {
        &self.aff_sop_class
    }

    #[must_use]
    pub fn aet_dest(&self) -> &str {
        &self.aet_dest
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn process_req<R: Read, W: Write>(
        &mut self,
        cmd: &CommandMessage,
        assoc: &CommonAssoc,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<DicomRoot, AssocError> {
        self.ctx_id = cmd.ctx_id();
        self.aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;
        self.aet_dest = cmd
            .get_string(&MoveDestination)
            .map_err(AssocError::ab_failure)?;

        let (_pres_ctx, ts) = assoc.get_pres_ctx_and_ts(self.ctx_id)?;
        let dcm_query =
            CommonAssoc::read_dataset_in_mem(reader, writer, assoc.get_pdu_max_rcv_size(), ts)?;
        Ok(dcm_query)
    }

    pub fn write_response<W: Write>(
        &mut self,
        mut writer: &mut W,
        pdu_max_snd_size: usize,
        status: &CommandStatus,
        progress: &SubOpProgress,
    ) -> Result<(), AssocError> {
        let cmd = CommandMessage::c_move_rsp(
            self.ctx_id,
            self.msg_id,
            &self.aff_sop_class,
            status,
            progress,
        );

        CommonAssoc::write_command(&cmd, &mut writer, pdu_max_snd_size)?;

        self.is_complete = !status.is_pending();

        Ok(())
    }
}

pub struct StoreSvcOp {
    msg_id: u16,
    ctx_id: u8,
    aff_sop_class: String,
    is_complete: bool,
}

impl StoreSvcOp {
    #[must_use]
    pub fn new(msg_id: u16) -> Self {
        Self {
            msg_id,
            ctx_id: 0,
            aff_sop_class: String::new(),
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    /// Initial processing of the request. After calling this the dataset can be read directly from
    /// the reader stream.
    pub fn process_req(&mut self, cmd: &CommandMessage) -> Result<(), AssocError> {
        self.ctx_id = cmd.ctx_id();
        self.aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;

        Ok(())
    }

    pub fn write_response<W: Write>(
        &mut self,
        mut writer: &mut W,
        pdu_max_snd_size: usize,
        status: &CommandStatus,
    ) -> Result<(), AssocError> {
        let cmd =
            CommandMessage::c_store_rsp(self.ctx_id, self.msg_id, &self.aff_sop_class, status);
        CommonAssoc::write_command(&cmd, &mut writer, pdu_max_snd_size)?;

        self.is_complete = !status.is_pending();

        Ok(())
    }
}

pub struct CancelSvcOp {
    msg_id: u16,
    ctx_id: u8,
    is_complete: bool,
}

impl CancelSvcOp {
    #[must_use]
    pub fn new(msg_id: u16) -> Self {
        Self {
            msg_id,
            ctx_id: 0,
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn process_req(&mut self, cmd: &CommandMessage) -> Result<(), AssocError> {
        self.ctx_id = cmd.ctx_id();
        Ok(())
    }
}
