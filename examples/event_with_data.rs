//! Event data example
//!
//! An example of using event data together with a guard and action.

#![deny(missing_docs)]

use smlang::statemachine;

/// Event data
#[derive(PartialEq)]
pub struct MyEventData(pub u32);

/// Events enum
pub enum Events {
    /// First event
    Event1(MyEventData),
}

statemachine! {
    transitions: {
        *State1 + Event1(_) [event == &MyEventData(42)] / ctx.action(event); = State2,
        // ...
    }
}

/// Context
pub struct Context;

impl Context {
    /// react to event data
    fn action(&mut self, event_data: &MyEventData) {
        println!("Got valid Event Data = {}", event_data.0);
    }
}

fn main() {
    let mut sm = StateMachine::new();
    let mut context = Context;
    let result = sm.process_event(Events::Event1(MyEventData(1)), &mut context); // Guard will fail

    assert!(result == None);

    let result = sm.process_event(Events::Event1(MyEventData(42)), &mut context); // Guard will pass

    assert!(result == Some(&States::State2));
}
