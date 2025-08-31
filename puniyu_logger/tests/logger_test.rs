use puniyu_logger::{LoggerOptions, init as log_init};
use tracing_log::log;

#[test]
fn log_info() {
    log_init(None);
    log::info!("{}", "info");
}
#[test]
fn log_error() {
    log_init(None);
    log::error!("{}", "error");
}

#[test]
fn log_warn() {
    log_init(None);
    log::warn!("{}", "warn");
}
#[test]
fn log_debug() {
    log_init(None);
    log::debug!("{}", "debug");
}

#[test]
fn log_trace() {
    log_init(None);
    log::trace!("{}", "trace");
}

#[test]
fn log_with_options() {
    let options = LoggerOptions::new("info")
        .with_file_logging(true)
        .with_log_directory("logs".to_string())
        .with_retention_days(7);
    log_init(Some(options));
    log::info!("{}", "info with options");
}
