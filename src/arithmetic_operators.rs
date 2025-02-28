/// # Arithmetic Operators
///
/// Operators are used to perform operations on values.
///
/// First we will discuss the most basic arithmetic operators, they may be familiar from math classes.
///
/// | Operator | Operation      | Example   |
/// |----------|----------------|-----------|
/// | +        | Addition       | 3 + 2 = 5 |
/// | -        | Subtraction    | 3 - 2 = 1 |
/// | *        | Multiplication | 3 * 2 = 6 |
/// | /        | Division       | 4 / 2 = 2 |
///
/// Let's see usage example,
/// ```rust
/// let a: i32 = 3;
/// let b: i32 = 5;
/// let c: i32 = a + b
/// // c holds 8
/// ```
///
/// When working with decimal numbers in Rust, we use the `f64` data type, which can store
/// numbers with decimal points. The same arithmetic operators (+, -, *, /) work with `f64`
/// just like they do with integers:
/// ```rust
/// let x: f64 = 3.3;
/// let y: f64 = 4.1;
/// let z: f64 = x + y;
/// // z holds 7.4
/// ```
///
/// ## Challenge
///
/// Write a code that initializes two variables, `a` and `b`, with the values `5.2` and `2.6` (respectively).

/// After that, initialize another variable `c` that will hold the result of `a / b`.
pub fn learn() {
    // Type your code below
    let a = 5.2;
    let b = 2.6;
    let c = a / b;

    // Don't change the line below
    println!("a = {}, b = {}, c = {}", a, b, c);
}
