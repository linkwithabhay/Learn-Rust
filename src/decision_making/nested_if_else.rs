/// # Nested If - Else
///
/// We can nest `if-elif-else` statements within each other.
/// This allows us to create hierarchical decision-making structures.
///
/// For example:
/// ```rust
/// if age > 18 {
///     if hasLicense {
///         println!("You can drive");
///     } else {
///         println!("Get a license first");
///     }
/// } else {
///     println!("Too young to drive");
/// }
/// ```
///
/// It can be infinite nested:
/// ```rust
/// if condition1 {
///     if condition2 {
///         if condition3 {
///             // if condition1, condition2 and condition3 are true
///             ...
///         }
///     }
/// }
/// ```
///
/// ## Challenge
///
/// Create a program that checks if someone can ride a rollercoaster.
/// * The requirements are:
/// * Must be at least **12** years old
/// * Must be taller than **150cm**
/// * If they meet both requirements but are under **15**, they need adult supervision
///
/// Print exactly these messages for each case:
/// * If too young: `"Sorry, you're too young"`
/// * If not tall enough: `"Sorry, you're not tall enough"`
/// * If under 15 and no adult: `"Sorry, you need an adult with you"`
/// * If under 15 with adult: `"You can ride with adult supervision!"`
/// * If 15 or older and tall enough: `"You can ride by yourself!"`
pub fn learn() {
    let age_input = "16"; // comment this line to take user-input
    let height_input = "155"; // comment this line to take user-input
    let adult_input = "true"; // comment this line to take user-input
    println!(
        "User inputs: {}, {}, {}",
        age_input, height_input, adult_input
    ); // comment this line to take user-input
    // uncomment below lines to take user-input
    /*
    let mut age_input = String::new();  // uncomment this line to take user-input
    let mut height_input = String::new();  // uncomment this line to take user-input
    let mut adult_input = String::new();  // uncomment this line to take user-input

    io::stdin().read_line(&mut age_input).unwrap();  // uncomment this line to take user-input
    io::stdin().read_line(&mut height_input).unwrap();  // uncomment this line to take user-input
    io::stdin().read_line(&mut adult_input).unwrap();  // uncomment this line to take user-input
    */

    let age: i32 = age_input.trim().parse().unwrap();
    let height: i32 = height_input.trim().parse().unwrap();
    let has_adult: bool = adult_input.trim().parse().unwrap();

    // Write your code below
    if age < 12 {
        println!("Sorry, you're too young");
    } else if height <= 150 {
        println!("Sorry, you're not tall enough");
    } else if age < 15 {
        if !has_adult {
            println!("Sorry, you need an adult with you");
        } else {
            println!("You can ride with adult supervision!");
        }
    } else {
        println!("You can ride by yourself!");
    }
}
