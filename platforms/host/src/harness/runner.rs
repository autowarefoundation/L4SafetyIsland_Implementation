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

//! The host node runner: steps a `StepNode` in simulated time.

use core::time::Duration;

use si_hal::Instant;
use si_nodes::StepNode;

use super::{Trace, after};

/// The test's side of a run: builds each step's input from the scripted
/// publishers and records what the node produced.
pub trait Rig {
    /// The node under test.
    type Node: StepNode;

    /// The input of the step at `now`: collect what the publishers and
    /// scripts delivered since the previous step, recording it in `trace`
    /// as the test wants it in the golden file.
    fn inputs(&mut self, now: Instant, trace: &mut Trace) -> <Self::Node as StepNode>::Input;

    /// Record the node's `output` of the step at `now`.
    fn outputs(
        &mut self,
        now: Instant,
        output: &<Self::Node as StepNode>::Output,
        trace: &mut Trace,
    );
}

/// Steps a node at a fixed period in simulated time and keeps the trace.
#[derive(Debug)]
pub struct Runner {
    period: Duration,
    now: Instant,
    trace: Trace,
}

impl Runner {
    /// A runner that steps every `period`, first at time 0.
    ///
    /// # Panics
    ///
    /// If `period` is zero.
    #[must_use]
    pub fn new(period: Duration) -> Self {
        assert!(!period.is_zero(), "the step period must not be zero");
        Self {
            period,
            now: Instant::from_nanos(0),
            trace: Trace::new(),
        }
    }

    /// Step `node` with inputs from `rig` at every period up to and
    /// including `end`. A later call continues from where this one stopped.
    pub fn run_until<R: Rig>(&mut self, rig: &mut R, node: &mut R::Node, end: Instant) {
        while self.now <= end {
            let input = rig.inputs(self.now, &mut self.trace);
            let output = node.step(&input, self.now);
            rig.outputs(self.now, &output, &mut self.trace);
            self.now = after(self.now, self.period);
        }
    }

    /// The time of the next step.
    #[must_use]
    pub fn now(&self) -> Instant {
        self.now
    }

    /// The trace recorded so far.
    #[must_use]
    pub fn trace(&self) -> &Trace {
        &self.trace
    }
}

#[cfg(test)]
mod tests {
    use core::time::Duration;

    use si_hal::Instant;
    use si_nodes::StepNode;

    use super::{Rig, Runner};
    use crate::harness::{Script, Trace, millis};

    /// Sums its inputs.
    struct Sum(u32);

    impl StepNode for Sum {
        type Input = u32;
        type Output = u32;

        fn step(&mut self, input: &u32, _now: Instant) -> u32 {
            self.0 += input;
            self.0
        }
    }

    struct SumRig {
        script: Script<u32>,
    }

    impl Rig for SumRig {
        type Node = Sum;

        fn inputs(&mut self, now: Instant, _trace: &mut Trace) -> u32 {
            self.script.due(now).into_iter().sum()
        }

        fn outputs(&mut self, now: Instant, output: &u32, trace: &mut Trace) {
            trace.record(now, "sum", &format!("total={output}"));
        }
    }

    #[test]
    fn steps_at_the_period_up_to_and_including_the_end() {
        let mut rig = SumRig {
            script: Script::new().at(millis(10), 1).at(millis(25), 2),
        };
        let mut runner = Runner::new(Duration::from_millis(10));
        runner.run_until(&mut rig, &mut Sum(0), millis(30));
        assert_eq!(
            runner.trace().render(),
            "    0.000 sum total=0\n   10.000 sum total=1\n   20.000 sum total=1\n   30.000 sum total=3\n"
        );
        assert_eq!(runner.now(), millis(40));
    }
}
