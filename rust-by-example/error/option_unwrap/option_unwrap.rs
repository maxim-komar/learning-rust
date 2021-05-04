fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest"),
        Some(inner)   => println!("{}? How nice", inner),
        None          => println!("No gift? Oh well"),
    }
}

fn give_royal(gift: Option<&str>) {
    // `unwrap` returns a `panic` when it receives s `None`
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaa!!!"); }

    println!("I love {}s!!!", inside);
}

fn main() {
    let food = Some("cabbage");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_royal(bird);
    give_royal(nothing);
}
