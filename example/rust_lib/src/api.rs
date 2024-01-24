use log::{info, LevelFilter};

pub fn test(i: i32) {
    info!("test called Log info!() with: {i}");
}
pub fn panic() {
    panic!("this should be passed to dart");
}

flutter_logger::flutter_logger_init!(LevelFilter::Info);
