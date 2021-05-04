// The most basic way of handling mixed error types is to just embed
// them in each other

use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

fn double_first2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(empty));
    println!("The first doubled is {:?}", double_first(strings));


    let numbers2 = vec!["42", "93", "18"];
    let empty2 = vec![];
    let strings2 = vec!["tofu", "93", "18"];
    println!("The first doubled is {:?}", double_first2(numbers2));
    println!("The first doubled is {:?}", double_first2(empty2));
    println!("The first doubled is {:?}", double_first2(strings2));
}
