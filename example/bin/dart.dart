import 'dart:io';

import 'api.dart';
import 'frb_generated.dart';

Future setupLogger() async {
  infoLogger().listen((msg) {
    // This should use a logging framework in real applications
    print("${msg.logLevel} ${msg.lbl.padRight(8)}: ${msg.msg}");
  });
}

void main(List<String> arguments) async {
  await RustLib.init();
  await setupLogger();
  await test(i: 12);
  try {
    await panic();
  } catch (e) {}
  exit(0);
}
