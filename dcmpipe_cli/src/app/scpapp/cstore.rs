use std::io::{Read, Write};

use dcmpipe_lib::{
    core::dcmobject::DicomRoot,
    dict::tags::{AffectedSOPClassUID, MessageID},
    dimse::{
        assoc::Association,
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

        let end_rsp = Association::create_cstore_end(
            ctx_id,
            msg_id,
            &aff_sop_class,
            &CommandStatus::success(),
        )?;
        self.assoc.write_pdu(&end_rsp, &mut self.writer)?;

        Ok(())
    }
}
