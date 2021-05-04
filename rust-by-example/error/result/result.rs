// `Result` is a reacher version of the `Option` type that describes
// possible error instead of possible absence
//
// That is, `Result<T, E>` could have one of two outcomes:
//
//  - `Ok(T)` - an element `T` was found
//  - `Err(E)` - an error was found with element `E`

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    // In the unsuccessful case, `parse()` leaves us with an error for 
    // `unwrap()` to `panic` on. Additionally the `panic` exits our program
    // and provides an unpleasant error message.
    let tt = multiply("t", "2");
    println!("double is {}", tt);
}
