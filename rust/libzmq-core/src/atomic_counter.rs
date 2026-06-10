// SPDX-License-Identifier: MPL-2.0

use core::sync::atomic::{AtomicU32, Ordering};

/// Rust-backed implementation of the public `zmq_atomic_counter_*` utility API.
pub struct AtomicCounter {
    value: AtomicU32,
}

impl AtomicCounter {
    pub fn new(value: u32) -> Self {
        Self {
            value: AtomicU32::new(value),
        }
    }

    pub fn set(&self, value: u32) {
        self.value.store(value, Ordering::Release);
    }

    pub fn inc(&self) -> u32 {
        self.value.fetch_add(1, Ordering::AcqRel)
    }

    pub fn dec(&self) -> bool {
        self.value.fetch_sub(1, Ordering::AcqRel).wrapping_sub(1) != 0
    }

    pub fn value(&self) -> u32 {
        self.value.load(Ordering::Acquire)
    }
}

#[cfg(test)]
mod tests {
    use super::AtomicCounter;

    #[test]
    fn atomic_counter_matches_public_utility_semantics() {
        let counter = AtomicCounter::new(0);
        assert_eq!(counter.value(), 0);
        assert_eq!(counter.inc(), 0);
        assert_eq!(counter.inc(), 1);
        assert_eq!(counter.inc(), 2);
        assert_eq!(counter.value(), 3);
        assert!(counter.dec());
        assert!(counter.dec());
        assert!(!counter.dec());
        counter.set(2);
        assert!(counter.dec());
        assert!(!counter.dec());
    }
}
