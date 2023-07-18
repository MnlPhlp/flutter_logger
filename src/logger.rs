use flutter_rust_bridge::{
    rust2dart::IntoIntoDart, support::ffi::DartCObject, IntoDart, StreamSink,
};
use once_cell::sync::OnceCell;
use std::{sync::RwLock, time};

use crate::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct LogEntry {
    pub time_millis: i64,
    pub msg: String,
    pub log_level: log::Level,
    pub lbl: String,
}

// implemented manually because we have no code generated from flutter_rust_bridge in this crate

fn level_into_dart(level: log::Level) -> DartCObject {
    match level {
        log::Level::Error => 0,
        log::Level::Warn => 1,
        log::Level::Info => 2,
        log::Level::Debug => 3,
        log::Level::Trace => 4,
    }
    .into_dart()
}
impl IntoDart for LogEntry {
    fn into_dart(self) -> DartCObject {
        vec![
            self.time_millis.into_dart(),
            self.msg.into_dart(),
            level_into_dart(self.log_level),
            self.lbl.into_dart(),
        ]
        .into_dart()
    }
}
impl IntoIntoDart<LogEntry> for LogEntry {
    fn into_into_dart(self) -> LogEntry {
        self
    }
}

pub fn log(level: log::Level, label: &str, msg: &str) {
    let logger = match LOGGER.read() {
        Ok(val) => val,
        Err(val) => val.into_inner(),
    };
    if let Some(logger) = logger.as_ref() {
        let start = START.get().unwrap();
        logger.send(LogEntry {
            time_millis: start.elapsed().as_millis() as i64,
            msg: String::from(msg),
            log_level: level,
            lbl: String::from(label),
        });
    }
}

static LOGGER: RwLock<Option<Box<dyn LogSink>>> = RwLock::new(None);
static START: OnceCell<time::Instant> = OnceCell::new();

pub trait LogSink: Send + Sync {
    fn send(&self, entry: LogEntry);
}
impl LogSink for StreamSink<LogEntry> {
    fn send(&self, entry: LogEntry) {
        self.add(entry);
    }
}

/// initialize a stream to pass log events to dart/flutter
pub fn init(s: impl LogSink + 'static) -> Result<(), Error> {
    let _ = START.set(time::Instant::now());
    let mut logger = match LOGGER.write() {
        Ok(val) => val,
        Err(val) => val.into_inner(),
    };
    *logger = Some(Box::new(s));
    #[cfg(feature = "panic")]
    std::panic::set_hook(Box::new(|p| log::error!("panic occured: {p:?}")));
    Ok(())
}
