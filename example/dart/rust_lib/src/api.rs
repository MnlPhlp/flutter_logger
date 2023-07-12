use flutter_logger::FlutterLogger;
pub use flutter_logger::{logi, LogEntry, LogLevel};
use flutter_rust_bridge::{frb, StreamSink};
use log::{info, LevelFilter};

pub fn test(i: i32) {
    logi!("test called logi!() with: {i}");
    info!("test called Log traits info!() with: {i}");
}
pub fn init(sink: StreamSink<LogEntry>) {
    flutter_logger::init(sink, LevelFilter::Trace).unwrap();
}

// We need this mirrors to make flutter_rust_bridge_codegen generate the types correctly

#[frb(mirror(LogEntry))]
struct _LogEntry {
    pub time_millis: i64,
    pub msg: String,
    pub log_level: LogLevel,
    pub lbl: String,
}
#[frb(mirror(LogLevel))]
enum _LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
