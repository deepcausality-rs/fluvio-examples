use std::fmt::{Display, Formatter};
use std::sync::atomic::{self, AtomicU64};

// Only the memory directly touched by the operation is synchronized.
// https://doc.rust-lang.org/nomicon/atomics.html#data-accesses
const ORDER: atomic::Ordering = atomic::Ordering::Relaxed;

/// A thread-safe counter that provides relaxed atomic increment operations.
///
/// This counter uses relaxed memory ordering for its atomic operations. This
/// means operations on it will not synchronize memory with other threads.
///
/// The counter is initialized to 0 and supports an atomic
/// `increment_and_get()` method to increment and retrieve the value.
///
/// # Examples
///
/// ```
/// use lib_inference::prelude::RelaxedAtomicCounter;
///
/// let counter = RelaxedAtomicCounter::new();
///
/// let v1 = counter.increment_and_get(); // v1 = 1
/// let v2 = counter.increment_and_get(); // v2 = 2
/// ```
///
/// # Notes
///
/// For performance-critical code where synchronization is not required, a
/// relaxed atomic counter provides efficient concurrent incrementing.
///
/// For code that requires synchronization, use a sequentially consistent
/// atomic counter instead.
///
#[derive(Debug)]
pub struct RelaxedAtomicCounter(AtomicU64);

impl RelaxedAtomicCounter {
    // Creates a new counter with atomic increment operation.
    pub fn new() -> Self {
        // No ordering constraints, only atomic operations.
        // https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html#variant.Relaxed
        RelaxedAtomicCounter(AtomicU64::new(0))
    }

    // Increment and return new value.
    pub fn increment_and_get(&self) -> u64 {
        self.0.fetch_add(1, ORDER) + 1
    }
}

impl Display for RelaxedAtomicCounter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.load(ORDER))
    }
}
