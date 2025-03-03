// use std::io;  // uncomment this line to take user-input

/// # While Loop
///
/// A `while` loop is different from the `for` loop. A `for` loop allows us to iterate over a
/// specific range, whereas a `while` loop allows us to keep iterating as long as a certain
/// condition is met.
///
/// To use a `while` loop write:
/// ```rust
/// while condition {
///     code
/// }
/// ```
/// The code will execute only if the condition is `true`.
///
/// There are many use cases where a `while` would solve the problem,
/// but the `for` loop would not.
///
/// ## Challenge
///
/// Write a program that gets one input, a **double** number.
///
/// Use a `while` loop to divide the input by `2` as long as the number is bigger or equal to `3.5`.
///
/// Print the first number that is smaller than `3.5`.
pub fn learn() {
    let input = "5.4"; // comment this line to take user-input
    println!("User Input: {}", input); // comment this line to take user-input

    // uncomment below lines to take user-input
    /*
    let mut input = String::new();  // uncomment this line to take user-input
    io::stdin().read_line(&mut input).unwrap();  // uncomment this line to take user-input
    */

    let mut number: f64 = input.trim().parse().unwrap();

    // Write your code below

    while number >= 3.5 {
        number /= 2.0;
    }

    println!("{}", number);
}
