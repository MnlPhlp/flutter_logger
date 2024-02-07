#! /bin/bash

cargo install 'flutter_rust_bridge_codegen@^2.0.0-dev.23'
flutter_rust_bridge_codegen generate
cd rust_lib && cargo build --release && cd ..
dart run
