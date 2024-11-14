//! Looping state machine
//!
//! An example of using guards and actions.
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
        *State1 + Event1 [ctx.guard()] / ctx.action1(); = State2,
        State2 + Event2 [ctx.guard_fail()] / ctx.action2(); = State3,
        _ + Event3 = State4
    }
}

/// Context
pub struct Context;

impl Context {
    fn guard(&mut self) -> bool {
        // Always ok
        true
    }

    fn guard_fail(&mut self) -> bool {
        // Always fail
        false
    }

    fn action1(&mut self) {
        println!("Action 1");
    }

    fn action2(&mut self) {
        println!("Action 1");
    }
}

fn main() {
    let mut sm = StateMachine::new(Context);
    assert!(sm.state() == &States::State1);

    println!("Before action 1");

    // Go through the first guard and action
    let r = sm.process_event(Events::Event1);
    assert!(r == Some(&States::State2));

    println!("After action 1");

    println!("Before action 2");

    // The action will never run as the guard will fail
    let r = sm.process_event(Events::Event2);
    assert!(r.is_none());

    println!("After action 2");

    // Now we are stuck due to the guard never returning true
    assert!(sm.state() == &States::State2);
}
