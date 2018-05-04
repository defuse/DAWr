extern crate rand;
extern crate portaudio;
extern crate hound;

pub mod clock;
pub mod consts;
pub mod synth;
pub mod events;
pub mod device;
pub mod effects;
pub mod conversions;
pub mod sampler;
pub mod files;

use clock::*;
use device::*;
use std::rc::Rc;

pub fn render_audio(clock: Rc<Clock>, master: Rc<StereoEmitter>, length: usize) -> (Vec<f32>, Vec<f32>) {
    let mut left = Vec::<f32>::new();
    let mut right = Vec::<f32>::new();

    while left.len() < length {
        assert!(master.output().0.len() == master.output().1.len());
        for i in 0..master.output().0.len() {
            left.push(master.output().0[i]);
            right.push(master.output().1[i]);
        }
        clock.increment();
    }

    assert!(left.len() == right.len());

    left.truncate(length);
    right.truncate(length);

    (left, right)
}
