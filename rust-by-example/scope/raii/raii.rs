// variables in Rust do more than just jold data in the stack: the also
// own resources, e.g. Box<T> owns memory in the heap. Rust enforces 
// RAII (Resource Acquisition Is Initialiation), so whenever an object
// goes out of scope, its destructor is called and its owned resources
// are freed
fn create_box() {
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    // allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // a nested scope:
    {
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
    }

    for _ in 0u32..1_000 {
        create_box();
    }

    let _x = ToDrop;
    println!("Made a ToDrop!");

    // `_box2` is destroyed here, and memory gets freed
}
