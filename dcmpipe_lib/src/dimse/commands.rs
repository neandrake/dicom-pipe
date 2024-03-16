//! Constants for DIMSE, DICOM Message Exchange

use crate::{
    core::{charset::DEFAULT_CHARACTER_SET, dcmobject::DicomRoot, RawValue},
    dict::{tags, transfer_syntaxes as ts},
};

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

pub struct CEchoReq {
    message: DicomRoot,
}

impl CEchoReq {
    pub fn new(msg_id: u16) -> Self {
        let mut message = DicomRoot::new_empty(&ts::ImplicitVRLittleEndian, DEFAULT_CHARACTER_SET);

        // Add CommandGroupLength with a value of zero that will be updated after computing the
        // number of bytes between the end of CommandGroupLength to the end of CommandDataSetType.
        let _ = message
            .create_and_add(&tags::CommandGroupLength)
            .element_mut()
            .encode_val(RawValue::uint(0));

        // Get the number of bytes that the `DicomRoot` consists of to this point.
        let bytes_to_cmdgrp: u32 = u32::try_from(message.byte_size()).unwrap_or_default();

        let _ = message
            .create_and_add(&tags::AffectedSOPClassUID)
            .element_mut()
            .encode_val(RawValue::empty());
        let _ = message
            .create_and_add(&tags::CommandField)
            .element_mut()
            .encode_val(RawValue::ushort(CommandType::CEchoReq as u16));
        let _ = message
            .create_and_add(&tags::MessageID)
            .element_mut()
            .encode_val(RawValue::ushort(msg_id));
        let _ = message
            .create_and_add(&tags::CommandDataSetType)
            .element_mut()
            .encode_val(RawValue::ushort(COMMAND_DATASET_TYPE_NONE));

        // Get the number of bytes now that the required contents of this message have been added
        // and values encoded. Subtract the number of bytes calculated prior to these required
        // contents in order to determine the number of bytes they consist of.
        let cmd_grp_len_val: u32 =
            u32::try_from(message.byte_size()).unwrap_or_default() - bytes_to_cmdgrp;
        if let Some(ref mut cmd_grp) = message.get_child_by_tag_mut(tags::CommandGroupLength.tag())
        {
            let _ = cmd_grp
                .element_mut()
                .encode_val(RawValue::uint(cmd_grp_len_val));
        }

        Self { message }
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
pub enum Priority {
    Low = 0x0002,
    Medium = 0x0000,
    High = 0x0001,
}

impl From<Priority> for u16 {
    fn from(value: Priority) -> Self {
        value as Self
    }
}

impl TryFrom<u16> for Priority {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0002 => Ok(Priority::Low),
            0x0000 => Ok(Priority::Medium),
            0x0001 => Ok(Priority::High),
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
pub enum Status {
    Success(u16),
    Warning(u16),
    Failure(u16),
    Cancel(u16),
    Pending(u16),
}

impl TryFrom<u16> for Status {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0x0000 {
            return Ok(Status::Success(value));
        }

        if value == 0x0001 || value == 0x0107 || value == 0x0116 || value >> 12 == 0x000B {
            return Ok(Status::Warning(value));
        }
        if value >> 12 == 0x000A
            || value >> 12 == 0x000C
            || value >> 8 == 0x0001
            || value >> 8 == 0x0002
        {
            return Ok(Status::Failure(value));
        }
        if value == 0xFE00 {
            return Ok(Status::Cancel(value));
        }
        if value == 0xFF00 || value == 0xFF01 {
            return Ok(Status::Pending(value));
        }
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::{CommandType, Priority, Status};

    /// Asserts that the given status's code roundtrips properly in `Status::try_from`.
    fn assert_status_eq(status: Status) {
        match status {
            Status::Success(c) => assert_eq!(status, Status::try_from(c).expect("success")),
            Status::Warning(c) => assert_eq!(status, Status::try_from(c).expect("warning")),
            Status::Failure(c) => assert_eq!(status, Status::try_from(c).expect("failure")),
            Status::Cancel(c) => assert_eq!(status, Status::try_from(c).expect("cancel")),
            Status::Pending(c) => assert_eq!(status, Status::try_from(c).expect("pending")),
        }
    }

    /// Chapter 7, Section 9.1, under each C-Operation's Parameters section there is a Status
    /// section which defines the statuses acceptable for the operation. The status codes and
    /// meanings are shared across the operations. This test's that a status's code is interpreted
    /// again as the right status.
    #[test]
    fn test_c_service_status_roundtrip() {
        // Success
        assert_status_eq(Status::Success(0x0000));

        // Refused: SOP Class not supported
        assert_status_eq(Status::Failure(0x0122));

        // Duplicate invocation
        assert_status_eq(Status::Failure(0x0210));

        // Invalid SOP Instance
        assert_status_eq(Status::Failure(0x0117));

        // Mistyped argument
        assert_status_eq(Status::Failure(0x0212));

        // Unrecognized operation
        assert_status_eq(Status::Failure(0x0211));

        // Refused: Not authorized
        assert_status_eq(Status::Failure(0x0124));

        // Cancel
        assert_status_eq(Status::Cancel(0xFE00));
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
            Priority::Low,
            (u16::from(Priority::Low)).try_into().unwrap()
        );
        assert_eq!(
            Priority::Medium,
            (u16::from(Priority::Medium)).try_into().unwrap()
        );
        assert_eq!(
            Priority::High,
            (u16::from(Priority::High)).try_into().unwrap()
        );
    }
}
