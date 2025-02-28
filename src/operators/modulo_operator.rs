/// # Modulo Operator
///
/// The modulo operator `%` gives the remainder of a division.
/// In Rust, it's used with a simple syntax:
/// ```rust
/// let result = dividend % divisor;
/// ```
///
/// * dividend: The number being divided.
/// * divisor: The number that divides the dividend.
/// * result: The remainder of the division.
///
/// For example:
/// ```rust
/// let result = 10 % 3;
/// ```
///
/// Here, `10` is divided by `3`. `3` goes into `10` three times, with a remainder of `1`. So, `result` will be `1`.
///
/// Usually modulo is used for checking if a number is even or odd:
/// * If a number is even, dividing it by 2 will leave a remainder of 0.
/// * If a number is odd, dividing it by 2 will leave a remainder of 1.
///
/// When using modulo with floating-point numbers (`f64`), it works similarly to integers
/// but keeps the decimal precision. For example:
/// ```rust
/// let result: f64 = 5.2 % 2.0;
/// // result is 1.2
/// ```
///
/// Here's how it works: `2.0` goes into `5.2` two times (`4.0`),
/// and the remainder is `1.2` (`5.2 - 4.0 = 1.2`).
///
/// Another example:
/// ```rust
/// let result: f64 = 7.8 % 3.5;
/// // result is 0.8
/// ```
///
/// ## Challenge
///
/// Write a code that initializes three variables, `a (i32)`, `b (f64)` and `c (i32)`
/// with the values `9`, `2.6`, and `11` (respectively).
///
/// After that, initialize the following variables:
/// * `d (i32)` that will hold the result of `a` modulo `2 `
/// * `e (i32)` that will hold the result of `a` modulo `3`
/// * `f (f64)` that will hold the result of `b` modulo `1.5`
/// * `g (f64)` that will hold the result of `b` modulo `3.9`
/// * `h (i32)` that will hold the result of `c` modulo `10`
///
/// Check out the result and see how different dividends and divisors affect the result.
pub fn learn() {
    // Type your code below
    let a = 9;
    let b = 2.6;
    let c = 11;
    let d = a % 2;
    let e = a % 3;
    let f = b % 1.5;
    let g = b % 3.9;
    let h = c % 10;

    // Don't change the line below
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("d = {}", d);
    println!("e = {}", e);
    println!("f = {}", f);
    println!("g = {}", g);
    println!("h = {}", h);
}
