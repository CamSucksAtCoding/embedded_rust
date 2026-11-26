/*
 * Error Handling: with Result Type Enum
 *
 * Result<T, E> is the type used for returning and propigating erros. It is an enum with the variants, Ok(T), which represents
 * success and contains a value, and Err(E), which represents an error, which also contains a value.
 *
 * Basic Format:
 *      enum Result<T, E> {
 *          Ok(T),
 *          Err(E),
 *      }
 *
 * Error Handling: Options for safe error handling in Rust
 *               - Result Enum: used for handling recoverable errors
 *               - Option Enum
 *               - Panic macro: When a function uses panic! its essentially making a
 *                              decision to stop the program execution due to an
 *                              unrecoverable error.
 *               - unwrap() and expect() methods
 *               - ? operator: Used for propagating errors or early returning from a function when
 *                             an error occurs. It works by unwrapping the Result or Option value
 *                             and returning early with an errot if it contains Err or None. If the
 *                             value is Ok or Some, the inner value is returned and execution continues.
 *                                  - Pros: I dont have to keep writing match statements, so thats good enough for me
 *
 */
//You can use any types in the Result fields, I just used two different ones to simpulate C's error exit code format
fn add_strings(str1: &str, str2: &str) -> Result<String, i32> {
    let ok_string = format!("{str1} {str2}");
    if str1.is_empty() || str2.is_empty() {
        println!("One of your inputs passed was empty");
        return Err(1011);
    }
    Ok(ok_string)
}
fn main() {
    let combined_strings = add_strings("Yo", "");
    //x is just a temporary variable
    match combined_strings {
        Ok(valid) => println!("Result = {valid}"),
        Err(error) => println!("Exit Code: {error}"),
    }
    /*
     * Come back later to understand how to implement the ? operator, it might need it own session
     */
    // let combined_strings_try_operator_valid = add_strings("Optimus", "Prime")?;
    // let combined_strings_try_operator_invalid = add_strings("K", "")?;
}
