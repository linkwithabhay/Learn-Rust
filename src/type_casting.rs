/// # Type Casting
///
/// Type casting is the process of converting a value from one data type to another.
/// In Rust, we use the `as` keyword for explicit type casting (also known as type conversion).
///
/// The most common type conversions are between numeric types:
/// * Integer to Float conversion
/// ```rust
///     let number: i32 = 5;
///     let decimal: f64 = number as f64;
///     // becomes 5.0
/// ```
///
/// * Float to Integer conversion
/// ```rust
///     let decimal: f64 = 9.7;
///     let number: i32 = decimal as i32;
///     // becomes 9 (decimal part is truncated)
/// ```
///
/// ## Challenge
///
/// Write a Rust program that demonstrates type casting. Perform the following operations:
/// * Declare an `f64` variable named `price` and initialize it with the value `99.99`.
/// * Cast the price variable to an `i32` and store the result in a new variable named `int_price`.
/// * Print the values of `price` and `int_price`, to the console.
pub fn learn() {
    // Declare and initialize variables
    let price: f64 = 99.99;
    let int_price: i32 = price as i32;

    // Output the values
    println!("Price: {}", price);
    println!("Int Price: {}", int_price);
}
