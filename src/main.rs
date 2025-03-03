mod basic_io;
mod decision_making;
mod loops;
mod operators;
mod project_calculator_app;
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

    // Basic IO
    basic_io::learn();

    // Project: Calculator App
    project_calculator_app::learn();

    // Loops
    loops::learn();
}
