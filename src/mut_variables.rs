/// # Mutable Variables
///
/// In Rust, variables are immutable by default. This means that once you assign a value
/// to a variable, you cannot change that value. However, you can make a variable mutable
/// by using the `mut` keyword when declaring the variable.
///
/// For example:
/// ```rust
/// let x = 5; // x is immutable
/// let mut y = 10; // y is mutable
/// ```
///
/// In this example, `x` is immutable, so you cannot change its value after it's initialized.
/// On the other hand, `y` is mutable, so you can change its value later in the code.
/// ```rust
/// y = 20; // This is allowed because y is mutable
/// x = 15; // This will cause an error because x is immutable
/// ```
///
/// ## Challenge
///
/// Declare a mutable variable named `add` and assign it the value `13`.
/// Change the value from `13` to `16` by adding `3`.
pub fn learn() {
    // Initialize variable add
    let mut add: i32 = 13;

    // Change value from 13 to 16
    add += 3;

    // Don't change the line below
    println!("add = {}", add);
}
