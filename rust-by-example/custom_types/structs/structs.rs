#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        bottom_right: Point { x: x1, y: y1 },
        top_left:     Point { x: x2, y: y2 },
    } = rect;

    (x1 - x2) * (y2 - y1)
}

fn square(point: Point, len: f32) -> Rectangle {
    Rectangle {
        bottom_right: Point { x: point.x + len, y: point.y },
        top_left:     Point { x: point.x, y: point.y + len },
    }
}

fn main() {
    let name = String::from("Maxim");
    let age = 27;
    let maxim = Person { name, age };

    println!("{:?}", maxim);


    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    // make a new point by using struct update syntax to use the fields of
    // our other one
    let bottom_right = Point { x: 5.2, ..point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    let new_rect = Rectangle {
        top_left:     Point { x: 1.5, y: 3.5 },
        bottom_right: Point { x: 4.5, y: 2.0 },
    };
    println!("new rect = {:?}", new_rect);
    println!("new_rect area = {}", rect_area(new_rect));

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} {:?}", integer, decimal);

    println!("square {:?}", square(point, 1.0));
}
