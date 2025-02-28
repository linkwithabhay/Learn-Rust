/// # Comparison Operators
///
/// Comparison operators are used to compare between two operands.
///
/// Sometimes we need to check whether an operand is bigger/smaller/... than another operand.
/// The following table shows possible operators for comparison:
///
/// | Operator | Meaning          | Example              |
/// |----------|------------------|----------------------|
/// | ==       | Equal            | 1 == 2 returns false |
/// | !=       | Not Equal        | 1 != 2 returns true  |
/// | >        | Greater Than     | 1 > 2 returns false  |
/// | <        | Lower Than       | 1 < 2 returns true   |
/// | >=       | Greater or Equal | 1 >= 2 returns false |
/// | <=       | Lower or Equal   | 1 <= 2 returns true  |
///
/// The comparison operator returns `true` if the comparison is correct or `false` otherwise.
///
/// For example:
/// ```rust
/// let var1: i32 = 13;
/// let var2: i32 = 12;
/// let var3: bool = var1 != var2;
/// ```
///
/// `var3` will hold `true` because `var1` and `var2` are not equal.
///
/// ## Challenge
///
/// Write a script that initializes `2` variables `n1` and `n2`
/// with the values `8` and `9` (accordingly).
///
/// After that initialize another variable `n3` that will hold whether `n1` is bigger than `n2`.
pub fn learn() {
    // Type your code below
    let n1 = 8;
    let n2 = 9;
    let n3 = n1 > n2;

    // Don't change the line below
    println!("n1 = {}, n2 = {}, n3 = {}", n1, n2, n3);
}
