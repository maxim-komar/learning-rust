// Sometimes it simplifies the code to mask all of the different errors
// with a single type of error. We'll show this with a custom error.
//
// Rust allows us to define our own error types. In general, a "good"
// error type:
//
//  - represents different errors with the same type
//  - presents nice error messages to the user
//  - is easy to compare with other tyoes
//      * Good: `Err(EmptyVec)`
//      * Bad: `Err("Please use a vector with at least one element".to_owned())
//  - can hold information about the error
//      * Good: `Err(BadChar(c, position))`
//      * Bad: `Err("+ cannot be used here".to_owned())
//
//  - composes well with other errors

use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
