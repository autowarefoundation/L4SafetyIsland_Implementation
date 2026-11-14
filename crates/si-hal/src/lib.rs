//! Hardware abstraction traits of the safety island (ARCHITECTURE.md section 3).
//!
//! Shared crates depend on these traits only. Each platform under `platforms/`
//! implements them; the implementation is chosen at link time.

#![no_std]

/// A point on the island's monotonic time base, in nanoseconds since boot.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(u64);

impl Instant {
    /// The instant `nanos` nanoseconds after boot.
    #[must_use]
    pub const fn from_nanos(nanos: u64) -> Self {
        Self(nanos)
    }

    /// Nanoseconds since boot.
    #[must_use]
    pub const fn as_nanos(self) -> u64 {
        self.0
    }

    /// Nanoseconds from `earlier` to `self`, or `None` when `earlier` is later.
    #[must_use]
    pub const fn checked_nanos_since(self, earlier: Self) -> Option<u64> {
        self.0.checked_sub(earlier.0)
    }
}

/// A monotonic clock.
pub trait Clock {
    /// The current instant. Never decreases.
    fn now(&self) -> Instant;
}

#[cfg(test)]
mod tests {
    use super::Instant;

    #[test]
    fn nanos_since_is_none_when_earlier_is_later() {
        let a = Instant::from_nanos(10);
        let b = Instant::from_nanos(25);
        assert_eq!(b.checked_nanos_since(a), Some(15));
        assert_eq!(a.checked_nanos_since(b), None);
    }
}
