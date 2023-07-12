import 'dart:io';

import 'generated.dart' as gen;
import 'dart:ffi' as ffi;

final rustLib = gen.RustLibImpl(
    ffi.DynamicLibrary.open("rust_lib/target/release/librust_lib.so"));

Future<void> setupLogger() async {
  rustLib.init().listen((msg) {
    // This should use a logging framework in real applications
    print("${msg.logLevel} ${msg.lbl.padRight(8)}: ${msg.msg}");
  });
}

void main(List<String> arguments) async {
  await setupLogger();
  await rustLib.test(i: 12);
  exit(0);
}
