// diverging functions never return. They are marked using `!`, which
// is an empty type
#[allow(dead_code)]
fn foo() -> ! {
    panic!("This call never returns.");
}

// as opposed to all the other types, this one cannot be instantiated, 
// beacause the set of all possible values this type can have is empty.
// Note that, it is different from the `()` type, which has exactly one 
// possible value
//
// for example, this function returns as usual
#[allow(dead_code)]
fn some_fn() {
    ()
}


// as opposed to this function, which will never return the control back to the caller
// #![feature(never_type)]

fn main() {
    // let x: ! = panic!("This call never returns.");
    // println!("You will never see this line!");

    // although this might seem like an abstract concept, it is in fact very
    // useful and often handy. The main advantage of this type is that it 
    // can be cast to any other one and therefore used at places where an
    // exact type is required, for instance in `match` branches. This allows 
    // us to write code like this
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1 {
                // the `i` variable is of type u32, whch is perfectly fine
                true  => i,
                // on the other hand, the 'continue' expression does not
                // return u32, but it is still fine, because it never returns
                // and therefore does not violate requirements of the match
                // expression
                false => continue,
            };
            acc += addition;
        }
        acc
    }

    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
