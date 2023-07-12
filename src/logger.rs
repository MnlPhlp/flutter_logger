use flutter_rust_bridge::support;
use flutter_rust_bridge::StreamSink;
use once_cell::sync::OnceCell;
use std::{sync::RwLock, time};

use crate::logi;
use crate::Error;

#[derive(Clone)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<log::Level> for LogLevel {
    fn from(value: log::Level) -> Self {
        match value {
            log::Level::Error => Self::Error,
            log::Level::Warn => Self::Warn,
            log::Level::Info => Self::Info,
            log::Level::Debug => Self::Debug,
            log::Level::Trace => Self::Trace,
        }
    }
}

impl support::IntoDart for LogLevel {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Error => 0,
            Self::Warn => 1,
            Self::Info => 2,
            Self::Debug => 3,
            Self::Trace => 4,
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
pub fn init(s: StreamSink<LogEntry>) -> Result<(), Error> {
    if LOGGER.get().is_none() && START.get().is_none() {
        let _ = START.set(time::Instant::now());
        let _ = LOGGER.set(RwLock::new(s));
        logi!("Logger ready!");
        #[cfg(feature = "panic")]
        std::panic::set_hook(Box::new(|p| crate::loge!("panic occured: {p:?}")));
        Ok(())
    } else {
        Err(Error::AlreadyInitialized)
    }
}
