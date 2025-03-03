// use std::io;  // uncomment this line to take user-input

/// # Nested Loop
///
/// A nested loop is simply a loop inside another loop. The inner loop will complete all its
/// iterations for each single iteration of the outer loop.
///
/// A good analogy for this is a clock: for each hour (outer loop),
/// the minute hand (inner loop) must complete its full 60-minute cycle.
///
/// Example of a nested loop:
/// ```rust
/// for x in 0..2 {
///     for y in 0..2 {
///         println!("{} {}", x, y);
///     }
/// }
///
/// // This will output:
/// // 0 0
/// // 0 1
/// // 1 0
/// // 1 1
/// ```
///
/// The outer loop (`x`) runs twice, and for each of those times,
/// the inner loop (`y`) runs twice.
///
/// ## Challenge
///
/// Write a program that finds all pairs of numbers that add up to `n`
/// using numbers from `1` to `n - 1`.
///
/// The program should show all possible combinations, including duplicate pairs in reverse
/// order. For example, both `1 5` and `5 1` should be shown, as they are considered different
/// arrangements of the same pair. Numbers can also be paired with themselves if their
/// sum equals `n`.
///
/// For example if `n = 6`, the output should be:
/// ```cli
/// 1 5
/// 2 4
/// 3 3
/// 4 2
/// 5 1
/// ```
///
/// Because:
/// ```cli
/// 1 + 5 = 6
/// 2 + 4 = 6
/// 3 + 3 = 6
/// 4 + 2 = 6
/// 5 + 1 = 6
/// ```
pub fn learn() {
    let input = "6"; // comment this line to take user-input
    println!("User Input: {}", input);

    // uncomment below lines to take user-input
    /*
    let mut input = String::new();  // uncomment this line to take user-input
    io::stdin().read_line(&mut input).unwrap();  // uncomment this line to take user-input
    */

    let n: i32 = input.trim().parse().unwrap();

    // Write your code below

    for i in 1..n {
        println!("{} {}", i, n - i);
    }
}
