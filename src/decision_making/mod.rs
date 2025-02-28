use crate::tools::decorator;

pub mod decision_making_challenge;
pub mod if_else_expressions;
pub mod if_expressions;
pub mod match_statement;
pub mod nested_if_else;

/// Learn Decision Making.
pub fn learn() {
    decorator::call("If Expressions", if_expressions::learn)();

    decorator::call("If-Else Expressions", if_else_expressions::learn)();

    decorator::call("Match Statement", match_statement::learn)();

    decorator::call("Nested If-Else", nested_if_else::learn)();

    decorator::call(
        "Decision Making - Challenge",
        decision_making_challenge::learn,
    )();
}
