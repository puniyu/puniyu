use puniyu_logger::{LoggerOptions, init};
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
	env::var("LOGGER_ENABLE").unwrap_or_else(|_| unsafe {
		env::set_var("LOGGER_FILE_ENABLE", "info");
		"info".to_string()
	});
	let log_level = env::var("LOGGER_LEVEL").unwrap_or("info".to_string());
	init(Some(LoggerOptions::new().with_level(log_level.as_str()).with_file_logging(false)));
	puniyu_server::run_server(None, None).await?;
	Ok(())
}
