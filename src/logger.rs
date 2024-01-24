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
