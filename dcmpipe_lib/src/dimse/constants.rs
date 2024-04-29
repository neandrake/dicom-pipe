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
            (CommandField::CStoreReq as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CStoreRsp,
            (CommandField::CStoreRsp as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CGetReq,
            (CommandField::CGetReq as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CGetRsp,
            (CommandField::CGetRsp as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CFindReq,
            (CommandField::CFindReq as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CFindRsp,
            (CommandField::CFindRsp as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CMoveReq,
            (CommandField::CMoveReq as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CMoveRsp,
            (CommandField::CMoveRsp as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CEchoReq,
            (CommandField::CEchoReq as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::CEchoRsp,
            (CommandField::CEchoRsp as u32).try_into().unwrap()
        );

        assert_eq!(
            CommandField::NEventReportReq,
            (CommandField::NEventReportReq as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NEventReportRsp,
            (CommandField::NEventReportRsp as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NGetReq,
            (CommandField::NGetReq as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NGetRsp,
            (CommandField::NGetRsp as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NSetReq,
            (CommandField::NSetReq as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NSetRsp,
            (CommandField::NSetRsp as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NActionReq,
            (CommandField::NActionReq as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NActionRsp,
            (CommandField::NActionRsp as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NCreateReq,
            (CommandField::NCreateReq as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NCreateRsp,
            (CommandField::NCreateRsp as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NDeleteReq,
            (CommandField::NDeleteReq as u32).try_into().unwrap()
        );
        assert_eq!(
            CommandField::NDeleteRsp,
            (CommandField::NDeleteRsp as u32).try_into().unwrap()
        );

        assert_eq!(
            CommandField::CCancelReq,
            (CommandField::CCancelReq as u32).try_into().unwrap()
        );
    }

    #[test]
    fn test_priority_roundtrip() {
        assert_eq!(Priority::Low, (Priority::Low as u32).try_into().unwrap());
        assert_eq!(
            Priority::Medium,
            (Priority::Medium as u32).try_into().unwrap()
        );
        assert_eq!(Priority::High, (Priority::High as u32).try_into().unwrap());
    }
}
