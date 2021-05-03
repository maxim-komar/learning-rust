// Mutable data can be mutably borrowed using `&mut T`. This is called a
// mutable reference and gives read/write access to the borrower. In
// contrast, `&T` borrows the data via an immutable reference, and the
// borrower can read the data but not modify it

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only
    // memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition",
        book.title,
        book.year
    );
}

fn new_edition(book: &mut Book) {
    book.year = 2021;
    println!("I mutable borrowed {} - {} edition",
        book.title,
        book.year
    );
}

fn main() {
    let immutabook = Book {
        author: "Daniel Kahneman",
        title: "Thinking slow and fast",
        year: 2011,
    };

    let mut mutabook = immutabook;

    borrow_book(&immutabook);

    borrow_book(&mutabook);

    new_edition(&mut mutabook);

    // new_edition(&mut immutabook);
}
