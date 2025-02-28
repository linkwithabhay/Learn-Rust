/// # Logical Operators Part 2
///
/// Logical operators have a special table called "Truth table" that shows
/// what the combination of logical operators returns.
///
/// Truth table for the `and` (`&&`) operator:
///
/// | a     | b     | a && b |
/// |-------|-------|--------|
/// | false | false | false  |
/// | false | true  | false  |
/// | true  | false | false  |
/// | true  | true  | true   |
///
/// The only way to get a `true` for the `and` (`&&`) operator is if both `a` and `b` are `true`.
///
///
/// Truth table for the `or` (`||`) operator:
///
/// | a     | b     | a \|\| b |
/// |-------|-------|----------|
/// | false | false | false    |
/// | false | true  | true     |
/// | true  | false | true     |
/// | true  | true  | true     |
///
/// In this case, to get a `true` result, either `a` or `b` should be `true`.
///
///
/// Truth table for the `not` (`!`) operator:
///
/// | a     | !a    |
/// |-------|-------|
/// | false | true  |
/// | true  | false |
///
/// Here the value of `a` is reversed. If `a` is `false` then `!a` is `true`.
///
/// ## Challenge
///
/// You need to assign integer values to variables `b1` and `b2` so that
/// `b3` evaluates to `true` in the expression: `b3 = !((b1 + b2) > (b1 * b2))`.
pub fn learn() {
    // Type your code below
    let b1: i32 = 2;
    let b2: i32 = 3;
    let b3: bool = !((b1 + b2) > (b1 * b2));

    // Don't change the line below
    println!("b3 = {}", b3);
}
