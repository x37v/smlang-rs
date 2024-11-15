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
            //get the state ident
            let state = sm.states.get(state).expect("should be able to get state");
            //see if we should capture state data
            let sdata: Option<proc_macro2::TokenStream> = match state.fields {
                Fields::Unit => None,
                _ => Some(quote! { (ref state) }),
            };
            let sident = state.ident.clone();

            //create the event matches
            let events: Vec<proc_macro2::TokenStream> = trans
                .iter()
                .map(|t| {
                    let t = t.clone();
                    let eident = t.event.clone();
                    let pat = t.event_pattern.map(|p| {
                        quote! {
                            (ref mut event @ #p)
                        }
                    });
                    let guard = t.guard.map(|a| {
                        quote! {
                            if #a
                        }
                    });

                    let actions = t.actions;

                    let transition = if let Some(out_state) = t.out_state {
                        let out_state_data_expr = t.out_state_data_expr.map(|expr| {
                            quote! {
                                (#expr)
                            }
                        });
                        quote! {
                            self.state = States::#out_state #out_state_data_expr;
                            Some(&self.state)
                        }
                    } else {
                        quote! {
                            None
                        }
                    };

                    quote! {
                        Events:: #eident #pat #guard => {
                            #actions;
                            #transition
                        }
                    }
                })
                .collect();

            quote! {
                States:: #sident #sdata => {
                    match &mut e {
                        #(#events),*
                        _ => None
                    }
                }
            }
        })
        .collect();

    let states_attrs = &sm.states_attrs;

    let process_async: Option<proc_macro2::Ident> = if transitions
        .iter()
        .find(|t| t.to_string().contains(".await"))
        .is_some()
    {
        Some(proc_macro2::Ident::new(
            "async",
            proc_macro2::Span::call_site(),
        ))
    } else {
        None
    };

    //hack in async, look for `.await`

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
            pub #process_async fn process_event(&mut self, mut e: Events) -> Option<&States> {
                let mut ctx = &mut self.context;
                match self.state {
                    #(#transitions)*
                    _ => None,
                }
            }
        }
    }
}
