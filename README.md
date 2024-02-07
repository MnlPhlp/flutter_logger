# flutter_logger

implementation of the `log` crate for using rust together with flutter/dart and [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) to get logs from rust into your app.

## features

- `panic`: print rust panics to the log stream.

## usage

The library contains a macro for all the code you have to include in your flutter_rust_bridge api definition.
Calling the macro without args creates the init function "setup_log_stream" with LeveFilter::Debug.
You can also specify function name and LevelFilter (or only one).
The macro can only be called once because of conflicting implementations

### rust
```rs

// only one of these calls can be active
flutter_logger::flutter_logger_init!(); // default
// flutter_logger::flutter_logger_init!(LeveFilter::Trace); // sepcify level
// flutter_logger::flutter_logger_init!(logger_init); // sepcify name
// flutter_logger::flutter_logger_init!(info_logger, LevelFilter::Info); // sepcify both

pub fn test(i: i32) {
    // using the 'log' crate macros
    info!("test called with: {i}")
}
```
### dart/flutter

```dart
void setupLogger(){
    setupLogStream().listen((msg){
    // This should use a logging framework in real applications
        print("${msg.logLevel} ${msg.lbl.padRight(8)}: ${msg.msg}");
    });
}

void main(){
    await RustLib.init();
    await setupLogger();
    await test(i: 5);
}

```

This works also on mobile apps like Android where println() in rust isn't shown in the console.
