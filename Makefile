survivor: src/main.rs src/input.rs src/random.rs src/signal.rs src/term.rs libc.rlib
	rustc -O --crate-name $@ --extern libc=libc.rlib src/main.rs
	strip $@

libc.rlib: src/libc.rs
	rustc src/libc.rs --crate-type lib --crate-name c
