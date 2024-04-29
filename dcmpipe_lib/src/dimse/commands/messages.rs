use crate::{
    core::{
        charset::DEFAULT_CHARACTER_SET,
        dcmobject::DicomRoot,
        defn::{tag::Tag, ts::TSRef},
        RawValue,
    },
    dict::tags::{
        AffectedSOPClassUID, CommandDataSetType, CommandField, CommandGroupLength, MessageID,
        MessageIDBeingRespondedTo, Priority, Status,
    },
    dimse::{
        commands::{CommandStatus, CommandType},
        DimseError,
    },
};

use super::CommandPriority;

/// Sentinel value of `CommandDataSetType` (0000,0800) to indicate that there is no Data Set
/// present in the message. Any other value in `CommandDataSetType` indicates a Data Set is present
/// in the message.
///
/// See Part 7, Appendix E.
pub const COMMAND_DATASET_TYPE_NONE: u16 = 0x0101;

/// Sentinel value of `CommandDataSetType` (0000,0800) to indicate that there is some Data Set
/// present in the message. This value is arbitrary, as long as it's not
/// `COMMAND_DATASET_TYPE_NONE`, and avoiding using zero.
const COMMAND_DATASET_TYPE_SOME: u16 = 0x1010;

#[derive(Debug)]
pub struct CommandMessage {
    msg_id: u16,
    cmd_type: CommandType,
    priority: CommandPriority,
    has_dataset: bool,
    status: CommandStatus,
    message: DicomRoot,
}

impl CommandMessage {
    pub fn new(message: DicomRoot) -> Self {
        let msg_id = message
            .get_value_by_tag(&MessageID)
            .or_else(|| message.get_value_by_tag(&MessageIDBeingRespondedTo))
            .and_then(|v| v.ushort())
            .unwrap_or_default();
        let cmd_type = CommandType::from(
            message
                .get_value_by_tag(&CommandField)
                .and_then(|v| v.ushort())
                .unwrap_or_default(),
        );
        let priority = CommandPriority::from(
            message
                .get_value_by_tag(&Priority)
                .and_then(|v| v.ushort())
                .unwrap_or_default(),
        );
        let has_dataset = message
            .get_value_by_tag(&CommandDataSetType)
            .and_then(|v| v.ushort())
            .unwrap_or_default()
            != COMMAND_DATASET_TYPE_NONE;
        let status = CommandStatus::from(
            message
                .get_value_by_tag(&Status)
                .and_then(|v| v.ushort())
                .unwrap_or_default(),
        );

        Self {
            msg_id,
            cmd_type,
            priority,
            has_dataset,
            status,
            message,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn cmd_type(&self) -> &CommandType {
        &self.cmd_type
    }

    #[must_use]
    pub fn priority(&self) -> &CommandPriority {
        &self.priority
    }

    #[must_use]
    pub fn message(&self) -> &DicomRoot {
        &self.message
    }

    #[must_use]
    pub fn has_dataset(&self) -> bool {
        self.has_dataset
    }

    #[must_use]
    pub fn status(&self) -> &CommandStatus {
        &self.status
    }

    fn create(ts: TSRef, elements: Vec<(&Tag, RawValue)>) -> Self {
        let mut message = DicomRoot::new_empty(ts, DEFAULT_CHARACTER_SET);
        for elem_pair in elements {
            message.add_child_with_val(elem_pair.0, elem_pair.1);
        }

        // Calculate the number of bytes of all the elements added above. This takes advantage of
        // the fact that CommandGroupLength will be the first element in the dataset but is not yet
        // added to the `DicomRoot`.
        let cmd_grp_len_val: u32 = u32::try_from(message.byte_size()).unwrap_or_default();
        message.add_child_with_val(&CommandGroupLength, RawValue::of_uint(cmd_grp_len_val));

        Self::new(message)
    }

    pub fn c_echo_req(ts: TSRef, msg_id: u16, sop_class_uid: &str) -> Self {
        CommandMessage::create(
            ts,
            vec![
                (
                    &AffectedSOPClassUID,
                    RawValue::Uid(sop_class_uid.to_string()),
                ),
                (
                    &CommandField,
                    RawValue::of_ushort(u16::from(&CommandType::CEchoReq)),
                ),
                (&MessageID, RawValue::of_ushort(msg_id)),
                (
                    &CommandDataSetType,
                    RawValue::of_ushort(COMMAND_DATASET_TYPE_NONE),
                ),
            ],
        )
    }

    pub fn c_echo_rsp_from_req(
        ts: TSRef,
        req: &CommandMessage,
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
        Ok(CommandMessage::create(
            ts,
            vec![
                (&AffectedSOPClassUID, aff_sop_uid),
                (
                    &CommandField,
                    RawValue::of_ushort(u16::from(&CommandType::CEchoRsp)),
                ),
                (&MessageIDBeingRespondedTo, msg_id),
                (
                    &CommandDataSetType,
                    RawValue::of_ushort(COMMAND_DATASET_TYPE_NONE),
                ),
                (&Status, RawValue::from(status)),
            ],
        ))
    }

    pub fn c_find_req(ts: TSRef, msg_id: u16, sop_class_uid: &str) -> Self {
        CommandMessage::create(
            ts,
            vec![
                (
                    &AffectedSOPClassUID,
                    RawValue::Uid(sop_class_uid.to_string()),
                ),
                (
                    &CommandField,
                    RawValue::of_ushort(u16::from(&CommandType::CFindReq)),
                ),
                (&MessageID, RawValue::of_ushort(msg_id)),
                (&Priority, RawValue::from(&CommandPriority::Medium)),
                (
                    &CommandDataSetType,
                    RawValue::of_ushort(COMMAND_DATASET_TYPE_SOME),
                ),
            ],
        )
    }

    pub fn c_find_rsp_from_req(
        ts: TSRef,
        req: &CommandMessage,
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
        let dataset_type = if status == &CommandStatus::Success(0) {
            COMMAND_DATASET_TYPE_NONE
        } else {
            COMMAND_DATASET_TYPE_SOME
        };
        Ok(CommandMessage::create(
            ts,
            vec![
                (&AffectedSOPClassUID, aff_sop_uid),
                (
                    &CommandField,
                    RawValue::of_ushort(u16::from(&CommandType::CFindRsp)),
                ),
                (&MessageIDBeingRespondedTo, msg_id),
                (&CommandDataSetType, RawValue::of_ushort(dataset_type)),
                (&Status, RawValue::from(status)),
            ],
        ))
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
            transfer_syntaxes::ImplicitVRLittleEndian,
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

        let req = CommandMessage::c_echo_req(&ImplicitVRLittleEndian, exp_msg_id, exp_affected_sop);
        let rsp = CommandMessage::c_echo_rsp_from_req(&ImplicitVRLittleEndian, &req, &exp_status)
            .expect("build response");

        let mut elem_iter = rsp.message().iter_child_nodes();

        assert_eq_elem(
            &CommandGroupLength,
            RawValue::of_uint(u32::try_from(exp_bytes).expect("exp_bytes as u32")),
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
            RawValue::of_ushort(u16::from(&CommandType::CEchoRsp)),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(
            &MessageIDBeingRespondedTo,
            RawValue::of_ushort(exp_msg_id),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(
            &CommandDataSetType,
            RawValue::of_ushort(COMMAND_DATASET_TYPE_NONE),
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

        let req = CommandMessage::c_echo_req(&ImplicitVRLittleEndian, exp_msg_id, exp_affected_sop);

        let mut elem_iter = req.message().iter_child_nodes();

        assert_eq_elem(
            &CommandGroupLength,
            RawValue::of_uint(u32::try_from(exp_bytes).expect("exp_bytes as u32")),
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
            RawValue::of_ushort(u16::from(&CommandType::CEchoReq)),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(
            &MessageID,
            RawValue::of_ushort(exp_msg_id),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(
            &CommandDataSetType,
            RawValue::of_ushort(COMMAND_DATASET_TYPE_NONE),
            elem_iter.next(),
        );

        assert_eq!(exp_bytes, bytes);
    }
}
