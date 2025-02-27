/// A higher-order function that takes a function name and a function, then returns a closure 
/// that when called, prints the function name, calls the provided function, and retuns the 
/// result of the function executed.
///
/// ### Parameters
/// - `func_name`: A string slice (`&str`) representing the name of the function to be printed.
/// - `f`: A function `f` that takes no arguments and returns a value of type `R`. This function 
///   is invoked inside the returned closure.
///
/// ### Returns
/// Returns a closure that, when called, will:
/// 1. Print a formatted header with the `func_name`.
/// 2. Call the function `f()` and capture the result.
/// 3. Print a footer after the function call.
/// 4. Return the result of the function `f`.
///
/// ### Example
/// ```rust
/// let hello_fn = || {
///     println!("Hello, world!");
/// };
/// let wrapped_hello = call("Hello Function", hello_fn);
/// wrapped_hello(); // Prints the function name, "Hello Function", call the provided function, and return its result.
/// ```
/// 
/// ### Notes
/// This function is generic, allowing it to work with any function that takes no arguments and returns a value.
///
/// ### Type Parameters
/// - `F`: The type of the function passed to `call`, which must implement `Fn() -> R`.
/// - `R`: The return type of the function `f`.
pub fn call<F, R>(func_name: &str, f: F) -> impl Fn() -> R
where
    F: Fn() -> R,
{
    move || {
        println!("\n=============================================");
        println!("{}", func_name);
        println!("---------------------------------------------");
        let result = f();
        println!("=============================================");
        result
    }
}