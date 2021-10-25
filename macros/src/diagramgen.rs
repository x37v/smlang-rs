use crate::parser::*;
use quote::quote;
use std::collections::VecDeque;

fn escape(v: String) -> String {
    let mut out = v.clone();
    for c in ["(", ")", "|", "[", "]", "{", "}"].iter() {
        out = out.replace(c, format!("\\{}", c).as_str());
    }
    out = out.replace("\"", "\\\"");
    out
}

/// Generates a string containing 'dot' syntax to generate a statemachine diagram with graphviz.
pub fn generate_diagram(sm: &ParsedStateMachine) -> String {
    let mapping = &sm.states_events_mapping;

    let diagram_states = sm.states.iter().map(|s| s.0);
    let mut diagram_events = VecDeque::new();
    let mut diagram_transitions = vec![];
    let mut index = 1;
    for (state, events) in mapping {
        for eventmapping in events {
            let mut label = eventmapping.event.to_string();

            if let Some(p) = &eventmapping.event_pattern {
                label += format!("({})", escape(quote! {#p}.to_string())).as_str();
            };

            if let Some(guard) = &eventmapping.guard {
                label += format!("[{}]", escape(quote! {#guard}.to_string())).as_str();
            };

            if let Some(actions) = &eventmapping.actions {
                label += format!(" / {}", escape(quote! {#actions}.to_string())).as_str();
            };
            label += format!(" = {}", eventmapping.out_state.to_string()).as_str();
            if let Some(e) = &eventmapping.out_state_data_expr {
                label += quote! {(#e)}.to_string().as_str();
            };

            let slabel = format!("{}", index);
            diagram_transitions.push((state, eventmapping.out_state.to_string(), slabel.clone()));
            diagram_events.push_front((slabel, label));
            index += 1;
        }
    }

    let state_string = diagram_states
        .map(|s| {
            format!(
                "\t{} [shape=box color=\"red\" fillcolor=\"#ffbb33\" style=filled]",
                s
            )
        })
        .collect::<Vec<String>>();
    let event_string = diagram_events
        .iter()
        .map(|s| format!("\t{0} [shape=box label=\"{0}: {1}\"]", s.0, s.1))
        .collect::<Vec<String>>();
    let transition_string = diagram_transitions
        .iter()
        .map(|t| format!("\t{0} -> {1} [color=blue label={2}];", t.0, t.1, t.2))
        .collect::<Vec<String>>();

    format!(
        "digraph G {{
    rankdir=\"LR\";
    node [fontname=Arial];
    edge [fontname=Arial];
    s [shape=circle size=2 color=\"black\" style=filled]
    
    s -> {}
{}

{}

{}
}}",
        sm.starting_state.ident,
        state_string.join("\n"),
        event_string.join("\n"),
        transition_string.join("\n")
    )
}
