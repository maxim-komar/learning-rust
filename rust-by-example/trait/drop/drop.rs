// The `Drop` trait only has one method: `drop`, which is called 
// automatically when an object goes out of scope. The main use of the
// `Drop` trait is to free the resources that the implementor instance
// owns.
//
// `Box`, `Vec`, `String`, `File` and `Process` are some examples of
// types that implement the `Drop` trait to free resources. The `Drop`
// trait can also be manually implemented for any custom data type.
//
// The following example adds a print to console to the `drop` function
// to announce when it is called.

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Jist exited block B");

        println!("Exiting block A");
    }
    println!("Jist exited block A");

    // variable can be manually dropped using the `drop` function
    drop(_a);

    println!("end of the main function");
}
