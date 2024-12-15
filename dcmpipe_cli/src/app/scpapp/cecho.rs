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

use dcmpipe_lib::dimse::{
    commands::messages::CommandMessage, error::AssocError, svcops::EchoSvcOp,
};

use crate::app::scpapp::AssociationDevice;

impl<R: Read, W: Write> AssociationDevice<R, W> {
    pub(crate) fn handle_c_echo_req(
        &mut self,
        op: &mut EchoSvcOp,
        cmd: &CommandMessage,
    ) -> Result<(), AssocError> {
        let pdu_max_snd_size = self.assoc.common().get_pdu_max_snd_size();
        let rsp = op.process_req(cmd)?;
        op.end_response(&rsp, &mut self.writer, pdu_max_snd_size)
    }
}
