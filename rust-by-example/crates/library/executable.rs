// to link a crate to this new library you may use `rustc` `--extern` flag.
// All of its items will then be imported under a module named the same
// as the library. This module generally behaves the same way as any
// other module
fn main() {
    rary::public_function();

    rary::indirect_access();
}
