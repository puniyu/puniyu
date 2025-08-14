use log::LevelFilter;
use puniyu_core::logger;
fn main() {
    logger::init_logger("info".to_string(), None);
    log::info!("hello world");
    logger::set_log_level("debug".to_string());
    log::debug!("hello world");
}
