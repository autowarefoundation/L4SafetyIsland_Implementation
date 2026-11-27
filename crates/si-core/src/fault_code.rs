// Copyright 2026 The Autoware Foundation
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The `N8` `si_fault_manager` fault-code table of WP3
//! (WP0-WP3-SafetyFloor.md sections 1.2 and 4).
//!
//! The numbers are the `CODE_*` constants of
//! `autoware_safety_island_msgs/msg/FaultEvent.msg`, which is their source.
//! This table adds what the message does not carry: the reporting node and
//! the edge the event travels on. A host test
//! (`platforms/host/tests/fault_code_table.rs`) fails when the two disagree.

/// A fault code carried in `FaultEvent.code`, `FaultVerdict.fault_code` and
/// `SensorValidity.fault_code`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum FaultCode {
    /// No fault.
    None = 0,
    /// Three consecutive 100 Hz HPC heartbeats missed.
    HpcLivenessLost = 1,
    /// An HPC command or stream outside its contract (rate, latency).
    HpcCommandInvalid = 2,
    /// An end-to-end protection check failed on an HPC link frame.
    E2eVerifyFail = 3,
    /// The time base is not synchronised, so age checks cannot be trusted.
    TimeBaseInvalid = 4,
    /// The actuators do not follow the commanded values.
    ActuatorNotTracking = 5,
    /// The camera is lost: the island is blind (B4). Distinct from the later
    /// in-lane-stop code.
    SensingBlackout = 6,
    /// The fault signalling line of the system on chip reports an error.
    FsiSocError = 7,
}

/// Every fault code except [`FaultCode::None`], in code order.
pub const WP3_FAULT_CODES: [FaultCode; 7] = [
    FaultCode::HpcLivenessLost,
    FaultCode::HpcCommandInvalid,
    FaultCode::E2eVerifyFail,
    FaultCode::TimeBaseInvalid,
    FaultCode::ActuatorNotTracking,
    FaultCode::SensingBlackout,
    FaultCode::FsiSocError,
];

impl FaultCode {
    /// The number on the wire.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    /// The code with this number, or `None` for a number the table lacks.
    #[must_use]
    pub const fn from_u32(code: u32) -> Option<Self> {
        match code {
            0 => Some(Self::None),
            1 => Some(Self::HpcLivenessLost),
            2 => Some(Self::HpcCommandInvalid),
            3 => Some(Self::E2eVerifyFail),
            4 => Some(Self::TimeBaseInvalid),
            5 => Some(Self::ActuatorNotTracking),
            6 => Some(Self::SensingBlackout),
            7 => Some(Self::FsiSocError),
            _ => None,
        }
    }

    /// The name used in WP0-WP3-SafetyFloor.md, the constant in
    /// `FaultEvent.msg` without its `CODE_` prefix.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::HpcLivenessLost => "HPC_LIVENESS_LOST",
            Self::HpcCommandInvalid => "HPC_COMMAND_INVALID",
            Self::E2eVerifyFail => "E2E_VERIFY_FAIL",
            Self::TimeBaseInvalid => "TIME_BASE_INVALID",
            Self::ActuatorNotTracking => "ACTUATOR_NOT_TRACKING",
            Self::SensingBlackout => "SENSING_BLACKOUT",
            Self::FsiSocError => "FSI_SOC_ERROR",
        }
    }

    /// The design id of the node that reports the code to `N8`, or `FSI` for
    /// the hardware fault line.
    #[must_use]
    pub const fn reporter(self) -> &'static str {
        match self {
            Self::None => "",
            Self::HpcLivenessLost | Self::HpcCommandInvalid => "N2",
            Self::E2eVerifyFail | Self::TimeBaseInvalid => "N1",
            Self::ActuatorNotTracking => "N9",
            Self::SensingBlackout => "N17",
            Self::FsiSocError => "FSI",
        }
    }

    /// The edge of the message-flow catalogue the report travels on.
    /// `TIME_BASE_INVALID` has no edge of its own: `N12` tells `N1` (A3), and
    /// `N1` reports it on its only edge to `N8`, B15.
    #[must_use]
    pub const fn edge(self) -> &'static str {
        match self {
            Self::None => "",
            Self::HpcLivenessLost | Self::HpcCommandInvalid => "D3",
            Self::E2eVerifyFail | Self::TimeBaseInvalid => "B15",
            Self::ActuatorNotTracking => "D22",
            Self::SensingBlackout => "C20",
            Self::FsiSocError => "A12",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FaultCode, WP3_FAULT_CODES};

    #[test]
    fn numbers_round_trip() {
        for code in WP3_FAULT_CODES {
            assert_eq!(FaultCode::from_u32(code.as_u32()), Some(code));
        }
        assert_eq!(FaultCode::from_u32(0), Some(FaultCode::None));
        assert_eq!(FaultCode::from_u32(8), None);
    }

    #[test]
    fn table_is_in_code_order_without_gaps() {
        for (index, code) in WP3_FAULT_CODES.iter().enumerate() {
            assert_eq!(code.as_u32() as usize, index + 1);
        }
    }

    #[test]
    fn every_code_has_a_reporter_and_an_edge() {
        for code in WP3_FAULT_CODES {
            assert!(
                !code.reporter().is_empty(),
                "{} has no reporter",
                code.name()
            );
            assert!(!code.edge().is_empty(), "{} has no edge", code.name());
        }
    }
}
