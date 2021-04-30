fn main() {
    for n in 1..101 {
    // for n in 1..=100 {
        if n % 15 == 0 {
            println!("{} fizzbuzz", n);
        } else if n % 3 == 0 {
            println!("{} fizz", n);
        } else if n % 5 == 0 {
            println!("{} buzz", n);
        } else {
            println!("{}", n);
        }
    }


    // the `for in` construct is able to interact with an `Iterator` in
    // several ways. By default the `for` loop will apply the `into_iter`
    // function to the collection. However, this is not the only means of 
    // converting collections into iterators.
    //
    // `into_iter`, `iter` and `iter_mut` all handle the conversion of 
    // a collection into an iterator in different ways, by providing
    // different views on the data within
    //
    // `iter` - this borrows each element of the collection through each
    // iteration. Thus leaving the collection untouched and available for
    // reuse after the loop

    let names = vec!["Bob", "Frank", "Alice"];

    for name in names.iter() {
        match name {
            &"Alice" => println!("There is a rustacean among us!"),
            _        => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    // `into_iter` - this consumes the collection so that on each iteration
    // the exact data is provided. Once the collection has been consumed it
    // is not longer available for reuse as it has been 'moved' within 
    // the loop
    let names2 = vec!["Maxim", "Kate", "Joseph"];
    for name in names2.into_iter() {
        match name {
            "Joseph" => println!("There is a cat among us!"),
            _        => println!("Hello {}", name),
        }
    }

    // println!("names: {:?}", names);

    // `iter_mut` - this mutably borrows each element of the collection,
    // allowing for the collection to be modified in place
    let mut names3 = vec!["John", "Jane", "Maria", "Diana"];
    for name in names3.iter_mut() {
        *name = match name {
            &mut "Jane" => "There is a stranger among us!",
            _           => "Hello",
        }
    }

    println!("names: {:?}", names3);
}
