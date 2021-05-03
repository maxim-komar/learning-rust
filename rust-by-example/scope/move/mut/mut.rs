// mutability of data can be changed when ownership is transferred
fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // mutability error
    // *immutable_box = 4;

    // *move* the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    *mutable_box = 4;

    println!("mutable_box contains {}", mutable_box);
}
