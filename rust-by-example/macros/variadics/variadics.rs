// A variadic interface takes an arbitrary number of arguments. For example,
// `println!` can take an arbitrary number of arguments, as determined by 
// the format string

macro_rules! calculate {
    (eval $e:expr) => {{
        let val: usize = $e;
        println!("{} = {}", stringify!{$e}, val);
    }};


    // decompose multiple `eval`s recursively
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

fn main() {
    calculate! {
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
