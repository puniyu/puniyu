use puniyu_config::app::{AppConfig, ServerConfig};

pub(crate) fn get_config() -> ServerConfig {
	AppConfig::get().server().clone()
}
