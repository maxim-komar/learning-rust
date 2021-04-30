#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        1                  => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19            => println!("A teen"),
        _                  => println!("Ain't special"),
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true  => 1,
    };

    println!("{} -> {}", boolean, binary);

    // destructuring tuples
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)   => println!("First is `1` and the rest doesn't matter"),
        _         => println!("It doesn't matter what they are"),
    }

    // destructuring enums
    let color = Color::RGB(122, 17, 40);

    match color {
        Color::Red   => println!("The color is Red"),
        Color::Blue  => println!("The color is Blue"),
        Color::Green => println!("The color is Green"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, blue: {}", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}", c, m, y),
        Color::CMYK(c, m, y, b) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, black: {}", c, m, y, b),
    }


    // destructuring pointers/ref
    //      dereferencing uses `*`
    //      destructuring uses `&`, `ref` and `ref mut`
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // to avoid the `&`, you dereference before matching
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;

    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10, `mut_value`: {:?}", m);
        }
    }

    // destructuring struct
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        Foo { y, .. }      => println!("y = {}, we don't care about x", y),

        // this will give an error: pattern does not mention field `x`
        // Foo { y } => println!("y = {}", y),
    }


    // guards
    let pair = (2, -2);

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y     => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _                    => println!("No correlation..."),
    }


    // bindings
    //
    // indirectly accessing a variable makes it impossible to branch
    // and use that variable without rebinding. `match` provides
    // the `@` sigil for binding values to names:

    fn age() -> u32 {
        15
    }

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        n             => println!("I'm an old person of age {:?}", n),
    }


    fn some_number() -> Option<u32> {
        Some(42)
    }

    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n)      => println!("Not interesting... {}", n),
        _            => (),
    }
}
