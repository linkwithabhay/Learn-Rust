/// # For Loop
///
/// Sometimes when programming it's necessary to perform same or almost the same operation a
/// couple of times.
///
/// To prevent writing the same thing over and over again we can use Loops.
///
/// The for loop has the following syntax
/// ```rust
/// for element in range {
///     code
/// }
/// ```
///
/// A range expression defines how many times the loop should run, typically written
/// as `start..end` (which runs from start to end-1) or `start..=end` (which runs from
/// start to end, including end).
///
/// For example, a loop from 0 to 5 (not including):
/// ```rust
/// for i in 0..5 {
///     println!("{}", i);
/// }
/// ```
/// It will execute the print statement 5 times:
/// ```cli
/// 0
/// 1
/// 2
/// 3
/// 4
/// ```
///
/// Loops have many use cases. For example, let's sum all the numbers from 1 to 100:
/// ```rust
/// let mut sum_numbers = 0;
/// for i in 1..=100 {
///     sum_numbers += i;
/// }
/// println!("{}", sum_numbers);
/// ```
/// This will first loop through all numbers between `1` to `100` (including 100 because
/// of `..=` sign) and sum all of them, then it will print the `sum_numbers` variable
///
/// If for some reason you want to create a loop without using a variable (`i`), you should
/// add underscore at the start of the name: `_i`. This will tell the compiler that it is okay
/// that it is not used, and it will prevent the program to produce a warning:
/// ```rust
/// for _i in 0..5 {
///     println!("Hello!");
/// }
/// ```
///
/// ## Challenge
///
/// Write a program that prints `"Hello Coddy: "` and the `i` value from `3` to `7`
/// (including, which means printing the numbers 3, 4, ..., 7,
/// making it 7 - 3 + 1 = 5 times in total), do it using a **for loop**.
///
/// It will look like this:
/// ```cli
/// Hello Coddy: 3
/// Hello Coddy: 4
/// ...
/// Hello Coddy: 7
/// ```
pub fn learn() {
    // Write code here
    for i in 0..5 {
        println!("Hello Coddy: {}", i + 3);
    }
}
