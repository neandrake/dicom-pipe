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

use dcmpipe_lib::{
    core::dcmobject::DicomRoot,
    dict::tags::{AffectedSOPClassUID, MessageID},
    dimse::{
        assoc::scp::ServiceAssoc,
        commands::{messages::CommandMessage, CommandStatus},
        error::AssocError,
    },
};

use crate::app::scpapp::AssociationDevice;

impl<R: Read, W: Write> AssociationDevice<R, W> {
    pub(crate) fn handle_c_store_req(
        &mut self,
        cmd: &CommandMessage,
        _dcm: &DicomRoot,
    ) -> Result<(), AssocError> {
        let ctx_id = cmd.ctx_id();
        let msg_id = cmd.get_ushort(&MessageID).map_err(AssocError::ab_failure)?;
        let aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;

        let end_rsp = ServiceAssoc::create_cstore_end(
            ctx_id,
            msg_id,
            &aff_sop_class,
            &CommandStatus::success(),
        )?;
        self.assoc.write_pdu(&end_rsp, &mut self.writer)?;

        Ok(())
    }
}
