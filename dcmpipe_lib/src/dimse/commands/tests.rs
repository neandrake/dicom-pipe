/*
   Copyright 2024 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use crate::dimse::commands::{CommandPriority, CommandStatus, CommandType};

/// Asserts that the given status's code roundtrips properly in `Status::try_from`.
fn assert_status_eq(status: CommandStatus) {
    match status {
        CommandStatus::Success(c) => {
            assert_eq!(status, CommandStatus::from(c))
        }
        CommandStatus::Warning(c) => {
            assert_eq!(status, CommandStatus::from(c))
        }
        CommandStatus::Failure(c) => {
            assert_eq!(status, CommandStatus::from(c))
        }
        CommandStatus::Cancel(c) => {
            assert_eq!(status, CommandStatus::from(c))
        }
        CommandStatus::Pending(c) => {
            assert_eq!(status, CommandStatus::from(c))
        }
        CommandStatus::INVALID(c) => {
            assert_eq!(status, CommandStatus::from(c))
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
        CommandType::from(u16::from(&CommandType::CStoreReq))
    );
    assert_eq!(
        CommandType::CStoreRsp,
        CommandType::from(u16::from(&CommandType::CStoreRsp))
    );
    assert_eq!(
        CommandType::CGetReq,
        CommandType::from(u16::from(&CommandType::CGetReq))
    );
    assert_eq!(
        CommandType::CGetRsp,
        CommandType::from(u16::from(&CommandType::CGetRsp))
    );
    assert_eq!(
        CommandType::CFindReq,
        CommandType::from(u16::from(&CommandType::CFindReq))
    );
    assert_eq!(
        CommandType::CFindRsp,
        CommandType::from(u16::from(&CommandType::CFindRsp))
    );
    assert_eq!(
        CommandType::CMoveReq,
        CommandType::from(u16::from(&CommandType::CMoveReq))
    );
    assert_eq!(
        CommandType::CMoveRsp,
        CommandType::from(u16::from(&CommandType::CMoveRsp))
    );
    assert_eq!(
        CommandType::CEchoReq,
        CommandType::from(u16::from(&CommandType::CEchoReq))
    );
    assert_eq!(
        CommandType::CEchoRsp,
        CommandType::from(u16::from(&CommandType::CEchoRsp))
    );

    assert_eq!(
        CommandType::NEventReportReq,
        CommandType::from(u16::from(&CommandType::NEventReportReq))
    );
    assert_eq!(
        CommandType::NEventReportRsp,
        CommandType::from(u16::from(&CommandType::NEventReportRsp))
    );
    assert_eq!(
        CommandType::NGetReq,
        CommandType::from(u16::from(&CommandType::NGetReq))
    );
    assert_eq!(
        CommandType::NGetRsp,
        CommandType::from(u16::from(&CommandType::NGetRsp))
    );
    assert_eq!(
        CommandType::NSetReq,
        CommandType::from(u16::from(&CommandType::NSetReq))
    );
    assert_eq!(
        CommandType::NSetRsp,
        CommandType::from(u16::from(&CommandType::NSetRsp))
    );
    assert_eq!(
        CommandType::NActionReq,
        CommandType::from(u16::from(&CommandType::NActionReq))
    );
    assert_eq!(
        CommandType::NActionRsp,
        CommandType::from(u16::from(&CommandType::NActionRsp))
    );
    assert_eq!(
        CommandType::NCreateReq,
        CommandType::from(u16::from(&CommandType::NCreateReq))
    );
    assert_eq!(
        CommandType::NCreateRsp,
        CommandType::from(u16::from(&CommandType::NCreateRsp))
    );
    assert_eq!(
        CommandType::NDeleteReq,
        CommandType::from(u16::from(&CommandType::NDeleteReq))
    );
    assert_eq!(
        CommandType::NDeleteRsp,
        CommandType::from(u16::from(&CommandType::NDeleteRsp))
    );

    assert_eq!(
        CommandType::CCancelReq,
        CommandType::from(u16::from(&CommandType::CCancelReq))
    );
}

#[test]
fn test_priority_roundtrip() {
    assert_eq!(
        CommandPriority::Low,
        CommandPriority::from(u16::from(&CommandPriority::Low))
    );
    assert_eq!(
        CommandPriority::Medium,
        CommandPriority::from(u16::from(&CommandPriority::Medium))
    );
    assert_eq!(
        CommandPriority::High,
        CommandPriority::from(u16::from(&CommandPriority::High))
    );
}
