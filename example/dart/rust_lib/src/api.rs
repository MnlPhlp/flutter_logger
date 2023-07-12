use flutter_logger::logi;
pub use flutter_logger::{LogEntry, LogLevel};
use flutter_rust_bridge::{frb, StreamSink};

pub fn test(i: i32) {
    logi!("test called with: {i}");
}

pub fn get_log_entry() -> LogEntry {
    todo!()
}

pub fn init(sink: StreamSink<LogEntry>) {
    flutter_logger::init(sink).unwrap();
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
    Warning,
    Info,
    Debug,
}
