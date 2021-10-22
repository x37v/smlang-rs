//! Event pattern example

use smlang::statemachine;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NoteEventData {
    /// num
    pub num: u8,
    /// vel
    pub vel: u8,
}

#[derive(Debug)]
pub struct Button {
    pub index: usize,
    pub down: bool,
}

pub enum Events {
    ButtonEvent(Button),
    NoteEvent(NoteEventData),
    FooEvent(&'static str),
    BarEvent(usize),
}

/// Context
pub struct Context;

statemachine! {
   transitions: {
        *State1 + ButtonEvent(Button { down: true, .. }) / {ctx.action(event_data)} = State2,
        State1 + ButtonEvent(Button { .. }) [!event_data.down] / {ctx.action(event_data)} = State3(2),
        State1 + FooEvent("blah") = State3(30),
        State3(usize) + ButtonEvent(Button { down: true, ..}) [event_data.index == 20 && state_data < 20]
            / { ctx.action(event_data); println!("foo {}", state_data) } = State3(ctx.action2(state_data, event_data)),
        State3(usize) + ButtonEvent(Button { down: false, ..}) = State3(state_data + 1),

        //can't express State3(0) + FooEvent = State1 but can use a guard
        State3(usize) + FooEvent("blah") [state_data == 0] = State1,
        State5(NoteEventData) + FooEvent("blah") [state_data.num == 0] = State1,
        State5(NoteEventData) + BarEvent(0) = State1,
   }
}

impl Context {
    pub fn action(&mut self, btn: &Button) {
        println!("action {:?}", btn);
    }

    pub fn action2(&mut self, state_data: usize, btn: &Button) -> usize {
        println!("action2 {} {:?}", state_data, btn);
        2084
    }
}

fn main() {
    let mut sm = StateMachine::new(Context);
    let result = sm.process_event(Events::ButtonEvent(Button {
        index: 0,
        down: false,
    }));
    assert_eq!(Ok(&States::State3(2)), result);

    let result = sm.process_event(Events::FooEvent(&"blah"));
    assert_eq!(Err(Error::InvalidEvent), result);

    let result = sm.process_event(Events::ButtonEvent(Button {
        index: 0,
        down: false,
    }));
    assert_eq!(Ok(&States::State3(3)), result);

    let result = sm.process_event(Events::ButtonEvent(Button {
        index: 0,
        down: false,
    }));
    assert_eq!(Ok(&States::State3(4)), result);

    let result = sm.process_event(Events::ButtonEvent(Button {
        index: 0,
        down: true,
    }));
    assert_eq!(Err(Error::InvalidEvent), result);

    let result = sm.process_event(Events::ButtonEvent(Button {
        index: 20,
        down: true,
    }));
    assert_eq!(Ok(&States::State3(2084)), result);
}
