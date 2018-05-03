use std::cell::*;
use std::rc::Rc;
use clock::Clock;
use consts;

pub struct StateContainer<T> {
    clock: Rc<Clock>,
    output_time: Cell<u64>,
    output: RefCell<Vec<f32>>,
    state: RefCell<T>
}

impl<T> StateContainer<T> {
    pub fn new(clock: Rc<Clock>, state: T) -> Self {
        Self {
            clock: clock,
            output_time: Cell::new(consts::TIME_INFINITY),
            output: RefCell::new(vec![0.0; consts::CHUNK_SIZE]),
            state: RefCell::new(state)
        }
    }

    pub fn clock_advanced(&self) -> bool {
        self.output_time.get() != self.clock.time()
    }

    pub fn time(&self) -> u64 {
        self.clock.time()
    }

    pub fn mark_as_up_to_date(&self) {
        self.output_time.set(self.clock.time());
    }

    pub fn borrow_output(&self) -> Ref<Vec<f32>> {
        self.output.borrow()
    }

    pub fn borrow_to_modify(&self) -> RefMut<Vec<f32>> {
        self.output.borrow_mut()
    }

    pub fn borrow_state_mut(&self) -> RefMut<T> {
        self.state.borrow_mut()
    }
}

pub trait SignalEmitter {
    // The returned Ref must go out of scope before output() can be called again.
    fn output(&self) -> Ref<Vec<f32>>;
}

pub struct StereoStateContainer<T> {
    clock: Rc<Clock>,
    output_time: Cell<u64>,
    left: RefCell<Vec<f32>>,
    right: RefCell<Vec<f32>>,
    state: RefCell<T>
}

impl<T> StereoStateContainer<T> {
    pub fn new(clock: Rc<Clock>, state: T) -> Self {
        Self {
            clock: clock,
            output_time: Cell::new(consts::TIME_INFINITY),
            left: RefCell::new(vec![0.0; consts::CHUNK_SIZE]),
            right: RefCell::new(vec![0.0; consts::CHUNK_SIZE]),
            state: RefCell::new(state)
        }
    }

    pub fn clock_advanced(&self) -> bool {
        self.output_time.get() != self.clock.time()
    }

    pub fn time(&self) -> u64 {
        self.clock.time()
    }

    pub fn mark_as_up_to_date(&self) {
        self.output_time.set(self.clock.time());
    }

    pub fn borrow_output(&self) -> (Ref<Vec<f32>>, Ref<Vec<f32>>) {
        (self.left.borrow(), self.right.borrow())
    }

    pub fn borrow_left_to_modify(&self) -> RefMut<Vec<f32>> {
        self.left.borrow_mut()
    }

    pub fn borrow_right_to_modify(&self) -> RefMut<Vec<f32>> {
        self.right.borrow_mut()
    }

    pub fn borrow_state_mut(&self) -> RefMut<T> {
        self.state.borrow_mut()
    }
}

pub trait StereoEmitter {
    fn output(&self) -> (Ref<Vec<f32>>, Ref<Vec<f32>>);
}
