use punicore_logger::{log_init, LoggerOptions};
use std::sync::Once;
use tracing::level_filters::LevelFilter;

static INIT: Once = Once::new();

/// 初始化日志系统
pub fn init_logger() {
    INIT.call_once(|| {
        let options = LoggerOptions::new(LevelFilter::INFO)
            .with_file_logging(true)
            .with_log_directory("logs".to_string())
            .with_retention_days(7);
        log_init(Some(options));
    });
}