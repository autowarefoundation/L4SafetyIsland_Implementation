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

//! The HPC stub: the HPC-to-island traffic of `H1`, `H2`, `H5` and `H7`
//! (design V1.0 section 8.1, PLAN-S32Z2.md section 3.3) at its design rates,
//! with faults injectable per topic or for the whole link.
//!
//! The stub models the frames' timing and end-to-end state, not their
//! content: a frame's payload is `()`. A test that needs content (a
//! self-test result, a trajectory) builds it from the frame's counter and
//! stamp, or scripts it with a [`super::Script`].

use core::time::Duration;

use si_hal::Instant;

use super::{Delivery, Fault, Publisher};

/// End-to-end protection profile of a link row (design V1.0 section 8.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E2eProfile {
    /// SI-E2E/A.
    A,
    /// SI-E2E/B.
    B,
}

/// A row of design V1.0 section 8.1 that `H1`, `H2`, `H5` or `H7` produces.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HpcTopic {
    /// `HpcHeartbeat` from `H7`.
    Heartbeat,
    /// `HpcCapability` from `H2`.
    Capability,
    /// The bounded `Trajectory` from `H5`.
    Trajectory,
    /// Reference `Odometry`, forwarded by `H1`.
    KinematicState,
    /// Reference `AccelWithCovarianceStamped`, forwarded by `H1`.
    Acceleration,
    /// `OperationModeState`, forwarded by `H1`.
    OperationModeState,
    /// Gear command, forwarded by `H1`.
    GearCommand,
    /// Turn indicators command, forwarded by `H1`.
    TurnIndicatorsCommand,
    /// Hazard lights command, forwarded by `H1`.
    HazardLightsCommand,
}

impl HpcTopic {
    /// Every topic the stub publishes.
    pub const ALL: [Self; 9] = [
        Self::Heartbeat,
        Self::Capability,
        Self::Trajectory,
        Self::KinematicState,
        Self::Acceleration,
        Self::OperationModeState,
        Self::GearCommand,
        Self::TurnIndicatorsCommand,
        Self::HazardLightsCommand,
    ];

    /// The HPC node producing the topic. Every frame leaves the HPC through
    /// `H1` `si_bridge_hpc`, the only HPC writer to the island.
    #[must_use]
    pub const fn producer(self) -> &'static str {
        match self {
            Self::Heartbeat => "H7",
            Self::Capability => "H2",
            Self::Trajectory => "H5",
            Self::KinematicState
            | Self::Acceleration
            | Self::OperationModeState
            | Self::GearCommand
            | Self::TurnIndicatorsCommand
            | Self::HazardLightsCommand => "H1",
        }
    }

    /// The topic name of section 8.1.
    #[must_use]
    pub const fn topic(self) -> &'static str {
        match self {
            Self::Heartbeat => "/si/in/heartbeat",
            Self::Capability => "/si/in/capability",
            Self::Trajectory => "/planning/scenario_planning/trajectory",
            Self::KinematicState => "/localization/kinematic_state",
            Self::Acceleration => "/localization/acceleration",
            Self::OperationModeState => "/api/operation_mode/state",
            Self::GearCommand => "/control/command/gear_cmd",
            Self::TurnIndicatorsCommand => "/control/command/turn_indicators_cmd",
            Self::HazardLightsCommand => "/control/command/hazard_lights_cmd",
        }
    }

    /// The publication period of section 8.1.
    #[must_use]
    pub const fn period(self) -> Duration {
        match self {
            Self::Heartbeat => Duration::from_millis(10),
            Self::KinematicState | Self::Acceleration => Duration::from_millis(20),
            Self::Capability
            | Self::Trajectory
            | Self::OperationModeState
            | Self::GearCommand
            | Self::TurnIndicatorsCommand
            | Self::HazardLightsCommand => Duration::from_millis(100),
        }
    }

    /// The age limit of section 8.1.
    #[must_use]
    pub const fn age_limit(self) -> Duration {
        match self {
            Self::Heartbeat => Duration::from_millis(20),
            Self::KinematicState | Self::Acceleration => Duration::from_millis(60),
            Self::Capability
            | Self::Trajectory
            | Self::OperationModeState
            | Self::GearCommand
            | Self::TurnIndicatorsCommand
            | Self::HazardLightsCommand => Duration::from_millis(300),
        }
    }

    /// The end-to-end profile of section 8.1.
    #[must_use]
    pub const fn profile(self) -> E2eProfile {
        match self {
            Self::Heartbeat | Self::Capability => E2eProfile::B,
            Self::Trajectory
            | Self::KinematicState
            | Self::Acceleration
            | Self::OperationModeState
            | Self::GearCommand
            | Self::TurnIndicatorsCommand
            | Self::HazardLightsCommand => E2eProfile::A,
        }
    }
}

/// One frame the stub delivered, and its topic.
#[derive(Clone, Debug, PartialEq)]
pub struct HpcFrame {
    /// The topic of the frame.
    pub topic: HpcTopic,
    /// The frame and its delivery time.
    pub delivery: Delivery<()>,
}

/// The HPC side of the link, publishing a chosen set of [`HpcTopic`]s.
pub struct HpcStub {
    publishers: Vec<(HpcTopic, Publisher<()>)>,
}

impl Default for HpcStub {
    fn default() -> Self {
        Self::new()
    }
}

impl HpcStub {
    /// A stub publishing every topic, first at time 0.
    #[must_use]
    pub fn new() -> Self {
        Self::with_topics(&HpcTopic::ALL)
    }

    /// A stub publishing only `topics`, first at time 0.
    #[must_use]
    pub fn with_topics(topics: &[HpcTopic]) -> Self {
        let publishers = topics
            .iter()
            .map(|&topic| {
                (
                    topic,
                    Publisher::new(topic.topic(), topic.period(), |_, _| ()),
                )
            })
            .collect();
        Self { publishers }
    }

    /// Apply `fault` to `topic` from `from` until `until` (`None`: until the
    /// end of the run).
    ///
    /// # Panics
    ///
    /// If the stub does not publish `topic`.
    pub fn inject(&mut self, topic: HpcTopic, fault: Fault, from: Instant, until: Option<Instant>) {
        let Some((_, publisher)) = self
            .publishers
            .iter_mut()
            .find(|(published, _)| *published == topic)
        else {
            panic!("the stub does not publish {topic:?}");
        };
        publisher.inject(fault, from, until);
    }

    /// Apply `fault` to every topic: a fault of `H1`, the link itself.
    pub fn inject_link(&mut self, fault: Fault, from: Instant, until: Option<Instant>) {
        for (_, publisher) in &mut self.publishers {
            publisher.inject(fault, from, until);
        }
    }

    /// Every frame due at or before `now`, in delivery order; frames due at
    /// the same time keep the order of the topics given to the stub.
    pub fn due(&mut self, now: Instant) -> Vec<HpcFrame> {
        let mut frames = Vec::new();
        for (topic, publisher) in &mut self.publishers {
            for delivery in publisher.due(now) {
                frames.push(HpcFrame {
                    topic: *topic,
                    delivery,
                });
            }
        }
        frames.sort_by_key(|frame| frame.delivery.at);
        frames
    }
}

#[cfg(test)]
mod tests {
    use super::{HpcStub, HpcTopic};
    use crate::harness::{Fault, millis};

    fn count(frames: &[super::HpcFrame], topic: HpcTopic) -> usize {
        frames.iter().filter(|frame| frame.topic == topic).count()
    }

    #[test]
    fn publishes_at_the_design_rates() {
        let mut stub = HpcStub::new();
        let frames = stub.due(millis(999));
        assert_eq!(count(&frames, HpcTopic::Heartbeat), 100);
        assert_eq!(count(&frames, HpcTopic::KinematicState), 50);
        assert_eq!(count(&frames, HpcTopic::Capability), 10);
        assert_eq!(count(&frames, HpcTopic::Trajectory), 10);
    }

    #[test]
    fn frames_come_in_delivery_order() {
        let mut stub = HpcStub::with_topics(&[HpcTopic::Capability, HpcTopic::Heartbeat]);
        let frames = stub.due(millis(100));
        assert!(
            frames
                .windows(2)
                .all(|w| w[0].delivery.at <= w[1].delivery.at)
        );
        assert_eq!(frames[0].topic, HpcTopic::Capability);
        assert_eq!(frames[1].topic, HpcTopic::Heartbeat);
    }

    #[test]
    fn a_topic_fault_leaves_the_other_topics_alone() {
        let mut stub = HpcStub::with_topics(&[HpcTopic::Heartbeat, HpcTopic::Capability]);
        stub.inject(HpcTopic::Heartbeat, Fault::Stop, millis(50), None);
        let frames = stub.due(millis(199));
        assert_eq!(count(&frames, HpcTopic::Heartbeat), 5);
        assert_eq!(count(&frames, HpcTopic::Capability), 2);
    }

    #[test]
    fn a_link_fault_reaches_every_topic() {
        let mut stub = HpcStub::with_topics(&[HpcTopic::Heartbeat, HpcTopic::Capability]);
        stub.inject_link(Fault::Corrupt, millis(0), None);
        assert!(
            stub.due(millis(100))
                .iter()
                .all(|f| !f.delivery.frame.intact)
        );
    }

    #[test]
    #[should_panic(expected = "does not publish")]
    fn refuses_a_fault_on_a_topic_it_does_not_publish() {
        let mut stub = HpcStub::with_topics(&[HpcTopic::Heartbeat]);
        stub.inject(HpcTopic::Trajectory, Fault::Stop, millis(0), None);
    }

    #[test]
    fn every_topic_is_on_the_link_within_its_age_limit() {
        for topic in HpcTopic::ALL {
            assert!(topic.period() <= topic.age_limit(), "{topic:?}");
        }
    }
}
