/// # Loop Labels
///
/// Labels in Rust help you break or continue specific loops when they're nested.
/// Add `'label`: before a loop to label it.
///
/// For example:
/// ```rust
/// 'outer: for i in 1..=3 {
///     'inner: for j in 1..=3 {
///         if i * j > 5 {
///             break 'outer;
///             // This will break the outer loop
///         }
///         println!("({}, {})", i, j);
///     }
/// }
/// ```
/// In this example, we have two nested loops labeled `'outer` and `'inner`. When the
/// product of `i` and `j` is greater than `5`, the `break 'outer;` statement is executed,
/// which exits the outer loop. The output of this code will be:
/// ```cli
/// (1, 1)
/// (1, 2)
/// (1, 3)
/// (2, 1)
/// (2, 2)
/// ```
/// Similarly, you can use `continue 'label;` to continue to the next iteration of the
/// specified loop. For example:
/// ```rust
/// 'outer: for i in 1..=3 {
///     'inner: for j in 1..=3 {
///         if i == j {
///             continue 'outer;
///             // This will continue the outer loop
///         }
///         println!("({}, {})", i, j);
///     }
/// }
/// ```
///
/// In this case, when `i` is equal to `j`, the `continue 'outer;` statement is executed,
/// which skips the rest of the inner loop and continues with the next iteration of the outer
/// loop. The output will be:
/// ```cli
/// (1, 2)
/// (1, 3)
/// (2, 1)
/// (2, 3)
/// (3, 1)
/// (3, 2)
/// ```
///
/// ## Challenge
///
/// Write a Rust program that uses labeled loops to find the first pair of numbers `(i, j)`
/// such that their sum is greater than `10` and their product is a multiple of `12`
/// (The product is divided by `12` without reminder).
/// The program should iterate through `i` from `1` to `20` and `j` from `1` to `20`.
///
/// When you find the pair, print it in the format `(i, j)` and break out of the outer loop.
pub fn learn() {
    'outer: for i in 1..=20 {
        for j in 1..=20 {
            // Your logic here
            let condition1 = (i + j) > 10;
            let condition2 = (i * j) % 12 == 0;

            if !condition1 || !condition2 {
                continue;
            }

            println!("({}, {})", i, j);
            break 'outer;
        }
    }
}
