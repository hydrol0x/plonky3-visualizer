use graphviz_rust::{dot_generator, dot_structures};
// Code courtesy of https://github.com/BrianSeong99/Plonky3_Fibonacci.git
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{Field, PrimeCharacteristicRing};
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;
use p3_mersenne_31::Mersenne31;
use p3_uni_stark::{get_symbolic_constraints, Entry, SymbolicExpression, SymbolicVariable};
use std::fs;

pub struct FibonacciAir {
    pub num_steps: usize,
    pub final_value: u32,
}

impl<F: Field> BaseAir<F> for FibonacciAir {
    fn width(&self) -> usize {
        2 // For current and next Fibonacci number
    }
}

impl<AB: AirBuilder> Air<AB> for FibonacciAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0).unwrap();
        let next = main.row_slice(1).unwrap();

        // Enforce starting values
        builder.when_first_row().assert_eq(local[0], AB::Expr::ZERO);
        // builder.when_first_row().assert_eq(local[1], AB::Expr::ONE);

        // Enforce state transition constraints
        // builder.when_transition().assert_eq(next[0], local[1]);
        // builder
        //     .when_transition()
        //     .assert_eq(next[1], local[0] + local[1]);

        // // Constrain the final value
        // let final_value = AB::Expr::from_u32(self.final_value);
        // builder.when_last_row().assert_eq(local[1], final_value);
    }
}

pub fn generate_fibonacci_trace<F: Field>(num_steps: usize) -> RowMajorMatrix<F> {
    let mut values = Vec::with_capacity(num_steps * 2);
    let mut a = F::ZERO;
    let mut b = F::ONE;
    for _ in 0..num_steps {
        values.push(a);
        values.push(b);
        let c = a + b;
        a = b;
        b = c;
    }
    RowMajorMatrix::new(values, 2)
}

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

// fn build_constraints_graph<F: Field>(constraints: &Vec<SymbolicExpression<F>>) {
//     let mut i = 0;
//     let mut output = String::new();
//     constraints.iter().for_each(|constraint| {
//         let parent_string = format!("Constraint {i}");
//         let mut constraint_output = String::new();
//         build_dotviz_graph(constraint, Some(&parent_string), &mut constraint_output);
//         i += 1;
//         output.push_str(&constraint_output)
//     });
// }

fn main() {
    let num_steps = 8; // Choose the number of Fibonacci steps
    let final_value = 21; // Choose the final Fibonacci value
    let air = FibonacciAir {
        num_steps,
        final_value,
    };

    type Val = Mersenne31;
    let constraints = get_symbolic_constraints::<Val, FibonacciAir>(&air, 2, 0);

    let mut output_string = String::new();
    let mut counter = 0;
    build_dotviz_graph(&constraints[0], None, &mut output_string, &mut counter);
    println!("{}", output_string);

    fs::write("./constraints.gv", output_string).expect("File write should work.");
    // verify(&config, &air, &proof, &vec![])
}
