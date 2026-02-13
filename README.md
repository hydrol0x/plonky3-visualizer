# AIR Visualizer

Visualize AIR constraints defined using plonky3. The AIR constraints are visualized as an AST using the graphviz format.

Boolean (selectors) are colored in red, trace variables are colored in green, constants are colored in blue.

Here is an example of the AIR constraints visualization implementing the [Fibonacci](https://github.com/BrianSeong99/Plonky3_Fibonacci) sequence:

<img width="1963" height="485" alt="image" src="https://github.com/user-attachments/assets/3fe2f700-a130-43b9-8f6a-827c4fce3863" />

Corresponding to these constraints:

```rs
        // Enforce starting values
        builder.when_first_row().assert_eq(local[0], AB::Expr::ZERO); // Constraint 1
        builder.when_first_row().assert_eq(local[1], AB::Expr::ONE); // Constraint 2

        // Enforce state transition constraints
        builder.when_transition().assert_eq(next[0], local[1]); // Constraint 3
        builder
            .when_transition()
            .assert_eq(next[1], local[0] + local[1]); // Constraint 4

        // Constrain the final value
        let final_value = AB::Expr::from_u32(self.final_value); 
        builder.when_last_row().assert_eq(local[1], final_value); // Constraint 5

```
# Usage

If cargo is installed, run `cargo run` or `cargo build` inside the project directory to install and build dependencies. This project also relies on the [Graphviz]((https://graphviz.org/download/)) CLI to compile the Graphviz source into `.svg` files. 

Alternatively, you can paste the generated `.gv` source into an online visalizer like [this one](https://dreampuf.github.io/GraphvizOnline)
