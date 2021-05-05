// Vectors are re-sizable arrays. Like slices, their size is not known at
// compile time, but they can grow or shrink at any time. A vector is 
// represented using 3 parameters:
//
//  - pointer to the data
//  - length
//  - capacity
//
//  The capacity indicates how much memory is reserved for the vector. The
//  vector can grow as long as the length is smaller than the capacity.
//  When this threshold needs to be surpassed, the vector is reallocated
//  with a larger capacity.

fn main() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // Error! Immutable vectors can't grow
    // collected_iterator.push(0);

    println!("Vector length: {}", xs.len());

    println!("Second element: {}", xs[1]);

    println!("Pop last element: {:?}", xs.pop());

    // Error! Out of bounds indexing yields a panic
    // println!("Fourth element: {}", xs[3]);

    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }

    println!("Updated vector: {:?}", xs);
}
