/// # Logical Operators Part 3
///
/// When checking multiple conditions, the computer stops checking as soon as it knows
/// the final answer (This is called short-circuit evaluation).
///
/// For example:
/// ```rust
/// let x: i32 = 0;
/// let y: i32 = 5;
/// let result: bool = x != 0 && y / x > 2;
/// ```
/// Here `x` equals `0`, therefore it will not evaluate `y / x > 2`. If we would reverse the order:
///
/// ```rust
/// let result: bool = y / x > 2 && x != 0;
/// ```
///
/// This will result in an error because `y` will be divided by `0` which is illegal in math.
///
/// This technique is used to optimize the evaluation of logical expressions. For example:
/// ```rust
/// let a: i32 = 0;
/// let b: i32 = 2;
/// let c: i32 = 3;
/// let d: i32 = 5;
/// let result: bool = (a > 0 && b < 2) || (c < -5 && d < 10);
/// ```
/// In this example `b < 2` and `d < 10` will not be evaluated
/// because `a > 0` and `c < -5` are both `false`.
///
/// ## Challenge
///
/// Let's create a program to decide if it's a good day for solar panel energy production.
///
/// Initialize these variables:
/// * `is_sunny` with the value `true`
/// * `wind_speed` with the value `5.4`
/// * `temperature` with the value `23`
/// * `solar_panel_output` with the value `9`
/// * `is_cloudy` with the value `false`
///
/// Create one logical expression that checks ALL of these conditions:
/// * It's sunny
/// * The wind speed is less than 10
/// * The solar panel output is less than 15
/// * The temperature is above 20 **OR** there are no clouds
pub fn learn() {
    // Initialize variables
    let is_sunny = true;
    let wind_speed = 5.4;
    let temperature = 23;
    let solar_panel_output = 9;
    let is_cloudy = false;

    // The complete logical expression
    let result: bool = is_sunny
        && wind_speed < 10.0
        && solar_panel_output < 15
        && (temperature > 20 || !is_cloudy);

    // Don't delete the lines below
    println!("Checking conditions for solar energy production...");
    println!("1. Is it sunny? {}", is_sunny);
    println!("2. Is wind speed safe? {}", (wind_speed < 10.0));
    println!("3. Can panels produce more? {}", (solar_panel_output < 15));
    println!(
        "4. Is temperature good OR no clouds? {}",
        (temperature > 20 || !is_cloudy)
    );
    println!(
        "\nFinal result - Good day for solar energy production: {}",
        result
    );
}
