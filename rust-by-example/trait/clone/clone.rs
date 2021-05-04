// When dealing with resources, the default behaviour is to transfer them 
// during assignments or function calls. However, sometimes we need to make
// a copy if the resource as well
//
// The `Clone` trait helps us do exactly this. Most commonly, we can use
// the `.clone()` method defined by the `Clone` trait
#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let unit = Unit;

    let copied_unit = unit;

    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Error! `pair` has lost its resources
    // println!("original: {:?}", pair);

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);

    // Error! `moved_pair` has been dropped
    // println!("copy: {:?}", moved_pair);

    println!("clone: {:?}", cloned_pair);
}
