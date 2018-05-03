use std::cell::*;
use std::rc::Rc;
use clock::Clock;
use consts;

pub struct EventSource<T> {
    clock: Rc<Clock>,
    events: Vec<(u64, T)>,
    cursor: Cell<usize>,
    last_time: Cell<u64>
}

impl<T> EventSource<T> {
    pub fn new(events: Vec<(u64, T)>, clock: Rc<Clock>) -> Self {
        // TODO: sort the events by .0, just in case
        Self {
            clock: clock,
            events: events,
            cursor: Cell::new(0),
            last_time: Cell::new(consts::TIME_INFINITY)
        }
    }

    pub fn events_this_chunk<'b>(&'b self) -> &'b[(u64, T)] {
        let time = self.clock.time();
        if self.last_time.get() != self.clock.time() {
            self.last_time.set(self.clock.time());

            // Skip past all events before the current time.
            while self.cursor.get() < self.events.len() && self.events[self.cursor.get()].0 < time {
                self.cursor.set(self.cursor.get() + 1);
            }
        }
        let start = self.cursor.get();
        let mut end = start;
        while end < self.events.len() && self.events[end].0 < time + (consts::CHUNK_SIZE as u64) {
            end += 1;
        }
        &self.events[start..end]
    }
}

// TODO: 4/4 only for now, add support for custom time sig later.
pub struct MusicalTime {
    bpm: f64
}

impl MusicalTime {
    pub fn new(bpm: f64) -> Self {
        Self {
            bpm: bpm
        }
    }

    pub fn calc(&self, bar: u32, sixteenth: u32) -> u64 {
        assert!(1 <= bar);
        assert!(1 <= sixteenth && sixteenth <= 16);
        let beats_per_bar = 4.0;
        let samples_per_sixteenth = (consts::SAMPLE_RATE as f64) * 60.0 / self.bpm / 16.0;
        let time = (bar as f64 - 1.0) * 16.0 * samples_per_sixteenth + (sixteenth as f64 - 1.0) * samples_per_sixteenth;
        time as u64
    }
}
