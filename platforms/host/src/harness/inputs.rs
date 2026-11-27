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

//! The inputs of the WP2 nodes (WP0-WP3-SafetyFloor.md section 3), so that a
//! node test scripts each one with a [`super::Publisher`] at its rate, a
//! [`super::Script`] for an event input, or the [`super::HpcStub`].
//!
//! Rates are those of the message-flow catalogue's edge rows. Where the
//! catalogue states none, the node's contract states it, and the test picks
//! the rate it exercises.

use core::time::Duration;

/// How an input arrives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rate {
    /// Periodically, at the period the edge row states.
    Periodic(Duration),
    /// On change or on event; scripted with a [`super::Script`].
    OnEvent,
    /// Periodically, at a rate the edge row leaves to the node's contract.
    FromContract,
}

/// One input of one WP2 node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputSpec {
    /// The consuming node's design id.
    pub node: &'static str,
    /// What the input carries.
    pub input: &'static str,
    /// The edge of the message-flow catalogue, or the badge for a shared
    /// stream.
    pub edge: &'static str,
    /// How it arrives.
    pub rate: Rate,
    /// True when the [`super::HpcStub`] produces it.
    pub from_hpc_stub: bool,
}

const fn periodic_ms(ms: u64) -> Rate {
    Rate::Periodic(Duration::from_millis(ms))
}

/// Every input of the nodes of WP0-WP3-SafetyFloor.md section 3, in the
/// section's order.
pub const SECTION_3_INPUTS: [InputSpec; 16] = [
    InputSpec {
        node: "I16",
        input: "selected command from I1",
        edge: "F3",
        rate: Rate::OnEvent,
        from_hpc_stub: false,
    },
    InputSpec {
        node: "I16",
        input: "actuator feedback over CAN FD",
        edge: "A13",
        rate: periodic_ms(10),
        from_hpc_stub: false,
    },
    InputSpec {
        node: "N12",
        input: "gPTP sync state",
        edge: "A2",
        rate: Rate::FromContract,
        from_hpc_stub: false,
    },
    InputSpec {
        node: "N17",
        input: "camera frames",
        edge: "A10",
        rate: Rate::FromContract,
        from_hpc_stub: false,
    },
    InputSpec {
        node: "N1",
        input: "section 8.1 signals from H1",
        edge: "A1",
        rate: Rate::FromContract,
        from_hpc_stub: true,
    },
    InputSpec {
        node: "N1",
        input: "time-base validity from N12",
        edge: "A3",
        rate: periodic_ms(1_000),
        from_hpc_stub: false,
    },
    InputSpec {
        node: "N9",
        input: "achieved values from I16",
        edge: "F5",
        rate: periodic_ms(10),
        from_hpc_stub: false,
    },
    InputSpec {
        node: "N9",
        input: "commanded values from I1",
        edge: "F6",
        rate: periodic_ms(10),
        from_hpc_stub: false,
    },
    InputSpec {
        node: "I5",
        input: "vehicle status",
        edge: "V",
        rate: Rate::FromContract,
        from_hpc_stub: false,
    },
    InputSpec {
        node: "N2",
        input: "HpcHeartbeat via N1",
        edge: "B7",
        rate: periodic_ms(10),
        from_hpc_stub: true,
    },
    InputSpec {
        node: "N2",
        input: "HpcCapability via N1",
        edge: "B7",
        rate: periodic_ms(100),
        from_hpc_stub: true,
    },
    InputSpec {
        node: "N2",
        input: "per-topic arrival times via N1",
        edge: "B7",
        rate: Rate::FromContract,
        from_hpc_stub: true,
    },
    InputSpec {
        node: "N2",
        input: "vehicle status",
        edge: "V",
        rate: Rate::FromContract,
        from_hpc_stub: false,
    },
    InputSpec {
        node: "N4",
        input: "wheel speed, steering, yaw rate, acceleration via I16",
        edge: "A4",
        rate: periodic_ms(10),
        from_hpc_stub: false,
    },
    InputSpec {
        node: "N4",
        input: "island IMU and GNSS",
        edge: "A4",
        rate: periodic_ms(10),
        from_hpc_stub: false,
    },
    InputSpec {
        node: "N4",
        input: "HPC reference odometry via N1",
        edge: "B9",
        rate: periodic_ms(20),
        from_hpc_stub: true,
    },
];

/// The inputs of `node` (a design id such as `N2`).
pub fn inputs_of(node: &str) -> impl Iterator<Item = &'static InputSpec> {
    SECTION_3_INPUTS
        .iter()
        .filter(move |spec| spec.node == node)
}

#[cfg(test)]
mod tests {
    use super::{Rate, SECTION_3_INPUTS, inputs_of};
    use crate::harness::HpcTopic;

    #[test]
    fn every_section_3_node_has_inputs() {
        for node in ["I16", "N12", "N17", "N1", "N9", "I5", "N2", "N4"] {
            assert!(inputs_of(node).count() > 0, "{node} has no inputs");
        }
    }

    fn rate_of(node: &str, input: &str) -> Option<Rate> {
        inputs_of(node)
            .find(|spec| spec.input == input)
            .map(|spec| spec.rate)
    }

    #[test]
    fn hpc_stub_rows_match_the_stub_rates() {
        assert_eq!(
            rate_of("N2", "HpcHeartbeat via N1"),
            Some(Rate::Periodic(HpcTopic::Heartbeat.period()))
        );
        assert_eq!(
            rate_of("N2", "HpcCapability via N1"),
            Some(Rate::Periodic(HpcTopic::Capability.period()))
        );
        assert_eq!(
            rate_of("N4", "HPC reference odometry via N1"),
            Some(Rate::Periodic(HpcTopic::KinematicState.period()))
        );
    }

    #[test]
    fn every_row_names_an_edge() {
        assert!(SECTION_3_INPUTS.iter().all(|spec| !spec.edge.is_empty()));
    }
}
