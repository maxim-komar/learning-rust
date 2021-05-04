// The arguments of a macro a prefixed by a dollar sign `$` and type
// annotated with a `designator`:
macro_rules! create_function {
    // This macro takes an argument of designator `ident` and creates
    // a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()",
                stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is` into a string
        println!("{:?} = {:?}",
            stringify!($expression),
            $expression
        );
    }
}

// These are some of the available designators:
// 
//  - `block`
//  - `expr` is used for expressions
//  - `ident` is used for variable/function names
//  - `item`
//  - `literal` is used for literal constants
//  - `pat` (pattern)
//  - `path`
//  - `stmt` (statement)
//  - `tt` (token tree)
//  - `ty` (type)
//  - `vis` (visibility qualifier)


// Macros can be overloaded to accept different combinations of arguments.
// In that regard, `macro_rules!` can work similarly to a match block
macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };

    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}


// Macros can use `+` in the argument list to indicate that an argument may
// repeat at least once, or `*`, to indicate that the argument may repeat 
// zero or more times
// 
// In the following example, surrounding the matcher with `$(...),+` will
// match one or more expression, separated by commas. Also note that the 
// semicolon is optional on the last case.
macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });

    //
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);

    //
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}
