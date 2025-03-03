/// # Break
///
/// The `break` statement stops the loop instantly when it's encountered.
///
/// For example,
/// ```rust
/// for i in 0..10 {
///     if i == 6 {
///         break;
///     }
///     println!("{}", i);
/// }
/// ```
/// In the following example the loop iterates regularly until it reaches number `6`.
/// Then the program enters the `if` statement and executes the `break` statement.
/// This exits the loop immediately. The output is:
/// ```cli
/// 0
/// 1
/// 2
/// 3
/// 4
/// 5
/// ```
///
/// ## Challenge
///
/// You are given a code that prints the numbers from `1` to `20` (including).
///
/// Your task is to add `if` and `break` statements so that only the numbers from `1` to `5`
/// will be printed, the loop will exit before printing the numbers from `6` to `20`.
pub fn learn() {
    for i in 1..=20 {
        if i > 5 {
            break;
        }
        println!("{}", i);
    }
}
