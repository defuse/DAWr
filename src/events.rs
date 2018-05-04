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
    pub fn new(events: Vec<(u64, T)>, clock: Rc<Clock>) -> Rc<Self> {
        if !Self::is_sorted(&events) {
            panic!("Events are not sorted by time!");
        }
        Rc::new(Self {
            clock: clock,
            events: events,
            cursor: Cell::new(0),
            last_time: Cell::new(consts::TIME_INFINITY)
        })
    }

    fn is_sorted(events: &Vec<(u64, T)>) -> bool {
        if events.len() == 0 {
            return true;
        } else {
            let mut max = events[0].0;
            for i in 1..events.len() {
                if events[i].0 < max {
                    //println!("{} {}", events[i].0, max);
                    return false;
                } else {
                    max = events[i].0;
                }
            }
            return true;
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
