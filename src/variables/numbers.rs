/// # Numbers
///
/// Variables are containers that hold data values. They are used to store, manipulate,
/// and display information within a program. In short a variable is like a memory unit
/// that we can access by typing the name of the variable.
///
/// Each variable has a unique name and a value that can be of different types.
/// Rust has various built-in data types that define the type of value a variable can hold.
///
/// To initialize a variable, we use the following format:
/// ```rust
/// let variable_name: variable_type = value;
/// ```
///
/// In Rust, numbers are typically represented using two main data types: `i32` and `f64`.
///
/// `i32` is used to store whole numbers without any decimal point. For example:
/// ```rust
/// let age: i32 = 30;
/// let temperature: i32 = -5;
/// let count: i32 = 100;
/// ```
///
/// `f64` is used to store numbers with a decimal point. For example:
/// ```rust
/// let price: f64 = 99.99;
/// let pi: f64 = 3.14159;
/// let fraction: f64 = 0.5;
/// ```
/// When declaring variables in Rust, you need to specify the type of the variable
/// after the variable name, followed by a colon. This is known as type declaration.
/// Once a variable is declared with a certain type, it can only hold values of that type.
///
/// ## Challenge
///
/// Write a Rust program that declares and initializes the following variables:
/// * Declare an `i32` variable named `quantity` and initialize it with the value `5`.
/// * Declare an `f64` variable named `item_price` and initialize it with the value `24.99`.
///
/// After declaring and initializing these variables, use `println!()` to output the values
/// of the variables to the console in the following format:
/// ```cli
/// Quantity: [value of quantity]
/// Price: [value of item_price]
/// ```
pub fn learn() {
    // Declare and initialize variables here
    let quantity: i32 = 5;
    let item_price: f64 = 24.99;

    // Output the values, Don't change the lines below
    println!("Quantity: {}", quantity);
    println!("Price: {}", item_price);
}
