all: main

main: ./src/*.rs
	cargo build && cargo run

debug: ./src/*.rs
	cargo build && cargo run
	
release: ./src/*.rs
	cargo build --release && cargo run --release

valgrind: ./src/*.rs
	cargo build
	valgrind --tool=callgrind --dump-instr=yes --simulate-cache=yes --collect-jumps=yes ./target/debug/sdl2_test

clean:
	cargo clean
