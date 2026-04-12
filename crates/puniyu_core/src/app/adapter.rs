use puniyu_adapter_core::Adapter;
use puniyu_adapter_core::AdapterRegistry;
use puniyu_common::source::SourceType;
use puniyu_path::adapter::*;
use std::sync::Arc;
use tokio::fs::create_dir_all;

pub async fn init_adapter(adapter: Arc<dyn Adapter>) {
	let name = adapter.runtime().adapter_info().name.clone();
	let hooks = adapter.hook();
	#[cfg(feature = "server")]
	let servers = adapter.server();
	let index = AdapterRegistry::register(Arc::clone(&adapter))
		.unwrap_or_else(|e| panic!("Failed to register adapter {}: {}", name, e));
	let source = SourceType::Adapter(index);
	if !hooks.is_empty() {
		super::hook::init_hook(source, hooks)
			.unwrap_or_else(|e| panic!("Failed to init hook for adapter {}: {}", name, e));
	}
	#[cfg(feature = "server")]
	{
		if let Some(server) = servers {
			super::server::init_server(source, server)
				.unwrap_or_else(|e| panic!("Failed to init server for adapter {}: {}", name, e));
		}
	}
	if !config_dir().join(&name).exists() {
		create_dir_all(config_dir().join(&name))
			.await
			.unwrap_or_else(|e| panic!("Failed to create config dir for adapter {}: {}", name, e));
	}
	if !data_dir().join(&name).exists() {
		create_dir_all(data_dir().join(&name))
			.await
			.unwrap_or_else(|e| panic!("Failed to create data dir for adapter {}: {}", name, e));
	}
	if !resource_dir().join(&name).exists() {
		create_dir_all(resource_dir().join(&name))
			.await
			.unwrap_or_else(|e| panic!("Failed to create resource dir for adapter {}: {}", name, e));
	}
	if !temp_dir().join(&name).exists() {
		create_dir_all(temp_dir().join(&name))
			.await
			.unwrap_or_else(|e| panic!("Failed to create temp dir for adapter {}: {}", name, e));
	}

	adapter
		.init()
		.await
		.unwrap_or_else(|e| panic!("Failed to init adapter {}: {}", name, e));

	super::config::init_config(&name, adapter.config()).await;
}
