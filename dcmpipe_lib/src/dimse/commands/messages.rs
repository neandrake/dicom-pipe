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
    pub fn new(message: DicomRoot) -> Self {
        Self { message }
    }

    pub fn message(&self) -> &DicomRoot {
        &self.message
    }

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
                RawValue::ushort(u16::from(&CommandType::CEchoReq)),
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
        status: &CommandStatus,
    ) -> Result<Self, DimseError> {
        let aff_sop_uid = req
            .message
            .get_child_by_tag(&AffectedSOPClassUID)
            .ok_or(DimseError::ElementMissingFromRequest(
                Tag::format_tag_to_display(&AffectedSOPClassUID),
            ))
            .map(|e| e.element().parse_value())??;
        let msg_id = req
            .message
            .get_child_by_tag(&MessageID)
            .ok_or(DimseError::ElementMissingFromRequest(
                Tag::format_tag_to_display(&MessageID),
            ))
            .map(|e| e.element().parse_value())??;
        Ok(CommandMessage::create(vec![
            (&AffectedSOPClassUID, aff_sop_uid),
            (
                &CommandField,
                RawValue::ushort(u16::from(&CommandType::CEchoRsp)),
            ),
            (&MessageIDBeingRespondedTo, msg_id),
            (
                &CommandDataSetType,
                RawValue::ushort(COMMAND_DATASET_TYPE_NONE),
            ),
            (&Status, RawValue::from(status)),
        ]))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{dcmobject::DicomObject, defn::tag::Tag, RawValue},
        dict::{
            tags::{
                AffectedSOPClassUID, CommandDataSetType, CommandField, CommandGroupLength,
                MessageID, MessageIDBeingRespondedTo, Status,
            },
            uids::{CTImageStorage, MRImageStorage},
        },
        dimse::commands::{
            messages::{CommandMessage, COMMAND_DATASET_TYPE_NONE},
            CommandStatus, CommandType,
        },
    };

    /// Asserts the `act_pair` contains an element matching the given expected tag and value. The
    /// actual value is taken as a pair for convenience for working with an iterator's `next()`.
    fn assert_eq_elem(
        exp_tag: &Tag,
        exp_val: RawValue,
        act_pair: Option<(&u32, &DicomObject)>,
    ) -> usize {
        let (_act_tag, act_obj) =
            act_pair.expect(&format!("Should have element: {}", exp_tag.ident()));
        assert_eq!(exp_tag.tag(), act_obj.element().tag());
        let act_val = act_obj
            .element()
            .parse_value()
            .expect(&format!("Should get value for: {}", exp_tag.ident()));
        assert_eq!(exp_val, act_val, "for {}", exp_tag.ident());
        act_obj.byte_size()
    }

    #[test]
    fn test_cecho_rsp() {
        let exp_msg_id = 718;
        let exp_affected_sop = CTImageStorage.uid();
        let exp_status = CommandStatus::Success(0);
        let exp_bytes = 74usize;

        let req = CommandMessage::c_echo_req(exp_msg_id, exp_affected_sop);
        let rsp = CommandMessage::c_echo_rsp_from_req(req, &exp_status).expect("build response");

        let mut elem_iter = rsp.message().iter_child_nodes();

        assert_eq_elem(
            &CommandGroupLength,
            RawValue::uint(u32::try_from(exp_bytes).expect("exp_bytes as u32")),
            elem_iter.next(),
        );

        let mut bytes = 0usize;

        bytes += assert_eq_elem(
            &AffectedSOPClassUID,
            RawValue::Uid(exp_affected_sop.to_string()),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(
            &CommandField,
            RawValue::ushort(u16::from(&CommandType::CEchoRsp)),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(
            &MessageIDBeingRespondedTo,
            RawValue::ushort(exp_msg_id),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(
            &CommandDataSetType,
            RawValue::ushort(COMMAND_DATASET_TYPE_NONE),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(&Status, RawValue::from(&exp_status), elem_iter.next());

        assert_eq!(exp_bytes, bytes);
    }

    #[test]
    fn test_cecho_req() {
        let exp_msg_id = 718;
        let exp_affected_sop = MRImageStorage.uid();
        let exp_bytes = 64usize;

        let req = CommandMessage::c_echo_req(exp_msg_id, exp_affected_sop);

        let mut elem_iter = req.message().iter_child_nodes();

        assert_eq_elem(
            &CommandGroupLength,
            RawValue::uint(u32::try_from(exp_bytes).expect("exp_bytes as u32")),
            elem_iter.next(),
        );

        let mut bytes = 0usize;

        bytes += assert_eq_elem(
            &AffectedSOPClassUID,
            RawValue::Uid(exp_affected_sop.to_string()),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(
            &CommandField,
            RawValue::ushort(u16::from(&CommandType::CEchoReq)),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(&MessageID, RawValue::ushort(exp_msg_id), elem_iter.next());

        bytes += assert_eq_elem(
            &CommandDataSetType,
            RawValue::ushort(COMMAND_DATASET_TYPE_NONE),
            elem_iter.next(),
        );

        assert_eq!(exp_bytes, bytes);
    }
}
