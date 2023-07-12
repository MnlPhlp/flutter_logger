use log::LevelFilter;
use logger::LogSink;
use std::{
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};
use thiserror::Error;
pub mod logger;
pub use logger::{LogEntry, LogLevel};
#[cfg(test)]
mod tests;

#[macro_export]
/// Log an error
/// This accepts the same input as println!() or format!()
macro_rules! loge {
    ($($args:tt)*) => {
        {
            let lbl = $crate::get_lbl(file!());
            $crate::logger::log($crate::logger::LogLevel::Error,lbl,&format!($($args)*));
        }
    };
}
#[macro_export]
/// Log a warning
/// This accepts the same input as println!() or format!()
macro_rules! logw {
    ($($args:tt)*) => {
        {
            let lbl = $crate::get_lbl(file!());
            $crate::logger::log($crate::logger::LogLevel::Warning,lbl,&format!($($args)*));
        }
    };
}
#[macro_export]
/// Log an info
/// This accepts the same input as println!() or format!()
macro_rules! logi {
    ($($args:tt)*) => {
        {
            let lbl = $crate::get_lbl(file!());
            $crate::logger::log($crate::logger::LogLevel::Info,lbl,&format!($($args)*));
        }
    };
}
#[macro_export]
/// Log debug information
/// This accepts the same input as println!() or format!()
macro_rules! logd {
    ($($args:tt)*) => {
        {
            let lbl = $crate::get_lbl(file!());
            $crate::logger::log($crate::logger::LogLevel::Debug,lbl,&format!($($args)*));
        }
    };
}

/// create a logger label from a src file path
/// ```
/// let lbl = flutter_logger::get_lbl(file!());
/// ```
pub fn get_lbl(path: &str) -> &str {
    let filename = Path::new(path).file_name().unwrap().to_str().unwrap();
    &filename[..filename.len() - 3]
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("logger was already initialized")]
    AlreadyInitialized,

    #[error("error setting logger {0}")]
    SetLoggerError(log::SetLoggerError),
}

static LOGGER: FlutterLogger = FlutterLogger;
static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);
/// initialize the Logger with a stream that sends LogEntries to dart/flutter
pub fn init(sink: impl LogSink + 'static, filter: LevelFilter) -> Result<(), Error> {
    if !IS_INITIALIZED.swap(true, Ordering::Relaxed) {
        log::set_logger(&LOGGER).map_err(|e| Error::SetLoggerError(e))?;
    }
    log::set_max_level(filter);
    logger::init(sink)?;
    Ok(())
}

pub struct FlutterLogger;
impl log::Log for FlutterLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() < log::Level::Debug
    }

    fn log(&self, record: &log::Record) {
        logger::log(
            LogLevel::from(record.level()),
            record.file().map(|f| get_lbl(f)).unwrap_or("unknown"),
            &std::fmt::format(record.args().to_owned()),
        )
    }

    fn flush(&self) {}
}
