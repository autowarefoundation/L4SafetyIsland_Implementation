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

//! The trace recorder: what a run published and decided, one line each.

use core::fmt;

use si_hal::Instant;

/// One recorded event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceLine {
    /// Simulated time of the event.
    pub at: Instant,
    /// Who published or decided it: a design id (`H7`, `N2`) or a name.
    pub source: String,
    /// What happened, as `kind key=value ...`.
    pub event: String,
}

/// `at` in milliseconds with three decimals, for example `30.000`.
#[must_use]
pub fn format_millis(at: Instant) -> String {
    let micros = at.as_nanos() / 1_000;
    format!("{}.{:03}", micros / 1_000, micros % 1_000)
}

impl fmt::Display for TraceLine {
    /// `<milliseconds with three decimals> <source> <event>`, the time right
    /// aligned, for example `   30.000 N2 fault HPC_LIVENESS_LOST`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:>9} {} {}",
            format_millis(self.at),
            self.source,
            self.event
        )
    }
}

/// The recorded events of a run, in the order they were recorded.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Trace {
    lines: Vec<TraceLine>,
}

impl Trace {
    /// An empty trace.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record `event` from `source` at `at`.
    pub fn record(&mut self, at: Instant, source: &str, event: &str) {
        self.lines.push(TraceLine {
            at,
            source: source.to_owned(),
            event: event.to_owned(),
        });
    }

    /// The recorded lines.
    #[must_use]
    pub fn lines(&self) -> &[TraceLine] {
        &self.lines
    }

    /// The lines from `source` only.
    pub fn from_source<'a>(&'a self, source: &'a str) -> impl Iterator<Item = &'a TraceLine> {
        self.lines.iter().filter(move |line| line.source == source)
    }

    /// The first line from `source` whose event starts with `prefix`.
    #[must_use]
    pub fn first(&self, source: &str, prefix: &str) -> Option<&TraceLine> {
        self.lines
            .iter()
            .find(|line| line.source == source && line.event.starts_with(prefix))
    }

    /// The trace as text, one [`TraceLine`] per line.
    #[must_use]
    pub fn render(&self) -> String {
        let mut text = String::new();
        for line in &self.lines {
            text.push_str(&line.to_string());
            text.push('\n');
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::Trace;
    use crate::harness::millis;
    use si_hal::Instant;

    #[test]
    fn renders_milliseconds_with_three_decimals() {
        let mut trace = Trace::new();
        trace.record(millis(30), "N2", "fault HPC_LIVENESS_LOST");
        trace.record(Instant::from_nanos(1_234_567), "H7", "heartbeat counter=1");
        assert_eq!(
            trace.render(),
            "   30.000 N2 fault HPC_LIVENESS_LOST\n    1.234 H7 heartbeat counter=1\n"
        );
    }

    #[test]
    fn finds_the_first_event_of_a_source() {
        let mut trace = Trace::new();
        trace.record(millis(10), "H7", "heartbeat counter=1");
        trace.record(millis(20), "N2", "fault E2E_VERIFY_FAIL");
        trace.record(millis(30), "N2", "fault HPC_LIVENESS_LOST");
        let line = trace.first("N2", "fault HPC_").map(|line| line.at);
        assert_eq!(line, Some(millis(30)));
        assert_eq!(trace.from_source("H7").count(), 1);
    }
}
