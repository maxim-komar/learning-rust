fn main() {
    fn function(i: i32) -> i32 { i + 1 }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i|               i + 1  ;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one());

    // capturing
    //
    // closures can capture variables:
    //   - by reference: &T
    //   - by mutable reference: &mut T
    //   - by value: T
    use std::mem;

    let color = String::from("green");

    let print = || println!("`color`: {}", color);

    // call the closure using the borrow
    print();

    // `color` can be borrowed immutably again, because the closure only
    // holds an immutable reference to `color`
    let _reborrow = &color;
    print();

    // a move or reborrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;
    // a closure to increment `count` could take either `&mut count` or
    // `count` but `&mut count` is less restrictive so it takes that.
    // Immediately borrows `count`
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus
    // calling the closure mutates the closure which requires a `mut`
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    // the closure still mutable borrows `count` beacause it is called later
    // an attempt to reborrow will lead to an error.
    // let _reborrow = &count;

    inc();

    // the closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

    // a non-copy type
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into 
    // the closure
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();

    // using `move` before vertical pipes forces closure to take ownership
    // of captured variables
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("contains 1: {}", contains(&1));
    println!("contains 4: {}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    // ^ uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after
    // it has been moved
    //
    // removing `move` from closure`s signature will cause closure
    // to borrow `haystack` variable immutably, hence `haystack` is 
    // still available and uncommenting above line will not cause an error


    // as input parameter
    fn apply<F>(f: F) where
        F: FnOnce() {
            f();
        }

    fn apply_to_3<F>(f: F) -> i32 where
        F: Fn(i32) -> i32 {
            f(3)
        }


    // input parameters

    let greeting = "hello";
    // a non-copy type
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I Can sleep. zzzzzz");

        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));


    // type anonimity
    let x = 7;

    // capture `x` into an anonymouse type and implement
    // `Fn` for it. Store it in `print`
    let print = || println!("{}", x);
    apply(print);


    // input functions

    // define a function which takes a generic `F` argument
    // bounded by `Fn`, and calls it
    fn call_me<F: Fn()>(f: F) {
        f();
    }

    fn func() {
        println!("I'm a function!");
    }

    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(func);


    // as output parameters
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();

        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
