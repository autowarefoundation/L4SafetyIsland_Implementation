//! A `si-nodes` step function driven by the host clock.

use si_hal::{Clock, Instant};
use si_nodes::StepNode;
use si_platform_host::HostClock;

/// Counts its steps and reports the time of the last one.
struct Counter {
    steps: u32,
}

impl StepNode for Counter {
    type Input = ();
    type Output = (u32, Instant);

    fn step(&mut self, (): &(), now: Instant) -> (u32, Instant) {
        self.steps += 1;
        (self.steps, now)
    }
}

#[test]
fn host_clock_drives_a_step_node() {
    let clock = HostClock::new();
    let mut node = Counter { steps: 0 };
    let (n1, t1) = node.step(&(), clock.now());
    let (n2, t2) = node.step(&(), clock.now());
    assert_eq!((n1, n2), (1, 2));
    assert!(t2 >= t1, "the host clock went backwards");
}
