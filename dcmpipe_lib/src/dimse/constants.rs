//! Constants for DIMSE, DICOM Message Exchange

/// Values of the `CommandField` (0000,0100) field of messages.
///
/// See Part 7, Appendix E.
#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub enum CommandField {
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

impl From<CommandField> for u32 {
    fn from(value: CommandField) -> Self {
        value as Self
    }
}

impl TryFrom<u32> for CommandField {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            // oof, can't have match arm with `(CommandField::CStoreRq as u32) =>`
            0x0001 => Ok(CommandField::CStoreReq),
            0x8001 => Ok(CommandField::CStoreRsp),
            0x0010 => Ok(CommandField::CGetReq),
            0x8010 => Ok(CommandField::CGetRsp),
            0x0020 => Ok(CommandField::CFindReq),
            0x8020 => Ok(CommandField::CFindRsp),
            0x0021 => Ok(CommandField::CMoveReq),
            0x8021 => Ok(CommandField::CMoveRsp),
            0x0030 => Ok(CommandField::CEchoReq),
            0x8030 => Ok(CommandField::CEchoRsp),

            0x0100 => Ok(CommandField::NEventReportReq),
            0x8100 => Ok(CommandField::NEventReportRsp),
            0x0110 => Ok(CommandField::NGetReq),
            0x8110 => Ok(CommandField::NGetRsp),
            0x0120 => Ok(CommandField::NSetReq),
            0x8120 => Ok(CommandField::NSetRsp),
            0x0130 => Ok(CommandField::NActionReq),
            0x8130 => Ok(CommandField::NActionRsp),
            0x0140 => Ok(CommandField::NCreateReq),
            0x8140 => Ok(CommandField::NCreateRsp),
            0x0150 => Ok(CommandField::NDeleteReq),
            0x8150 => Ok(CommandField::NDeleteRsp),

            0xFFFF => Ok(CommandField::CCancelReq),

            _ => Err(()),
        }
    }
}

/// Values of the `Priority` (0000,0700) field of messages.
///
/// See Part 7, Appendix E.
#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub enum Priority {
    Low = 0x0002,
    Medium = 0x0000,
    High = 0x0001,
}

impl From<Priority> for u32 {
    fn from(value: Priority) -> Self {
        value as Self
    }
}

impl TryFrom<u32> for Priority {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
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
pub const COMMAND_DATASET_TYPE_NONE: u32 = 0x0101;

/// Values of the `Status` (0000,0900) field of messages.
///
/// See Part 7, Appendix C.
#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Success(u32),
    Warning(u32),
    Failure(u32),
    Cancel(u32),
    Pending(u32),
}

impl TryFrom<u32> for Status {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
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
    use super::{CommandField, Priority};

    #[test]
    fn test_command_field_roundtrip() {
        assert_eq!(
            CommandField::CStoreReq,
            (u32::from(CommandField::CStoreReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CStoreRsp,
            (u32::from(CommandField::CStoreRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CGetReq,
            (u32::from(CommandField::CGetReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CGetRsp,
            (u32::from(CommandField::CGetRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CFindReq,
            (u32::from(CommandField::CFindReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CFindRsp,
            (u32::from(CommandField::CFindRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CMoveReq,
            (u32::from(CommandField::CMoveReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CMoveRsp,
            (u32::from(CommandField::CMoveRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CEchoReq,
            (u32::from(CommandField::CEchoReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CEchoRsp,
            (u32::from(CommandField::CEchoRsp)).try_into().unwrap()
        );

        assert_eq!(
            CommandField::NEventReportReq,
            (u32::from(CommandField::NEventReportReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NEventReportRsp,
            (u32::from(CommandField::NEventReportRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NGetReq,
            (u32::from(CommandField::NGetReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NGetRsp,
            (u32::from(CommandField::NGetRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NSetReq,
            (u32::from(CommandField::NSetReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NSetRsp,
            (u32::from(CommandField::NSetRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NActionReq,
            (u32::from(CommandField::NActionReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NActionRsp,
            (u32::from(CommandField::NActionRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NCreateReq,
            (u32::from(CommandField::NCreateReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NCreateRsp,
            (u32::from(CommandField::NCreateRsp)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NDeleteReq,
            (u32::from(CommandField::NDeleteReq)).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NDeleteRsp,
            (u32::from(CommandField::NDeleteRsp)).try_into().unwrap()
        );

        assert_eq!(
            CommandField::CCancelReq,
            (u32::from(CommandField::CCancelReq)).try_into().unwrap()
        );
    }

    #[test]
    fn test_priority_roundtrip() {
        assert_eq!(Priority::Low, (u32::from(Priority::Low)).try_into().unwrap());
        assert_eq!(
            Priority::Medium,
            (u32::from(Priority::Medium)).try_into().unwrap()
        );
        assert_eq!(Priority::High, (u32::from(Priority::High)).try_into().unwrap());
    }
}
