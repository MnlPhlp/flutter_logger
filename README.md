# flutter_logger

logging library for using rust together with flutter/dart and [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) to get logs from rust into your app.

## features

- `panic`: print rust panics to the log stream

## usage

This code is not complete. Look into the example folder for a full working example.

### rust

```rs
pub fn test(i: i32) {
		// using the 'log' crate macros
		info!("test called with: {i}")
}

pub fn init(sink: StreamSink<LogEntry>) {
		flutter_logger::init(sink).unwrap();
}
```

### dart/flutter

```dart
final rust_lib =
void setupLogger(){
		rustLib.init().listen((msg){
			// This should use a logging framework in real applications
			print("${msg.logLevel} ${msg.lbl.padRight(8)}: ${msg.msg}");
		});
}

void main(){
	setupLogger();
	rustLib.test(i: 5);
}

```

This works also on mobile apps like Android where println() in rust isn't shown in the console.
