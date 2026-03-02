use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::error::Error;

// ─── Trait ────────────────────────────────────────────────────────────────────

pub trait MemoryPool: Send + Sync {
    /// Try to grow this reservation by `additional` bytes.
    /// Returns Err(OutOfMemory) if the pool is exhausted.
    fn try_grow(
        &self,
        reservation: &mut MemoryReservation,
        additional: usize,
    ) -> Result<(), Error>;

    /// Release `bytes` back to the pool.
    /// Called automatically on Drop.
    fn shrink(
        &self,
        reservation: &mut MemoryReservation,
        bytes: usize,
    );

    /// Total bytes currently claimed across ALL reservations.
    fn reserved(&self) -> usize;

    /// The hard cap this pool was created with (e.g. maxmemory).
    fn capacity(&self) -> usize;
}

// ─── MemoryReservation ────────────────────────────────────────────────────────

pub struct MemoryReservation {
    /// How many bytes THIS reservation has currently claimed.
    size: usize,
    pool: Arc<dyn MemoryPool>,
}

impl MemoryReservation {
    pub fn new(pool: Arc<dyn MemoryPool>) -> Self {
        Self { size: 0, pool }
    }

    /// Ask the pool before allocating.
    pub fn try_grow(&mut self, additional: usize) -> Result<(), Error> {
        Arc::clone(&self.pool).try_grow(self, additional)
    }

    /// Release bytes back to the pool.
    pub fn shrink(&mut self, bytes: usize) {
        Arc::clone(&self.pool).shrink(self, bytes);
    }

    /// How many bytes this reservation currently holds.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Convenience: free everything this reservation holds.
    pub fn free(&mut self) {
        let size = self.size;
        if size > 0 {
            self.shrink(size);
        }
    }
}

impl Drop for MemoryReservation {
    fn drop(&mut self) {
        self.free();
    }
}

// ─── GreedyMemoryPool ─────────────────────────────────────────────────────────
// First-come-first-served hard cap. 
// This is your maxmemory implementation.

pub struct GreedyMemoryPool {
    capacity: usize,
    used: AtomicUsize,
}

impl GreedyMemoryPool {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            used: AtomicUsize::new(0),
        }
    }
}

impl MemoryPool for GreedyMemoryPool {
    fn try_grow(
        &self,
        reservation: &mut MemoryReservation,
        additional: usize,
    ) -> Result<(), Error> {
        let used = self.used.fetch_add(additional, Ordering::Relaxed);

        if used + additional > self.capacity {
            // Roll back — can't fulfill the request
            self.used.fetch_sub(additional, Ordering::Relaxed);
            return Err(Error::OutOfMemory(format!(
                "OOM used={} capacity={} requested={}",
                used, self.capacity, additional
            )));
        }

        reservation.size += additional;
        Ok(())
    }

    fn shrink(&self, reservation: &mut MemoryReservation, bytes: usize) {
        // Never shrink more than the reservation actually holds
        let actual = bytes.min(reservation.size);
        self.used.fetch_sub(actual, Ordering::Relaxed);
        reservation.size -= actual;
    }

    fn reserved(&self) -> usize {
        self.used.load(Ordering::Relaxed)
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}