
use std::rc::Rc;
use device;
use device::*;
use clock::Clock;
use events::EventSource;
use consts;

use std::cell::*;

pub enum NoteEvent {
    NoteOn(f32),
    NoteOff
}

pub struct StupidOsc {
    device : device::StateContainer<StupidOscState>,
    event_source: Rc<EventSource<NoteEvent>>
}

struct StupidOscState {
    position: u64,
    on: bool,
    freq: f32
}

impl StupidOsc {
    pub fn new(clock: Rc<Clock>, event_source: Rc<EventSource<NoteEvent>>) -> Self {
        Self {
            device: device::StateContainer::<StupidOscState>::new(clock, StupidOscState { position: 0, on: false, freq: 0.0 }),
            event_source: event_source
        }
    }
}

impl device::SignalEmitter for StupidOsc {
    fn output(&self) -> Ref<Vec<f32>> {
        if self.device.clock_advanced() {
            self.device.mark_as_up_to_date();

            let events = self.event_source.events_this_chunk();
            let mut cursor = 0;

            let mut chunk = self.device.borrow_to_modify();
            let mut state = self.device.borrow_state_mut();

            for i in 0..chunk.len() {
                if cursor < events.len() && events[cursor].0 == self.device.time() + i as u64 {
                    match events[cursor].1 {
                        NoteEvent::NoteOff => {
                            state.on = false;
                            state.position = 0;
                        },
                        NoteEvent::NoteOn(freq) => {
                            state.on = true;
                            state.freq = freq;
                        }
                    }
                    cursor += 1
                }
                if state.on {
                    let samples_per_period = consts::SAMPLE_RATE as f32 / state.freq;
                    chunk[i] = 1.0 - 2.0 * (state.position as f32 / samples_per_period);
                    state.position += 1;
                    if state.position as f32 >= samples_per_period {
                        state.position = 0;
                    }
                } else {
                    chunk[i] = 0.0;
                }
            }
        }
        self.device.borrow_output()
    }
}

// Provides full-resolution down to 21 Hz for 44.1kHz sample rate.
const WAVE_SAMPLES : usize = 2010;


#[derive(Clone)]
pub struct Wave {
    samples: [f32; WAVE_SAMPLES]
}

impl Wave {
    pub fn square() -> Self {
        let mut samples = [-1.0; WAVE_SAMPLES];
        for i in 0..samples.len()/2 {
            samples[i] = 1.0;
        }
        Self { samples }
    }

    // TODO: add more default types
}

#[derive(Clone)]
pub struct WaveTable {
    waves: Vec<Wave>
}

impl WaveTable {
    pub fn new(waves: Vec<Wave>) -> Self {
        assert!(waves.len() != 0);
        Self { waves }
    }
}

pub struct Envelope {
    device: StateContainer<EnvelopeState>,
    // TODO: attack and release
    note_events: Rc<EventSource<NoteEvent>>
}

struct EnvelopeState {
    on: bool
}

impl Envelope {
    pub fn new (clock: Rc<Clock>, note_events: Rc<EventSource<NoteEvent>>) -> Self {
        Self { device: StateContainer::<EnvelopeState>::new(clock, EnvelopeState { on: false }), note_events }
    }
}

impl SignalEmitter for Envelope {
    fn output(&self) -> Ref<Vec<f32>> {
        if self.device.clock_advanced() {
            self.device.mark_as_up_to_date();

            let events = self.note_events.events_this_chunk();
            let mut cursor = 0;

            let mut chunk = self.device.borrow_to_modify();
            let mut state = self.device.borrow_state_mut();


            for i in 0..chunk.len() {
                if cursor < events.len() && events[cursor].0 == self.device.time() + i as u64 {
                    match events[cursor].1 {
                        NoteEvent::NoteOff => {
                            state.on = false;
                        },
                        NoteEvent::NoteOn(freq) => {
                            state.on = true;
                        }
                    }
                    cursor += 1
                }
                if state.on {
                    chunk[i] = 1.0;
                } else {
                    chunk[i] = 0.0;
                }
            }
        }
        self.device.borrow_output()
    }
}

pub struct Oscillator {
    device: StateContainer<OscillatorState>,
    note_events: Rc<EventSource<NoteEvent>>,
    detune_multiplier: Rc<SignalEmitter>
    // TODO: retrigger
}

struct OscillatorState {
    position: f32,
    frequency: f32
}

impl Oscillator {
    pub fn new(clock: Rc<Clock>, note_events: Rc<EventSource<NoteEvent>>, detune_multiplier: Rc<SignalEmitter>) -> Self {
        Self {
            device: StateContainer::<OscillatorState>::new(clock, OscillatorState { position: 0.0, frequency: 0.0 }),
            note_events,
            detune_multiplier
        }
    }

    //fn retrigger(&mut self, event_source: Rc<EventSource<NoteEvent>>) {
    //    self.retrigger = Some(event_source)
    //}
}

// TODO: instead of this pattern with 'device', we could probably implement output() in the trait
// itself and just require an implementation of some function that takes an &mut to the buffer and
// an &mut to the state.

impl SignalEmitter for Oscillator {
    fn output(&self) -> Ref<Vec<f32>> {
        if self.device.clock_advanced() {
            self.device.mark_as_up_to_date();

            let events = self.note_events.events_this_chunk();
            let mut cursor = 0;

            let mut chunk = self.device.borrow_to_modify();
            let multipliers = self.detune_multiplier.output();
            let mut state = self.device.borrow_state_mut();

            for i in 0..chunk.len() {
                if cursor < events.len() && events[cursor].0 == self.device.time() + i as u64 {
                    match events[cursor].1 {
                        NoteEvent::NoteOff => {
                            // do nothing, osc keeps running
                        },
                        NoteEvent::NoteOn(freq) => {
                            state.frequency = freq;
                            // TODO: make it possible to turn retrigger off.
                            state.position = 0.0;
                        }
                    }
                    cursor += 1
                }

                chunk[i] = state.position;
                // XXX: possible floating point inaccuracy over the long term
                let increment = WAVE_SAMPLES as f32 * state.frequency * multipliers[i] / (consts::SAMPLE_RATE as f32);
                state.position = (state.position + increment) % (WAVE_SAMPLES as f32);

            }

        }
        self.device.borrow_output()
    }
}

pub struct MonoSynth {
    device: StereoStateContainer<()>,
    wavetable: WaveTable,
    // Values in [0, WAVE_SAMPLES)
    oscillator: Rc<SignalEmitter>,
    // Values in [0, 1], 0 being the first wave, 1 being the last.
    wavetable_position: Rc<SignalEmitter>,
    // Values in [0, 1].
    envelope: Rc<SignalEmitter>,

    // TODO: use the oscillator for detune, and Pan to pan the osc
}

impl MonoSynth {
    pub fn new(clock: Rc<Clock>, wavetable: WaveTable, oscillator: Rc<SignalEmitter>, wavetable_position: Rc<SignalEmitter>, envelope: Rc<SignalEmitter>) -> Self {
        Self {
            device: StereoStateContainer::<()>::new(clock, ()),
            wavetable,
            oscillator,
            wavetable_position,
            envelope
        }
    }
}

// FUCK XXX TODO Everything is MONO! 

impl StereoEmitter for MonoSynth {
    fn output(&self) -> (Ref<Vec<f32>>, Ref<Vec<f32>>) {
        if self.device.clock_advanced() {
            self.device.mark_as_up_to_date();

            let mut left = self.device.borrow_left_to_modify();
            let mut right = self.device.borrow_right_to_modify();

            let wave_position = self.oscillator.output();
            let wavetable_position = self.wavetable_position.output();
            let amplitude = self.envelope.output();

            // TODO: actually implement wavetable position
            for i in 0..consts::CHUNK_SIZE {
                left[i] = amplitude[i] * self.wavetable.waves[
                    0
                    ].samples[wave_position[i] as usize];
                right[i] = left[i];
            }

        }
        self.device.borrow_output()
    }
}

struct Lfo {

}

