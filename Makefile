survivor: src/main.rs src/input.rs src/random.rs src/term.rs src/enemies.rs liblibc.rlib libsprites.rlib
	rustc -O --crate-name $@ --extern libc=liblibc.rlib --extern sprites=libsprites.rlib src/main.rs
	strip $@

lib%.rlib: src/%.rs
	rustc $< --crate-type lib
