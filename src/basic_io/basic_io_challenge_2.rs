// use std::io; // uncomment this line to take user-input

/// # Basic IO - Challenge
///
/// Write a program that gets an input from the user.
///
/// The program will output `T` if the input equals to `1` and `F` otherwise.
pub fn learn() {
    // Write your code below

    let num_input = "g"; // comment this line to take user-input
    println!("User input: {}", num_input); // comment this line to take user-input

    // Read the user input
    // let mut num_input = String::new(); // uncomment this line to take user-input
    // io::stdin().read_line(&mut num_input).unwrap(); // uncomment this line to take user-input

    // Parse the input
    let num = num_input.trim();

    // Condition check
    if num == "1" {
        println!("T");
    } else {
        println!("F");
    }
}
