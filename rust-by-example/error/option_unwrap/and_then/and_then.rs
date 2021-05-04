// `map()` was described as a chainable way to simplify `match` statements.
// However, using `map()` on a function that returns an `Option<T>` results
// in the nested `Option<Option<T>>`. Chaining multiple calls together can 
// then become confusing. That's where anothe combinator called `and_then()`,
// known in some languages as flatmap, comes in.
//
// `and_then()` calls its function input with the wrapped value and returns 
// the result. If the `Option` is `None`, then it returns `None` instead.
//
// In the following example, `cookable_v2()` results in an `Option<Food>`.
// Using `map()` instead of `and_then` would have given an 
// `Option<Option<Food>>`, which is an invalid type for `eat()`

#![allow(dead_code)]

#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _                => Some(food),
    }
}

fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None       => None,
        Some(food) => match have_ingredients(food) {
            None       => None,
            Some(food) => Some(food),
        },
    }
}

fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}", day),
    }
}

fn main() {
    let cordon_bleu = Food::CordonBleu;
    let steak       = Food::Steak;
    let sushi       = Food::Sushi;

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}
