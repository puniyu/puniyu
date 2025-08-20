use puniyu_logger::{LoggerOptions, log_init};
use std::sync::OnceLock;

static INIT: OnceLock<()> = OnceLock::new();

/// 初始化日志系统
pub fn init_logger(level: String, log_dir: Option<&str>) {
    INIT.get_or_init(|| {
        let options = LoggerOptions::new(&level)
            .with_file_logging(true)
            .with_log_directory(log_dir.unwrap_or("logs").to_string())
            .with_retention_days(7);
        log_init(Some(options));
    });
}

pub use puniyu_logger::set_log_level;
