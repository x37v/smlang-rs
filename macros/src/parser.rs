use proc_macro2::Span;
use std::collections::HashMap;
use syn::{
    braced, bracketed, parenthesized, parse, token, Attribute, Expr, Ident, Pat, Stmt, Token,
    Variant,
};

#[derive(Debug)]
pub struct StateMachine {
    pub transitions: Vec<StateTransition>,
    pub wildcards: Vec<StateTransition>,
    pub states_attrs: Vec<Attribute>,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            transitions: Vec::new(),
            wildcards: Vec::new(),
            states_attrs: Vec::new(),
        }
    }

    pub fn add_transition(&mut self, transition: StateTransition) {
        self.transitions.push(transition);
    }

    pub fn add_wildcard(&mut self, transition: StateTransition) {
        self.wildcards.push(transition);
    }

    pub fn add_state_attrs(&mut self, attrs: Vec<Attribute>) {
        self.states_attrs.extend(attrs);
    }
}

#[derive(Debug)]
pub struct ParsedStateMachine {
    pub starting_state: Variant,

    pub states: HashMap<String, Variant>,
    pub states_events_mapping: HashMap<String, Vec<StateTransition>>,
    pub states_attrs: Vec<Attribute>,
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
            .clone()
            .expect("start state must not be wildcard");

        let mut states = HashMap::new();
        let mut states_events_mapping = HashMap::<String, Vec<StateTransition>>::new();

        //create out state variant, might get overwritten by in state
        let add_out_state = |states: &mut HashMap<String, Variant>,
                             transition: &StateTransition| {
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
        };

        for transition in sm.transitions.iter() {
            //always insert in state, it has data type
            let state = transition.in_state.clone().expect("no wildcards");
            let s = state.ident.to_string();
            states.insert(s.clone(), state);

            //create the states -> transition map
            if !states_events_mapping.contains_key(&s) {
                states_events_mapping.insert(s.clone(), Vec::new());
            }

            states_events_mapping
                .get_mut(&s)
                .unwrap()
                .push(transition.clone());

            add_out_state(&mut states, &transition);
        }

        //add wildcards
        for wc in sm.wildcards.iter() {
            for v in states_events_mapping.values_mut() {
                v.push(wc.clone());
            }
            add_out_state(&mut states, &wc);
        }

        Ok(ParsedStateMachine {
            states,
            starting_state,
            states_events_mapping,
            states_attrs: sm.states_attrs,
        })
    }
}

#[derive(Debug, Clone)]
pub struct StateTransition {
    pub start: bool,
    pub event: Ident,
    pub event_pattern: Option<Pat>,
    pub in_state: Option<Variant>,
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
        // _ + Event(OptionalPattern) [ guard ] / { actions } = DstState(OptionalExpr)

        // Input State
        // Variant or _
        let in_state: Option<Variant> = if let Ok(s) = input.parse::<Variant>() {
            Some(s)
        } else {
            input.parse::<Token![_]>().expect("underscore");
            None
        };

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
                            if transition.in_state.is_some() {
                                statemachine.add_transition(transition);
                            } else {
                                statemachine.add_wildcard(transition);
                            }

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
                "states_attr" => {
                    input.parse::<Token![:]>()?;
                    statemachine.add_state_attrs(Attribute::parse_outer(input)?);
                }
                keyword => {
                    return Err(parse::Error::new(
                        input.span(),
                        format!(
                        "Unknown keyword {}. Support keywords: [\"transitions\", \"states_attr\"]",
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
