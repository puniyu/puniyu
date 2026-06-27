use std::{env, str::FromStr};

use puniyu_core::{app_name, path::log_dir};
use puniyu_logger::{LogLevel, LoggerOptions};
use crate::config::LoggerConfig;

pub(crate) fn log_init() {
    let config = LoggerConfig::get();
    let log_level = env::var("LOGGER_LEVEL").unwrap_or(config.level().to_string());
    let log_path = log_dir().to_string_lossy().to_string();
    let log_retention_days = config.retention_days();
    let is_file_logging = config.enable_file();
    let options = LoggerOptions::default()
        .with_prefix(app_name())
        .with_level(LogLevel::from_str(log_level.as_str()).unwrap_or(LogLevel::Info))
        .with_file_logging(is_file_logging)
        .with_log_directory(log_path)
        .with_retention_days(log_retention_days);
    puniyu_logger::init(Some(options));
}