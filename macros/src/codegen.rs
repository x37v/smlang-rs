// Move guards to return a Result

use crate::parser::*;
use proc_macro2;
use proc_macro2::Span;
use quote::quote;
use std::vec::Vec;
use syn::{punctuated::Punctuated, token::Paren, Arm, Fields, Pat, Type, TypeTuple};

pub fn generate_code(sm: &ParsedStateMachine) -> proc_macro2::TokenStream {
    let mut state_list: Vec<_> = sm.states.iter().map(|(_, value)| value).collect();
    state_list.sort_by(|a, b| a.ident.to_string().cmp(&b.ident.to_string()));

    let starting_state = &sm.starting_state;

    let transitions: Vec<proc_macro2::TokenStream> = sm
        .states_events_mapping
        .iter()
        .map(|(state, trans)| {
            let state = sm.states.get(state).unwrap();

            //get the state ident
            let sident = state.ident.clone();

            //see if we should capture state data
            let sdata: Option<proc_macro2::TokenStream> = match state.fields {
                Fields::Unit => None,
                _ => Some(quote! { (state_data) }),
            };

            //create the event matches
            let events: Vec<proc_macro2::TokenStream> = trans
                .iter()
                .map(|t| {
                    let t = t.clone();
                    let eident = t.event.clone();
                    let pat = t.event_pattern.map(|p| {
                        quote! {
                            (ref event_data @ #p)
                        }
                    });
                    let guard = t.guard.map(|a| {
                        quote! {
                            if #a
                        }
                    });

                    let actions = t.actions;
                    let out_state = t.out_state;
                    let out_state_data_expr = t.out_state_data_expr.map(|expr| {
                        quote! {
                            (#expr)
                        }
                    });
                    quote! {
                        Events:: #eident #pat #guard => {
                            #actions;
                            self.state = States::#out_state #out_state_data_expr;
                            Ok(&self.state)
                        }
                    }
                })
                .collect();

            quote! {
                States::#sident #sdata => {
                    let mut ctx = self.context_mut();
                    match &event {
                        #(#events),*
                        _ => Err(Error::InvalidEvent)
                    }
                }
            }
        })
        .collect();

    // Build the states and events output
    quote! {

        /// List of auto-generated states.
        #[allow(missing_docs)]
        #[derive(PartialEq, Clone, Debug)]
        pub enum States { #(#state_list),* }


        /// State machine structure definition.
        pub struct StateMachine {
            state: States,
            context: Context
        }

        #[derive(Debug, PartialEq)]
        pub enum Error {
            InvalidEvent
        }

        impl StateMachine {
            /// Creates a new state machine with the specified starting state.
            #[inline(always)]
            pub fn new(context: Context) -> Self {
                StateMachine {
                    state: States::#starting_state,
                    context
                }
            }

            /// Creates a new state machine with an initial state.
            #[inline(always)]
            pub fn new_with_state(context: Context, initial_state: States) -> Self {
                StateMachine {
                    state: initial_state,
                    context
                }
            }

            /// Returns the current state.
            #[inline(always)]
            pub fn state(&self) -> &States {
                &self.state
            }

            /// Returns the current context.
            #[inline(always)]
            pub fn context(&self) -> &Context {
                &self.context
            }

            /// Returns the current context as a mutable reference.
            #[inline(always)]
            pub fn context_mut(&mut self) -> &mut Context {
                &mut self.context
            }

            /// Process an event.
            ///
            /// It will return `Ok(&NextState)` if the transition was successful, or `Err(Error)`
            /// if there was an error in the transition.
            pub fn process_event(&mut self, mut event: Events) -> Result<&States, Error> {
                match self.state {
                    #(#transitions)*
                    _ => Err(Error::InvalidEvent),
                }
            }
        }
    }
}
