use std::{env, str::FromStr};

use crate::{NAME, VERSION};
use convert_case::{Case, Casing};
use figlet_rs::FIGlet;
use puniyu::config::app::AppConfig;
use puniyu_logger::{LogLevel, LoggerOptions};
use puniyu_path::log_dir;

pub fn init() {
	let config = AppConfig::get().logger();
	let log_level = env::var("LOGGER_LEVEL").unwrap_or(config.level().to_string());
	let log_path = log_dir().to_string_lossy().to_string();
	let log_retention_days = config.retention_days();
	let is_file_logging = config.enable_file();
	let options = LoggerOptions::default()
		.with_prefix(crate::NAME)
		.with_level(LogLevel::from_str(log_level.as_str()).unwrap_or(LogLevel::Info))
		.with_file_logging(is_file_logging)
		.with_log_directory(log_path)
		.with_retention_days(log_retention_days);
	puniyu_logger::init(Some(options));
}

pub fn start_log() {
	let app_name = NAME.to_case(Case::Lower);
	println!("{} starting...", app_name);
	if let Ok(standard_font) = FIGlet::standard()
		&& let Some(art_text) = standard_font.convert(app_name.as_str())
	{
		println!("{}", art_text);
	} else {
		println!("{}", app_name);
	}
	println!("Version: {}", VERSION);
	println!("Git SHA: {}", env!("VERGEN_GIT_SHA"));
	println!("Github: {}", env!("CARGO_PKG_REPOSITORY"));
}
