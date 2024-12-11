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

/// A C-ECHO operation to be managed by an SCP.
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

    /// Process the C-ECHO-RQ request returning an appropriate C-ECHO-RP command indicating success.
    ///
    /// # Errors
    /// - If the request does not contain `AffectedSOPClassUID`.
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

    /// Write the given C-ECHO-RP response, mark this operation as completed.
    ///
    /// # Errors
    /// - If there is an I/O error writing the response to the given `writer`.
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

/// A C-FIND operation to be managed by an SCP.
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

    /// Process the C-FIND-RQ request returning parsed dataset of the request, representing the
    /// query.
    ///
    /// # Errors
    /// - If the presentation context is not present for this message.
    /// - I/O errors parsing the dicom dataset from the request.
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

    /// Write the DICOM object as a C-FIND-RP. This is to be used for sending individual results,
    /// with a status of pending. To finish the response with success/failure use `end_response`.
    ///
    /// # Errors
    /// - I/O errors writing the command or dataset.
    pub fn write_response<W: Write>(
        &mut self,
        mut writer: &mut W,
        pdu_max_snd_size: usize,
        dcm_result: &DicomRoot,
    ) -> Result<(), AssocError> {
        let cmd = CommandMessage::c_find_rsp(
            self.ctx_id,
            self.msg_id,
            &self.aff_sop_class,
            &CommandStatus::pending(),
        );
        CommonAssoc::write_command(&cmd, &mut writer, pdu_max_snd_size)?;
        CommonAssoc::write_dataset(self.ctx_id, dcm_result, &mut writer, pdu_max_snd_size)?;

        Ok(())
    }

    /// Writes a final response, marking this operation as completed.
    ///
    /// # Errors
    /// - I/O errors writing the command result.
    pub fn end_response<W: Write>(
        &mut self,
        mut writer: &mut W,
        pdu_max_snd_size: usize,
        status: &CommandStatus,
    ) -> Result<(), AssocError> {
        let cmd = CommandMessage::c_find_rsp(self.ctx_id, self.msg_id, &self.aff_sop_class, status);
        CommonAssoc::write_command(&cmd, &mut writer, pdu_max_snd_size)?;
        self.is_complete = true;
        Ok(())
    }
}

/// A C-GET operation to be managed by an SCP.
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

    /// Process a C-GET-RQ request, returning the parsed DICOM object representing the query for
    /// the request.
    ///
    /// # Errors
    /// - If the request does not contain `AffectedSOPClassUID`.
    /// - I/O errors reading the DICOM query dataset.
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

    /// Write a C-GET-RP response, which is a progress indicator as C-GET invokes a C-STORE
    /// sub-operation to send the DICOM datasets matching the query. If the status is not pending
    /// then this operation is marked as completed.
    ///
    /// # Errors
    /// - I/O errors writing the response.
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

/// A C-MOVE operation to be managed by an SCP.
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

    /// Process a C-MOVE-RQ request returning the DICOM dataset representing the query for the
    /// operation.
    ///
    /// # Errors
    /// - If the request has no `AffectedSOPClassUID` or `MoveDestination`.
    /// - I/O errors reading or parsing the DICOM dataset representing the query.
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

    /// Write a C-MOVE-RP response, which is just a progress indicator as the C-MOVE invokes a
    /// C-STORE sub-operation to do the actual move. If the status is not pending then this
    /// operation is marked as completed.
    ///
    /// # Errors
    /// - I/O errors writing the response.
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

/// A C-STORE operation to be managed by an SCP.
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

    /// Process a C-STORE-RQ request. After this is called the DICOM dataset payload should be read
    /// directly from the reader stream. This processing does not read the DICOM dataset as C-STORE
    /// payloads may be large in size and it is desireable for the caller of this operation to
    /// manage the payload in an efficient way, and not read it entirely into memory.
    ///
    /// # Errors
    /// - If there is no `AffectedSOPClassUID`.
    pub fn process_req(&mut self, cmd: &CommandMessage) -> Result<(), AssocError> {
        self.ctx_id = cmd.ctx_id();
        self.aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;

        Ok(())
    }

    /// Write a response, which is a progress indicator. If the status is not pending then this
    /// operation is marked as completed.
    ///
    /// # Errors
    /// - I/O errors writing the response.
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

/// A C-CANCEL request for stopping an SCP's active operation.
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

    /// Process the C-CANCEL-RQ request.
    ///
    /// # Errors
    /// - None, this returns a result for consistency with the other operation implementations.
    pub fn process_req(&mut self, cmd: &CommandMessage) -> Result<(), AssocError> {
        self.ctx_id = cmd.ctx_id();
        Ok(())
    }
}
