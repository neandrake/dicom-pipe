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
    dict::tags::AffectedSOPClassUID,
    dimse::{
        commands::{messages::CommandMessage, CommandStatus},
        error::AssocError,
    },
};

use super::assoc::CommonAssoc;

pub enum AssocSvcOp {
    Echo(EchoSvcOp),
    Find(FindSvcOp),
    Get(GetSvcOp),
    Move(MoveSvcOp),
    Store(StoreSvcOp),
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

    pub fn write_response<W: Write>(
        &mut self,
        rsp: &CommandMessage,
        assoc: &CommonAssoc,
        writer: &mut W,
    ) -> Result<(), AssocError> {
        assoc.write_command(&rsp, writer)?;
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
        let dcm_query = assoc.read_dataset_in_mem(reader, writer, ts)?;
        Ok(dcm_query)
    }

    pub fn write_response<W: Write>(
        &mut self,
        assoc: &CommonAssoc,
        mut writer: &mut W,
        dcm_results: &[DicomRoot],
    ) -> Result<(), AssocError> {
        for result in dcm_results {
            let cmd = CommandMessage::c_find_rsp(
                self.ctx_id,
                self.msg_id,
                &self.aff_sop_class,
                &CommandStatus::pending(),
            );
            assoc.write_command(&cmd, &mut writer)?;
            assoc.write_dataset(self.ctx_id, &result, &mut writer)?;
        }

        let cmd = CommandMessage::c_find_rsp(
            self.ctx_id,
            self.msg_id,
            &self.aff_sop_class,
            &CommandStatus::success(),
        );
        assoc.write_command(&cmd, &mut writer)?;

        self.is_complete = true;

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
        self.this_ae = assoc.this_ae().clone();
        self.aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;

        let (_pres_ctx, ts) = assoc.get_pres_ctx_and_ts(self.ctx_id)?;

        let dcm_query = assoc.read_dataset_in_mem(reader, writer, ts)?;
        Ok(dcm_query)
    }

    pub fn write_response<R: Read, W: Write>(
        &mut self,
        assoc: &CommonAssoc,
        mut writer: &mut W,
        dcm_results: &[DicomRoot],
    ) -> Result<(), AssocError> {
        Ok(())
    }
}

pub struct MoveSvcOp {
    msg_id: u16,
    is_complete: bool,
}

impl MoveSvcOp {
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
}

pub struct StoreSvcOp {
    msg_id: u16,
    is_complete: bool,
}

impl StoreSvcOp {
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
}
