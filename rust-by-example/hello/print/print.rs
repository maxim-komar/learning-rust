fn main() {
    // in generak, the `{}` will be automatically replaced with any
    // arguments. These will be stringified
    println!("{} days", 31);

    // without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 has the type i64

    // there are various optional patterns this works with. 
    // Position arguments can be used
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // as can named arguments
    println!("{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over");

    // special formatting can be specified after a `:`
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // you can right-align text with a specified width. This will output
    // "     1".
    println!("{number:>width$}", number = 1, width = 6);

    // you can pad numbers with extra zeroes. This will output "000001"
    println!("{number:>0width$}", number = 1, width = 6);

    // rust event checks to make sure the correct number of arguments are used
    // println!("My name is {0}, {1} {0}", "Bond");
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // create a structure named `Structure` which contains an `i32`
    #[allow(dead_code)]
    struct Structure(i32);

    // however, custom types such as this structure require more complicated
    // handling. This will not work
    // println!("This stuct `{}` won't print...", Structure(3));
}
