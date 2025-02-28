use crate::tools::decorator;

pub mod arithmetic_operators;
pub mod arithmetic_shortcuts;
pub mod comparison_operators;
pub mod logical_operators_1;
pub mod logical_operators_2;
pub mod logical_operators_3;
pub mod modulo_operator;

/// Learn Operators.
pub fn learn() {
    decorator::call("Arithmetic Operators", arithmetic_operators::learn)();

    decorator::call("Modulo Operator", modulo_operator::learn)();

    decorator::call("Arithmetic Shortcuts", arithmetic_shortcuts::learn)();

    decorator::call("Comparison Operators", comparison_operators::learn)();

    decorator::call("Logical Operators Part 1", logical_operators_1::learn)();

    decorator::call("Logical Operators Part 2", logical_operators_2::learn)();

    decorator::call("Logical Operators Part 3", logical_operators_3::learn)();
}
