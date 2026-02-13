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
    root_constraint: &SymbolicExpression<F>,
    parent_string: Option<&String>,
    output: &mut String,
) {
    match root_constraint {
        SymbolicExpression::Variable(v) => match *v {
            SymbolicVariable { entry, index, .. } => {
                let (name, offset_opt) = match entry {
                    Entry::Preprocessed { offset } => ("Preprocessed", Some(offset)),
                    Entry::Main { offset } => ("Main", Some(offset)),
                    Entry::Permutation { offset } => ("Permutation", Some(offset)),
                    Entry::Public => ("Public", None),
                    Entry::Challenge => ("Entry", None),
                };

                let output_string = if let Some(offset) = offset_opt {
                    format!("{name}(index: {index} offset: {offset})")
                } else {
                    format!("{name}(index: {index})")
                };

                output.push_str(
                    format!(
                        "\"{}\" -> \"{}\"\n\"{}\" [shape=\"box\",style=\"filled\",fillcolor=\"lightgreen\"]\n",
                        parent_string.unwrap_or(&String::default()),
                        output_string,
                        output_string
                    )
                    .as_str(),
                )
            }
        },
        bool_expr @ (SymbolicExpression::IsFirstRow
        | SymbolicExpression::IsLastRow
        | SymbolicExpression::IsTransition) => {
            output.push_str(
                format!(
                    "\"{}\" -> \"{:?}\"\n\"{:?}\" [shape=\"box\",style=\"filled\",fillcolor=\"pink\"]\n",
                    parent_string.unwrap_or(&String::default()),
                    bool_expr,
                    bool_expr,
                )
                .as_str(),
            );
        }
        binary_expr @ (SymbolicExpression::Mul {
            x,
            y,
            degree_multiple,
        }
        | SymbolicExpression::Add {
            x,
            y,
            degree_multiple,
        }
        | SymbolicExpression::Sub {
            x,
            y,
            degree_multiple,
        }) => {
            let name = match binary_expr {
                SymbolicExpression::Mul { .. } => "Mul",
                SymbolicExpression::Add { .. } => "Add",
                SymbolicExpression::Sub { .. } => "Sub",
                _ => unreachable!(),
            };
            output.push_str(
                format!(
                    "\"{}\" -> \"{}\"\n",
                    parent_string.unwrap_or(&String::default()),
                    name
                )
                .as_str(),
            );
            build_dotviz_graph(x, Some(&String::from(name)), output);
            build_dotviz_graph(y, Some(&String::from(name)), output);
        }
        SymbolicExpression::Constant(c) => output.push_str(
            format!(
                "\"{}\" -> \"Const({})\"\n\"Const({})\" [shape=\"box\",style=\"filled\",fillcolor=\"lightblue\"]\n",
                parent_string.unwrap_or(&String::default()),
                c,
                c
            )
            .as_str(),
        ),
        SymbolicExpression::Neg { x, degree_multiple } => {
            output.push_str(
                format!(
                    "\"{}\" -> \"Neg\"\n",
                    parent_string.unwrap_or(&String::default()),
                )
                .as_str(),
            );
            build_dotviz_graph(x, Some(&String::from("Neg")), output);
        }
    }
}

fn main() {
    let num_steps = 8; // Choose the number of Fibonacci steps
    let final_value = 21; // Choose the final Fibonacci value
    let air = FibonacciAir {
        num_steps,
        final_value,
    };

    type Val = Mersenne31;
    let constraints = get_symbolic_constraints::<Val, FibonacciAir>(&air, 2, 0);

    // let mut n = 1;
    // let k = constraints.iter().next();
    let mut output_string = String::new();
    build_dotviz_graph(&constraints[0], None, &mut output_string);
    println!("{}", output_string);

    fs::write("./constraints.gv", output_string).expect("File write should work.");
    // verify(&config, &air, &proof, &vec![])
}
