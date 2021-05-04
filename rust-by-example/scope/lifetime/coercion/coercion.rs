// A longer lifetime can be coerced into a shorted one so that it works
// inside a scope it normally wouldn't work in. This comes in the form of
// inferred coercion by the Rust compiler, and also in the form of declaring
// a lifetime difference

fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2;  // longer lifetime

    {
        let second = 3; // shorter lifetime

        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}
