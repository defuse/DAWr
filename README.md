# DAWr

DAWr (pronounced "door") is an audio playground for people who like to write
Rust code. It has some features of a simple DAW, including a build-in wavetable
synthesizer, sampler, and basic audio effects. However, it's still missing
a lot: there's no equalizer, the sampler only works in one-shot mode, etc.
I don't plan on adding new features for the time being, but I'd appreciate
suggestions and pull requests!

The feature set is limited, but it's enough to make a shitty future bass drop
section in Rust! You can find code for this in
[src/bin/demotrack.rs](src/bin/demotrack.rs). Here's what it sounds like:

TODO: export to wav and upload to soundcloud

## Devices

In this library, devices are things that emit a signal, which can either be mono
or stereo. Stereo is usually audio, and mono is usually some kind of control
parameter. There are two important traits, `MonoEmitter` and `StereoEmitter` for
devices which output mono or stereo respectively. There is also an `EventSource`
type representing a collection of events that occur at specific moments in time
(e.g. `NoteOn(frequency)` and `NoteOff`). The philosophy is that each device
should do one simple thing, and do it well, and then more powerful devices can
be created by chaining simpler ones.

New devices are constructed by providing references to all of their input
devices and references to all of the event sources they need to listen to. For
example, to build the `MonoSynth` device, it needs to be provided references to
an `Oscillator` (a `MonoEmitter` which tells it what its current phase in the
wave is), another `MonoEmitter` to tell it which wave of the wavetable is
selected at the current moment in time, and another `MonoEmitter` specifying the
amplitude envelope. In order to get notes to play, the amp envelope device will
listen to an `EventSource<NoteEvent>` and output a non-zero amplitude value
during the notes. We use Rust's `Rc` pointers extensively, so multiple devices
can use the same device as input.

Here is a list of all the built-in devices:

### Mono Sources

- `ConstSignal`: Outputs a constant-valued mono signal.
- `Envelope`: Listens for `NoteOn` events and outputs 0.0 when there are no notes
  playing and 1.0 while a note is playing.
- `Oscillator`: For use with `MonoSynth`, outputs the current phase of the wave
  being played.

### Instruments

- `MonoSynth`: A monophonic wavetable synthesizer. You can create polyphony and
  unison by using more than one instance along with the `Pan` effect and the
  `MonoSynth`'s `Oscillator`'s detune parameter.

- `Sampler`: Listens to an `EventSource<SamplerEvent>` and plays some audio
  whenever it sees a `Play` or `PlayAtSpeed(speed)` event. In the latter case,
  the playback is sped up by a factor of `speed` (by skipping over or
  duplicating samples).

### Effects

- `Mixer`: Sums the outputs of multiple stereo inputs.
- `Gain`: Multiplies a stereo input signal with a mono input signal.
- `Pan`: Adjusts the balance of a stereo signal according to a mono input signal
  in the range [-1, 1].
- `MonoToStereo`: Duplicates a mono signal to both channels of a stereo signal.
- `StereoToMono`: Sums the left and right channels of a stereo input to create
  a mono signal.
- `WaveShaperEffect`: Applies waveshaping (e.g. hard clipping) to a stereo
  signal.

## Wishlist

This library is missing some really important stuff, like:

- High-pass and low-pass filters.
- An equalizer.
- MIDI support.
- A polyphonic synth device that listens to MIDI events, based on `MonoSynth`.

