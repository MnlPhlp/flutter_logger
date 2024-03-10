use log::{LevelFilter, SetLoggerError};
use logger::LogSink;
use std::{
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};
pub mod logger;
pub use logger::LogEntry;
#[cfg(test)]
mod tests;

/// create a logger label from a src file path
/// ```
/// let lbl = flutter_logger::get_lbl(file!());
/// ```
/// # Panics
/// panics if an invalid path is given
#[must_use]
pub fn get_lbl(path: &str) -> &str {
    let filename = Path::new(path).file_name().unwrap().to_str().unwrap();
    &filename[..filename.len() - 3]
}

static LOGGER: FlutterLogger = FlutterLogger;
static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// initialize the Logger with a stream that sends `LogEntries` to dart/flutter
/// # Errors
/// return an error if a logger was already set
pub fn init(sink: impl LogSink + 'static, filter: LevelFilter) -> Result<(), SetLoggerError> {
    if !IS_INITIALIZED.swap(true, Ordering::Relaxed) {
        log::set_logger(&LOGGER)?;
    }
    log::set_max_level(filter);
    logger::init(sink);
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
            record.file().map_or("unknown", get_lbl),
            &std::fmt::format(record.args().to_owned()),
        );
    }

    fn flush(&self) {}
}

/// calling the macro without args creates init function `setup_log_stream` with `LeveFilter::Debug`.
/// You can also specify function name and `LevelFilter` (or only one).
/// The macro can only be called once because of conflicting implementations
///
/// ```rs
/// flutter_logger::flutter_logger_init!(); // default
/// flutter_logger::flutter_logger_init!(LeveFilter::Trace); // sepcify level
/// flutter_logger::flutter_logger_init!(logger_init); // sepcify name
/// flutter_logger::flutter_logger_init!(info_logger, LevelFilter::Info); // sepcify both
/// ```
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! flutter_logger_init {
    ($func_name: ident,$min_lvl: path) => {
        use crate::frb_generated;
        pub use flutter_logger::LogEntry;
        use flutter_rust_bridge::frb;
        pub use log::Level;

        pub fn $func_name(sink: frb_generated::StreamSink<flutter_logger::LogEntry>) {
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
    ($func_name: ident) => {
        $crate::flutter_logger_init!($func_name, log::LevelFilter::Debug);
    };
    ($min_lvl: path) => {
        $crate::flutter_logger_init!(setup_log_stream, $min_lvl);
    };
    () => {
        $crate::flutter_logger_init!(setup_log_stream, log::LevelFilter::Debug);
    };
}
