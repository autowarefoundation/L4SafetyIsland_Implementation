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

//! The WP0 harness exit: every input of the WP2 nodes
//! (WP0-WP3-SafetyFloor.md section 3) can be scripted.

use core::time::Duration;

use si_platform_host::harness::{
    Fault, HpcStub, InputSpec, Publisher, Rate, SECTION_3_INPUTS, Script, millis,
};

/// The period a test picks for an input whose rate the contract states.
const TEST_PERIOD: Duration = Duration::from_millis(50);

/// Script `spec` for one second, with a stop fault in its second half where
/// it is periodic, and return how many samples reached the node.
fn deliveries_in_one_second(spec: &InputSpec) -> usize {
    let end = millis(999);
    if spec.from_hpc_stub {
        let mut stub = HpcStub::new();
        stub.inject_link(Fault::Stop, millis(500), None);
        return stub.due(end).len();
    }
    match spec.rate {
        Rate::Periodic(period) => periodic_with_stop(spec, period).due(end).len(),
        Rate::FromContract => periodic_with_stop(spec, TEST_PERIOD).due(end).len(),
        Rate::OnEvent => {
            let mut script = Script::new().at(millis(100), true).at(millis(600), false);
            script.due(end).len()
        }
    }
}

fn periodic_with_stop(spec: &InputSpec, period: Duration) -> Publisher<u32> {
    let mut publisher = Publisher::new(spec.input, period, |counter, _| counter);
    publisher.inject(Fault::Stop, millis(500), None);
    publisher
}

#[test]
fn every_section_3_input_can_be_scripted() {
    for spec in &SECTION_3_INPUTS {
        let count = deliveries_in_one_second(spec);
        assert!(count > 0, "{} {} delivered nothing", spec.node, spec.input);
    }
}

#[test]
fn a_periodic_input_delivers_at_its_rate_until_stopped() {
    let spec = SECTION_3_INPUTS
        .iter()
        .find(|spec| spec.node == "N1" && spec.edge == "A3")
        .expect("N1 reads the time-base validity");
    assert_eq!(spec.rate, Rate::Periodic(Duration::from_secs(1)));
    assert_eq!(deliveries_in_one_second(spec), 1);
}
