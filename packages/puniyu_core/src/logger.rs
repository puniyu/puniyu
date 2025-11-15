use puniyu_logger::{LoggerOptions, init};
use std::{env, sync::OnceLock};

static LOGGER_INIT: OnceLock<()> = OnceLock::new();

/// 初始化日志系统
pub(crate) fn log_init() {
	LOGGER_INIT.get_or_init(|| {
		let log_level = env::var("LOGGER_LEVEL").unwrap_or("info".to_string());
		let log_path = LOG_DIR.as_path().to_string_lossy().to_string();
		let log_retention_days =
			env::var("LOGGER_RETENTION_DAYS").unwrap_or("7".to_string()).parse::<u8>().unwrap_or(7);
		let is_file_logging = env::var("LOGGER_FILE_ENABLE")
			.unwrap_or("true".to_string())
			.parse::<bool>()
			.unwrap_or(true);
		let options = LoggerOptions::new()
			.with_level(&log_level)
			.with_file_logging(is_file_logging)
			.with_log_directory(log_path)
			.with_retention_days(log_retention_days);

		init(Some(options));
	});
}

use puniyu_common::path::LOG_DIR;
pub use puniyu_logger::{
	SharedLogger, debug, error, info, owo_colors::OwoColorize, setup_shared_logger, warn,
};
