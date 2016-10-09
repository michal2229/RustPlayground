#!/bin/bash

## info from:
# http://doc.rust-lang.org/book/getting-started.html

## NAIVE METHOD - JUST COMPILE AND RUN
#chmod a+x ./hello.rs
#rustc ./hello.rs
#./hello


## USING CARGO
# needs ./src/main.rs and ./Cargo.toml

# cargo build
cargo build --release # longer build, faster code, makes Cargo.lock
cargo run # it builds if needed


## easy starting new project from template
# cargo new hello_world --bin

## most projects can be built using something like
# git clone someurl.com/foo
# cd foo
# cargo build
