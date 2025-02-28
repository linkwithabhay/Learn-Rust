/// # If Expressions
///
/// If expressions allow us to execute code with conditions.
///
/// For example, let's look at the following code:
/// ```rust
/// let age: i32 = 20;
/// let mut status = "Child";
/// if age > 18 {
///     status = "Adult";
/// }
/// age += 1;
/// ```
/// The above code checks whether the age variable is bigger than `18`.
/// If it is, it will set `status` to hold `"Adult"` string.
///
/// In the end, the code will increment `age` by `1` whether the `age` is bigger than `18` or not.
///
/// To use an `if` statement in Rust, we need to use curly braces `{}` to define the code block,
/// and everything inside the `if` statement should be placed between these braces:
/// ```rust
/// if condition {
///     code;
///     code;
///     code;
/// }
/// ```
/// If the `condition` is `true`, we will enter the `code` block inside the `if` (The indented code).
///
/// ## Challenge
///
/// You are given a code.
///
/// The variables `a` and `b` have missing values, fill them so that the code inside
/// the `if` statement will be executed and `c` will equal `3`.
pub fn learn() {
    let a: i32 = 11;
    let b: i32 = 10;

    // Don't change below this line
    let mut c: i32 = 0;
    if a >= b && !(b < 10) {
        c = 2;
    }

    c += 1;
    println!("c = {}", c);
}
