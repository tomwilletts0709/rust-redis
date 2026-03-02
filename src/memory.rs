use std::sync::Arc;

use crate::error::Error;

pub trait MemoryPool: Send + Sync {
    fn try_grow(
        &self,
        reservation: &mut MemoryReservation,
        additional_size: usize,
    ) -> Result<Vec<u8>, Error>;
    fn shrink(
        &self,
        reservation: &mut MemoryReservation,
        additional_size: usize,
    ) -> Result<Vec<u8>, Error>;
    fn try_reserve(&self, reservation: &mut MemoryReservation, size: usize) -> Result<Vec<u8>, Error>;
    fn capacity(&self, reservation: &MemoryReservation) -> usize;
}

pub struct MemoryReservation {
    pool: Arc<dyn MemoryPool>,
    capacity: usize,
    used: usize,
}

impl MemoryReservation {
    pub fn new(pool: Arc<dyn MemoryPool>) -> Self {
        Self {
            pool,
            capacity: 0,
            used: 0,
        }
    }

    pub fn try_grow(&mut self, additional_size: usize) -> Result<Vec<u8>, Error> {
        let pool = Arc::clone(&self.pool);
        pool.try_grow(self, additional_size)
    }

    pub fn shrink(&mut self, additional_size: usize) -> Result<Vec<u8>, Error> {
        let pool = Arc::clone(&self.pool);
        pool.shrink(self, additional_size)
    }

    pub fn try_reserve(&mut self, size: usize) -> Result<Vec<u8>, Error> {
        let pool = Arc::clone(&self.pool);
        pool.try_reserve(self, size)
    }

    pub fn capacity(&self) -> usize {
        self.pool.capacity(self)
    }
}

impl Drop for MemoryReservation {
    fn drop(&mut self) {
        if self.used > 0 {
            let pool = Arc::clone(&self.pool);
            let _ = pool.shrink(self, self.used);
        }
    }
}
