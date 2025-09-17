use puniyu_logger::{LoggerOptions, init};
use std::{env, sync::OnceLock};

static LOGGER_INIT: OnceLock<()> = OnceLock::new();

/// 初始化日志系统
pub fn log_init() {
	LOGGER_INIT.get_or_init(|| {
		let log_level = env::var("LOGGER_LEVEL").unwrap_or("info".to_string());
		let log_path = env::var("LOGGER_PATH").unwrap_or("logs".to_string());
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

pub use puniyu_logger::{debug, error, info, trace, warn};
