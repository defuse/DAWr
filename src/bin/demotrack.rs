extern crate dawr;
extern crate portaudio;

use dawr::clock::*;
use dawr::synth::*;
use dawr::events::*;
use dawr::device::*;
use dawr::effects::*;
use dawr::conversions::*;
use dawr::sampler::*;
use dawr::files;
use dawr::consts;
use std::rc::Rc;

use portaudio as pa;


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
    let c = Clock::new();

    let mut kickevents = Vec::<(u64, SamplerEvent)>::new();
    let mut snareevents = Vec::<(u64, SamplerEvent)>::new();
    let mut hatevents = Vec::<(u64, SamplerEvent)>::new();
    let mut hat2events = Vec::<(u64, SamplerEvent)>::new();
    let mut bassevents = Vec::<(u64, SamplerEvent)>::new();

    let mut notes_a = Vec::<(u64, NoteEvent)>::new();
    let mut notes_b = Vec::<(u64, NoteEvent)>::new();
    let mut notes_c = Vec::<(u64, NoteEvent)>::new();
    let mut notes_d = Vec::<(u64, NoteEvent)>::new();

    let mut m = TimeCalculator::new(160.0);
    for bar in 0..16 {
        for beat in 0..4 {
            if (bar % 2 == 0 && beat == 0) || (bar % 2 == 1 && beat == 1) || (bar % 4 == 1 && beat == 3)  {
                kickevents.push((m.add_quarters(beat as f64).time(), SamplerEvent::Play));
                let bass_speed = {
                    if bar % 2 == 0 {
                        1.0
                    } else {
                        1.5
                    }
                };
                bassevents.push((m.add_quarters(beat as f64).time(), SamplerEvent::PlayAtSpeed(bass_speed)));
            }
            if beat == 2 {
                snareevents.push((m.add_quarters(beat as f64).time(), SamplerEvent::Play));
            }
        }
        for eights in 0..8 {
            hatevents.push((m.add_eighths(eights as f64).time(), SamplerEvent::Play));
            if bar >= 4 {
                hat2events.push((m.add_eighths(eights as f64).add_sixteenths(1.0).time(), SamplerEvent::Play));
            }
        }

        const E1_FREQ : f32 = 41.20;
        let half_step : f32 = 2.0_f32.powf(1.0/12.0);

        match bar % 4 {
            0 => {
                // III7
                let shift = half_step.powf(3.0);
                for i in 0..4 {
                    notes_a.push((m.add_eighths(i as f64).time(), NoteEvent::NoteOn(shift*8.0*E1_FREQ)));
                    notes_a.push((m.add_eighths(i as f64).add_sixteenths(1.0).time(), NoteEvent::NoteOff));

                    notes_b.push((m.add_eighths(i as f64).time(), NoteEvent::NoteOn(shift*8.0*E1_FREQ * half_step.powf(4.0) )));
                    notes_b.push((m.add_eighths(i as f64).add_sixteenths(1.0).time(), NoteEvent::NoteOff));

                    notes_c.push((m.add_eighths(i as f64).time(), NoteEvent::NoteOn(shift*8.0*E1_FREQ * half_step.powf(7.0) )));
                    notes_c.push((m.add_eighths(i as f64).add_sixteenths(1.0).time(), NoteEvent::NoteOff));

                    notes_d.push((m.add_eighths(i as f64).time(), NoteEvent::NoteOn(shift*8.0*E1_FREQ * half_step.powf(11.0) )));
                    notes_d.push((m.add_eighths(i as f64).add_sixteenths(1.0).time(), NoteEvent::NoteOff));
                }
            },
            2 => {
                // i7
                notes_a.push((m.time(), NoteEvent::NoteOn(8.0*E1_FREQ)));
                notes_a.push((m.add_quarters(2.0).time(), NoteEvent::NoteOff));

                notes_b.push((m.time(), NoteEvent::NoteOn(8.0*E1_FREQ * half_step.powf(3.0) )));
                notes_b.push((m.add_quarters(2.0).time(), NoteEvent::NoteOff));

                notes_c.push((m.time(), NoteEvent::NoteOn(8.0*E1_FREQ * half_step.powf(7.0) )));
                notes_c.push((m.add_quarters(2.0).time(), NoteEvent::NoteOff));

                notes_d.push((m.time(), NoteEvent::NoteOn(8.0*E1_FREQ * half_step.powf(10.0))));
                notes_d.push((m.add_quarters(2.0).time(), NoteEvent::NoteOff));
            },
            1 | 3 => {
                let fifth = half_step.powf(7.0);
                // v7
                notes_a.push((m.add_quarters(1.0).time(), NoteEvent::NoteOn(fifth*8.0*E1_FREQ)));
                notes_a.push((m.add_quarters(2.0).time(), NoteEvent::NoteOff));

                notes_a.push((m.add_quarters(3.0).time(), NoteEvent::NoteOn(fifth*8.0*E1_FREQ)));
                notes_a.push((m.add_quarters(4.0).time(), NoteEvent::NoteOff));

                notes_b.push((m.add_quarters(1.0).time(), NoteEvent::NoteOn(fifth*8.0*E1_FREQ * half_step.powf(3.0) )));
                notes_b.push((m.add_quarters(2.0).time(), NoteEvent::NoteOff));

                notes_b.push((m.add_quarters(3.0).time(), NoteEvent::NoteOn(fifth*8.0*E1_FREQ * half_step.powf(3.0) )));
                notes_b.push((m.add_quarters(4.0).time(), NoteEvent::NoteOff));

                notes_c.push((m.add_quarters(1.0).time(), NoteEvent::NoteOn(fifth*8.0*E1_FREQ * half_step.powf(7.0) )));
                notes_c.push((m.add_quarters(2.0).time(), NoteEvent::NoteOff));

                notes_c.push((m.add_quarters(3.0).time(), NoteEvent::NoteOn(fifth*8.0*E1_FREQ * half_step.powf(7.0) )));
                notes_c.push((m.add_quarters(4.0).time(), NoteEvent::NoteOff));

                notes_d.push((m.add_quarters(1.0).time(), NoteEvent::NoteOn(fifth*8.0*E1_FREQ * half_step.powf(10.0))));
                notes_d.push((m.add_quarters(2.0).time(), NoteEvent::NoteOff));

                notes_d.push((m.add_quarters(3.0).time(), NoteEvent::NoteOn(fifth*8.0*E1_FREQ * half_step.powf(10.0))));
                notes_d.push((m.add_quarters(4.0).time(), NoteEvent::NoteOff));
            },
            _ => {
                panic!("This will never happen!");
            }
        };
        m = m.add_bars(1.0);
    }

    let (kick_l, kick_r) = files::load_wav_to_stereo("sounds/Kick.wav");
    let kick = Gain::new(
        c.clone(),
        Sampler::new(c.clone(), EventSource::new(kickevents, c.clone()), kick_l, kick_r),
        ConstSignal::new(c.clone(), decibels(6.0))
    );

    let (hat_l, hat_r) = files::load_wav_to_stereo("sounds/HiHat.wav");
    let hihat = Sampler::new(c.clone(), EventSource::new(hatevents, c.clone()), hat_l, hat_r);

    let (hat2_l, hat2_r) = files::load_wav_to_stereo("sounds/HiHat2.wav");
    let hihat2 = Sampler::new(c.clone(), EventSource::new(hat2events, c.clone()), hat2_l, hat2_r);

    let (snare_l, snare_r) = files::load_wav_to_stereo("sounds/Snare.wav");
    let snare = Gain::new(
        c.clone(),
        Sampler::new(c.clone(), EventSource::new(snareevents, c.clone()), snare_l, snare_r),
        ConstSignal::new(c.clone(), decibels(6.0))
    );

    let (bass_l, bass_r) = files::load_wav_to_stereo("sounds/808.wav");
    let bass = Gain::new(
        c.clone(),
        Sampler::new(c.clone(), EventSource::new(bassevents, c.clone()), bass_l, bass_r),
        ConstSignal::new(c.clone(), decibels(6.0))
    );

    let note_channels = vec![notes_a, notes_b, notes_c, notes_d];
    let mut synths = Vec::<Rc<StereoEmitter>>::new();

    for events in note_channels {
        let es = EventSource::new(events, c.clone());
        let voices = 4;
        for voice in 0..voices {
            let ratio = voice as f32 / (voices - 1) as f32;
            let detune = 0.99 + 0.02 * ratio;
            let pan = 1.0 - 2.0 * ratio;
            let synth = Pan::new(
                c.clone(),
                MonoToStereo::new(
                    c.clone(),
                    MonoSynth::new(
                        c.clone(),
                        WaveTable::new(vec![Wave::saw()]),
                        Oscillator::new(c.clone(), es.clone(), ConstSignal::new(c.clone(), detune)),
                        // Wavetable position.
                        ConstSignal::new(c.clone(), 0.0),
                        Envelope::new(c.clone(), es.clone())
                    )
                ),
                ConstSignal::new(c.clone(), pan)
            );
            synths.push(synth);
        }
    }

    let synths_mixed = Gain::new(
        c.clone(),
        Mixer::new(c.clone(), synths),
        ConstSignal::new(c.clone(), decibels(-10.0))
    );

    let mix = Mixer::new(c.clone(), vec![kick, hihat, hihat2, snare, bass, synths_mixed]);
    let master = WaveShaperEffect::new(
        c.clone(),
        Gain::new(
            c.clone(),
            mix.clone(),
            ConstSignal::new(c.clone(), decibels(-5.0))
        ),
        HardClipper::new()
    );

    println!("Rendering audio...");
    let (left, right) = dawr::render_audio(c, master, TimeCalculator::new(160.0).add_bars(16.0).time() as usize);

    println!("Saving audio to Output.wav...");
    dawr::files::save_stereo_to_wav(&left, &right, "Output.wav");

    let pa = try!(pa::PortAudio::new());
    let mut settings = try!(pa.default_output_stream_settings(
            2, // num channels
            consts::SAMPLE_RATE as f64,
            consts::CHUNK_SIZE as u32
        ));
    settings.flags = pa::stream_flags::CLIP_OFF;

    let mut clipped = false;

    let mut position = 0;

    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        assert_eq!(frames, consts::CHUNK_SIZE);
        for f in 0..frames {
            if position < right.len() && position < left.len() {
                let left_sample = left[position];
                let right_sample = right[position];
                position += 1;

                if clipped == false && (left_sample.abs() > 1.0 || right_sample.abs() > 1.0) {
                    clipped = true;
                    println!("WARNING: The signal is clipping!");
                }

                buffer[2*f] = left_sample;
                buffer[2*f+1] = right_sample;
            } else {
                buffer[2*f] = 0.0;
                buffer[2*f+1] = 0.0;
            }
        }
        pa::Continue
    };

    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));
    try!(stream.start());

    println!("Playing for 20 seconds.");
    pa.sleep(20 * 1_000);

    try!(stream.stop());
    try!(stream.close());

    println!("Finished!");

    Ok(())
}
