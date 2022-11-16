use std::cmp;

pub trait CRDT {
    type Op;
    type State;
    fn init(state: Self::State) -> Self;
    fn apply(&mut self, op: Self::Op);
    fn state(&self) -> Self::State;
    fn merge(&mut self, other: Self::State);
}

pub enum CounterOp {
    Inc,
}

pub struct Counter {
    payload: i32,
}

impl CRDT for Counter {
    type State = i32;
    type Op = CounterOp;

    fn init(state: Self::State) -> Self {
        Counter { payload: state }
    }

    fn apply(&mut self, op: Self::Op) {
        match op {
            CounterOp::Inc => self.payload += 1,
        }
    }

    fn state(&self) -> Self::State {
        self.payload
    }

    fn merge(&mut self, other: Self::State) {
        self.payload = cmp::max(self.payload, other);
    }
}
