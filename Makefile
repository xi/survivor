survivor: src/*.rs
	rustc -O src/main.rs --crate-name $@
	strip $@
