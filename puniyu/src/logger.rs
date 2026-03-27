use puniyu_config::app_config;
use puniyu_logger::LoggerOptions;
use std::env;

/// 初始化日志系统
pub(crate) fn log_init() {
	use puniyu_path::log_dir;
	let config = app_config();
	let logger = config.logger();
	let log_level = env::var("LOGGER_LEVEL").unwrap_or(logger.level().to_string());
	let log_path = log_dir().to_string_lossy().to_string();
	let log_retention_days = logger.retention_days();
	let is_file_logging = logger.enable_file();
	let options = LoggerOptions::default()
		.with_level(&log_level)
		.with_file_logging(is_file_logging)
		.with_log_directory(log_path)
		.with_retention_days(log_retention_days);
	puniyu_logger::init(Some(options));
}
