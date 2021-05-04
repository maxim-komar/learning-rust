// data can be immutably borrowed any number of times, but while 
// immutably borrowed, the original data can't be mutably borrowed.
// On the other hand, only one mutable borrow is allowed at a time.
// The original data can be borrowed again only after the mutable 
// reference has been used for the last time.

struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    println!("Point has coordinates: ({}, {}, {})",
        borrowed_point.x,
        another_borrow.y,
        point.z
    );

    // Error! Can't borrow `point` as mutable because it's currently
    // borrowed as immutable
    // let mutable_borrow = &mut point;

    println!("Point has coordinates: ({}, {}, {})",
        borrowed_point.x,
        another_borrow.y,
        point.z
    );

    // the immutable references are no longer used for the rest of the code
    // so it is possible to reborrow with a mutable reference
    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // Error! Can't borrow `point` as immutable because it's currently 
    // borrowed as mutable
    // let y = &point.y;

    // println!("Point Z coordinate is {}", point.z);

    println!("Point has coordinates: ({}, {}, {})",
        mutable_borrow.x,
        mutable_borrow.y,
        mutable_borrow.z
    );

    // the mutable reference is no longer used for the rest of the code so
    // it is possible to reborrow
    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
        new_borrowed_point.x,
        new_borrowed_point.y,
        new_borrowed_point.z
    );
}
