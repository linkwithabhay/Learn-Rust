/// # Continue
///
/// The continue statement stops the current iteration and continues to the next iteration.
/// For example:
/// ```rust
/// for i in 3..9 {
///     if i == 5 {
///         continue;
///     }
///     println!("{}", i);
/// }
/// ```
/// The loop will iterate through all of the numbers. When it reaches `i=5` it will skip that
/// iteration and continue to the next one. The output is:
/// ```cli
/// 3
/// 4
/// 6
/// 7
/// 8
/// ```
/// Notice, number `5` is not in the output.
///
/// ## Challenge
///
/// You are given a code which prints the numbers from `1` to `10` (including).
///
/// Your task is to add `if` and `continue` statements so that only the even numbers will be
/// printed (2, 4, 6, ...).
pub fn learn() {
    for i in 1..=10 {
        if i % 2 != 0 {
            continue;
        }
        println!("{}", i);
    }
}
