#![allow(unreachable_code)]
#![allow(unused_labels)]

fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // infinite loop
    loop {
        count += 1;
        if count == 3 {
            println!("three");

            // skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Ok, that's enough");

            // exit this loop
            break;
        }
    }


    // nesting and labels
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // this would break only the inner loop
            // break;

            // this breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");


    // returning from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
