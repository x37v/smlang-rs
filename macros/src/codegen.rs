// Move guards to return a Result

use crate::parser::*;
use proc_macro2;
use quote::quote;
use std::vec::Vec;
use syn::Fields;

pub fn generate_code(sm: &ParsedStateMachine) -> proc_macro2::TokenStream {
    let mut state_list: Vec<_> = sm.states.iter().map(|(_, value)| value).collect();
    state_list.sort_by(|a, b| a.ident.to_string().cmp(&b.ident.to_string()));

    let i = sm.starting_state.ident.clone();
    let starting_state = match sm.starting_state.fields {
        Fields::Unit => quote! { #i },
        _ => quote! { #i(Default::default()) },
    };

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
                _ => Some(quote! { (ref state_data) }),
            };

            //create the event matches
            let events: Vec<proc_macro2::TokenStream> = trans
                .iter()
                .map(|t| {
                    let t = t.clone();
                    let eident = t.event.clone();
                    let pat = t.event_pattern.map(|p| {
                        quote! {
                            (ref mut event_data @ #p)
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
                            Some(&self.state)
                        }
                    }
                })
                .collect();

            quote! {
                States::#sident #sdata => {
                    match &mut event {
                        #(#events),*
                        _ => None
                    }
                }
            }
        })
        .collect();

    let states_attrs = &sm.states_attrs;

    // Build the states and events output
    quote! {

        /// List of auto-generated states.
        #[allow(missing_docs)]
        #[derive(PartialEq)]
        #(#states_attrs)*
        pub enum States { #(#state_list),* }

        impl Default for States {
            fn default() -> Self {
                Self::#starting_state
            }
        }


        /// State machine structure definition.
        pub struct StateMachine {
            state: States,
            context: Context
        }

        impl StateMachine {
            /// Creates a new state machine with the specified starting state.
            #[inline(always)]
            pub fn new(context: Context) -> Self {
                Self::new_with_state(context, Default::default())
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
            /// It will return `Some(&NextState)` if the transition was successful, or `None`
            /// if there was no transition.
            #[allow(unused)]
            pub fn process_event(&mut self, mut event: Events) -> Option<&States> {
                let mut ctx = &mut self.context;
                match self.state {
                    #(#transitions)*
                    _ => None,
                }
            }
        }
    }
}
