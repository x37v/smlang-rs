//! State data example
//!
//! An example of using state data together with an action.

#![deny(missing_docs)]

use smlang::statemachine;

/// State data
#[derive(PartialEq, Debug)]
pub struct MyStateData(pub u32);

///Events
pub enum Events {
    ///1
    Event1,
    ///2
    Event2,
    ///3
    Event3,
}

statemachine! {
    transitions: {
        *State1 + Event1 = State2(MyStateData(42)),
        State2(MyStateData) + Event2 = State1,
        // ...
    }
}

/// Context
pub struct Context;

fn main() {
    let mut sm = StateMachine::new(Context);
    let result = sm.process_event(Events::Event1);

    assert!(result == Some(&States::State2(MyStateData(42))));
}
