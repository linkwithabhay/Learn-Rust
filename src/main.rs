mod booleans;
mod chars;
mod decorator;
mod mut_variables;
mod numbers;
mod strings;
mod type_casting;
mod type_declaration;
mod type_inference;

/// Main function is the entrypoint of a rust program.
fn main() {
    println!("Hello, world!");

    decorator::call("Number", numbers::learn)();

    decorator::call("Char", chars::learn)();

    decorator::call("String", strings::learn)();

    decorator::call("Boolean", booleans::learn)();

    decorator::call("Mutable Variables", mut_variables::learn)();

    decorator::call("Type Declarations", type_declaration::learn)();

    decorator::call("Type Inference", type_inference::learn)();

    decorator::call("Type Casting", type_casting::learn)();
}
