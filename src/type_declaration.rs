/// # Type Declaration
///
/// Once a variable is declared with a certain type, it can only hold values of that type.
/// For instance, an `i32` variable can only hold integer values,
/// and a `String` variable can only hold text.
///
/// For example:
/// ```rust
/// let age: i32 = 25;  // Can only hold whole numbers
/// let name: String = "Alice";  // Can only hold text
/// ```
///
/// These would cause errors:
/// ```rust
/// age = "Bob";  // Error: can't put text in an i32 variable
/// name = 30;  // Error: can't put a number in a String variable
/// ```
///
/// These are valid:
/// ```rust
/// age = 26;  // OK: assigning a new integer
/// name = "Jane";  // OK: assigning a new text string
/// ```
///
/// ## Challenge
///
/// Declare the following variables with their corresponding types and values:
/// * An i32 variable named `count` with the value `10`.
/// * An f64 variable named `total` with the value `150.75`.
/// * A char variable named `grade` with the value `'A'`.
/// * A bool variable named `is_active` with the value `false`.
/// * A String variable named `user_name` with the value `"Bob123"`.
///
/// After declaring these variables, use `println!()` to output the values of the variables
/// to the console in the following format:
/// ```cli
/// Count: [value of count]
/// Total: [value of total]
/// Grade: [value of grade]
/// Active: [value of is_active]
/// User Name: [value of user_name]
/// ```
pub fn learn() {
    // Declare variables here
    let count: i32 = 10;
    let total: f64 = 150.75;
    let grade: char = 'A';
    let is_active: bool = false;
    let user_name: String = "Bob123".to_string();

    // Output the values
    println!("Count: {}", count);
    println!("Total: {}", total);
    println!("Grade: {}", grade);
    println!("Active: {}", is_active);
    println!("User Name: {}", user_name);
}
