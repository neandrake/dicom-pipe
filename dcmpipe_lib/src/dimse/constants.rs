//! Constants for DIMSE, DICOM Message Exchange

/// Values of the `CommandField` (0000,0100) field of messages.
#[repr(u32)]
pub enum CommandField {
    CStoreRq = 0x0001,
    CStoreRsp = 0x8001,
    CGetRq = 0x0010,
    CGetRsp = 0x8010,
    CFindRq = 0x0020,
    CFindRsp = 0x8020,
    CMoveRq = 0x0021,
    CMoveRsp = 0x8021,
    CEchoRq = 0x0030,
    CEchoRsp = 0x8030,

    NEventReportRq = 0x0100,
    NEventReportRsp = 0x8100,
    NGetRq = 0x0110,
    NGetRsp = 0x8110,
    NSetRq = 0x0120,
    NSetRsp = 0x8120,
    NActionRq = 0x0130,
    NActionRsp = 0x8130,
    NCreateRq = 0x0140,
    NCreateRsp = 0x8140,
    NDeleteRq = 0x0150,
    NDeleteRsp = 0x8150,

    CCancelRq = 0xFFFF,
}

impl TryFrom<u32> for CommandField {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x0001 => Ok(CommandField::CStoreRq),
            0x8001 => Ok(CommandField::CStoreRsp),
            0x0010 => Ok(CommandField::CGetRq),
            0x8010 => Ok(CommandField::CGetRsp),
            0x0020 => Ok(CommandField::CFindRq),
            0x8020 => Ok(CommandField::CFindRsp),
            0x0021 => Ok(CommandField::CMoveRq),
            0x8021 => Ok(CommandField::CMoveRsp),
            0x0030 => Ok(CommandField::CEchoRq),
            0x8030 => Ok(CommandField::CEchoRsp),

            0x0100 => Ok(CommandField::NEventReportRq),
            0x8100 => Ok(CommandField::NEventReportRsp),
            0x0110 => Ok(CommandField::NGetRq),
            0x8110 => Ok(CommandField::NGetRsp),
            0x0120 => Ok(CommandField::NSetRq),
            0x8120 => Ok(CommandField::NSetRsp),
            0x0130 => Ok(CommandField::NActionRq),
            0x8130 => Ok(CommandField::NActionRsp),
            0x0140 => Ok(CommandField::NCreateRq),
            0x8140 => Ok(CommandField::NCreateRsp),
            0x0150 => Ok(CommandField::NDeleteRq),
            0x8150 => Ok(CommandField::NDeleteRsp),

            0xFFFF => Ok(CommandField::CCancelRq),

            _ => Err(()),
        }
    }
}

/// Values of the `Priorty` (0000,0700) field of messages.
#[repr(u32)]
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
            _ => Err(())
        }
    }
}

/// Sentinel value of `CommandDataSetType` (0000,0800) to indicate that there is no Data Set
/// present in the message. Any other value in `CommandDataSetType` indicates a Data Set is present
/// in the message.
pub const COMMAND_DATASET_TYPE_NONE: u32 = 0x0101;

/// Values of the `Status` (0000,0900) field of messages.
pub enum Status {
    Success,
    Warning(u32),
    Failure(u32),
    Cancel,
    Pending(u32),
}

impl TryFrom<u32> for Status {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0x0000 {
            return Ok(Status::Success);
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
            return Ok(Status::Cancel);
        }
        if value == 0xFF00 || value == 0xFF01 {
            return Ok(Status::Pending(value));
        }
        Err(())
    }
}
