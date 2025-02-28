mod arithmetic_operators;
mod arithmetic_shortcuts;
mod booleans;
mod chars;
mod comparison_operators;
mod decorator;
mod logical_operators_1;
mod logical_operators_2;
mod logical_operators_3;
mod modulo_operator;
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

    decorator::call("Arithmetic Operators", arithmetic_operators::learn)();

    decorator::call("Modulo Operator", modulo_operator::learn)();

    decorator::call("Arithmetic Shortcuts", arithmetic_shortcuts::learn)();

    decorator::call("Comparison Operators", comparison_operators::learn)();

    decorator::call("Logical Operators Part 1", logical_operators_1::learn)();

    decorator::call("Logical Operators Part 2", logical_operators_2::learn)();

    decorator::call("Logical Operators Part 3", logical_operators_3::learn)();
}
