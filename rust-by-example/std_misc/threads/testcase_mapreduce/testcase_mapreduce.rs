// Rust makes it very easy to parallelise data processing, without many
// of the headaches traditionally associated with such an attempt.
//
// The standard library provides great threading primitives out of the box.
// These, combined with Rust's concept of Ownership and aliasing rules, 
// automatically prevent data races.
//
// The aliasing rules (one writeable reference XOR many readable references)
// automatically prevent you from manipulating state that is visible to 
// other threads. (Where synchronisation is needed, there are synchronisation
// primitives like `Mutex` or `Channel`)
//
// In this example, we will calculate the sum of all digits in a block of
// numbers. We will do this by parcelling out chunks of the block into 
// different threads. Each thread will sum its tiny block of digits, and
// subsequently we will sum the intermediate sums produced by each thread.
//
// Note that, although we're passing references across thread boundaries, 
// Rust understands that we're only passing read-only references, and that
// thus no unsafety or data races can occur. Because we're `move`-ing the
// data segments into the thread, Rust will also ensure the data is kept 
// alive until threads exit, so no dangling pointers occur.

use std::thread;

const NUMBER_OF_THREADS: u32 = 10;

fn main() {
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    let mut children = vec![];

    // Map phase
    let chunked_data = data.split_whitespace();

    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        children.push(thread::spawn(move || -> u32 {
            let result =
                data_segment
                    .chars()
                    .map(|c| c.to_digit(10).expect("should be a digit"))
                    .sum();

            println!("processed segment {}, result={}", i, result);

            result
        }));
    }


    // Reduce phase
    let final_result =
        children
            .into_iter()
            .map(|c| c.join().unwrap())
            .sum::<u32>();

    println!("Final sum result: {}", final_result);
}
