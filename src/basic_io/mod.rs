use crate::tools::decorator;

pub mod basic_io_challenge_1;
pub mod basic_io_challenge_2;
pub mod printing_to_console;
pub mod reading_user_input;

/// Learn Basic IO.
pub fn learn() {
    decorator::call("Printing to Console", printing_to_console::learn)();

    decorator::call("Reading User Input", reading_user_input::learn)();

    decorator::call("Basic IO - Challenge 1", basic_io_challenge_1::learn)();

    decorator::call("Basic IO - Challenge 2", basic_io_challenge_2::learn)();
}
