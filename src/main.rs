

pub mod clock;
pub mod consts;
pub mod synth;
pub mod events;
pub mod device;
pub mod daw;

use std::rc::Rc;

use clock::*;
use synth::*;
use events::*;
use device::*;
use daw::*;

use portaudio as pa;

extern crate portaudio;

fn main() {
    match run() {
        Ok(_) => {},
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}

// device

fn run() -> Result<(), pa::Error> {
    let c = Rc::new(Clock::new());
    let m = MusicalTime::new(60.0);

    let mut events = Vec::<(u64, NoteEvent)>::new();
    events.push((m.calc(1, 1), NoteEvent::NoteOn(220.0)));
    events.push((m.calc(1, 2), NoteEvent::NoteOff));
    events.push((m.calc(1, 3), NoteEvent::NoteOn(440.0)));
    events.push((m.calc(1, 4), NoteEvent::NoteOff));
    events.push((m.calc(1, 5), NoteEvent::NoteOn(220.0)));
    events.push((m.calc(1, 6), NoteEvent::NoteOff));
    events.push((m.calc(1, 7), NoteEvent::NoteOn(440.0)));
    events.push((m.calc(1, 8), NoteEvent::NoteOff));
    events.push((m.calc(1, 9), NoteEvent::NoteOn(220.0)));
    events.push((m.calc(1, 16), NoteEvent::NoteOff));
    events.push((m.calc(2, 1), NoteEvent::NoteOn(440.0)));
    events.push((m.calc(2, 2), NoteEvent::NoteOff));
    events.push((m.calc(2, 3), NoteEvent::NoteOn(220.0)));
    events.push((m.calc(2, 4), NoteEvent::NoteOff));
    events.push((m.calc(2, 5), NoteEvent::NoteOn(440.0)));
    events.push((m.calc(2, 6), NoteEvent::NoteOff));
    events.push((m.calc(2, 7), NoteEvent::NoteOn(220.0)));
    events.push((m.calc(2, 8), NoteEvent::NoteOff));
    events.push((m.calc(2, 9), NoteEvent::NoteOn(440.0)));
    events.push((m.calc(2, 16), NoteEvent::NoteOff));

    let mut eventsfifth = Vec::<(u64, NoteEvent)>::new();
    eventsfifth.push((m.calc(1, 1), NoteEvent::NoteOn(2.0*1.5*220.0)));
    eventsfifth.push((m.calc(1, 2), NoteEvent::NoteOff));
    eventsfifth.push((m.calc(1, 3), NoteEvent::NoteOn(2.0*1.5*440.0)));
    eventsfifth.push((m.calc(1, 4), NoteEvent::NoteOff));
    eventsfifth.push((m.calc(1, 5), NoteEvent::NoteOn(2.0*1.5*220.0)));
    eventsfifth.push((m.calc(1, 6), NoteEvent::NoteOff));
    eventsfifth.push((m.calc(1, 7), NoteEvent::NoteOn(2.0*1.5*440.0)));
    eventsfifth.push((m.calc(1, 8), NoteEvent::NoteOff));
    eventsfifth.push((m.calc(1, 9), NoteEvent::NoteOn(2.0*1.5*220.0)));
    eventsfifth.push((m.calc(1, 16), NoteEvent::NoteOff));
    eventsfifth.push((m.calc(2, 1), NoteEvent::NoteOn(2.0*1.5*440.0)));
    eventsfifth.push((m.calc(2, 2), NoteEvent::NoteOff));
    eventsfifth.push((m.calc(2, 3), NoteEvent::NoteOn(2.0*1.5*220.0)));
    eventsfifth.push((m.calc(2, 4), NoteEvent::NoteOff));
    eventsfifth.push((m.calc(2, 5), NoteEvent::NoteOn(2.0*1.5*440.0)));
    eventsfifth.push((m.calc(2, 6), NoteEvent::NoteOff));
    eventsfifth.push((m.calc(2, 7), NoteEvent::NoteOn(2.0*1.5*220.0)));
    eventsfifth.push((m.calc(2, 8), NoteEvent::NoteOff));
    eventsfifth.push((m.calc(2, 9), NoteEvent::NoteOn(2.0*1.5*440.0)));
    eventsfifth.push((m.calc(2, 16), NoteEvent::NoteOff));

    let es5 = Rc::new(EventSource::new(eventsfifth, c.clone()));
    let osc5 = Rc::new(Oscillator::new(c.clone(), es5.clone(), Rc::new(ConstSignal::new(c.clone(), 0.5))));
    let wtp5 = Rc::new(ConstSignal::new(c.clone(), 0.0));
    let wave5 = Wave::triangle();
    let wavetable5 = WaveTable::new(vec![wave5]);
    let env5 = Rc::new(Envelope::new(c.clone(), es5.clone()));
    let n5 = Rc::new(MonoSynth::new(c.clone(), wavetable5.clone(), osc5.clone(), wtp5.clone(), env5.clone()));
    let gn5 = Rc::new(Gain::new(c.clone(), Rc::new(MonoToStereo::new(c.clone(), n5.clone())), Rc::new(ConstSignal::new(c.clone(), 1.0))));

    let es = Rc::new(EventSource::new(events, c.clone()));
    //let n = Rc::new(StupidOsc::new(c.clone(), es.clone()));
    let wave = Wave::saw();
    let wavetable = WaveTable::new(vec![wave]);
    let detune_multiplier_1 = Rc::new(ConstSignal::new(c.clone(), 1.01));
    let detune_multiplier_2 = Rc::new(ConstSignal::new(c.clone(), 0.98));
    let wavetable_position = Rc::new(ConstSignal::new(c.clone(), 0.0));
    let oscillator1 = Rc::new(Oscillator::new(c.clone(), es.clone(), detune_multiplier_1.clone()));
    let oscillator2 = Rc::new(Oscillator::new(c.clone(), es.clone(), detune_multiplier_2.clone()));
    let envelope = Rc::new(Envelope::new(c.clone(), es.clone()));
    let n1 = Rc::new(MonoSynth::new(c.clone(), wavetable.clone(), oscillator1.clone(), wavetable_position.clone(), envelope.clone()));
    let n2 = Rc::new(MonoSynth::new(c.clone(), wavetable.clone(), oscillator2.clone(), wavetable_position.clone(), envelope.clone()));

    let n1s = Rc::new(MonoToStereo::new(c.clone(), n1.clone()));
    let n2s = Rc::new(MonoToStereo::new(c.clone(), n2.clone()));

    let left = Rc::new(ConstSignal::new(c.clone(), -1.0));
    let right = Rc::new(ConstSignal::new(c.clone(), 1.0));

    let nn1 = Rc::new(Pan::new(c.clone(), n1s, left));
    let nn2 = Rc::new(Pan::new(c.clone(), n2s, right));

    let nn1shaped = Rc::new(WaveShaperEffect::new(
            c.clone(),
            Rc::new(Gain::new(c.clone(), nn1.clone(), Rc::new(ConstSignal::new(c.clone(), 1.0)))),
            Rc::new(HardClipper { })
        ));

    let nn2shaped = Rc::new(WaveShaperEffect::new(
            c.clone(),
            Rc::new(Gain::new(c.clone(), nn2.clone(), Rc::new(ConstSignal::new(c.clone(), 1.0)))),
            Rc::new(HardClipper { })
        ));

    let mix = Rc::new(Mixer::new(c.clone(), vec![nn1shaped, nn2shaped, gn5]));
    let mastergain = Rc::new(ConstSignal::new(c.clone(), 0.1));
    let master = Rc::new(Gain::new(c.clone(), mix.clone(), mastergain.clone()));

    let pa = try!(pa::PortAudio::new());
    let mut settings = try!(pa.default_output_stream_settings(
            2, // num channels
            consts::SAMPLE_RATE as f64,
            consts::CHUNK_SIZE as u32
        ));
    settings.flags = pa::stream_flags::CLIP_OFF;


    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let left = master.output().0;
        let right = master.output().1;
        assert_eq!(frames, consts::CHUNK_SIZE);
        for f in 0..frames {
            if left[f].abs() > 1.0 || right[f].abs() > 1.0 {
                println!("WARNING: The signal is clipping!");
            }
            buffer[2*f] = left[f];
            buffer[2*f+1] = right[f];
        }
        c.increment();
        pa::Continue
    };

    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));
    try!(stream.start());

    println!("Playing for 5 seconds.");
    pa.sleep(5 * 1_000);

    try!(stream.stop());
    try!(stream.close());

    println!("Finished!");

    Ok(())
}
