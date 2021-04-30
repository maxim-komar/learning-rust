// rust has two different types of constants which can be declared
// in any scope including global. Both require explicit type annotation
//  - const: an unchangeable value (the common case)
//  - static: a possibly mutable variable with 'static lifetime
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // THRESHOLD = 5;
}
