use crate::parser::*;
use quote::quote;

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
    let mut diagram_events = vec![];
    let mut diagram_transitions = vec![];
    for (state, events) in mapping {
        for eventmapping in events {
            let e = (
                eventmapping.event.to_string(),
                eventmapping
                    .guard
                    .as_ref()
                    .map(|i| escape(quote! {#i}.to_string()))
                    .unwrap_or_else(|| "_".to_string()),
                eventmapping
                    .actions
                    .as_ref()
                    .map(|i| escape(quote! {#i}.to_string()))
                    .unwrap_or_else(|| "_".to_string()),
            );
            let mut label = e.0.clone();

            if let Some(p) = &eventmapping.event_pattern {
                label += format!("({})", escape(quote! {#p}.to_string())).as_str();
            };

            if let Some(guard) = &eventmapping.guard {
                label += format!("[{}]", escape(quote! {#guard}.to_string())).as_str();
            };

            if let Some(actions) = &eventmapping.actions {
                label += format!(" / {}", escape(quote! {#actions}.to_string())).as_str();
            };
            if let Some(e) = &eventmapping.out_state_data_expr {
                let s = &eventmapping.out_state;
                label += format!(" -> {}", escape(quote! {#s(#e)}.to_string())).as_str();
            };

            diagram_transitions.push((
                state,
                eventmapping.out_state.to_string(),
                format!("\"{}\"", label),
            ));
            diagram_events.push(e);
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
        .map(|s| {
            format!(
                "\t{0} [shape=box label=\"{0}\\n[{1}] / {2}\"]",
                s.0, s.1, s.2
            )
        })
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
