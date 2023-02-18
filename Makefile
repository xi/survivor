survivor: src/main.rs src/input.rs src/random.rs src/term.rs src/sprites.rs libc.rlib
	rustc -O --crate-name $@ --extern libc=libc.rlib src/main.rs
	strip $@

libc.rlib: src/libc.rs
	rustc src/libc.rs --crate-type lib --crate-name c
