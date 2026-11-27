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

//! The harness example: the D1 HPC-loss scenario (WP0-WP3-SafetyFloor.md
//! section 5) against a placeholder heartbeat watch, with golden traces.
//!
//! The watch stands in for `N2` until the `N2` issue brings the real node:
//! it declares `HPC_LIVENESS_LOST` once no fresh heartbeat came for three
//! heartbeat periods, and reports every frame that fails its end-to-end
//! check as `E2E_VERIFY_FAIL`, the way `N1` must.

use std::path::PathBuf;

use core::time::Duration;

use si_core::FaultCode;
use si_hal::Instant;
use si_nodes::StepNode;
use si_platform_host::harness::{
    Fault, Frame, HpcStub, HpcTopic, Rig, Runner, Trace, assert_golden, format_millis, millis,
};

/// Three missed 100 Hz heartbeats.
const LIVENESS_TIMEOUT_NS: u64 = 30_000_000;

/// The placeholder node.
#[derive(Default)]
struct HeartbeatWatch {
    last_counter: u32,
    last_fresh: Option<Instant>,
    lost: bool,
}

impl StepNode for HeartbeatWatch {
    type Input = Vec<Frame<()>>;
    type Output = Vec<FaultCode>;

    fn step(&mut self, heartbeats: &Vec<Frame<()>>, now: Instant) -> Vec<FaultCode> {
        let mut faults = Vec::new();
        for frame in heartbeats {
            if !frame.intact {
                faults.push(FaultCode::E2eVerifyFail);
            } else if frame.counter > self.last_counter {
                self.last_counter = frame.counter;
                self.last_fresh = Some(now);
            }
        }
        if let Some(last_fresh) = self.last_fresh {
            let silent_for = now.checked_nanos_since(last_fresh).unwrap_or(0);
            if !self.lost && silent_for >= LIVENESS_TIMEOUT_NS {
                self.lost = true;
                faults.push(FaultCode::HpcLivenessLost);
            }
        }
        faults
    }
}

/// Feeds the watch from the HPC stub and records the heartbeat and
/// capability frames and the faults.
struct HpcLossRig {
    stub: HpcStub,
}

impl HpcLossRig {
    fn new() -> Self {
        Self {
            stub: HpcStub::with_topics(&[HpcTopic::Heartbeat, HpcTopic::Capability]),
        }
    }
}

fn describe(name: &str, frame: &Frame<()>) -> String {
    let stamp = format_millis(frame.stamp);
    let mut text = format!("{name} counter={} stamp={stamp}", frame.counter);
    if !frame.intact {
        text.push_str(" corrupt");
    }
    text
}

impl Rig for HpcLossRig {
    type Node = HeartbeatWatch;

    fn inputs(&mut self, now: Instant, trace: &mut Trace) -> Vec<Frame<()>> {
        let mut heartbeats = Vec::new();
        for frame in self.stub.due(now) {
            let topic = frame.topic;
            let delivery = frame.delivery;
            let name = match topic {
                HpcTopic::Heartbeat => "heartbeat",
                _ => "capability",
            };
            trace.record(
                delivery.at,
                topic.producer(),
                &describe(name, &delivery.frame),
            );
            if topic == HpcTopic::Heartbeat {
                heartbeats.push(delivery.frame);
            }
        }
        heartbeats
    }

    fn outputs(&mut self, now: Instant, faults: &Vec<FaultCode>, trace: &mut Trace) {
        for fault in faults {
            trace.record(now, "N2", &format!("fault {}", fault.name()));
        }
    }
}

fn golden(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden")
        .join(name)
}

fn run(rig: &mut HpcLossRig) -> Trace {
    let mut runner = Runner::new(Duration::from_millis(1));
    runner.run_until(rig, &mut HeartbeatWatch::default(), millis(200));
    runner.trace().clone()
}

fn first_fault_at(trace: &Trace, code: FaultCode) -> Option<Instant> {
    let event = format!("fault {}", code.name());
    trace.first("N2", &event).map(|line| line.at)
}

fn fault_count(trace: &Trace, code: FaultCode) -> usize {
    let event = format!("fault {}", code.name());
    trace
        .from_source("N2")
        .filter(|line| line.event == event)
        .count()
}

#[test]
fn heartbeat_stop_is_detected_within_three_periods() {
    let mut rig = HpcLossRig::new();
    rig.stub
        .inject(HpcTopic::Heartbeat, Fault::Stop, millis(105), None);
    let trace = run(&mut rig);

    let lost_at = first_fault_at(&trace, FaultCode::HpcLivenessLost);
    assert_eq!(lost_at, Some(millis(130)), "last heartbeat at 100 ms");
    assert_golden(&trace, &golden("hpc_loss_stop.trace"));
}

#[test]
fn frozen_heartbeat_is_a_lost_heartbeat() {
    let mut rig = HpcLossRig::new();
    rig.stub
        .inject(HpcTopic::Heartbeat, Fault::Freeze, millis(105), None);
    let trace = run(&mut rig);

    let lost_at = first_fault_at(&trace, FaultCode::HpcLivenessLost);
    assert_eq!(lost_at, Some(millis(130)), "last fresh heartbeat at 100 ms");
    assert_golden(&trace, &golden("hpc_loss_freeze.trace"));
}

#[test]
fn corrupt_heartbeats_are_reported_one_by_one_without_flapping() {
    let mut rig = HpcLossRig::new();
    rig.stub.inject(
        HpcTopic::Heartbeat,
        Fault::Corrupt,
        millis(105),
        Some(millis(125)),
    );
    let trace = run(&mut rig);

    assert_eq!(fault_count(&trace, FaultCode::E2eVerifyFail), 2);
    assert_eq!(first_fault_at(&trace, FaultCode::HpcLivenessLost), None);
    assert_golden(&trace, &golden("hpc_corrupt.trace"));
}
