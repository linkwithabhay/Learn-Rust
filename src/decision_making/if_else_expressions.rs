//use std::io; // uncomment this line to take user-input

/// # If-Else
///
/// `if` allows us to execute particular code if a condition is met, but what if we want to
/// execute something else if the condition is not met?
///
/// For that we have the `else` statement:
/// ```rust
/// let age: i32 = 15;
/// let status = if age >= 18 {
///     "Adult"
/// } else {
///     "Young"
/// };
/// ```
/// In the above example, `age` is smaller than `18` which means it enters the `else` code
/// and `status` will hold `"Young"`.
///
/// We can even make it more profound using the `else if` statement:
/// ```rust
/// let age: i32 = 68;
/// let status = if age < 18 {
///     "Young"
/// } else if age >= 18 && age <= 65 {
///     "Adult"
/// } else {
///     "Old"
/// };
/// ```
/// Here it checks whether `age` is smaller than `18`, if not it will continue to the next
/// condition and check whether `age` is between `18` and `65`.
/// If that condition is also not met it will set `status` to `"Old"`.
///
/// We can add as many `else if` statements as we want:
/// ```rust
/// if condition1 {
///     code;
/// } else if condition2 {
///     code;
/// } else if condition3 {
///     code;
/// }
/// ```
///
/// ## Challenge
///
/// You are given a code which gets as input a number that indicates the wind speed
/// and stores it in a variable named `wind`.
///
/// Note: we will learn in next lessons how to get input from the user,
/// currently just don't touch the first lines.
///
///
/// Your task is to initialize variable status based on the conditions:
/// * `"Calm"` if `wind` is smaller than `8`,
/// * `"Breeze"` if `wind` is between `8` and `31` (including 8 and 31).
/// * `"Gale"` if `wind` is between `32` and `63` (including 32 and 63)
/// * `"Storm"` otherwise
pub fn learn() {
    let input = "8"; // comment this line to take user-input
    println!("User input: {}", input); // comment this line to take user-input
    // let mut input = String::new(); // uncomment this line to take user-input
    // io::stdin().read_line(&mut input).unwrap(); // uncomment this line to take user-input
    let wind: i32 = input.trim().parse().unwrap();
    // Type your code below
    let status = if wind < 8 {
        "Calm"
    } else if wind >= 8 && wind <= 31 {
        "Breeze"
    } else if wind >= 32 && wind <= 63 {
        "Gale"
    } else {
        "Storm"
    };

    // Don't change the line below
    println!("wind_status = {}", status);
}
