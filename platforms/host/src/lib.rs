//! Host platform: `si-hal` implemented on `std`, for unit tests,
//! software-in-the-loop runs and the WP0 test harness.

use std::time::Instant as StdInstant;

use si_hal::{Clock, Instant};

/// [`Clock`] on the host's monotonic clock, zero at construction.
#[derive(Debug)]
pub struct HostClock {
    origin: StdInstant,
}

impl HostClock {
    /// A clock whose zero is now.
    #[must_use]
    pub fn new() -> Self {
        Self {
            origin: StdInstant::now(),
        }
    }
}

impl Default for HostClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for HostClock {
    fn now(&self) -> Instant {
        let nanos = self.origin.elapsed().as_nanos();
        Instant::from_nanos(u64::try_from(nanos).unwrap_or(u64::MAX))
    }
}
