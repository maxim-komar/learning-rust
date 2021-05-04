use std::fmt::Debug;

static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn print_it(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn main() {
    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // when `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary
    }

    {
        let lifetime_num = 9;

        // coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);

    // `i` is owned and contains no references, thus it's 'static
    let i = 5;
    print_it(i);

    // Error! `&i` only has the lifetime defined by the scope of
    // `print_it()`, so it's not 'static:
    // print_it(&i);
}
