use puniyu_adapter_core::AdapterRegistry;
use puniyu_config::ConfigRegistry;
use puniyu_error::AnyError;
use std::sync::Arc;

pub async fn init(adapter: Arc<dyn puniyu_adapter_core::Adapter>) -> AnyError {
	let id = AdapterRegistry::register(adapter.clone())?;

	for config in adapter.config() {
		ConfigRegistry::register(config.name(), config.path(), config.value())?;
	}

	#[cfg(feature = "server")]
	if let Some(router) = adapter.server() {
		let source = puniyu_server::SourceType::Adapter(id);
		crate::app::server::init_server(source, router)?;
	}

	adapter.on_load().await?;

	Ok(())
}
