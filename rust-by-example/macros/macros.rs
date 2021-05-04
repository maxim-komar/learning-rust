// Rust provides a powerful macro system that allows metaprogramming. Macros
// look like functions, except that their name ends with a bang `!`, but 
// instead of generating a function call, macros are expanded into source
// code that gets compiled with the rest of the program. However, unlike 
// macros in C and other languages, Rust macros are expanded into abstract
// syntax trees, rather than string preprocessing, so you don't get 
// unexpected precedence bugs.
//
// Macros are created using the `macro_rules!` macro

macro_rules! say_hello {
    // `()` indicates that the macro takes no argument
    () => {
        println!("Hello!");
    };
}

fn main() {
    // this call will expand into `println!("Hello");`
    say_hello!();
}
