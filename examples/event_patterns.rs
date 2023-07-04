//! Event pattern example

use smlang::statemachine;

#[derive(Debug, PartialEq, Clone)]
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
    //add extra attributes to apply to the states enum
    states_attr: #[derive(Debug, Clone)],
    transitions: {
        *State1 + ButtonEvent(Button { down: true, .. }) / ctx.action(event); = State2,
        State1 + ButtonEvent(_) [!event.down] / {ctx.action(event)} = State3(2),
        State1 + FooEvent("blah") = State3(30),
        State3(usize) + ButtonEvent(Button { down: true, ..}) [event.index >= 20 && *state < 20]
            / { ctx.action(event); println!("foo {}", state) } = State3(ctx.action2(*state, event)),
        State3(usize) + ButtonEvent(Button { down: false, ..}) = State3(*state + 1),

        //can't express State3(0) + FooEvent = State1 but can use a guard
        State3(usize) + FooEvent("blah") [state == &0] = State1,
        State5(NoteEventData) + FooEvent("blah") [state.num == 0] = State1,
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
    let mut sm = StateMachine::new();
    let mut ctx = &mut Context;

    let result = sm.process_event(
        Events::ButtonEvent(Button {
            index: 0,
            down: false,
        }),
        &mut ctx,
    );

    assert_eq!(Some(&States::State3(2)), result);

    let result = sm.process_event(Events::FooEvent(&"blah"), &mut ctx);
    assert_eq!(None, result);

    let result = sm.process_event(
        Events::ButtonEvent(Button {
            index: 0,
            down: false,
        }),
        &mut ctx,
    );
    assert_eq!(Some(&States::State3(3)), result);

    let result = sm.process_event(
        Events::ButtonEvent(Button {
            index: 0,
            down: false,
        }),
        &mut ctx,
    );
    assert_eq!(Some(&States::State3(4)), result);

    let result = sm.process_event(
        Events::ButtonEvent(Button {
            index: 0,
            down: true,
        }),
        &mut ctx,
    );
    assert_eq!(None, result);

    let result = sm.process_event(
        Events::ButtonEvent(Button {
            index: 20,
            down: true,
        }),
        &mut ctx,
    );
    assert_eq!(Some(&States::State3(2084)), result);
}
