// an attribute is metadata applied to some module, crate or item. This
// metadata can be used to/for:
//
//  - conditional compilation of code
//  - set crate name, version and type (binary or library)
//  - disable lints (warnings)
//  - enable compile features (macros, glob imports, etc.)
//  - link to a foreign library
//  - mark functions as unit tests
//  - mark functions that will be part of a benchmark

// when attributes apply to a whole crate, their syntax is 
// `#![crate_attribute]`, and when they apply to a module
// or item, the syntax is `#[item_attribute]` 

// attributes can take arguments with different syntaxes
//
//  - #[attribute = "value"]
//  - #[attribute(key = "value")]
//  - #[attribute(value)]
//
//  attributes can have multiple values and can be separated over 
//  multiple lines, too:
//
//  #[attribute(value, value2)]
//
//  #[attribute(value, value2, value3,
//              value4, value5)]


fn used_function() {}

// `#[allow(dead_code)]` is an attribute that disables the 
// `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {}

// configuration conditional checks are possible through two
// different operators:
//   - the `cfg` attribute: `#[cfg(...)]` in attribute position
//   - the `cfg!` macro: `cfg!(...)` in boolean expressions
//
// while the former enables conditional compilation, the latter
// conditionally evaluates to `true` or `false` literals allowing 
// for checks at run-time. Both utilize identical argument syntax
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux");
}

#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    used_function();

    are_you_on_linux();
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }

    conditional_function();
}
