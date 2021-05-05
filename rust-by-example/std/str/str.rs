// There are two types of strings in Rust: `String` and `&str`
//
// A `String` is stored as a vector of bytes (`Vec<u8>`), but guaranteed
// to always be a valid UTF-8 sequence. `String` is heap allocated, growable
// and not null terminated.
//
// `&str` is slice (`&[u8]`) that always points to a valid UTF-8 sequence,
// and can be used to view into a `String`, just like `&[T]` is a view
// into `Vec<T>`
fn main() {
    // all the type annotations are superfluous
    // A reference to a string allocated in read only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);


    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }


    // copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // create an empty and growable `String`
    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }


    // The trimmed string is a slice to the original string, hence no new
    // allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);


    // heap allocate a string
    let alice = String::from("I like dogs");
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}
