#[cfg(feature = "logger")]
pub(crate) fn log_init() {
	use puniyu_logger::{LoggerOptions, init};
	use std::env;
	use std::sync::OnceLock;
	static LOGGER_INIT: OnceLock<()> = OnceLock::new();
	LOGGER_INIT.get_or_init(|| {
		env::var("LOGGER_ENABLE").unwrap_or_else(|_| unsafe {
			env::set_var("LOGGER_FILE_ENABLE", "info");
			"info".to_string()
		});
		let log_level = env::var("LOGGER_LEVEL").unwrap_or("info".to_string());
		init(Some(LoggerOptions::new().with_level(log_level.as_str()).with_file_logging(false)));
	});
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        {
            use ::puniyu_logger::owo_colors::OwoColorize;
            let prefix = "Server".fg_rgb::<132,112,255>();
            ::puniyu_logger::info!("[{}] {}", prefix, format!($($arg)*))
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        {
            use ::puniyu_logger::owo_colors::OwoColorize;
            let prefix = "Server".fg_rgb::<132,112,255>();
            ::puniyu_logger::warn!("[{}] {}", prefix, format!($($arg)*))
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        {
            use ::puniyu_logger::owo_colors::OwoColorize;
            let prefix = "Server".fg_rgb::<132,112,255>();
            ::puniyu_logger::error!("[{}] {}", prefix, format!($($arg)*))
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        {
            use ::puniyu_logger::owo_colors::OwoColorize;
            let prefix = "Server".fg_rgb::<132,112,255>();
            ::puniyu_logger::debug!("[{}] {}", prefix, format!($($arg)*))
        }
    };
}
