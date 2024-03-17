//! Constants for DIMSE, DICOM Message Exchange

use crate::{
    core::{charset::DEFAULT_CHARACTER_SET, dcmobject::DicomRoot, defn::tag::Tag, RawValue},
    dict::{
        tags::{
            AffectedSOPClassUID, CommandDataSetType, CommandField, CommandGroupLength, MessageID,
            MessageIDBeingRespondedTo, Status,
        },
        transfer_syntaxes::ImplicitVRLittleEndian,
    },
};

use super::DimseError;

/// Values of the `CommandField` (0000,0100) field of messages.
///
/// See Part 7, Appendix E.
#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum CommandType {
    CStoreReq = 0x0001,
    CStoreRsp = 0x8001,
    CGetReq = 0x0010,
    CGetRsp = 0x8010,
    CFindReq = 0x0020,
    CFindRsp = 0x8020,
    CMoveReq = 0x0021,
    CMoveRsp = 0x8021,
    CEchoReq = 0x0030,
    CEchoRsp = 0x8030,

    NEventReportReq = 0x0100,
    NEventReportRsp = 0x8100,
    NGetReq = 0x0110,
    NGetRsp = 0x8110,
    NSetReq = 0x0120,
    NSetRsp = 0x8120,
    NActionReq = 0x0130,
    NActionRsp = 0x8130,
    NCreateReq = 0x0140,
    NCreateRsp = 0x8140,
    NDeleteReq = 0x0150,
    NDeleteRsp = 0x8150,

    CCancelReq = 0xFFFF,
}

impl From<CommandType> for u16 {
    fn from(value: CommandType) -> Self {
        value as Self
    }
}

impl TryFrom<u16> for CommandType {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            // oof, can't have match arm with `(CommandField::CStoreRq as u32) =>`
            0x0001 => Ok(CommandType::CStoreReq),
            0x8001 => Ok(CommandType::CStoreRsp),
            0x0010 => Ok(CommandType::CGetReq),
            0x8010 => Ok(CommandType::CGetRsp),
            0x0020 => Ok(CommandType::CFindReq),
            0x8020 => Ok(CommandType::CFindRsp),
            0x0021 => Ok(CommandType::CMoveReq),
            0x8021 => Ok(CommandType::CMoveRsp),
            0x0030 => Ok(CommandType::CEchoReq),
            0x8030 => Ok(CommandType::CEchoRsp),

            0x0100 => Ok(CommandType::NEventReportReq),
            0x8100 => Ok(CommandType::NEventReportRsp),
            0x0110 => Ok(CommandType::NGetReq),
            0x8110 => Ok(CommandType::NGetRsp),
            0x0120 => Ok(CommandType::NSetReq),
            0x8120 => Ok(CommandType::NSetRsp),
            0x0130 => Ok(CommandType::NActionReq),
            0x8130 => Ok(CommandType::NActionRsp),
            0x0140 => Ok(CommandType::NCreateReq),
            0x8140 => Ok(CommandType::NCreateRsp),
            0x0150 => Ok(CommandType::NDeleteReq),
            0x8150 => Ok(CommandType::NDeleteRsp),

            0xFFFF => Ok(CommandType::CCancelReq),

            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    CStoreReq,
    CStoreRsp,
    CGetReq,
    CGetRsp,
    CFindReq,
    CFindRsp,
    CMoveReq,
    CMoveRsp,
    CEchoReq,
    CEchoRsp,

    NEventReportReq,
    NEventReportRsp,
    NGetReq,
    NGetRsp,
    NSetReq,
    NSetRsp,
    NActionReq,
    NActionRsp,
    NCreateReq,
    NCreateRsp,
    NDeleteReq,
    NDeleteRsp,

    CCancelReq,
}

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
                RawValue::ushort(CommandType::CEchoReq as u16),
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
                RawValue::ushort(CommandType::CEchoRsp as u16),
            ),
            (&MessageIDBeingRespondedTo, msg_id),
            (
                &CommandDataSetType,
                RawValue::ushort(COMMAND_DATASET_TYPE_NONE),
            ),
            (&Status, status.into()),
        ]))
    }

    pub fn message(&self) -> &DicomRoot {
        &self.message
    }
}

/// Values of the `Priority` (0000,0700) field of messages.
///
/// See Part 7, Appendix E.
#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum CommandPriority {
    Low = 0x0002,
    Medium = 0x0000,
    High = 0x0001,
}

impl From<CommandPriority> for u16 {
    fn from(value: CommandPriority) -> Self {
        value as Self
    }
}

impl From<CommandPriority> for RawValue {
    fn from(value: CommandPriority) -> Self {
        RawValue::ushort(value as u16)
    }
}

impl TryFrom<u16> for CommandPriority {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0002 => Ok(CommandPriority::Low),
            0x0000 => Ok(CommandPriority::Medium),
            0x0001 => Ok(CommandPriority::High),
            _ => Err(()),
        }
    }
}

/// Sentinel value of `CommandDataSetType` (0000,0800) to indicate that there is no Data Set
/// present in the message. Any other value in `CommandDataSetType` indicates a Data Set is present
/// in the message.
///
/// See Part 7, Appendix E.
pub const COMMAND_DATASET_TYPE_NONE: u16 = 0x0101;

/// Values of the `Status` (0000,0900) field of messages.
///
/// See Part 7, Appendix C.
#[derive(Debug, PartialEq, Eq)]
pub enum CommandStatus {
    Success(u16),
    Warning(u16),
    Failure(u16),
    Cancel(u16),
    Pending(u16),
}

impl From<CommandStatus> for RawValue {
    fn from(value: CommandStatus) -> Self {
        match value {
            CommandStatus::Success(c) => RawValue::ushort(c),
            CommandStatus::Warning(c) => RawValue::ushort(c),
            CommandStatus::Failure(c) => RawValue::ushort(c),
            CommandStatus::Cancel(c) => RawValue::ushort(c),
            CommandStatus::Pending(c) => RawValue::ushort(c),
        }
    }
}

impl TryFrom<u16> for CommandStatus {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0x0000 {
            return Ok(CommandStatus::Success(value));
        }

        if value == 0x0001 || value == 0x0107 || value == 0x0116 || value >> 12 == 0x000B {
            return Ok(CommandStatus::Warning(value));
        }
        if value >> 12 == 0x000A
            || value >> 12 == 0x000C
            || value >> 8 == 0x0001
            || value >> 8 == 0x0002
        {
            return Ok(CommandStatus::Failure(value));
        }
        if value == 0xFE00 {
            return Ok(CommandStatus::Cancel(value));
        }
        if value == 0xFF00 || value == 0xFF01 {
            return Ok(CommandStatus::Pending(value));
        }
        Err(())
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
        dimse::commands::COMMAND_DATASET_TYPE_NONE,
    };

    use super::{CommandMessage, CommandPriority, CommandStatus, CommandType};

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
            RawValue::ushort(CommandType::CEchoReq as u16),
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

    /// Asserts that the given status's code roundtrips properly in `Status::try_from`.
    fn assert_status_eq(status: CommandStatus) {
        match status {
            CommandStatus::Success(c) => {
                assert_eq!(status, CommandStatus::try_from(c).expect("success"))
            }
            CommandStatus::Warning(c) => {
                assert_eq!(status, CommandStatus::try_from(c).expect("warning"))
            }
            CommandStatus::Failure(c) => {
                assert_eq!(status, CommandStatus::try_from(c).expect("failure"))
            }
            CommandStatus::Cancel(c) => {
                assert_eq!(status, CommandStatus::try_from(c).expect("cancel"))
            }
            CommandStatus::Pending(c) => {
                assert_eq!(status, CommandStatus::try_from(c).expect("pending"))
            }
        }
    }

    /// Chapter 7, Section 9.1, under each C-Operation's Parameters section there is a Status
    /// section which defines the statuses acceptable for the operation. The status codes and
    /// meanings are shared across the operations. This test's that a status's code is interpreted
    /// again as the right status.
    #[test]
    fn test_c_service_status_roundtrip() {
        // Success
        assert_status_eq(CommandStatus::Success(0x0000));

        // Refused: SOP Class not supported
        assert_status_eq(CommandStatus::Failure(0x0122));

        // Duplicate invocation
        assert_status_eq(CommandStatus::Failure(0x0210));

        // Invalid SOP Instance
        assert_status_eq(CommandStatus::Failure(0x0117));

        // Mistyped argument
        assert_status_eq(CommandStatus::Failure(0x0212));

        // Unrecognized operation
        assert_status_eq(CommandStatus::Failure(0x0211));

        // Refused: Not authorized
        assert_status_eq(CommandStatus::Failure(0x0124));

        // Cancel
        assert_status_eq(CommandStatus::Cancel(0xFE00));
    }

    #[test]
    fn test_command_field_roundtrip() {
        assert_eq!(
            CommandType::CStoreReq,
            (u16::from(CommandType::CStoreReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::CStoreRsp,
            (u16::from(CommandType::CStoreRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::CGetReq,
            (u16::from(CommandType::CGetReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::CGetRsp,
            (u16::from(CommandType::CGetRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::CFindReq,
            (u16::from(CommandType::CFindReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::CFindRsp,
            (u16::from(CommandType::CFindRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::CMoveReq,
            (u16::from(CommandType::CMoveReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::CMoveRsp,
            (u16::from(CommandType::CMoveRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::CEchoReq,
            (u16::from(CommandType::CEchoReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::CEchoRsp,
            (u16::from(CommandType::CEchoRsp)).try_into().unwrap()
        );

        assert_eq!(
            CommandType::NEventReportReq,
            (u16::from(CommandType::NEventReportReq))
                .try_into()
                .unwrap()
        );
        assert_eq!(
            CommandType::NEventReportRsp,
            (u16::from(CommandType::NEventReportRsp))
                .try_into()
                .unwrap()
        );
        assert_eq!(
            CommandType::NGetReq,
            (u16::from(CommandType::NGetReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::NGetRsp,
            (u16::from(CommandType::NGetRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::NSetReq,
            (u16::from(CommandType::NSetReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::NSetRsp,
            (u16::from(CommandType::NSetRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::NActionReq,
            (u16::from(CommandType::NActionReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::NActionRsp,
            (u16::from(CommandType::NActionRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::NCreateReq,
            (u16::from(CommandType::NCreateReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::NCreateRsp,
            (u16::from(CommandType::NCreateRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::NDeleteReq,
            (u16::from(CommandType::NDeleteReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandType::NDeleteRsp,
            (u16::from(CommandType::NDeleteRsp)).try_into().unwrap()
        );

        assert_eq!(
            CommandType::CCancelReq,
            (u16::from(CommandType::CCancelReq)).try_into().unwrap()
        );
    }

    #[test]
    fn test_priority_roundtrip() {
        assert_eq!(
            CommandPriority::Low,
            (u16::from(CommandPriority::Low)).try_into().unwrap()
        );
        assert_eq!(
            CommandPriority::Medium,
            (u16::from(CommandPriority::Medium)).try_into().unwrap()
        );
        assert_eq!(
            CommandPriority::High,
            (u16::from(CommandPriority::High)).try_into().unwrap()
        );
    }
}
