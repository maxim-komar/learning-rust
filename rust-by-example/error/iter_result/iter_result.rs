fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> =
        strings
            .into_iter()
            .map(|s| s.parse::<i32>())
            .collect();
    println!("Results {:?}", numbers);

    let strings2 = vec!["tofu", "93", "18"];
    let numbers2: Vec<_> =
        strings2
            .into_iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
    println!("Results {:?}", numbers2);

    // `Result` implements `FromIter` so that a vector of results
    // (`Vec<Result<T, E>>`) can be turned into a result with a vector
    // (`Result<Vec<T>, E>`). Once an Result::err is foundm the iteration
    // will terminate
    let strings3 = vec!["tofu", "93", "18"];
    let numbers3: Result<Vec<_>, _> =
        strings3
            .into_iter()
            .map(|s| s.parse::<i32>())
            .collect();
    println!("Results: {:?}", numbers3);

    let strings4 = vec!["tofu", "93", "18"];
    let (numbers4, errors4): (Vec<_>, Vec<_>) =
        strings4
            .into_iter()
            .map(|s| s.parse::<i32>())
            .partition(Result::is_ok);
        println!("Numbers: {:?}", numbers4);
        println!("Errors: {:?}", errors4);

    let strings5 = vec!["tofu", "93", "18"];
    let (numbers5, errors5): (Vec<_>, Vec<_>) =
        strings5
            .into_iter()
            .map(|s| s.parse::<i32>())
            .partition(Result::is_ok);
    let numbers5: Vec<_> = numbers5.into_iter().map(Result::unwrap).collect();
    let errors5: Vec<_> = errors5.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers5);
    println!("Errors: {:?}", errors5);
}
