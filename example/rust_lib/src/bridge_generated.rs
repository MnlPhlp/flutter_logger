#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.79.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_test_impl(port_: MessagePort, i: impl Wire2Api<i32> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "test",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_i = i.wire2api();
            move |task_callback| Ok(test(api_i))
        },
    )
}
fn wire_init_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "init",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || move |task_callback| Ok(init(task_callback.stream_sink::<_, mirror_LogEntry>())),
    )
}
fn wire_panic_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "panic",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(panic()),
    )
}
// Section: wrapper structs

#[derive(Clone)]
pub struct mirror_Level(Level);

#[derive(Clone)]
pub struct mirror_LogEntry(LogEntry);

// Section: static checks

const _: fn() = || {
    match None::<Level>.unwrap() {
        Level::Error => {}
        Level::Warn => {}
        Level::Info => {}
        Level::Debug => {}
        Level::Trace => {}
    }
    {
        let LogEntry = None::<LogEntry>.unwrap();
        let _: i64 = LogEntry.time_millis;
        let _: String = LogEntry.msg;
        let _: Level = LogEntry.log_level;
        let _: String = LogEntry.lbl;
    }
};
// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}
// Section: impl IntoDart

impl support::IntoDart for mirror_Level {
    fn into_dart(self) -> support::DartAbi {
        match self.0 {
            Level::Error => 0,
            Level::Warn => 1,
            Level::Info => 2,
            Level::Debug => 3,
            Level::Trace => 4,
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_Level {}
impl rust2dart::IntoIntoDart<mirror_Level> for Level {
    fn into_into_dart(self) -> mirror_Level {
        mirror_Level(self)
    }
}

impl support::IntoDart for mirror_LogEntry {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.0.time_millis.into_into_dart().into_dart(),
            self.0.msg.into_into_dart().into_dart(),
            self.0.log_level.into_into_dart().into_dart(),
            self.0.lbl.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_LogEntry {}
impl rust2dart::IntoIntoDart<mirror_LogEntry> for LogEntry {
    fn into_into_dart(self) -> mirror_LogEntry {
        mirror_LogEntry(self)
    }
}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;