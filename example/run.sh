#! /bin/bash

cargo install flutter_rust_bridge_codegen
flutter_rust_bridge_codegen --rust-input rust_lib/src/api.rs --dart-output bin/generated.dart
cd rust_lib && cargo build --release && cd ..
dart run
