/// # Printing to Console
///
/// In Rust, you can print output to the console using the `print!` and `println!` macros.
/// These macros allow you to display text, variables, and expressions in the console.
/// The main difference between `print!` and `println!` is that `println!` adds a newline
/// character at the end of the output, causing the next output to start on a new line,
/// while `print!` does not.
///
/// Here's how you can use these macros:
/// ```rust
/// let name = "Alice";
/// let age = 30;
///
/// print!("Name: ");
/// print!("{}", name);
/// println!(" is {} years old.", age);
///
/// println!("Hello, {}!", name);
/// ```
///
/// This code will produce the following output:
/// ```cli
/// Name: Alice is 30 years old.
/// Hello, Alice!
/// ```
///
/// ## Challenge
///
/// Write a Rust program that uses `print!` and `println!` macros to output the following:
///
/// 1. Print `"Hello, "` and `"Noddy!"` on the same line.
/// 2. Print `"Rust is fun!"` on a new line using `println!`.
///
/// Use the given variables inside the print macros.
pub fn learn() {
    let hello = "Hello, ";
    let noddy = "Noddy!";
    let rust_is_fun = "Rust is fun!";

    // Write your code below
    print!("{}", hello);
    println!("{}", noddy);
    println!("{}", rust_is_fun);
}
