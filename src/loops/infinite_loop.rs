// use std::io;  // uncomment this line to take user-input

/// # Infinite Loop
///
/// In Rust, infinite loops can be created using either `loop` or `while true`.
/// Both run until explicitly stopped with `break` or program termination.
///
/// For example:
/// ```rust
/// loop {
///     // Code to be executed repeatedly
///     if condition {
///         break; // Exit the loop if the condition is met
///     }
/// }
/// ```
/// The loop continues until it reaches a `break`. It's useful for programs that need to
/// run continuously, like servers or devices.
///
/// Here's an example of an infinite loop that prints numbers until a specific condition is met:
/// ```rust
/// let mut count = 0;
/// loop {
///     println!("Count: {}", count);
///     count += 1;
///     if count > 5 {
///         break;
///         // Exit the loop when count is greater than 5
///     }
/// }
/// ```
///
/// In this example, the loop will print numbers from `0` to `5` and then exit when `count` becomes `6`.
///
/// ## Challenge
///
/// Write a Rust program that simulates a simple counter. The program should use an infinite
/// loop to continuously increment a counter and print its value. The loop should terminate
/// when the counter reaches a specific value determined by the input.
///
/// Steps for the program:
/// * Initialize a mutable variable `counter` to `0`.
/// * Use an infinite loop (`loop`) to continuously increment the `counter` by `1` in each iteration.
/// * Inside the loop, print the current value of `counter`.
/// * Check if `counter` is equal to the user-provided termination value.
/// If it is, break out of the loop.
pub fn learn() {
    let input = "5"; // comment this line to take user-input
    println!("User Input: {}", input); // comment this line to take user-input

    // uncomment below lines to take user-input
    /*
    let mut input = String::new();  // uncomment this line to take user-input
    io::stdin().read_line(&mut input).unwrap();  // uncomment this line to take user-input
    */

    let limit: i32 = input.trim().parse().unwrap();
    let mut counter = 0;

    // Write your code below

    loop {
        counter += 1;
        println!("Count: {}", counter);
        if counter >= limit {
            break;
        }
    }
}
