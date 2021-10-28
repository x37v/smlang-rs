//! Guard and action syntax example
//!
//! An example of using guards and actions with state and event data.

#![deny(missing_docs)]

use smlang::statemachine;

/// Event data
#[derive(PartialEq)]
pub struct MyEventData(pub u32);

/// State data
#[derive(PartialEq)]
pub struct MyStateData(pub u32);

#[derive(PartialEq)]
///Events
pub enum Events {
    ///Event1 with data
    Event1(MyEventData),
    ///Event2
    Event2,
}

///guard as a function
pub fn guard(_event_data: &MyEventData) -> bool {
    todo!()
}

///action as a function
pub fn action(_event_data: &MyEventData) {
    todo!()
}

statemachine! {
    transitions: {
        *State1 + Event1(_) [guard(event)] / action(event); = State2(MyStateData(2)),
        State1 + Event2 [false] / println!("won't happen"); = State2(MyStateData(2084)),
        State2(MyStateData) + Event2 [ctx.guard(state)] / ctx.action(state); = State3,
        State2(MyStateData) + Event1(_) [true] / { println!("multiple"); println!("lines)") } = State3,
    }
}

/// Context
pub struct Context;

impl Context {
    ///context method guard
    fn guard(&mut self, _s: &MyStateData) -> bool {
        true
    }

    ///context method action
    fn action(&mut self, _s: &MyStateData) {
        todo!()
    }
}

fn main() {
    //TODO?
}
