fn main() {
    let i = 3;  // lifetime for `i` starts

    {
        let borrow1 = &i;   // `borrow1` lifetimes starts

        println!("borrow1: {}", borrow1);
        // `borrow1` ends
    }

    {
        let borrow2 = &i;   // `borrow2` lifetime starts
        println!("borrow2: {}", borrow2);
    }

    // lifetime for `i` ends
}
