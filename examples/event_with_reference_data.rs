//! Reference types in events
//!
//! A simple example of a state machine which will get events that contain references.

use smlang::statemachine;

/// Reference wrapper
#[derive(PartialEq, Debug)]
pub struct MyReferenceWrapper<'a>(pub &'a u32);

pub enum Events<'a, 'b> {
    Event1(&'a [u8]),
    Event2(MyReferenceWrapper<'b>),
}

statemachine! {
    transitions: {
        *State1 + Event1(_) [ctx.guard1(event)] / {ctx.action1(event)} = State2,
        State2 + Event2(_) [ctx.guard2(event)] / {ctx.action2(event)} = State3,
    }
}

/// Context
pub struct Context;

impl Context {
    fn guard1(&mut self, event_data: &[u8]) -> bool {
        // Only ok if the slice is not empty
        !event_data.is_empty()
    }

    fn action1(&mut self, event_data: &[u8]) {
        println!("Got valid Event Data = {:?}", event_data);
    }

    fn guard2(&mut self, event_data: &MyReferenceWrapper) -> bool {
        *event_data.0 > 9000
    }

    fn action2(&mut self, event_data: &MyReferenceWrapper) {
        println!("Got valid Event Data = {}", event_data.0);
    }
}

fn main() {
    let mut sm = StateMachine::new();
    let mut context = Context;

    let result = sm.process_event(Events::Event1(&[]), &mut context); // Guard will fail
    assert!(result == None);
    let result = sm.process_event(Events::Event1(&[1, 2, 3]), &mut context); // Guard will pass
    assert!(result == Some(&States::State2));

    let r = 42;
    let result = sm.process_event(Events::Event2(MyReferenceWrapper(&r)), &mut context); // Guard will fail
    assert!(result == None);

    let r = 9001;
    let result = sm.process_event(Events::Event2(MyReferenceWrapper(&r)), &mut context); // Guard will pass
    assert!(result == Some(&States::State3));
}
