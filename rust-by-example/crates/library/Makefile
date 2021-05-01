compile:
	rustc --crate-type=lib rary.rs
	rustc executable.rs --extern rary=library.rlib --edition=2018

run: compile
	./executable

clean:
	rm -f library.rlib
	rm -f executable
