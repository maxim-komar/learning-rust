compile:
	rustc doc.rs --crate-type lib
	
test_doc:
	rustdoc --test --extern doc="libdoc.rlib" doc.rs

run: compile test_doc

clean:
	rm -f libdoc.rlib
