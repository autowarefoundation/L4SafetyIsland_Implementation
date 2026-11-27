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

//! Samples delivered at set times, for inputs that change on events.

use si_hal::Instant;

/// A list of samples, each delivered once at its time.
#[derive(Clone, Debug)]
pub struct Script<T> {
    events: Vec<(Instant, T)>,
    next: usize,
}

impl<T> Default for Script<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Script<T> {
    /// An empty script.
    #[must_use]
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            next: 0,
        }
    }

    /// The same script with `value` delivered at `at`.
    ///
    /// # Panics
    ///
    /// If `at` is earlier than the previous event's time.
    #[must_use]
    pub fn at(mut self, at: Instant, value: T) -> Self {
        if let Some((last, _)) = self.events.last() {
            assert!(at >= *last, "script events must be in time order");
        }
        self.events.push((at, value));
        self
    }

    /// Every event due at or before `now` and not yet delivered, oldest
    /// first.
    pub fn due(&mut self, now: Instant) -> Vec<&T> {
        let start = self.next;
        while self.next < self.events.len() && self.events[self.next].0 <= now {
            self.next += 1;
        }
        self.events[start..self.next]
            .iter()
            .map(|(_, value)| value)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Script;
    use crate::harness::millis;

    #[test]
    fn delivers_each_event_once_at_its_time() {
        let mut script = Script::new()
            .at(millis(10), "valid")
            .at(millis(30), "invalid");
        let nothing: Vec<&&str> = Vec::new();
        assert_eq!(script.due(millis(5)), nothing);
        assert_eq!(script.due(millis(10)), vec![&"valid"]);
        assert_eq!(script.due(millis(20)), nothing);
        assert_eq!(script.due(millis(40)), vec![&"invalid"]);
        assert_eq!(script.due(millis(50)), nothing);
    }

    #[test]
    #[should_panic(expected = "time order")]
    fn refuses_events_out_of_order() {
        let _ = Script::new().at(millis(10), 1).at(millis(5), 2);
    }
}
