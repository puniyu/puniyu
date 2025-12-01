use puniyu_logger::{LoggerOptions, init};
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let log_level = env::var("LOGGER_LEVEL").unwrap_or("info".to_string());
	init(Some(LoggerOptions::default().with_level(log_level.as_str()).with_file_logging(true)));
	puniyu_server::run_server_with_control(None, None).await?;
	Ok(())
}
