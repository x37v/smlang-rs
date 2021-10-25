//! State data example
//!
//! An example of using state data together with an action.

#![deny(missing_docs)]

use smlang::statemachine;

/// State data
#[derive(PartialEq, Debug, Default)]
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
        *State1(MyStateData) + Event1 = State2(MyStateData(42)),
        State2(MyStateData) + Event2 [state_data.0 == 42] = State1(MyStateData(2084)),
        State2(MyStateData) + Event2 [state_data.0 == 2084] = State3(1),

        //hack to get around not being able to have data with terminal state..
        //add a transition that will never happen (guard is false)
        State3(usize) + Event1 [false] = State3(3)
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
