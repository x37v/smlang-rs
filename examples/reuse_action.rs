//! Reuse the same aciton more than once
//!
//! This example shows how to use the same action in multiple transitions.

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
        *State1 + Event1 / ctx.action(); = State2,
        State1 + Event2 / ctx.action(); = State3,
        State2 + Event2 = State1,
    }
}

/// Action will increment our context
pub struct Context(usize);

impl Context {
    fn action(&mut self) {
        self.0 += 1;
    }
}

fn main() {
    let mut sm = StateMachine::new();
    let mut context = Context(0);
    assert!(sm.state() == &States::State1);
    assert!(context.0 == 0);

    // triggers action
    let r = sm.process_event(Events::Event1, &mut context);
    assert!(r == Some(&States::State2));
    assert!(context.0 == 1);

    let r = sm.process_event(Events::Event2, &mut context);
    assert!(r == Some(&States::State1));
    assert!(context.0 == 1);

    // triggers the same action
    let r = sm.process_event(Events::Event2, &mut context);
    assert!(r == Some(&States::State3));
    assert!(context.0 == 2);
}
