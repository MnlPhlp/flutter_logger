use flutter_logger::FlutterLogger;
pub use flutter_logger::LogEntry;
use flutter_rust_bridge::{frb, StreamSink};
pub use log::Level;
use log::{info, LevelFilter};

pub fn test(i: i32) {
    info!("test called Log info!() with: {i}");
}
pub fn init(sink: StreamSink<LogEntry>) {
    flutter_logger::init(sink, LevelFilter::Trace).unwrap();
}
pub fn panic() {
    panic!("this should be passed to dart");
}

#[frb(mirror(LogEntry))]
struct _LogEntry {
    time_millis: i64,
    msg: String,
    log_level: Level,
    lbl: String,
}
#[frb(mirror(Level))]
enum _Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
