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

//! A scripted periodic publisher with injectable faults.

use core::time::Duration;

use si_hal::Instant;

use super::after;

/// What a faulty sender does instead of publishing a fresh sample.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    /// Publish nothing.
    Stop,
    /// Resend the last intact frame unchanged: same counter, same stamp,
    /// same payload (a stale resend).
    Freeze,
    /// Publish a fresh frame whose end-to-end check fails.
    Corrupt,
}

/// One publication as the receiver's end-to-end check sees it.
#[derive(Clone, Debug, PartialEq)]
pub struct Frame<T> {
    /// The sender's counter, one more per fresh frame, starting at 1.
    pub counter: u32,
    /// When the sender produced the payload. A frozen frame keeps its stamp.
    pub stamp: Instant,
    /// False when the end-to-end check of the frame fails.
    pub intact: bool,
    /// The sample.
    pub payload: T,
}

/// A frame and the time it is delivered.
#[derive(Clone, Debug, PartialEq)]
pub struct Delivery<T> {
    /// When the frame reaches the receiver.
    pub at: Instant,
    /// The frame.
    pub frame: Frame<T>,
}

/// A fault active from `from` until `until` (exclusive; `None` = forever).
#[derive(Clone, Copy, Debug)]
struct FaultWindow {
    fault: Fault,
    from: Instant,
    until: Option<Instant>,
}

impl FaultWindow {
    fn covers(&self, at: Instant) -> bool {
        at >= self.from && self.until.is_none_or(|until| at < until)
    }
}

/// A publisher that sends one sample per period, built by a payload
/// function of the frame counter and the send time. Faults injected for a
/// time window replace the fresh sample there.
pub struct Publisher<T> {
    name: String,
    period: Duration,
    next_due: Instant,
    counter: u32,
    payload: Box<dyn FnMut(u32, Instant) -> T>,
    faults: Vec<FaultWindow>,
    last_intact: Option<Frame<T>>,
}

impl<T: Clone> Publisher<T> {
    /// A publisher named `name` that sends every `period`, first at time 0.
    ///
    /// # Panics
    ///
    /// If `period` is zero.
    pub fn new(
        name: &str,
        period: Duration,
        payload: impl FnMut(u32, Instant) -> T + 'static,
    ) -> Self {
        assert!(
            !period.is_zero(),
            "publisher {name}: the period must not be zero"
        );
        Self {
            name: name.to_owned(),
            period,
            next_due: Instant::from_nanos(0),
            counter: 0,
            payload: Box::new(payload),
            faults: Vec::new(),
            last_intact: None,
        }
    }

    /// The same publisher, first sending at `start`.
    #[must_use]
    pub fn starting_at(mut self, start: Instant) -> Self {
        self.next_due = start;
        self
    }

    /// The publisher's name, as it appears in traces.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The publication period.
    #[must_use]
    pub fn period(&self) -> Duration {
        self.period
    }

    /// Apply `fault` to every publication at or after `from` and before
    /// `until` (`None`: until the end of the run). The first window that
    /// covers a publication wins.
    pub fn inject(&mut self, fault: Fault, from: Instant, until: Option<Instant>) {
        self.faults.push(FaultWindow { fault, from, until });
    }

    /// Every delivery due at or before `now`, oldest first.
    pub fn due(&mut self, now: Instant) -> Vec<Delivery<T>> {
        let mut deliveries = Vec::new();
        while self.next_due <= now {
            let at = self.next_due;
            self.next_due = after(at, self.period);
            if let Some(frame) = self.publish(at) {
                deliveries.push(Delivery { at, frame });
            }
        }
        deliveries
    }

    fn active_fault(&self, at: Instant) -> Option<Fault> {
        self.faults
            .iter()
            .find(|window| window.covers(at))
            .map(|window| window.fault)
    }

    fn publish(&mut self, at: Instant) -> Option<Frame<T>> {
        match self.active_fault(at) {
            None => {
                let frame = self.fresh_frame(at, true);
                self.last_intact = Some(frame.clone());
                Some(frame)
            }
            Some(Fault::Stop) => None,
            Some(Fault::Freeze) => self.last_intact.clone(),
            Some(Fault::Corrupt) => Some(self.fresh_frame(at, false)),
        }
    }

    fn fresh_frame(&mut self, at: Instant, intact: bool) -> Frame<T> {
        self.counter = self.counter.wrapping_add(1);
        Frame {
            counter: self.counter,
            stamp: at,
            intact,
            payload: (self.payload)(self.counter, at),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::time::Duration;

    use super::{Fault, Publisher};
    use crate::harness::millis;

    fn counters(publisher: &mut Publisher<u32>, now: u64) -> Vec<(u64, u32, bool)> {
        publisher
            .due(millis(now))
            .into_iter()
            .map(|d| (d.at.as_nanos() / 1_000_000, d.frame.counter, d.frame.intact))
            .collect()
    }

    fn ten_ms() -> Publisher<u32> {
        Publisher::new("test", Duration::from_millis(10), |counter, _| {
            counter * 100
        })
    }

    #[test]
    fn publishes_once_per_period_from_time_zero() {
        let mut publisher = ten_ms();
        assert_eq!(
            counters(&mut publisher, 25),
            vec![(0, 1, true), (10, 2, true), (20, 3, true)]
        );
        assert_eq!(counters(&mut publisher, 29), vec![]);
        assert_eq!(counters(&mut publisher, 30), vec![(30, 4, true)]);
    }

    #[test]
    fn payload_is_built_from_the_counter() {
        let mut publisher = ten_ms();
        let deliveries = publisher.due(millis(10));
        assert_eq!(deliveries[1].frame.payload, 200);
    }

    #[test]
    fn stop_sends_nothing_and_the_counter_resumes() {
        let mut publisher = ten_ms();
        publisher.inject(Fault::Stop, millis(10), Some(millis(30)));
        assert_eq!(
            counters(&mut publisher, 30),
            vec![(0, 1, true), (30, 2, true)]
        );
    }

    #[test]
    fn freeze_resends_the_last_intact_frame() {
        let mut publisher = ten_ms();
        publisher.inject(Fault::Freeze, millis(10), None);
        let deliveries = publisher.due(millis(20));
        assert_eq!(deliveries.len(), 3);
        assert_eq!(deliveries[2].at, millis(20));
        assert_eq!(deliveries[2].frame, deliveries[0].frame);
    }

    #[test]
    fn freeze_before_any_frame_sends_nothing() {
        let mut publisher = ten_ms();
        publisher.inject(Fault::Freeze, millis(0), None);
        assert_eq!(counters(&mut publisher, 20), vec![]);
    }

    #[test]
    fn corrupt_sends_fresh_frames_that_fail_the_check() {
        let mut publisher = ten_ms();
        publisher.inject(Fault::Corrupt, millis(10), Some(millis(20)));
        assert_eq!(
            counters(&mut publisher, 20),
            vec![(0, 1, true), (10, 2, false), (20, 3, true)]
        );
    }

    #[test]
    fn a_frozen_stream_after_corruption_repeats_the_last_intact_frame() {
        let mut publisher = ten_ms();
        publisher.inject(Fault::Corrupt, millis(10), Some(millis(20)));
        publisher.inject(Fault::Freeze, millis(20), None);
        assert_eq!(
            counters(&mut publisher, 20),
            vec![(0, 1, true), (10, 2, false), (20, 1, true)]
        );
    }

    #[test]
    fn starting_at_delays_the_first_frame() {
        let mut publisher = ten_ms().starting_at(millis(5));
        assert_eq!(
            counters(&mut publisher, 15),
            vec![(5, 1, true), (15, 2, true)]
        );
    }
}
