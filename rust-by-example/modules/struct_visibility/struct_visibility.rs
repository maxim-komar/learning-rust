// structs have an extra level of visibility with their fields. The 
// visibility defaults to private, and can be overridden with the `pub`
// modifier. The visibility only matters when a struct is accessed from
// outside the module where it is defined, and has the goal of hiding
// information (encapsulation)
mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    let open_box = my::OpenBox { contents: "public information" };

    println!("The open box contains: {}", open_box.contents);

    // public structs with private fields cannot be constructed using
    // field names
    // let closed_box = my::ClosedBox { contents: "classified information" };

    let _closed_box = my::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed
    // println!("The closed box contains: {}", _closed_box.contents);
}
