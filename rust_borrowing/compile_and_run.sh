#!/bin/bash

## SOURCE
# shared borrows: https://youtu.be/61bFe3jqi1E

## USING CARGO
# needs ./src/main.rs and ./Cargo.toml

# cargo build
cargo build --release # longer build, faster code, makes Cargo.lock
cargo run --release # it builds if needed

