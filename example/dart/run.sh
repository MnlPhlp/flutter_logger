#! /bin/bash

flutter_rust_bridge_codegen --rust-input rust_lib/src/api.rs --dart-output bin/generated.dart
# fix en error in the current version of flutter_rust_bridge
sed -i 's/mirror_LogEntry(init(task_callback.stream_sink()))/init(task_callback.stream_sink())/' rust_lib/src/bridge_generated.rs 
cd rust_lib && cargo build --release && cd ..
dart run