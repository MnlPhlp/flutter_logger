use log::info;

use crate::{logger::MockLogSink, logi};

#[test]
fn test_own_macros() {
    let mut sink = MockLogSink::new();
    sink.expect_send()
        .once()
        .return_const(())
        .withf(|entry| entry.msg == "info log" && entry.lbl == "tests");
    assert!(crate::init(sink, log::LevelFilter::Off).is_ok());
    logi!("info log");
}

#[test]
fn test_log_crate_macros() {
    let mut sink = MockLogSink::new();
    sink.expect_send()
        .once()
        .return_const(())
        .withf(|entry| entry.msg == "info log" && entry.lbl == "tests");
    crate::init(sink, log::LevelFilter::Trace).unwrap();
    info!("info log");
}

// #[cfg(features = "panic")]
#[test]
#[should_panic]
fn test_panic() {
    let mut sink = MockLogSink::new();
    sink.expect_send()
        .once()
        .return_const(())
        .withf(|entry| entry.msg.starts_with("panic occured:") && entry.lbl == "logger");
    crate::init(sink, log::LevelFilter::Trace).unwrap();
    panic!("test")
}
