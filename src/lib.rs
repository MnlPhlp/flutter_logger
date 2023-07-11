use std::path::Path;

use flutter_rust_bridge::StreamSink;
use logger::{AlreadyInitializedError, LogEntry};

pub mod logger;

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
/// let lbl = get_lbl(file!())
/// ```
pub fn get_lbl(path: &str) -> &str {
    let filename = Path::new(path).file_name().unwrap().to_str().unwrap();
    &filename[..filename.len() - 3]
}

/// initialize the Logger with a stream that sends LogEntries to dart/flutter
pub fn init(sink: StreamSink<LogEntry>) -> Result<(), AlreadyInitializedError> {
    logger::init(sink)
}
