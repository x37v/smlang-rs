//! Event pattern example

use smlang::statemachine;

pub struct NoteEventData {
    /// num
    pub num: u8,
    /// vel
    pub vel: u8,
}

pub struct Button {
    index: usize,
    down: bool,
}

pub enum Events {
    ButtonEvent(Button),
    NoteEvent(NoteEventData),
    FooEvent,
}

#[derive(Clone, PartialEq)]
pub enum States {
    State1,
    State2,
    State3(usize),
}

/// Context
pub struct Context;

/*
   statemachine! {
       transitions: {
            *State1 + ButtonEvent(Button) [event_data.down] / [self.action(event_data)] = State2,
            State1 + ButtonEvent(Button) [!event_data.down] / [self.action(event_data)] = State3(20),
            State1 + FooEvent = State3(30),
            State3(usize) + ButtonEvent(Button { down: true, ..}) [!event_data.index < 20 && state_data < 20]
                / [self.action(event_data); println!("foo {}", state_data)] = State3(self.action2(state_data, event_data)),
            State3(usize) + ButtonEvent(Button { down: false, ..}) = State3(state_data),
       }
   }
*/

pub struct StateMachine {
    state: States,
}

impl StateMachine {
    fn action(&mut self, _button: &mut Button) {
        //TODO
    }

    fn action2(&mut self, state_data: usize, _button: &mut Button) -> usize {
        state_data + 23
    }
    pub fn process_event(&mut self, mut event: Events) {
        match self.state {
            States::State1 => match event {
                Events::ButtonEvent(ref mut event_data @ Button { .. }) if event_data.down => {
                    self.action(event_data);
                    self.state = States::State2;
                }
                Events::ButtonEvent(ref mut event_data @ Button { .. }) if !event_data.down => {
                    self.action(event_data);
                    self.state = States::State3(20);
                }
                Events::FooEvent => {
                    self.state = States::State3(30);
                }
                _ => (),
            },
            States::State3(state_data) => match event {
                Events::ButtonEvent(ref mut event_data @ Button { down: true, .. })
                    if event_data.index < 20 && state_data < 20 =>
                {
                    self.action(event_data);
                    self.state = States::State3(self.action2(state_data, event_data));
                }
                Events::ButtonEvent(ref mut event_data @ Button { down: false, .. }) => {
                    self.state = States::State3(state_data);
                }
                _ => (),
            },
            _ => (),
        }
    }
}

fn main() {}
