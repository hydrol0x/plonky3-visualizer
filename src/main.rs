// Fib Constraints Code courtesy of https://github.com/BrianSeong99/Plonky3_Fibonacci.git
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{Field, PrimeCharacteristicRing};
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;
use p3_mersenne_31::Mersenne31;
use p3_uni_stark::get_symbolic_constraints;
use std::fs;
use std::io::ErrorKind;
use std::process::Command;

mod visualizer;
use visualizer::build_constraints_graph;

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
        builder.when_first_row().assert_eq(local[1], AB::Expr::ONE);

        // Enforce state transition constraints
        builder.when_transition().assert_eq(next[0], local[1]);
        builder
            .when_transition()
            .assert_eq(next[1], local[0] + local[1]);

        // // Constrain the final value
        let final_value = AB::Expr::from_u32(self.final_value);
        builder.when_last_row().assert_eq(local[1], final_value);
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

fn main() {
    let num_steps = 8;
    let final_value = 21;
    let air = FibonacciAir {
        num_steps,
        final_value,
    };

    type Val = Mersenne31;
    // Assuming get_symbolic_constraints and visualizer exist in your context
    let constraints = get_symbolic_constraints::<Val, FibonacciAir>(&air, 2, 0);

    let dotgraph = visualizer::build_constraints_graph(&constraints);
    println!("{}", dotgraph);

    let filename_gv = "./constraints.gv";
    let filename_svg = "./constraints.svg";

    fs::write(filename_gv, dotgraph).expect("Failed to write graph file.");

    println!("Graph content written to {}", filename_gv);

    println!("Attempting to compile to {}...", filename_svg);

    let output_result = Command::new("dot")
        .arg("-Tsvg")
        .arg(filename_gv)
        .arg("-o")
        .arg(filename_svg)
        .output();

    match output_result {
        Ok(output) => {
            if output.status.success() {
                println!("Successfully compiled graph to {}", filename_svg);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error: 'dot' command failed with status: {}", output.status);
                eprintln!("Graphviz Error Output:\n{}", stderr);
            }
        }
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                eprintln!("Error: 'dot' command not found.");
                eprintln!(
                    "Please install Graphviz (https://graphviz.org/download/) to generate SVGs."
                );
                eprintln!("  - MacOS: brew install graphviz");
                eprintln!("  - Ubuntu: sudo apt install graphviz");
            } else {
                eprintln!("Error: Failed to execute 'dot': {}", e);
            }
        }
    }
}
