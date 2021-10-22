use proc_macro2::Span;
use std::collections::HashMap;
use syn::{braced, bracketed, parenthesized, parse, token, Expr, Ident, Pat, Stmt, Token, Variant};

#[derive(Debug)]
pub struct StateMachine {
    pub transitions: Vec<StateTransition>,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            transitions: Vec::new(),
        }
    }

    pub fn add_transition(&mut self, transition: StateTransition) {
        self.transitions.push(transition);
    }
}

#[derive(Debug)]
pub struct ParsedStateMachine {
    pub starting_state: Variant,

    pub states: HashMap<String, Variant>,
    pub states_events_mapping: HashMap<String, Vec<StateTransition>>,
}

impl ParsedStateMachine {
    pub fn new(sm: StateMachine) -> parse::Result<Self> {
        // Check the initial state definition
        let num_start: usize = sm
            .transitions
            .iter()
            .map(|sm| if sm.start { 1 } else { 0 })
            .sum();

        if num_start == 0 {
            return Err(parse::Error::new(
                Span::call_site(),
                "No starting state defined, indicate the starting state with a *.",
            ));
        } else if num_start > 1 {
            return Err(parse::Error::new(
                Span::call_site(),
                "More than one starting state defined (indicated with *), remove duplicates.",
            ));
        }

        // Extract the starting state
        let starting_state = sm
            .transitions
            .iter()
            .find(|sm| sm.start)
            .unwrap()
            .in_state
            .clone();

        let mut states = HashMap::new();
        let mut states_events_mapping = HashMap::<String, Vec<StateTransition>>::new();

        for transition in sm.transitions.iter() {
            //always insert in state, it has data type
            let s = transition.in_state.ident.to_string();
            states.insert(s.clone(), transition.in_state.clone());

            //create the states -> transition map
            if !states_events_mapping.contains_key(&s) {
                states_events_mapping.insert(s.clone(), Vec::new());
            }
            states_events_mapping
                .get_mut(&s)
                .unwrap()
                .push(transition.clone());

            //create out state variant, might get overwritten by in state
            let s = transition.out_state.to_string();
            if !states.contains_key(&s) {
                states.insert(
                    s.clone(),
                    Variant {
                        attrs: Vec::new(),
                        ident: transition.out_state.clone(),
                        fields: syn::Fields::Unit,
                        discriminant: None,
                    },
                );
            }
        }

        Ok(ParsedStateMachine {
            states,
            starting_state,
            states_events_mapping,
        })
    }
}

#[derive(Debug, Clone)]
pub struct StateTransition {
    pub start: bool,
    pub event: Ident,
    pub event_pattern: Option<Pat>,
    pub in_state: Variant,
    pub out_state: Ident,
    pub out_state_data_expr: Option<Expr>,
    pub guard: Option<Expr>,
    pub actions: Option<Stmt>,
}

impl parse::Parse for StateTransition {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        // Check for starting state definition
        let start = input.parse::<Token![*]>().is_ok();

        // Parse the DSL
        //
        // Transition DSL:
        // SrcStateVariant + Event(OptionalPattern) [ guard ] / { actions } = DstState(OptionalExpr)

        // Input State
        let in_state: Variant = input.parse()?;

        // Event
        input.parse::<Token![+]>()?;
        let event: Ident = input.parse()?;

        //optional pattern
        let event_pattern: Option<Pat> = if input.peek(token::Paren) {
            let content;
            parenthesized!(content in input);
            Some(content.parse()?)
        } else {
            None
        };

        // Possible guard
        let guard: Option<Expr> = if input.peek(token::Bracket) {
            let content;
            bracketed!(content in input);
            Some(content.parse()?)
        } else {
            None
        };

        // Possible action
        let actions: Option<Stmt> = if let Ok(_) = input.parse::<Token![/]>() {
            Some(input.parse()?)
        } else {
            None
        };

        input.parse::<Token![=]>()?;

        let out_state: Ident = input.parse()?;
        let out_state_data_expr: Option<Expr> = if input.peek(token::Paren) {
            let content;
            parenthesized!(content in input);
            Some(content.parse()?)
        } else {
            None
        };

        Ok(StateTransition {
            start,
            in_state,
            out_state,
            out_state_data_expr,
            event,
            event_pattern,
            guard,
            actions,
        })
    }
}

impl parse::Parse for StateMachine {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let mut statemachine = StateMachine::new();

        loop {
            // If the last line ends with a comma this is true
            if input.is_empty() {
                break;
            }

            match input.parse::<Ident>()?.to_string().as_str() {
                "transitions" => {
                    input.parse::<Token![:]>()?;
                    if input.peek(token::Brace) {
                        let content;
                        braced!(content in input);
                        loop {
                            if content.is_empty() {
                                break;
                            }

                            let transition: StateTransition = content.parse()?;
                            statemachine.add_transition(transition);

                            // No comma at end of line, no more transitions
                            if content.is_empty() {
                                break;
                            }

                            if let Err(_) = content.parse::<Token![,]>() {
                                break;
                            };
                        }
                    }
                }
                //TODO states_attrs (Clone, etc)
                keyword => {
                    return Err(parse::Error::new(
                        input.span(),
                        format!(
                            "Unknown keyword {}. Support keywords: [\"transitions\"]",
                            keyword
                        ),
                    ))
                }
            }

            // No comma at end of line, no more transitions
            if input.is_empty() {
                break;
            }

            if let Err(_) = input.parse::<Token![,]>() {
                break;
            };
        }

        Ok(statemachine)
    }
}
