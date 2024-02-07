use std::{
    process::Command,
    sync::{Arc, Mutex},
};

use log::info;

use crate::{logger::LogSink, LogEntry};

#[derive(Default, Clone)]
struct TestLogSink {
    calls: Arc<Mutex<i32>>,
    last_arg: Arc<Mutex<Option<LogEntry>>>,
}

impl LogSink for TestLogSink {
    fn send(&self, entry: LogEntry) {
        *self.last_arg.lock().unwrap() = Some(entry);
        *self.calls.lock().unwrap() += 1
    }
}

#[test]
fn test_log_crate_macros() {
    let sink = TestLogSink::default();
    crate::init(sink.clone(), log::LevelFilter::Trace).unwrap();

    info!("info log");

    assert!(*sink.calls.lock().unwrap() == 1);
    let msg = sink.last_arg.lock().unwrap().as_ref().unwrap().clone();
    assert!(msg.msg == "info log");
    assert!(msg.lbl == "tests");
}

#[test]
fn test_example() {
    let output = Command::new("./run.sh").current_dir("./example").output();
    println!("{output:?}");
    assert!(output.is_ok());
    let output = output.unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout);
    assert!(text.is_ok());
    let text = text.unwrap();
    assert!(text.contains("test called Log info!() with: 12"));
    assert!(text.contains("this should be passed to dart"))
}
