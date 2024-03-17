use crate::{
    core::{charset::DEFAULT_CHARACTER_SET, dcmobject::DicomRoot, defn::tag::Tag, RawValue},
    dict::{
        tags::{
            AffectedSOPClassUID, CommandDataSetType, CommandField, CommandGroupLength, MessageID,
            MessageIDBeingRespondedTo, Status,
        },
        transfer_syntaxes::ImplicitVRLittleEndian,
    },
    dimse::{
        commands::{CommandStatus, CommandType},
        DimseError,
    },
};

/// Sentinel value of `CommandDataSetType` (0000,0800) to indicate that there is no Data Set
/// present in the message. Any other value in `CommandDataSetType` indicates a Data Set is present
/// in the message.
///
/// See Part 7, Appendix E.
pub const COMMAND_DATASET_TYPE_NONE: u16 = 0x0101;

#[derive(Debug)]
pub struct CommandMessage {
    message: DicomRoot,
}

impl CommandMessage {
    fn create(elements: Vec<(&Tag, RawValue)>) -> Self {
        let mut message = DicomRoot::new_empty(&ImplicitVRLittleEndian, DEFAULT_CHARACTER_SET);
        for elem_pair in elements {
            message.add_child_with_val(elem_pair.0, elem_pair.1);
        }

        // Calculate the number of bytes of all the elements added above. This takes advantage of
        // the fact that CommandGroupLength will be the first element in the dataset but is not yet
        // added to the `DicomRoot`.
        let cmd_grp_len_val: u32 = u32::try_from(message.byte_size()).unwrap_or_default();
        message.add_child_with_val(&CommandGroupLength, RawValue::uint(cmd_grp_len_val));

        Self { message }
    }

    pub fn c_echo_req(msg_id: u16, sop_class_uid: &str) -> Self {
        CommandMessage::create(vec![
            (
                &AffectedSOPClassUID,
                RawValue::Uid(sop_class_uid.to_string()),
            ),
            (
                &CommandField,
                RawValue::ushort(u16::from(CommandType::CEchoReq)),
            ),
            (&MessageID, RawValue::ushort(msg_id)),
            (
                &CommandDataSetType,
                RawValue::ushort(COMMAND_DATASET_TYPE_NONE),
            ),
        ])
    }

    pub fn c_echo_rsp_from_req(
        req: CommandMessage,
        status: CommandStatus,
    ) -> Result<Self, DimseError> {
        let aff_sop_uid = req
            .message
            .get_child_by_tag(AffectedSOPClassUID.tag())
            .ok_or(DimseError::ElementMissingFromRequest(
                Tag::format_tag_to_display(AffectedSOPClassUID.tag()),
            ))
            .map(|e| e.element().parse_value())??;
        let msg_id = req
            .message
            .get_child_by_tag(MessageID.tag())
            .ok_or(DimseError::ElementMissingFromRequest(
                Tag::format_tag_to_display(MessageID.tag()),
            ))
            .map(|e| e.element().parse_value())??;
        Ok(CommandMessage::create(vec![
            (&AffectedSOPClassUID, aff_sop_uid),
            (
                &CommandField,
                RawValue::ushort(u16::from(CommandType::CEchoRsp)),
            ),
            (&MessageIDBeingRespondedTo, msg_id),
            (
                &CommandDataSetType,
                RawValue::ushort(COMMAND_DATASET_TYPE_NONE),
            ),
            (&Status, RawValue::from(status)),
        ]))
    }

    pub fn message(&self) -> &DicomRoot {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::RawValue,
        dict::{
            tags::{
                AffectedSOPClassUID, CommandDataSetType, CommandField, CommandGroupLength,
                MessageID,
            },
            transfer_syntaxes::ExplicitVRLittleEndian,
        },
        dimse::commands::{
            messages::{CommandMessage, COMMAND_DATASET_TYPE_NONE},
            CommandType,
        },
    };

    #[test]
    fn test_cecho_req() {
        let rq = CommandMessage::c_echo_req(718, ExplicitVRLittleEndian.uid().uid());
        let mut elem_iter = rq.message().iter_child_nodes();

        let cmd_grp_len = elem_iter.next().expect("Should have CommandGroupLength").1;
        assert_eq!(CommandGroupLength.tag(), cmd_grp_len.element().tag());
        let cmd_grp_len_val = cmd_grp_len
            .element()
            .parse_value()
            .expect("Should get value for CommandGroupLenth");
        assert_eq!(RawValue::uint(58), cmd_grp_len_val);

        let mut bytes = 0usize;

        let aff_sop_uid = elem_iter.next().expect("Should have AffectedSOPClassUID").1;
        assert_eq!(AffectedSOPClassUID.tag(), aff_sop_uid.element().tag());
        let aff_sop_uid_val = aff_sop_uid
            .element()
            .parse_value()
            .expect("Should get value for AffectedSOPClassUID");
        assert_eq!(
            RawValue::Uid(ExplicitVRLittleEndian.uid().uid().to_string()),
            aff_sop_uid_val
        );
        bytes += aff_sop_uid.byte_size();

        let cmd_field = elem_iter.next().expect("Should have CommandField").1;
        assert_eq!(CommandField.tag(), cmd_field.element().tag());
        let cmd_field_val = cmd_field
            .element()
            .parse_value()
            .expect("Should get value for CommandField");
        assert_eq!(
            RawValue::ushort(u16::from(CommandType::CEchoReq)),
            cmd_field_val
        );
        bytes += cmd_field.byte_size();

        let msg_id = elem_iter.next().expect("Should have MessageID").1;
        assert_eq!(MessageID.tag(), msg_id.element().tag());
        let msg_id_val = msg_id
            .element()
            .parse_value()
            .expect("Should get value for MessageID");
        assert_eq!(RawValue::ushort(718), msg_id_val);
        bytes += msg_id.byte_size();

        let cmd_ds_type = elem_iter.next().expect("Should have CommandDataSetType").1;
        assert_eq!(CommandDataSetType.tag(), cmd_ds_type.element().tag());
        let cmd_ds_type_val = cmd_ds_type
            .element()
            .parse_value()
            .expect("Should get value for CommandDataSetType");
        assert_eq!(RawValue::ushort(COMMAND_DATASET_TYPE_NONE), cmd_ds_type_val);
        bytes += cmd_ds_type.byte_size();

        assert_eq!(58, bytes);
    }
}
