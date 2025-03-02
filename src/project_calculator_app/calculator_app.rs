// use std::io;  // uncomment this line to take user-input

/// # Calculator App
///
/// ### Welcome Message
///
/// Did you know that creating a simple calculator in Rust can be a great way to practice
/// what you've learned about variables, operators, and basic input/output?
/// In this project, you'll create a basic calculator app that performs arithmetic operations
/// like addition, subtraction, multiplication, and division. It's a practical way to apply
/// your Rust skills and build a useful tool. By the end of this project, you'll have a working
/// calculator that can perform basic math operations! Let's get started!
///
/// ### Getting Numbers
///
/// The next part of our program is to get input from the program user.
///
/// ### Basic Operations
///
/// The next part of our program is to perform basic operations on the inputs.
///
/// ### Formatted Output
///
/// And finally, let's format the arithmetic results.
///
/// ## Challenge
///
/// Every good program starts with a welcome message. Output to the screen the following string:
/// ```cli
/// Calculator App
/// ```
///
/// After the welcome message, get two numbers (`f64`) from the user and store them in
/// variables named `num1` and `num2`.
///
/// Add the basic arithmetic operations (addition, subtraction, multiplication, and division)
/// on `num1` and `num2`. Store the results in variables named `sum`, `difference`, `product`,
/// and `quotient`, respectively. Print the results to the console using `println!` in the
/// following format:
/// ```cli
/// Sum: [sum]
/// Difference: [difference]
/// Product: [product]
/// Quotient: [quotient]
/// ```
///
/// Modify the output so that it will always print a float value with two decimal places.
/// To do it use the {:.2} sign:
/// ```rust
/// let num = 1.5576734;
/// println!("This is rounded: {:.2}", num);
/// // Output: This is rounded: 1.56
/// ```
pub fn learn() {
    // Welcome message
    println!("Calculator App");

    // Getting Numbers
    let num1_input = "11"; // comment this line to take user-input
    let num2_input = "33"; // comment this line to take user-input
    // uncomment below lines to take user-input
    /*
    let mut num1_input: String = String::new();  // uncomment this line to take user-input
    let mut num2_input: String = String::new();  // uncomment this line to take user-input

    io::stdin().read_line(&mut num1_input).unwrap();  // uncomment this line to take user-input
    io::stdin().read_line(&mut num2_input).unwrap();  // uncomment this line to take user-input
    */

    let num1: f64 = num1_input.trim().parse().unwrap();
    let num2: f64 = num2_input.trim().parse().unwrap();

    let sum = num1 + num2;
    let difference = num1 - num2;
    let product = num1 * num2;
    let quotient = num1 / num2;

    println!("Sum: {:.2}", sum);
    println!("Difference: {:.2}", difference);
    println!("Product: {:.2}", product);
    println!("Quotient: {:.2}", quotient);
}
