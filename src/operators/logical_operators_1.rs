/// # Logical Operators Part 1
///
/// Logical operators are used to check combinations of comparisons that return `true` or `false`.
///
/// For example the following statement contains two comparisons:
///
/// Is 5 greater than 3 and less than 6?
///
/// | Operator | Meaning                             | Example  |
/// |----------|-------------------------------------|----------|
/// | &&       | And - true if all operands are true | a && b   |
/// | \|\|     | Or - true if any operand is true    | a \|\| b |
/// | !        | Not - true if the operand is false  | !a       |
///  
///
/// Let's see some examples,
///
/// 5 is bigger than 3 and 1 equals 1,
/// ```rust
/// let b1: bool = (5 > 3) && (1 == 1); // holds true
/// ```
/// Explanation: All of the operands are `true`, so b1 will hold `true`
/// (`and` operation is `true` if both operands are `true`).
///
///
/// 5 is not equals 4 or 5 equals 2,
/// ```rust
/// let b2: bool = !(5 == 4) || (5 == 2); // holds true
/// ```
/// Explanation: The first operand `(5 != 4)` is `true` so `b2` is also `true`
/// (`or` operation is `true` if either one of the operands is `true`)
///
///
/// 1 is not equals 1 or false,
/// ```rust
/// let b3: bool = !(1 == 1) || false; // holds false
/// ```
/// Explanation: All of the operands are `false`, so `b3` will hold `false` (`or` operation).
///
///
/// not (3 is bigger than 4),
/// ```rust
/// let b4: bool = !(3 > 4); // holds true
/// ```
/// Explanation: The operand is `false`, so `b4` will hold `true` (`not` operation).
///
///
/// not (5 is bigger than 10 or 5 is bigger than 1),
/// ```rust
/// let b5: bool = !(5 > 10 || 5 > 1); // holds false
/// ```
/// Explanation: `5 > 10 || 5 > 1` is `true` (one of the operands is `true`),
/// so in total `b5` is `false` (`not` operation).
///
/// ## Challenge
///
/// You are provided with code. Assign values to the variables `b1` and `b2`
/// with Boolean values so that `b3` holds true.
pub fn learn() {
    // Type your code below
    let b1: bool = true;
    let b2: bool = false;
    let b3: bool = b1 || b2;

    // Don't change the line below
    println!("b3 = {}", b3);
}
