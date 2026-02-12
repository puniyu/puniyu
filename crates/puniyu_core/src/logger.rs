use puniyu_config::Config;
use puniyu_logger::{init, LoggerOptions};
use std::env;


/// 初始化日志系统
pub(crate) fn log_init() {
	use puniyu_path::LOG_DIR;
	let config = Config::app();
	let logger = config.logger();
	let log_level = env::var("LOGGER_LEVEL").unwrap_or(logger.level().to_string());
	let log_path = LOG_DIR.as_path().to_string_lossy().to_string();
	let log_retention_days = logger.retention_days();
	let is_file_logging = logger.enable_file();
	let options = LoggerOptions::default()
		.with_level(&log_level)
		.with_file_logging(is_file_logging)
		.with_log_directory(log_path)
		.with_retention_days(log_retention_days);
	init(Some(options));
}

pub use puniyu_logger::{debug, error, info, owo_colors::OwoColorize, warn};
