use crate::tools::decorator;

pub mod booleans;
pub mod chars;
pub mod mut_variables;
pub mod numbers;
pub mod strings;
pub mod type_casting;
pub mod type_declaration;
pub mod type_inference;

/// Learn variables.
pub fn learn() {
    decorator::call("Number", numbers::learn)();

    decorator::call("Char", chars::learn)();

    decorator::call("String", strings::learn)();

    decorator::call("Boolean", booleans::learn)();

    decorator::call("Mutable Variables", mut_variables::learn)();

    decorator::call("Type Declarations", type_declaration::learn)();

    decorator::call("Type Inference", type_inference::learn)();

    decorator::call("Type Casting", type_casting::learn)();
}
