use log::LevelFilter;
use logger::LogSink;
use std::{
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};
use thiserror::Error;
pub mod logger;
pub use logger::LogEntry;
#[cfg(test)]
mod tests;

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
        log::set_logger(&LOGGER).map_err(Error::SetLoggerError)?;
    }
    log::set_max_level(filter);
    logger::init(sink)?;
    Ok(())
}

pub struct FlutterLogger;
impl log::Log for FlutterLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        logger::log(
            record.level(),
            record.file().map(get_lbl).unwrap_or("unknown"),
            &std::fmt::format(record.args().to_owned()),
        )
    }

    fn flush(&self) {}
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! flutter_logger_init {
    ($min_lvl: path) => {
        use crate::frb_generated;
        pub use flutter_logger::LogEntry;
        use flutter_rust_bridge::frb;
        pub use log::Level;

        #[flutter_rust_bridge::frb(sync)]
        pub fn setup_log_stream(sink: frb_generated::StreamSink<flutter_logger::LogEntry>) {
            flutter_logger::init(sink, $min_lvl).unwrap();
        }

        impl flutter_logger::logger::LogSink
            for frb_generated::StreamSink<flutter_logger::LogEntry>
        {
            fn send(&self, entry: flutter_logger::LogEntry) {
                self.add(entry).unwrap();
            }
        }

        #[frb(mirror(LogEntry))]
        struct _LogEntry {
            pub time_millis: i64,
            pub msg: String,
            pub log_level: log::Level,
            pub lbl: String,
        }
        #[frb(mirror(Level))]
        enum _LogLevel {
            Error,
            Warn,
            Info,
            Debug,
            Trace,
        }
    };
}
