fn main() {
    // variables can be type annotated
    let _logical: bool = true;

    let _a_float: f64 = 1.0;     // Regular annotation
    let _an_integer   = 5i32;    // Suffix annotation

    let _default_float   = 3.0;  // f64
    let _default_integer = 7;    // i32

    // a type can also be inferred from context
    let mut _inferred_type = 12;     // type i64 is inferred from another line
    _inferred_type = 4294967296i64;

    // a mutable variable's value can be changed
    let mut _mutable = 12;       // mutable i32
    _mutable = 21;

    // error! the type of a variable can't be changed
    // mutable = true;

    // variables can be overwritten with shadowing
    let _mutable = true;
}
