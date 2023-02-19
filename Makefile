survivor: src/main.rs src/input.rs src/random.rs src/term.rs src/enemies.rs src/sprites.rs liblibc.rlib
	rustc -O --crate-name $@ --extern libc=liblibc.rlib --extern sprites=libsprites.rlib src/main.rs
	strip $@

lib%.rlib: src/%.rs
	rustc $< --crate-type lib

src/sprites.rs: ppm/*.ppm ppm2rust.py
	echo 'pub const HEIGHT: usize = 24;' > $@
	echo 'pub const WIDTH: usize = 18;' >> $@
	echo 'pub type Sprite = [[[u8; 3]; WIDTH]; HEIGHT];' >> $@
	echo >> $@
	find ppm/ -type f | while read l; do python ppm2rust.py "$$l"; done >> $@
