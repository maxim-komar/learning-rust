// The borrow checker uses explicit lifetime annotations to determine
// how long references should be valid. In cases where lifetimes are not 
// elided, Rust requires explicit annotations to determine what the
// lifetime of a reference should be. The syntax for explictily annotating
// a lifetime uses an aposstrophe character as follows
//
// foo<'a>
//
// In cases with multiple lifetimes, the syntax is similar
//
// foo<'a, 'b>


fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;

    // Error! `_x` does not live long enough
    // let y: &'a i32 = &_x;
    //
    // Attempting to use the lifetime `'a` as an explicit type annotation
    // inside the function will fail beacause the lifetime of `&_x` is shorter
    // than that of `y`. A short lifetime can't be coerced into a longer one.
}

fn main() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);

    failed_borrow();
}
