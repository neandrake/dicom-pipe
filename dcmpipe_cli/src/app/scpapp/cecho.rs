use std::io::{Read, Write};

use dcmpipe_lib::{
    dict::tags::AffectedSOPClassUID,
    dimse::{assoc::Association, commands::messages::CommandMessage, error::AssocError},
};

use crate::app::scpapp::AssociationDevice;

impl<R: Read, W: Write> AssociationDevice<R, W> {
    pub(crate) fn handle_c_echo_req(&mut self, cmd: &CommandMessage) -> Result<(), AssocError> {
        let aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;
        let end_rsp = Association::create_cecho_end(cmd.ctx_id(), cmd.msg_id(), &aff_sop_class)?;
        self.assoc.write_pdu(&end_rsp, &mut self.writer)?;
        Ok(())
    }
}
