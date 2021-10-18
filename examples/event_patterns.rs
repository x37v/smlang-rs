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

/// Note event data
#[derive(PartialEq)]
pub struct NoteEventData {
    /// num
    pub num: u8,
    /// vel
    pub vel: u8,
}

#[derive(PartialEq)]
#[allow(missing_docs)]
pub enum Events {
    Event1(MyEventData),
    NoteEvent(NoteEventData),
}

statemachine! {
    transitions: {
        *State1 + Event1(MyEventData { index: 1, down: false }) / action = State3,
        State1 + Event1(MyEventData { index: 42, down: false }) / action = State2,
        State3 + Event1(MyEventData { index: 42, down: true }) / action = State1,
        State1 + NoteEvent(NoteEventData) [guard] = State5
        //State2 + Event1(MyEventData)  / action = State1
    }
}

/// Context
pub struct Context;

impl StateMachineContext for Context {
    fn action(&mut self, event_data: &MyEventData) {
        println!(
            "Got valid Event Data = {} {}",
            event_data.index, event_data.down
        );
    }
    fn guard(&mut self, event_data: &NoteEventData) -> Result<(), ()> {
        Ok(())
    }
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

    let result = sm.process_event(Events::Event1(MyEventData {
        index: 42,
        down: true,
    }));
    assert!(result == Ok(&States::State1));
}
