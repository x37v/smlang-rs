//! Event pattern example

use smlang::statemachine;

/// Note event data
#[derive(PartialEq)]
pub struct NoteEventData {
    /// num
    pub num: u8,
    /// vel
    pub vel: u8,
}

#[derive(PartialEq)]
pub enum Button {
    Shift { index: usize, down: bool },
    Pad { index: usize, down: bool },
}

#[derive(PartialEq)]
#[allow(missing_docs)]
pub enum Events {
    ButtonEvent(Button),
    NoteEvent(NoteEventData),
}

statemachine! {
    transitions: {
        *State1 + ButtonEvent(Button::Pad { index: 1, .. }) / action = State3,
        State1 + ButtonEvent(Button::Pad { index: 42, down: false }) / action = State2,
        State3 + ButtonEvent(Button::Pad { index: 42, down: true }) / action = State1,
        State1 + NoteEvent(NoteEventData) [guard] = State5,
        State1 + ButtonEvent(Button::Shift{index: 1, down: true}) = State5
        //State2 + ButtonEvent(Button::Pad)  / action = State1
    }
}

/// Context
pub struct Context;

impl StateMachineContext for Context {
    fn action(&mut self, event_data: &Button) {
        let (index, down) = match event_data {
            Button::Shift { index, down } => (index, down),
            Button::Pad { index, down } => (index, down),
        };
        println!("Got valid Button = {} {}", index, down);
    }
    fn guard(&mut self, event_data: &NoteEventData) -> Result<(), ()> {
        Ok(())
    }
}

fn main() {
    let mut sm = StateMachine::new(Context);
    let result = sm.process_event(Events::ButtonEvent(Button::Pad {
        index: 2,
        down: true,
    }));
    assert!(result == Err(Error::InvalidEvent));

    let result = sm.process_event(Events::ButtonEvent(Button::Pad {
        index: 2084,
        down: false,
    }));
    assert!(result == Err(Error::InvalidEvent));

    let result = sm.process_event(Events::ButtonEvent(Button::Pad {
        index: 1,
        down: false,
    }));
    assert!(result == Ok(&States::State3));

    let result = sm.process_event(Events::ButtonEvent(Button::Pad {
        index: 42,
        down: true,
    }));
    assert!(result == Ok(&States::State1));
}
