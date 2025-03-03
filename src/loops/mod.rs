use crate::tools::decorator;

pub mod break_loop;
pub mod continue_loop;
pub mod for_loop;
pub mod infinite_loop;
pub mod loop_labels;
pub mod loops_challenge;
pub mod nested_loop;
pub mod while_loop;

/// Learn Loops
pub fn learn() {
    decorator::call("For Loop", for_loop::learn)();

    decorator::call("While Loop", while_loop::learn)();

    decorator::call("Break Loop", break_loop::learn)();

    decorator::call("Continue Loop", continue_loop::learn)();

    decorator::call("Nested Loop", nested_loop::learn)();

    decorator::call("Loop Labels", loop_labels::learn)();

    decorator::call("Infinite Loop", infinite_loop::learn)();

    decorator::call("Loops - Challenge", loops_challenge::learn)();
}
