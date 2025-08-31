use puniyu_logger::{LoggerOptions, init as logger};
use std::sync::OnceLock;

static INIT: OnceLock<()> = OnceLock::new();

pub fn log_init() {
    INIT.get_or_init(|| {
        let options = LoggerOptions::new("info")
            .with_file_logging(true)
            .with_log_directory("logs".to_string())
            .with_retention_days(7);
        logger(Some(options));
    });
}
