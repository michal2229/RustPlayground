#!/bin/bash

## https://github.com/cyndis/qmlrs/issues/15
# sudo apt install qt5-qmake qtdeclarative5-dev qtquick1-5-dev qml-module-qtquick-controls

# cargo build
cargo build --release # longer build, faster code, makes Cargo.lock
cargo run # it builds if needed

