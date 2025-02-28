/// # Type Inference
///
/// In Rust, type inference allows the compiler to automatically deduce the type of a variable
/// based on its value and usage. This means you often don't need to explicitly specify
/// the type when declaring a variable, making your code more concise and easier to read.
///
/// For example:
/// ```rust
/// let x = 5;
/// // Rust infers that x is an i32
///
/// let y = 3.14;
/// // Rust infers that y is an f64
///
/// let message = "Hello, world!";
/// // Rust infers that message is a &str (string)
///
/// let is_true = true;
/// // Rust infers that is_true is a bool
/// ```
///
/// In these examples, we didn't specify the types of `x`, `y`, `message`, and `is_true`.
/// The Rust compiler automatically inferred their types based on the values assigned to them.
///
/// Type inference is not only convenient but also helps to prevent errors.
/// The compiler checks how the variables are used and ensures that the inferred types are
/// consistent throughout the code. If there's a conflict, the compiler will generate an error.
///
/// ## Challenge
///
/// Write a Rust program that demonstrates type inference.
/// Declare and initialize the following variables without explicit type annotations:
/// * A variable named `quantity` with the value `10`.
/// * A variable named `price` with the value `99.99`.
/// * A variable named `message` with the value `"Coddy is awesome!"`.
/// * A variable named `is_available` with the value `true`.
///
/// After declaring these variables, use `println!()` to output their values to the console.
/// Observe how Rust infers the types of these variables based on their values.
pub fn learn() {
    // Declare variables here
    let quantity = 10;
    let price = 99.99;
    let message = "Coddy is awesome!";
    let is_available = true;

    // Output the values
    println!("Quantity: {}", quantity);
    println!("Price: {}", price);
    println!("Message: {}", message);
    println!("Is available: {}", is_available);
}
