
use device::*;
use clock::*;
use std::rc::Rc;
use std::cell::*;
use consts;


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
    boost: Rc<SignalEmitter>
}

impl Gain {
    pub fn new(clock: Rc<Clock>, input: Rc<StereoEmitter>, boost: Rc<SignalEmitter>) -> Self {
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
    device: StateContainer<()>,
    value: f32
}

impl ConstSignal {
    pub fn new(clock: Rc<Clock>, value: f32) -> Self {
        Self { device: StateContainer::<()>::new(clock, ()), value: value }
    }
}

impl SignalEmitter for ConstSignal {
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
    position: Rc<SignalEmitter>,
}

impl Pan {
    pub fn new(clock: Rc<Clock>, input: Rc<StereoEmitter>, position: Rc<SignalEmitter>) -> Self {
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

            // FIXME: implement a real panning algorithm
            for i in 0..consts::CHUNK_SIZE {
                if positions[i] < -0.1 {
                    left[i] = input_left[i] + input_right[i];
                } else if positions[i] > 0.1 {
                    right[i] = input_left[i] + input_right[i]
                } else {
                    // do nothing
                }
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
