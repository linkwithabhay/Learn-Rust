// use std::io; // uncomment this line to take user-input

/// # Basic IO - Challenge
///
/// Write a program that gets the user's age as input.
///
/// The program will output (print) the number of missing years
/// till `120` (in a specific format, shown below).
///
/// For example, for input `25`, the expected output
/// is `"95 years till 120"`.
pub fn learn() {
    // Write your code below

    let age_input = "25"; // comment this line to take user-input
    println!("User input: {}", age_input); // comment this line to take user-input

    // Read the user input
    // let mut age_input = String::new(); // uncomment this line to take user-input
    // io::stdin().read_line(&mut age_input).unwrap(); // uncomment this line to take user-input

    // Parse the input
    let age: i32 = age_input.trim().parse().unwrap();

    // Print the message
    println!("{} years till 120", 120 - age);
}
