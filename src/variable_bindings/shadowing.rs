/// # Shadowing
///
/// You can declare a new variable with the same name as a previous variable in a different
/// scope. The new variable shadows the previous one until the end of its scope:
/// ```rust
/// fn main() {
///     let x = 5;
///     {
///         let x = 12;
///         // shadows the outer x
///         println!("{}", x);
///         // prints "12"
///     }
///     
///     println!("{}", x);
///     // prints "5"
/// }
/// ```
///
/// When you shadow a variable in Rust, the new variable is completely independent from the
/// original one, and it needs its own mutability declaration. Shadowing does not inherit the
/// mutability of the original variable.
/// ```rust
/// let mut mut_val = 7;
/// // Original variable is mutable
/// {
///     let mut_val = 8;
///     // This is a new variable that shadows the outer mut_val
///       
///     mut_val = 50;
///     // An error occurred because
///     // mut_val is not mutable
/// }
/// // Here it is okay
/// // mut_val is not frozen in this scope
/// mut_val = 3;
/// ```
///
/// To fix it we can add mut in the inner scope:
/// ```rust
/// {
///     let mut mut_val = 8;
///     mut_val = 50;
/// }
/// ```
///
/// ## Challenge
///
/// Write a Rust program that demonstrates the concept of shadowing.
/// Perform the following steps:
/// * Declare a variable named `x` and initialize it with the value `5`.
/// * Declare a variable named `y` and initialize it with the value `25`.
/// * Print the value of `x` and `y` to the console.
/// * Shadow `x` with a new value that is the original `x` plus `3` inside a new scope.
/// * Shadow `y` but with mutability and assign the original value inside a new scope.
/// * Change the value of 'y' by adding `50` to it.
/// * Print the shadowed `x` and `y`.
/// * Finally print `x` and `y` outside of the scope.
pub fn learn() {
    // Declare x and initialize it with 5
    let x = 5;
    let y = 25;
    // Print the value of x and y
    println!("x is: {}, and y is: {}", x, y);
    {
        // Shadow x with the original x plus 3
        let x = x + 3;
        let mut y = y;
        y += 50;
        // Print the value of the shadowed x
        println!("x is: {}, and y is: {}", x, y);
    }
    // Print the value of outer x
    println!("x is: {}, and y is: {}", x, y);
}
