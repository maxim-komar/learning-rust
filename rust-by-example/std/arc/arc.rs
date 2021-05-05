// When shared ownership between threads is needed, `Arc` (Atomic Reference 
// Counted) can be used. This struct, via the `Clone` implementation can 
// create a reference pointer for the location of a value in the memory
// heap while increasing the reference counter. As it shares ownership
// between threads, when the last reference pointer to a value is out of
// scope, the variable is dropped
fn main() {
    use std::sync::Arc;
    use std::thread;

    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            println!("{:?}", apple);
        });
    }
}
