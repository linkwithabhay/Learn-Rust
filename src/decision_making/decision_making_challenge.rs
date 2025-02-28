//use std::io; // uncomment this line to take user-input

/// # Challenge
/// You are given a code which gets as input two numbers `n1` and `n2`
/// and a single char string `op`.
///
/// Note: we will learn in next lessons how to get input from the user,
/// currently just don't touch the three first lines.
///
/// The possible values for `op` are `'+'`, `'-'`, `'/'` and `'*'`.
///
/// Your task is to set the variable `result` based on the conditions:
/// * if `op` is `'+'`, set `result` with `n1 + n2`.
/// * if `op` is `'-'`, set `result` with `n1 - n2`.
/// * if `op` is `'/'`, set `result` with `n1 / n2`.
/// * if `op` is `'*'`, set `result` with `n1 * n2`.
pub fn learn() {
    let n1_input = "8"; // comment this line to take user-input
    let n2_input = "8"; // comment this line to take user-input
    let op_input = "*"; // comment this line to take user-input
    println!("User inputs: {}, {}, {}", n1_input, n2_input, op_input); // comment this line to take user-input
    // uncomment below lines to take user-input
    /*
    let mut n1_input = String::new();  // uncomment this line to take user-input
    let mut n2_input = String::new();  // uncomment this line to take user-input
    let mut op_input = String::new();  // uncomment this line to take user-input

    io::stdin().read_line(&mut n1_input).unwrap();  // uncomment this line to take user-input
    io::stdin().read_line(&mut n2_input).unwrap();  // uncomment this line to take user-input
    io::stdin().read_line(&mut op_input).unwrap();  // uncomment this line to take user-input
    */

    let n1: f64 = n1_input.trim().parse().unwrap();
    let n2: f64 = n2_input.trim().parse().unwrap();
    let op = op_input.trim();

    // Write your code below, use n1, n2 and op
    let result: f64 = match op {
        "+" => n1 + n2,
        "-" => n1 - n2,
        "/" => n1 / n2,
        "*" => n1 * n2,
        _ => 0.0,
    };

    println!("result = {}", result);
}
