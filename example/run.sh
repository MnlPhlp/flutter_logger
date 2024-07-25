#! /bin/bash

cargo install 'flutter_rust_bridge_codegen'
flutter_rust_bridge_codegen generate
cd rust_lib && cargo build --release && cd ..
dart run
