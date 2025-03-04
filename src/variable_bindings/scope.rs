/// # Scope
///
/// A scope is a place between curly braces `{}` that variables or
/// other form of declaration live.
///
/// For example:
/// ```rust
/// fn main() {
///     let x = 5;
///     // x comes into scope
///     println!("x is {}", x);
/// } // x goes out of scope here
/// ```
/// Scopes can be nested within other scopes. Inner scopes can access variables from
/// outer scopes, but not vice versa:
/// ```rust
/// fn main() {
///     let outer = 42;
///     
///     {  // new inner scope
///         let inner = 99;
///         println!("{}, {}", outer, inner);
///     }  // inner goes out of scope here
///     
///     // This would cause an error
///     // inner is not accessible here
///     // println!("{}", inner);
/// }
/// ```
#[allow(unused_variables)]
pub fn learn() {
    let outer = 10;
    {
        let inner = 20;
        let sum = outer + inner;
    }
    println!("println!(\"{{}}\", sum);");
    print!("Above line will throw an error because we are trying to print 'sum'");
    println!(" in outer scope but it was declared in an inside scope.");
}
