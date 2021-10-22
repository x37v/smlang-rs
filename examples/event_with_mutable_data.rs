//! Event data example
//!
//! An example of using event data together with a guard and action.

#![deny(missing_docs)]

use smlang::statemachine;

/// Event data
#[derive(PartialEq)]
pub struct MyEventData(pub u32);

///events
pub enum Events<'a> {
    ///with mut data
    Event1(&'a mut MyEventData),
}

statemachine! {
    transitions: {
        *State1 + Event1(_) [ctx.guard(event_data)] / ctx.action(event_data); = State2,
        // ...
    }
}

/// Context
pub struct Context;

impl Context {
    //guards cannot mutate data, but can mutate context
    fn guard(&mut self, _event_data: &MyEventData) -> bool {
        true
    }

    fn action(&mut self, event_data: &mut MyEventData) {
        println!("Got valid Event Data = {}", event_data.0);
    }
}

fn main() {
    let mut sm = StateMachine::new(Context);

    let result = sm.process_event(Events::Event1(&mut MyEventData(42))); // Guard will pass

    assert!(result == Some(&States::State2));
}
