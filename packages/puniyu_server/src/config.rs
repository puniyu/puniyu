use puniyu_config::app::{AppConfig, ServerConfig};

pub(crate) fn get_config<'c>() -> &'c ServerConfig {
	AppConfig::get().server()
}
