use std::cell::*;

use consts;

pub struct Clock {
    time: Cell<u64>
}

impl Clock {
    pub fn new() -> Self {
        Self { time: Cell::new(0) }
    }

    pub fn time(&self) -> u64 {
        self.time.get()
    }

    pub fn increment(&self) {
        self.time.set(self.time.get() + (consts::CHUNK_SIZE as u64));
    }
}
