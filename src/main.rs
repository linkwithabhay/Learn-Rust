mod decision_making;
mod operators;
mod tools;
mod variables;

/// Main function is the entrypoint of a rust program.
fn main() {
    println!("Hello, world!");

    // Variables
    variables::learn();

    // Operators
    operators::learn();

    // Decision Making
    decision_making::learn();
}
