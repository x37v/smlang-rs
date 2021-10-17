//! Event pattern example

#![deny(missing_docs)]

use smlang::statemachine;

/// Event data
#[derive(PartialEq)]
pub struct MyEventData {
    /// index of button?
    pub index: usize,
    /// button down?
    pub down: bool,
}

#[derive(PartialEq)]
#[allow(missing_docs)]
pub enum Events {
    Event1(MyEventData),
}

statemachine! {
    transitions: {
        *State1 + Event1(MyEventData { index: 1, down: false }) = State3,
        State1 + Event1(MyEventData { index: 42, down: false }) = State2,
        //State2 + Event1(MyEventData)  / action = State1
    }
}

/// Context
pub struct Context;

impl StateMachineContext for Context {
    /*
    fn action(&mut self, event_data: &MyEventData) {
        println!(
            "Got valid Event Data = {} {}",
            event_data.index, event_data.down
        );
    }
    */
}

fn main() {
    let mut sm = StateMachine::new(Context);
    let result = sm.process_event(Events::Event1(MyEventData {
        index: 1,
        down: true,
    }));
    assert!(result == Err(Error::InvalidEvent));

    let result = sm.process_event(Events::Event1(MyEventData {
        index: 2084,
        down: false,
    }));
    assert!(result == Err(Error::InvalidEvent));

    let result = sm.process_event(Events::Event1(MyEventData {
        index: 1,
        down: false,
    }));
    assert!(result == Ok(&States::State3));
}
