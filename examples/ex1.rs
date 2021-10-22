//! Linear state machine
//!
//! A simple example of a state machine which will get stuck in the final state.
//! A picture depicting the state machine can be found in the README.

#![deny(missing_docs)]

use smlang::statemachine;

///Events
pub enum Events {
    ///1
    Event1,
    ///2
    Event2,
}

statemachine! {
    transitions: {
        *State1 + Event1 = State2,
        State2 + Event2 = State3,
    },
}

/// Context
pub struct Context;

fn main() {
    let mut sm = StateMachine::new(Context);
    assert!(sm.state() == &States::State1);

    let r = sm.process_event(Events::Event1);
    assert!(r == Some(&States::State2));

    let r = sm.process_event(Events::Event2);
    assert!(r == Some(&States::State3));

    // Now all events will not give any change of state
    let r = sm.process_event(Events::Event1);
    assert!(r == None);
    assert!(sm.state() == &States::State3);

    let r = sm.process_event(Events::Event2);
    assert!(r == None);
    assert!(sm.state() == &States::State3);
}
