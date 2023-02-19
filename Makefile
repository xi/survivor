survivor: src/main.rs src/input.rs src/random.rs src/term.rs src/enemies.rs src/sprites.rs liblibc.rlib libppm.so
	rustc -O --crate-name $@ --extern libc=liblibc.rlib --extern ppm=libppm.so src/main.rs
	strip $@

lib%.rlib: src/%.rs
	rustc $< --crate-type lib

libppm.so: ppm/src/lib.rs
	rustc ppm/src/lib.rs --crate-type proc-macro --crate-name=ppm
