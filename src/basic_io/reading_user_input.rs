// use std::io;  // uncomment this line to take user-input

/// # Reading User Input
///
/// So far, we've been creating programs where we directly write values in our code.
/// But real programs often need to interact with users and get information from them.
///
/// In Rust, getting input from a user is done using the `std::io` library.
/// This library provides methods to read different types of input,
/// such as integers, floating-point numbers, and strings.
///
/// To use the `std::io` library, you first need to import it:
/// ```rust
/// use std::io;
/// ```
///
/// To read input from a user, we need two lines of code:
/// ```rust
/// let mut my_var = String::new();
/// // Creates an empty string to store the input
///
/// io::stdin().read_line(&mut my_var).unwrap();
///  // Reads the input
/// ```
/// This will store the input value in the `my_var` variable as a string type.
///
/// If you want to get a number, you need an extra step to convert the text input into a number:
/// ```rust
/// let age: i32 = input.trim().parse().unwrap();
/// ```
/// The `.trim()` is needed because when users press Enter, a newline character is added to
/// their input. For example, if someone types "25" and presses Enter, the actual input
/// is "25\n". The `.trim()` removes these extra whitespace characters (spaces, newlines, etc.)
/// from both the start and end of the input.
///
/// Don't worry too much about `unwrap()` for now - we'll learn about it later.
/// Just know that it helps us handle potential errors in a simple (though not ideal) way.
///
/// ## Challenge
///
/// Write a program that gets input from the user (their name), and then outputs `Hello, `
/// followed by the user's input.
///
/// For example, if the user inputs `Bob`, the expected output is `Hello, Bob!`.
///
/// You will need to:
/// 1. Create a `String` object to read input.
/// 2. Prompt the user to enter their name: `“Enter your name: ”`.
/// 3. Read the user's name using the appropriate `std::io` method.
/// 4. Print `Hello, ` and the stored variable in the end.
pub fn learn() {
    let name_input = "William"; // comment this line to take user-input
    // Create a String object
    // let mut name_input = String::new();  // uncomment this line to take user-input

    // Prompt the user to enter their name (use println)
    println!("Enter your name: ");
    println!("User input: {}", name_input); // comment this line to take user-input
    // Read the user's name
    // io::stdin().read_line(&mut name_input).unwrap();  // uncomment this line to take user-input

    // Removing any unwanted characters
    let name = name_input.trim();

    // Print the greeting message
    println!("Hello, {}!", name);
}
