# smlang: A `no_std` State Machine Language DSL in Rust

[![Build Status](https://travis-ci.org/korken89/smlang-rs.svg?branch=master)](https://travis-ci.org/korken89/smlang-rs)

> A state machine language DSL based on the syntax of [Boost-SML](https://boost-ext.github.io/sml/).

## Aim

The aim of this DSL is to facilitate the use of state machines, as they quite fast can become overly complicated to write and get an overview of.

## Transition DSL

The DSL is defined as follows:

```rust
statemachine!{
    transitions: {
        *SrcState1 + Event1 [ guard() ] / action(); = DstState2, // * denotes starting state
        SrcState2 + Event2(_) [ ctx.guard2(event) ] / { ctx.action2(event); println!("{}", event) } = DstState1,
    }
    // ...
}
```

Where `[ guard() ]` and `/ action();` are optional and can be left out. A `guard` is a function which returns `true` if the state transition should happen, and `false` if the transition should not happen, while `actions` are code that run during the transition which are guaranteed to finish before entering the new state.

> This implies that any state machine must be written as a list of transitions.

### State machine context

The state machine needs a context to be defined.
This is simply a struct that you define called `Context`. You can access the context via the variable `ctx` in your guards and actions.

```rust
statemachine!{
    transitions: {
        State1 + Event1 [ctx.guard()] / ctx.action(); = State2,
    }
    // ...
}

pub struct Context;
pub enum Events {
  Event1
}

impl Context {
  pub fn guard(&self) -> bool {
    true
  }
  pub fn action(&self) {
    //TODO
  }
}

fn main() {
    let mut sm = StateMachine::new(Context);

    // ...
}
```

See example `examples/context.rs` for a usage example.

### States

An enum `States` is automatically generated based on the entries in your DSL.
One note is that at the time of this writing there is no way to specify a terminal state with data.

### State data

Any state may have some data associated with it, which means that this data is only exists while in this state.
You can access the state data in your actions and guards via the variable `state`.
You can also set the destination state value via an expression.

If your initial state has data, that data type must have `Default` implemented.

```rust
#[derive(Default, PartialEq, Clone)]
pub struct MyStateData(pub u32);

statemachine! {
    transitions: {
        *State1(MyStateData) + Event1 = State2(state.clone()),
        State2(MyStateData) + Event2 [state.0 == 42] = State1(MyStateData(2084)),
        State2(MyStateData) + Event2 [state.0 == 2084] = State3(1),

        //hack to get around not being able to have data with terminal state..
        //add a transition that will never happen (guard is false)
        State3(usize) + Event1 [false] = State3(3)
        // ...
    }
}
```

See example `examples/state_with_data.rs` for a usage example.

### Events

You must define an enum named `Events` that encapsulates the events you wish to use.
Any Event you supply in the DSL will be prefixed with `Events` to create valid Rust code.

### Event data

Data may be passed along with an event into the `guard` and `action`, it is accessed via the `event` variable:

```rust
pub struct MyEventData(pub u32);

pub enum Events {
    Event1(MyEventData),
}

statemachine!{
    transitions: {
        State1 + Event1(_) [ctx.guard(event)] = State2,
    }
    // ...
}
```

Event data may also have associated lifetimes. This means the following will also work:

```rust
pub struct MyReferenceWrapper<'a>(pub &'a u32);

pub enum Events<'a, 'b> {
    Event1(&'a [u8]),
    Event2(MyReferenceWrapper<'b>),
}

statemachine!{
    transitions: {
        State1 + Event2(_) [ctx.guard1(event)] = State2,
        State1 + Event1(_) [ctx.guard2(event)] = State3,
    }
    // ...
}
```

See example `examples/event_with_data.rs` for a usage example.

### Guard and Action syntax

See example `examples/guard_action_syntax.rs` for a usage-example.

## State Machine Examples

Here are some examples of state machines converted from UML to the State Machine Language DSL. Runnable versions of each example is available in the `examples` folder.

### Linear state machine

![alt text](./docs/sm1.png "")

DSL implementation:

```rust
statemachine!{
    transitions: {
        *State1 + Event1 = State2,
        State2 + Event2 = State3,
    }
}
```

This example is available in `ex1.rs`.

### Looping state machine

![alt text](./docs/sm2.png "")

DSL implementation:

```rust
statemachine!{
    transitions: {
        *State1 + Event1 = State2,
        State2 + Event2 = State3,
        State3 + Event3 = State2,
    }
}
```

This example is available in `ex2.rs`.

### Using guards and actions

![alt text](./docs/sm3.png "")

DSL implementation:

```rust
statemachine!{
    transitions: {
        *State1 + Event1 [ctx.guard()] / ctx.action1(); = State2,
        State2 + Event2 [ctx.guard_fail()] / ctx.action2(); = State3,
    }
}
```

This example is available in `ex3.rs`.

## Contributors

List of contributors in alphabetical order:

* Emil Fresk ([@korken89](https://github.com/korken89))
* Mathias Koch ([@MathiasKoch](https://github.com/MathiasKoch))
* Alex Norman ([@x37v](https://github.com/x37v))

---

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

