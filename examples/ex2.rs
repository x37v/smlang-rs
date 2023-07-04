//! Looping state machine
//!
//! An example of a state machine which will loop between State 2 and State 3.
//! A picture depicting the state machine can be found in the README.

#![deny(missing_docs)]

use smlang::statemachine;

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
        *State1 + Event1 = State2,
        State2 + Event2 = State3,
        State3 + Event3 = State2,
    }
}

/// Context
pub struct Context;

fn main() {
    let mut sm = StateMachine::new();
    let mut context = Context;

    assert!(sm.state() == &States::State1);

    let r = sm.process_event(Events::Event1, &mut context);
    assert!(r == Some(&States::State2));

    let r = sm.process_event(Events::Event2, &mut context);
    assert!(r == Some(&States::State3));

    // Go back in the loop a few time
    let r = sm.process_event(Events::Event3, &mut context);
    assert!(r == Some(&States::State2));

    let r = sm.process_event(Events::Event2, &mut context);
    assert!(r == Some(&States::State3));

    let r = sm.process_event(Events::Event3, &mut context);
    assert!(r == Some(&States::State2));

    // Now we cannot use Event1 again, as it is outside the state machine loop
    let r = sm.process_event(Events::Event1, &mut context);
    assert!(r == None);
    assert!(sm.state() == &States::State2);
}
