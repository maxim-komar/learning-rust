// methods are functions attached to objects. These methods have access to
// the data of the object and its other methods via the `self` keyword.
// Methods are defined under an `impl` block

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // this is a static method
    // static methods don't need to be called by an instance
    // these methods are generally used as constructors
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // another static method, taking two arguments
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // this is an instance method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // this method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // this method 'consumes' the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self) {
        // destructure `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
    }
}

fn main() {
    // static methods are called using double colons
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // instance methods are called using the dot operator
    // note that the first argument `&self` is implicitly passed, i.e.
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // rectangle.translate(1.0, 0.0);

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
    // error! Previous `destroy` call 'consumed' `pair`
    // pair.destroy();
}
