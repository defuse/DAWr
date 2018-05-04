
use std::rc::Rc;
use device;
use device::*;
use clock::Clock;
use events::EventSource;
use consts;
use std::cell::*;

// Provides full-resolution down to 21 Hz for 44.1kHz sample rate.
const WAVE_SAMPLES : usize = 2010;

pub enum NoteEvent {
    NoteOn(f32),
    NoteOff
}

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

    pub fn saw() -> Self {
        let mut samples = [0.0; WAVE_SAMPLES];
        for i in 0..samples.len() {
            samples[i] = 1.0 - 2.0*(i as f32 / (samples.len() - 1) as f32);
        }
        Self { samples }
    }

    pub fn triangle() -> Self {
        let mut samples = [0.0; WAVE_SAMPLES];
        debug_assert!(samples.len() % 2 == 0);
        for i in 0..samples.len() {
            if i < samples.len() / 2 {
                samples[i] = -1.0 + 2.0 * (i as f32 / (samples.len() / 2 - 1) as f32);
            } else {
                samples[i] = samples[samples.len() - i - 1]
            }
        }
        Self { samples }
    }

    pub fn zero() -> Self {
        Self { samples: [0.0; WAVE_SAMPLES] }
    }
}

#[derive(Clone)]
pub struct WaveTable {
    waves: Vec<Wave>
    // TODO: ensure that waves.len() == WAVE_SAMPLES
}

impl WaveTable {
    pub fn new(waves: Vec<Wave>) -> Self {
        assert!(waves.len() >= 1);
        Self { waves }
    }
}

pub struct Envelope {
    device: MonoStateContainer<EnvelopeState>,
    // TODO: attack and release
    note_events: Rc<EventSource<NoteEvent>>
}

struct EnvelopeState {
    on: bool
}

impl Envelope {
    pub fn new (clock: Rc<Clock>, note_events: Rc<EventSource<NoteEvent>>) -> Self {
        Self { device: MonoStateContainer::<EnvelopeState>::new(clock, EnvelopeState { on: false }), note_events }
    }
}

impl MonoEmitter for Envelope {
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
    device: MonoStateContainer<OscillatorState>,
    note_events: Rc<EventSource<NoteEvent>>,
    detune_multiplier: Rc<MonoEmitter>,
}

struct OscillatorState {
    position: f32,
    frequency: f32
}

impl Oscillator {
    pub fn new(clock: Rc<Clock>, note_events: Rc<EventSource<NoteEvent>>, detune_multiplier: Rc<MonoEmitter>) -> Self {
        Self {
            device: MonoStateContainer::<OscillatorState>::new(clock, OscillatorState { position: 0.0, frequency: 0.0 }),
            note_events,
            detune_multiplier
        }
    }
}


impl MonoEmitter for Oscillator {
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
    device: MonoStateContainer<()>,
    wavetable: WaveTable,
    // Values in [0, WAVE_SAMPLES)
    oscillator: Rc<MonoEmitter>,
    // Values in [0, 1], 0 being the first wave, 1 being the last.
    wavetable_position: Rc<MonoEmitter>,
    // Values in [0, 1].
    envelope: Rc<MonoEmitter>,
}

impl MonoSynth {
    pub fn new(clock: Rc<Clock>, wavetable: WaveTable, oscillator: Rc<MonoEmitter>, wavetable_position: Rc<MonoEmitter>, envelope: Rc<MonoEmitter>) -> Self {
        Self {
            device: MonoStateContainer::<()>::new(clock, ()),
            wavetable,
            oscillator,
            wavetable_position,
            envelope
        }
    }
}

impl MonoEmitter for MonoSynth {
    fn output(&self) -> Ref<Vec<f32>> {
        if self.device.clock_advanced() {
            self.device.mark_as_up_to_date();

            let mut output = self.device.borrow_to_modify();

            let wave_position = self.oscillator.output();
            let wavetable_position = self.wavetable_position.output();
            let amplitude = self.envelope.output();

            // TODO: actually implement wavetable position
            for i in 0..consts::CHUNK_SIZE {
                output[i] = amplitude[i] * self.wavetable.waves[
                    0
                    ].samples[wave_position[i] as usize];
            }

        }
        self.device.borrow_output()
    }
}

struct Lfo {

}

