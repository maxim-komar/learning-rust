use std::mem;

// this function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // fixed-size array (type signatures is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // all elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // len return the count of elements in the array
    println!("number of elements in array: {}", xs.len());

    // arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // arrays van be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slces can point to a section of an array
    // they are of the form [starting_index .. ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // out of bound indexing causes compile error
    // println!("{}", xs[5]);
}
