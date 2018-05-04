
use device::*;
use clock::*;
use std::rc::Rc;
use std::cell::*;
use consts;
use conversions;


pub struct Mixer {
    inputs: Vec<Rc<StereoEmitter>>,
    device: StereoStateContainer<()>
}

impl Mixer {
    pub fn new(clock: Rc<Clock>, inputs: Vec<Rc<StereoEmitter>>) -> Self {
        Self { inputs: inputs, device: StereoStateContainer::<()>::new(clock, ()) }
    }
}

impl StereoEmitter for Mixer {
    fn output(&self) -> (Ref<Vec<f32>>, Ref<Vec<f32>>) {
        if self.device.clock_advanced() {
            self.device.mark_as_up_to_date();

            let mut left = self.device.borrow_left_to_modify();
            let mut right = self.device.borrow_right_to_modify();

            for i in 0..consts::CHUNK_SIZE {
                // TODO: just use the buffer instead of temp variables
                let mut ls = 0.0;
                for j in 0..self.inputs.len() {
                    ls += self.inputs[j].output().0[i];
                }
                left[i] = ls;

                let mut rs = 0.0;
                for j in 0..self.inputs.len() {
                    rs += self.inputs[j].output().1[i];
                }
                right[i] = rs;
            }

        }
        self.device.borrow_output()
    }
}

pub struct Gain {
    device: StereoStateContainer<()>,
    input: Rc<StereoEmitter>,
    boost: Rc<MonoEmitter>
}

impl Gain {
    pub fn new(clock: Rc<Clock>, input: Rc<StereoEmitter>, boost: Rc<MonoEmitter>) -> Self {
        Self { device: StereoStateContainer::<()>::new(clock, ()), input, boost }
    }
}

impl StereoEmitter for Gain {
    fn output(&self) -> (Ref<Vec<f32>>, Ref<Vec<f32>>) {
        if self.device.clock_advanced() {
            self.device.mark_as_up_to_date();

            let mut left = self.device.borrow_left_to_modify();
            let mut right = self.device.borrow_right_to_modify();

            let boost_buf = self.boost.output();
            let input_left = self.input.output().0;
            let input_right = self.input.output().1;

            for i in 0..consts::CHUNK_SIZE {
                left[i] = boost_buf[i]*input_left[i];
                right[i] = boost_buf[i]*input_right[i];
            }
        }
        self.device.borrow_output()
    }
}

pub struct ConstSignal {
    device: MonoStateContainer<()>,
    value: f32
}

impl ConstSignal {
    pub fn new(clock: Rc<Clock>, value: f32) -> Self {
        Self { device: MonoStateContainer::<()>::new(clock, ()), value: value }
    }
}

impl MonoEmitter for ConstSignal {
    fn output(&self) -> Ref<Vec<f32>> {
        if self.device.clock_advanced() {
            self.device.mark_as_up_to_date();
            let mut chunk = self.device.borrow_to_modify();

            for i in 0..chunk.len() {
                chunk[i] = self.value;
            }
        }
        self.device.borrow_output()
    }
}

pub struct Pan {
    device: StereoStateContainer<()>,
    input: Rc<StereoEmitter>,
    position: Rc<MonoEmitter>,
}

impl Pan {
    pub fn new(clock: Rc<Clock>, input: Rc<StereoEmitter>, position: Rc<MonoEmitter>) -> Self {
        Self { device: StereoStateContainer::<()>::new(clock, ()), input, position }
    }
}

impl StereoEmitter for Pan {
    fn output(&self) -> (Ref<Vec<f32>>, Ref<Vec<f32>>) {
        if self.device.clock_advanced() {
            self.device.mark_as_up_to_date();

            let mut left = self.device.borrow_left_to_modify();
            let mut right = self.device.borrow_right_to_modify();
            let input_left = self.input.output().0;
            let input_right = self.input.output().1;

            let positions = self.position.output();

            for i in 0..consts::CHUNK_SIZE {

                let minus3 = conversions::decibels(-3.0);

                let pan_pow2 = positions[i].powf(2.0);
                let less = minus3 - pan_pow2 * minus3;
                let more = minus3 + pan_pow2 * (1.0 - minus3);

                if positions[i] > 0.0 {
                    left[i] = input_left[i]*less;
                    right[i] = input_right[i]*more;
                } else {
                    left[i] = input_left[i]*more;
                    right[i] = input_right[i]*less;
                }
            }
        }
        self.device.borrow_output()
    }
}

pub struct MonoToStereo {
    device: StereoStateContainer<()>,
    input: Rc<MonoEmitter>
}

impl MonoToStereo {
    pub fn new(clock: Rc<Clock>, input: Rc<MonoEmitter>) -> Self {
        Self {
            device: StereoStateContainer::<()>::new(clock, ()),
            input
        }
    }
}

impl StereoEmitter for MonoToStereo {
    fn output(&self) -> (Ref<Vec<f32>>, Ref<Vec<f32>>) {
        if self.device.clock_advanced() {
            self.device.mark_as_up_to_date();

            let output = self.input.output();
            let mut left = self.device.borrow_left_to_modify();
            let mut right = self.device.borrow_right_to_modify();

            for i in 0..consts::CHUNK_SIZE {
                // TODO: I'm not sure if this is the right algorithm, but when you sum the channels
                // to mono you should recover the same signal.
                left[i] = 0.5*output[i];
                right[i] = 0.5*output[i];
            }
        }
        self.device.borrow_output()
    }
}

pub struct StereoToMono {
    device: MonoStateContainer<()>,
    input: Rc<StereoEmitter>
}

impl StereoToMono {
    pub fn new(clock: Rc<Clock>, input: Rc<StereoEmitter>) -> Self {
        Self {
            device: MonoStateContainer::<()>::new(clock, ()),
            input
        }
    }
}

impl MonoEmitter for StereoToMono {
    fn output(&self) -> Ref<Vec<f32>> {
        if self.device.clock_advanced() {
            self.device.mark_as_up_to_date();

            let mut output = self.device.borrow_to_modify();
            let left = self.input.output().0;
            let right = self.input.output().1;

            for i in 0..consts::CHUNK_SIZE {
                output[i] = left[i] + right[i];
            }
        }
        self.device.borrow_output()
    }
}


pub trait WaveShaper {
    fn shape(&self, sample: f32) -> f32;
}

pub struct HardClipper {
}

impl WaveShaper for HardClipper {
    fn shape(&self, sample: f32) -> f32 {
        if (sample > 1.0) {
            1.0
        } else if (sample < -1.0) {
            -1.0
        } else {
            sample
        }
    }
}

pub struct WaveShaperEffect {
    device: StereoStateContainer<()>,
    input: Rc<StereoEmitter>,
    shaper: Rc<WaveShaper>
}

impl WaveShaperEffect {
    pub fn new(clock: Rc<Clock>, input: Rc<StereoEmitter>, shaper: Rc<WaveShaper>) -> Self {
        Self {
            device: StereoStateContainer::<()>::new(clock, ()),
            input,
            shaper
        }
    }
}

impl StereoEmitter for WaveShaperEffect {
    fn output(&self) -> (Ref<Vec<f32>>, Ref<Vec<f32>>) {
        if self.device.clock_advanced() {
            self.device.mark_as_up_to_date();

            let input_left = self.input.output().0;
            let input_right = self.input.output().1;
            let mut left = self.device.borrow_left_to_modify();
            let mut right = self.device.borrow_right_to_modify();

            for i in 0..consts::CHUNK_SIZE {
                left[i] = self.shaper.shape(input_left[i]);
                right[i] = self.shaper.shape(input_right[i]);
            }
        }
        self.device.borrow_output()
    }
}

//
//struct Delay {
//
//}
//
//struct Fader {
//
//}
// struct SoftClipper {
//
// }
