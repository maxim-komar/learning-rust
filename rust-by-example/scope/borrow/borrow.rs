// Most of the time, we'd like to access data without taking ownership over
// it. To accomplish this, Rust uses a borrowing mechanism. Instead of 
// passing objects by value (`T`), objects can be passed by reference (`&T`)
//
// The compiler statically guarantees (via its borrow checker) that
// reference always point to valid object. That is, while references to 
// an object exist, the object cannot be destroyed.

fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Borrow the contents of the box. Ownership is not taken,
    // so the contents can be borrowed again
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error!
        // Can't destroy `boxed_i32` while the inner value is borrowed later
        // in scope
        // eat_box_i32(boxed_i32);

        borrow_i32(_ref_to_i32);
    }

    // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
    eat_box_i32(boxed_i32);
}
