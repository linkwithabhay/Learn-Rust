/// # String
///
/// A char is a single character (For example: 1, 6, %, b, p, ., T, etc.)
///
/// The String type is a special type that consists of multiple chars.
///
/// To initialize a string value in a variable, enclose it within double quotation marks:
/// ```rust
/// let s1 = "This is a string";
/// ```
///
/// In the above example, a string variable named `s1` is initialized.
///
/// Unlike with numbers, when working with strings we need to be more careful with types.
/// If you want to explicitly declare a String type, you need to convert the string like this:
/// ```rust
/// let s1: String = "This is a string".to_string();
/// ```
///
/// ## Challenge
///
/// Store the string `"I am learning to code with Coddy!"` in a variable named `coddy`
/// using a string literal (with double quotes).
pub fn learn() {
    // Type your code below

    let str_coddy = "I am learning to code with Coddy!".to_string();
    // String Literal
    let lit_str_coddy = "I am learning to code with Coddy!";

    // Don't change the line below
    println!("String :: coddy = \"{}\"", str_coddy);
    println!("Literal string :: coddy = \"{}\"", lit_str_coddy);
}
