extern crate smlang;

use smlang::statemachine;

#[derive(PartialEq)]
pub enum Events {
    Event1,
}

pub struct Context;

statemachine! {
    transitions: {
        *State1 + Event1 = State2(u32), //~ This state has data associated, but not action is define here to provide it.
    }
}

fn main() {}
