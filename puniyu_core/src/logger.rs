use puniyu_logger::{LoggerOptions, init};
use std::{env, sync::OnceLock};

static LOGGER_INIT: OnceLock<()> = OnceLock::new();

/// 初始化日志系统
pub fn log_init() {
    LOGGER_INIT.get_or_init(|| {
        let log_level = env::var("LOGGER_LEVEL").unwrap_or_else(|_| "info".to_string());
        let log_path = env::var("LOGGER_PATH").unwrap_or_else(|_| "logs".to_string());
        let log_retention_days = env::var("LOGGER_RETENTION_DAYS")
            .unwrap_or_else(|_| "7".to_string())
            .parse::<u8>()
            .unwrap_or(7);
        let options = LoggerOptions::new()
            .with_level(&log_level)
            .with_file_logging(true)
            .with_log_directory(log_path)
            .with_retention_days(log_retention_days);

        init(Some(options));
    });
}

pub use puniyu_logger::set_log_level;
pub use puniyu_logger::{debug, error, info, trace, warn};
