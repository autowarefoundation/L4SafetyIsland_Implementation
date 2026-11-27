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

//! The WP0 test harness (WP0-WP3-SafetyFloor.md section 1.2).
//!
//! Time is simulated: a run is deterministic and measures no timing.

mod golden;
mod publisher;
mod script;
mod trace;

use core::time::Duration;

use si_hal::Instant;

pub use golden::{GoldenMismatch, assert_golden, compare_with_golden};
pub use publisher::{Delivery, Fault, Frame, Publisher};
pub use script::Script;
pub use trace::{Trace, TraceLine, format_millis};

/// The instant `ms` milliseconds after the start of a run.
#[must_use]
pub const fn millis(ms: u64) -> Instant {
    Instant::from_nanos(ms.saturating_mul(1_000_000))
}

/// The instant `duration` after `instant`.
#[must_use]
pub fn after(instant: Instant, duration: Duration) -> Instant {
    let nanos = u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX);
    Instant::from_nanos(instant.as_nanos().saturating_add(nanos))
}
