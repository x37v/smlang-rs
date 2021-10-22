//! Context with members example
//!
//! An example of using the context structure with members for counting the number of transitions.

#![deny(missing_docs)]

use smlang::statemachine;

/// my events
pub enum Events {
    /// 1
    Event1,
    /// 2
    Event2,
}

statemachine! {
    transitions: {
        *State1 + Event1 / ctx.count_transition1(); = State2,
        State2 + Event2 / ctx.count_transition2(); = State1,
    }
}

/// Context with member
pub struct Context {
    /// Number of transitions
    pub num_transitions: usize,
}

impl Context {
    fn count_transition1(&mut self) {
        self.num_transitions += 1;
    }

    fn count_transition2(&mut self) {
        self.num_transitions += 1;
    }
}

fn main() {
    let mut sm = StateMachine::new(Context { num_transitions: 0 });

    assert!(sm.process_event(Events::Event1).is_some()); // ++
    assert!(sm.process_event(Events::Event1).is_none()); // Will fail
    assert!(sm.process_event(Events::Event2).is_some()); // ++

    assert_eq!(sm.context().num_transitions, 2);

    // ...
}
