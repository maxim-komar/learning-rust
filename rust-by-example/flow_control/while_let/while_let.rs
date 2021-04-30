fn main() {
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` as `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}
