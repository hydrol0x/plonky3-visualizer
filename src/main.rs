// Code courtesy of https://github.com/BrianSeong99/Plonky3_Fibonacci.git
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{Field, PrimeCharacteristicRing};
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;
use p3_mersenne_31::Mersenne31;
use p3_uni_stark::{get_symbolic_constraints, SymbolicExpression, SymbolicVariable};

mod visualizer;

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

fn traverse_constraints_tree<F: Field>(
    root_constraint: &SymbolicExpression<F>,
    parent_string: Option<&String>,
    output: &mut String,
) {
    match root_constraint {
        SymbolicExpression::Variable(v) => match *v {
            SymbolicVariable { entry, index, .. } => {
                let entry_text: String = match entry {
                    p3_uni_stark::Entry::Preprocessed { offset } => {
                        format!("Preprocessed(offset:{}, index:{})", offset, index)
                    }
                    p3_uni_stark::Entry::Main { offset } => {
                        format!("Preprocessed(offset:{}, index:{})", offset, index)
                    }
                    p3_uni_stark::Entry::Permutation { offset } => {
                        format!("Preprocessed(offset:{}, index:{})", offset, index)
                    }
                    p3_uni_stark::Entry::Public => String::from("Public"),
                    p3_uni_stark::Entry::Challenge => String::from("Entry"),
                };

                output.push_str(
                    format!(
                        "\"{}\" -> \"{}\"",
                        parent_string.unwrap_or(&String::default()),
                        entry_text
                    )
                    .as_str(),
                )
            }
        },
        SymbolicExpression::IsFirstRow => output.push_str(
            format!(
                "\"{}\" -> \"IsFirstRow\"",
                parent_string.unwrap_or(&String::default()),
            )
            .as_str(),
        ),
        SymbolicExpression::IsLastRow => output.push_str(
            format!(
                "\"{}\" -> \"IsLastRow\"",
                parent_string.unwrap_or(&String::default()),
            )
            .as_str(),
        ),
        SymbolicExpression::IsTransition => output.push_str(
            format!(
                "\"{}\" -> \"IsTransition\"",
                parent_string.unwrap_or(&String::default()),
            )
            .as_str(),
        ),
        SymbolicExpression::Constant(c) => output.push_str(
            format!(
                "\"{}\" -> \"Const({})\"",
                parent_string.unwrap_or(&String::default()),
                c
            )
            .as_str(),
        ),
        SymbolicExpression::Mul {
            x,
            y,
            degree_multiple,
        } => {
            output.push_str(
                format!(
                    "\"{}\" -> \"Mul\"",
                    parent_string.unwrap_or(&String::default()),
                )
                .as_str(),
            );
            traverse_constraints_tree(x, Some(&String::from("Mul")), output);
            traverse_constraints_tree(y, Some(&String::from("Mul")), output);
        }
        SymbolicExpression::Sub {
            x,
            y,
            degree_multiple,
        } => {}
        SymbolicExpression::Add {
            x,
            y,
            degree_multiple,
        } => {}
        SymbolicExpression::Neg { x, degree_multiple } => {}
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

    let mut n = 1;
    let k = constraints.iter().next();
    let output = constraints.iter().for_each(|constraint| {
        println!("Constraint {n}: {:#?}\n", constraint);

        match constraint {
            SymbolicExpression::Variable(v) => match *v {
                SymbolicVariable { entry, index, .. } => {}
            },
            SymbolicExpression::IsFirstRow => {}
            SymbolicExpression::IsLastRow => {}
            SymbolicExpression::IsTransition => {}
            SymbolicExpression::Constant(c) => {}
            SymbolicExpression::Mul {
                x,
                y,
                degree_multiple,
            } => {}
            SymbolicExpression::Sub {
                x,
                y,
                degree_multiple,
            } => {}
            SymbolicExpression::Add {
                x,
                y,
                degree_multiple,
            } => {}
            SymbolicExpression::Neg { x, degree_multiple } => {}
        }
    });

    // verify(&config, &air, &proof, &vec![])
}

