//! State data example
//!
//! An example of using state data together with an action.

#![deny(missing_docs)]

use smlang::statemachine;

/// State data
#[derive(PartialEq, Debug, Clone)]
pub struct MyStateData(pub u32);

// you must implement (or derive) Default to have an initial state with data
impl Default for MyStateData {
    fn default() -> Self {
        Self(20)
    }
}

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
        *State1(MyStateData) + Event1 = State2(state.clone()),
        State2(MyStateData) + Event2 [state.0 == 42] = State1(MyStateData(2084)),
        State2(MyStateData) + Event2 [state.0 == 2084] = State3(1),

        //hack to get around not being able to have data with terminal state..
        //add a transition that will never happen (guard is false)
        State3(usize) + Event1 [false] = State3(3)
        // ...
    }
}

/// Context
pub struct Context;

fn main() {
    let mut sm = StateMachine::new();
    let mut context = Context;

    let result = sm.process_event(Events::Event1, &mut context);

    assert!(result == Some(&States::State2(MyStateData(42))));
}
