use p3_field::Field;
use p3_uni_stark::{Entry, SymbolicExpression, SymbolicVariable};

fn build_dotviz_graph<F: Field>(
    node: &SymbolicExpression<F>,
    parent_id: Option<usize>,
    output: &mut String,
    counter: &mut usize,
) {
    let current_id = *counter;
    *counter += 1;

    let (label, attributes) = match node {
        SymbolicExpression::Variable(v) => match *v {
            SymbolicVariable { entry, index, .. } => {
                let (name, offset_opt) = match entry {
                    Entry::Preprocessed { offset } => ("Preprocessed", Some(offset)),
                    Entry::Main { offset } => ("Main", Some(offset)),
                    Entry::Permutation { offset } => ("Permutation", Some(offset)),
                    Entry::Public => ("Public", None),
                    Entry::Challenge => ("Entry", None),
                };

                let text = if let Some(offset) = offset_opt {
                    format!("{name}(idx: {index} off: {offset})")
                } else {
                    format!("{name}(idx: {index})")
                };
                (
                    text,
                    ",shape=\"box\",style=\"filled\",fillcolor=\"lightgreen\"",
                )
            }
        },
        SymbolicExpression::IsFirstRow => (
            "IsFirstRow".to_string(),
            ",shape=\"box\",style=\"filled\",fillcolor=\"pink\"",
        ),
        SymbolicExpression::IsLastRow => (
            "IsLastRow".to_string(),
            ",shape=\"box\",style=\"filled\",fillcolor=\"pink\"",
        ),
        SymbolicExpression::IsTransition => (
            "IsTransition".to_string(),
            ",shape=\"box\",style=\"filled\",fillcolor=\"pink\"",
        ),

        SymbolicExpression::Add { .. } => ("Add".to_string(), ""),
        SymbolicExpression::Sub { .. } => ("Sub".to_string(), ""),
        SymbolicExpression::Mul { .. } => ("Mul".to_string(), ""),
        SymbolicExpression::Neg { .. } => ("Neg".to_string(), ""),

        SymbolicExpression::Constant(c) => (
            format!("Const({})", c),
            ",shape=\"box\",style=\"filled\",fillcolor=\"lightblue\"",
        ),
    };

    output.push_str(&format!(
        "node_{} [label=\"{}\"{}];\n",
        current_id, label, attributes
    ));

    if let Some(pid) = parent_id {
        output.push_str(&format!("node_{} -> node_{};\n", pid, current_id));
    }

    // recurse for nodes with children
    match node {
        SymbolicExpression::Add { x, y, .. }
        | SymbolicExpression::Sub { x, y, .. }
        | SymbolicExpression::Mul { x, y, .. } => {
            build_dotviz_graph(x, Some(current_id), output, counter);
            build_dotviz_graph(y, Some(current_id), output, counter);
        }
        SymbolicExpression::Neg { x, .. } => {
            build_dotviz_graph(x, Some(current_id), output, counter);
        }
        _ => {}
    }
}

pub fn build_constraints_graph<F: Field>(constraints: &Vec<SymbolicExpression<F>>) -> String {
    let mut counter = 0;
    let mut constraint_count = 1;
    let mut output = String::new();
    output.push_str("digraph {\n");
    constraints.iter().for_each(|constraint| {
        output.push_str(&format!("subgraph cluster_c{constraint_count} {{\n"));
        let parent_string = format!("Constraint {constraint_count}");
        output.push_str(&format!("node_{counter} [label=\"{parent_string}\"];\n"));
        let parent_count = counter;
        let mut constraint_output = String::new();
        counter += 1;
        build_dotviz_graph(
            constraint,
            Some(parent_count),
            &mut constraint_output,
            &mut counter,
        );
        output.push_str(&constraint_output);
        output.push_str("}\n");
        constraint_count += 1;
    });
    output.push_str("}");
    output
}
