//! Constants for DIMSE, DICOM Message Exchangy

use crate::core::RawValue;

pub mod messages;

#[cfg(test)]
mod tests;

/// Values of the `CommandField` (0000,0100) field of messages.
///
/// See Part 7, Appendix E.
#[derive(Debug, PartialEq, Eq)]
pub enum CommandType {
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

    INVALID(u16),
}

impl From<CommandType> for u16 {
    fn from(value: CommandType) -> Self {
        match value {
            CommandType::CStoreReq => 0x0001,
            CommandType::CStoreRsp => 0x8001,
            CommandType::CGetReq => 0x0010,
            CommandType::CGetRsp => 0x8010,
            CommandType::CFindReq => 0x0020,
            CommandType::CFindRsp => 0x8020,
            CommandType::CMoveReq => 0x0021,
            CommandType::CMoveRsp => 0x8021,
            CommandType::CEchoReq => 0x0030,
            CommandType::CEchoRsp => 0x8030,

            CommandType::NEventReportReq => 0x0100,
            CommandType::NEventReportRsp => 0x8100,
            CommandType::NGetReq => 0x0110,
            CommandType::NGetRsp => 0x8110,
            CommandType::NSetReq => 0x0120,
            CommandType::NSetRsp => 0x8120,
            CommandType::NActionReq => 0x0130,
            CommandType::NActionRsp => 0x8130,
            CommandType::NCreateReq => 0x0140,
            CommandType::NCreateRsp => 0x8140,
            CommandType::NDeleteReq => 0x0150,
            CommandType::NDeleteRsp => 0x8150,

            CommandType::CCancelReq => 0xFFFF,

            CommandType::INVALID(c) => c,
        }
    }
}

impl From<u16> for CommandType {
    fn from(value: u16) -> Self {
        match value {
            0x0001 => CommandType::CStoreReq,
            0x8001 => CommandType::CStoreRsp,
            0x0010 => CommandType::CGetReq,
            0x8010 => CommandType::CGetRsp,
            0x0020 => CommandType::CFindReq,
            0x8020 => CommandType::CFindRsp,
            0x0021 => CommandType::CMoveReq,
            0x8021 => CommandType::CMoveRsp,
            0x0030 => CommandType::CEchoReq,
            0x8030 => CommandType::CEchoRsp,

            0x0100 => CommandType::NEventReportReq,
            0x8100 => CommandType::NEventReportRsp,
            0x0110 => CommandType::NGetReq,
            0x8110 => CommandType::NGetRsp,
            0x0120 => CommandType::NSetReq,
            0x8120 => CommandType::NSetRsp,
            0x0130 => CommandType::NActionReq,
            0x8130 => CommandType::NActionRsp,
            0x0140 => CommandType::NCreateReq,
            0x8140 => CommandType::NCreateRsp,
            0x0150 => CommandType::NDeleteReq,
            0x8150 => CommandType::NDeleteRsp,

            0xFFFF => CommandType::CCancelReq,

            c => CommandType::INVALID(c),
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

/// Values of the `Priority` (0000,0700) field of messages.
///
/// See Part 7, Appendix E.
#[derive(Debug, PartialEq, Eq)]
pub enum CommandPriority {
    Low,
    Medium,
    High,
    INVALID(u16),
}

impl From<CommandPriority> for u16 {
    fn from(value: CommandPriority) -> Self {
        match value {
            CommandPriority::Low => 0x0002,
            CommandPriority::Medium => 0x0000,
            CommandPriority::High => 0x0001,
            CommandPriority::INVALID(c) => c,
        }
    }
}

impl From<CommandPriority> for RawValue {
    fn from(value: CommandPriority) -> Self {
        RawValue::ushort(u16::from(value))
    }
}

impl From<u16> for CommandPriority {
    fn from(value: u16) -> Self {
        match value {
            0x0002 => CommandPriority::Low,
            0x0000 => CommandPriority::Medium,
            0x0001 => CommandPriority::High,
            c => CommandPriority::INVALID(c),
        }
    }
}

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
    INVALID(u16),
}

impl From<CommandStatus> for RawValue {
    fn from(value: CommandStatus) -> Self {
        match value {
            CommandStatus::Success(c) => RawValue::ushort(c),
            CommandStatus::Warning(c) => RawValue::ushort(c),
            CommandStatus::Failure(c) => RawValue::ushort(c),
            CommandStatus::Cancel(c) => RawValue::ushort(c),
            CommandStatus::Pending(c) => RawValue::ushort(c),
            CommandStatus::INVALID(c) => RawValue::ushort(c),
        }
    }
}

impl From<u16> for CommandStatus {
    fn from(value: u16) -> Self {
        if value == 0x0000 {
            return CommandStatus::Success(value);
        }

        if value == 0x0001 || value == 0x0107 || value == 0x0116 || value >> 12 == 0x000B {
            return CommandStatus::Warning(value);
        }
        if value >> 12 == 0x000A
            || value >> 12 == 0x000C
            || value >> 8 == 0x0001
            || value >> 8 == 0x0002
        {
            return CommandStatus::Failure(value);
        }
        if value == 0xFE00 {
            return CommandStatus::Cancel(value);
        }
        if value == 0xFF00 || value == 0xFF01 {
            return CommandStatus::Pending(value);
        }

        CommandStatus::INVALID(value)
    }
}
