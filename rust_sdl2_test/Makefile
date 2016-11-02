all: main

main: ./src/*.rs
	cargo build --release

debug: ./src/*.rs
	cargo build && cargo run
	
run: ./src/*.rs
	cargo run --release
	
debugrun: ./src/*.rs
	cargo run

valgrind: ./src/*.rs
	cargo build
	valgrind --tool=callgrind --dump-instr=yes --simulate-cache=yes --collect-jumps=yes ./target/debug/sdl2_test

clean:
	cargo clean
