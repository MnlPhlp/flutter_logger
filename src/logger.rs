use flutter_rust_bridge::support;
use flutter_rust_bridge::StreamSink;
use once_cell::sync::OnceCell;
use std::{sync::RwLock, time};

use crate::logi;

#[derive(Clone)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

impl support::IntoDart for LogLevel {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Error => 0,
            Self::Warning => 1,
            Self::Info => 2,
            Self::Debug => 3,
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for LogLevel {}

#[derive(Clone)]
pub struct LogEntry {
    pub time_millis: i64,
    pub msg: String,
    pub log_level: LogLevel,
    pub lbl: String,
}
impl support::IntoDart for LogEntry {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.time_millis.into_dart(),
            self.msg.into_dart(),
            self.log_level.into_dart(),
            self.lbl.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for LogEntry {}

pub fn log(level: LogLevel, label: &str, msg: &str) {
    if let Some(logger) = LOGGER.get() {
        let logger = logger.read().unwrap();
        let start = START.get().unwrap();
        logger.add(LogEntry {
            time_millis: start.elapsed().as_millis() as i64,
            msg: String::from(msg),
            log_level: level,
            lbl: String::from(label),
        });
    }
}

static LOGGER: OnceCell<RwLock<StreamSink<LogEntry>>> = OnceCell::new();
static START: OnceCell<time::Instant> = OnceCell::new();
/// initialize a stream to pass log events to dart/flutter
pub fn init(s: StreamSink<LogEntry>) -> Result<(), AlreadyInitializedError> {
    if LOGGER.get().is_none() && START.get().is_none() {
        let _ = START.set(time::Instant::now());
        let _ = LOGGER.set(RwLock::new(s));
        logi!("Logger ready!");
        #[cfg(feature = "panic")]
        std::panic::set_hook(Box::new(|p| crate::loge!("panic occured: {p:?}")));
        Ok(())
    } else {
        Err(AlreadyInitializedError)
    }
}

pub struct AlreadyInitializedError;

impl std::fmt::Display for AlreadyInitializedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Logger was already initialized")
    }
}

impl std::fmt::Debug for AlreadyInitializedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for AlreadyInitializedError {}
