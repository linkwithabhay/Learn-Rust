// use std::io;  // uncomment this line to take user-input

/// # Challenge
///
/// Write a program that gets a **dynamic number of input values**.
///
/// The first input is a number that represents the number of the input values following it.
/// The next input values are whole numbers.
///
/// In the end, print the sum of all the input numbers (not including the first input).
///
/// For example,
///
/// Input:
/// ```cli
/// 3
/// 1
/// 5
/// 6
/// ```
/// Expected output: `12`
///
/// Explanation:
///
/// `1 + 5 + 6 = 12`, and there are exactly 3 numbers following the first input number (`3`).
pub fn learn() {
    let input1 = "2"; // comment this line to take user-input
    let input2 = "10"; // comment this line to take user-input
    let input3 = "15"; // comment this line to take user-input
    let input = input1; // comment this line to take user-input
    println!("User Inputs: {input1}, {input2}, {input3}"); // comment this line to take user-input

    // uncomment below lines to take user-input
    /*
    let mut input = String::new();  // uncomment this line to take user-input
    io::stdin().read_line(&mut input).unwrap();  // uncomment this line to take user-input
    */

    let count: i32 = input.trim().parse().unwrap();

    // Write your code below

    let mut sum: i32 = 0;

    for i in 0..count {
        let input = if i == 0 { input2 } else { input3 };
        // uncomment below lines to take user-input
        /*
        input.clear();  // uncomment this line to take user-input
        io::stdin().read_line(&mut input).unwrap();  // uncomment this line to take user-input
        */

        let number: i32 = input.trim().parse().unwrap();
        sum += number;
    }

    println!("{}", sum);
}
