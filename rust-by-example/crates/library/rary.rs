// a crate can be compiled into a binary or into a library. By default, 
// `rustc` will produce a binary from a crate. This behavior can be 
// overridden by passing the `--crate-type` flag to `lib`

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
