//use std::io; // uncomment this line to take user-input

/// # Match Statement
///
/// The match expression allows you to compare a value against a series of patterns
/// and execute code based on which pattern matches.
///
/// Here's the basic structure of a match expression:
/// ```rust
/// match variable {
///     pattern1 => expression1,
///     pattern2 => expression2,
///     // ... more patterns
///     _ => default_expression,
/// }
/// ```
/// The `match` keyword is followed by the value you want to test.
///
/// Each arm of the `match` consists of a pattern followed by `=>` and the code to execute.
///
/// The underscore `_` is the default case that matches anything not matched by other patterns.
///
/// Here's an example:
/// ```rust
/// let day = 3;
/// let day_name = match day {
///     1 => "Monday",
///     2 => "Tuesday",
///     3 => "Wednesday",
///     // ... more patterns,
///     _ => "Invalid day",
/// };
/// ```
///
/// For multiple lines of code in an arm, use a block:
/// ```rust
/// let day = 3;
/// let day_name = match day {
///     1 => {
///         println!("First day of the week!");
///         "Monday"
///     },
///     2 => "Tuesday",
///     // ... other cases
///     _ => "Invalid day",
/// };
/// ```
///
/// You can match multiple patterns using `|`:
/// ```rust
/// let day = 3;
/// let day_type = match day {
///     1 | 2 | 3 | 4 | 5 => "Weekday",
///     6 | 7 => "Weekend",
///     _ => "Invalid day",
/// };
/// ```
///
/// Match expressions in Rust can be exhaustive, meaning they must cover all possible values.
/// The compiler will check this for you.
///
/// ## Challenge
///
/// Create a program that takes a month number (1 for January, 2 for February, etc.)
/// and prints the season it belongs to. Use a match statement for the logic.
///
/// The seasons and their corresponding months are:
/// - **Winter**: December (12), January (1), February (2)
/// - **Spring**: March (3), April (4), May (5)
/// - **Summer**: June (6), July (7), August (8)
/// - **Autumn**: September (9), October (10), November (11)
pub fn learn() {
    let input = "8"; // comment this line to take user-input
    println!("User input: {}", input); // comment this line to take user-input
    // let mut input = String::new(); // uncomment this line to take user-input
    // io::stdin().read_line(&mut input).unwrap(); // uncomment this line to take user-input
    let month: i32 = input.trim().parse().unwrap();
    // Write your code below
    let season = match month {
        12 | 1 | 2 => "Winter",
        3 | 4 | 5 => "Spring",
        6 | 7 | 8 => "Summer",
        9 | 10 | 11 => "Autumn",
        _ => "",
    };

    // Don't change the line below
    println!("season_status = {}", season);
}
