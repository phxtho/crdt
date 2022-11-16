use crdts::{Counter, CounterOp, CRDT};
fn main() {
    const INITIAL_STATE: i32 = 0;
    let mut counter1 = Counter::init(INITIAL_STATE);
    let mut counter2 = Counter::init(INITIAL_STATE);

    for _ in 0..10 {
        counter1.apply(CounterOp::Inc);
    }

    for _ in 0..5 {
        counter2.apply(CounterOp::Inc);
    }

    counter1.merge(counter2.state());
    counter2.merge(counter1.state());

    println!("Counter 1: {}", counter1.state());
    println!("Counter 2: {}", counter2.state());
}
