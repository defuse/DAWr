use std::rc::Rc;
use device::*;
use clock::Clock;
use events::EventSource;
use consts;
use std::cell::*;

pub enum SamplerEvent {
    Play,
    PlayAtSpeed(f64)
}

pub struct Sampler {
    device: StereoStateContainer<SamplerState>,
    sampler_events: Rc<EventSource<SamplerEvent>>,
    left_samples: Vec<f32>,
    right_samples: Vec<f32>
}

struct SamplerState {
    playing: bool,
    position: f64,
    playspeed: f64
}

impl Sampler {
    pub fn new(clock: Rc<Clock>, sampler_events: Rc<EventSource<SamplerEvent>>, left_samples: Vec<f32>, right_samples: Vec<f32>) -> Rc<Sampler> {
        assert!(left_samples.len() == right_samples.len());
        Rc::new(Self {
            device: StereoStateContainer::new(clock, SamplerState { playing: false, position: 0.0, playspeed: 1.0 }),
            left_samples, right_samples, sampler_events
        })
    }
}

impl StereoEmitter for Sampler {
    fn output(&self) -> (Ref<Vec<f32>>, Ref<Vec<f32>>) {
        if self.device.clock_advanced() {
            self.device.mark_as_up_to_date();

            let mut left = self.device.borrow_left_to_modify();
            let mut right = self.device.borrow_right_to_modify();
            let mut state = self.device.borrow_state_mut();

            let events = self.sampler_events.events_this_chunk();
            let mut cursor = 0;

            for i in 0..consts::CHUNK_SIZE {

                while cursor < events.len() && events[cursor].0 == self.device.time() + i as u64 {
                    match events[cursor].1 {
                        SamplerEvent::Play => {
                            state.playing = true;
                            state.position = 0.0;
                            state.playspeed = 1.0;
                        }
                        SamplerEvent::PlayAtSpeed(speed) => {
                            state.playing = true;
                            state.position = 0.0;
                            state.playspeed = speed;
                        }
                    }
                    cursor += 1
                }

                debug_assert!(self.left_samples.len() == self.right_samples.len());
                if state.position.floor() as usize >= self.left_samples.len() {
                    state.playing = false;
                }
                if state.playing {
                    left[i] = self.left_samples[state.position.floor() as usize];
                    right[i] = self.right_samples[state.position.floor() as usize];
                    state.position += 1.0 * state.playspeed;
                } else {
                    left[i] = 0.0;
                    right[i] = 0.0;
                }
            }
        }
        self.device.borrow_output()
    }
}
