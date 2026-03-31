use puniyu_adapter_core::Adapter;
use puniyu_common::source::SourceType;
use puniyu_logger::error;
use puniyu_path::adapter::*;
use std::sync::Arc;
use tokio::fs::create_dir_all;

pub async fn init_adapter(adapter: Arc<dyn Adapter>) {
	use puniyu_adapter_core::AdapterRegistry;
	let name = adapter.info().name;
	let hooks = adapter.hook();
	#[cfg(feature = "server")]
	let servers = adapter.server();
	let index = match AdapterRegistry::register(Arc::clone(&adapter)) {
		Ok(index) => index,
		Err(e) => return error!("Failed to register adapter {}: {}", name, e),
	};
	let source = SourceType::Adapter(index);
	if let Err(e) = super::hook::init_hook(source, hooks) {
		error!("Failed to init hook for adapter {}: {}", name, e);
	}
	#[cfg(feature = "server")]
	{
		if let Some(server) = servers
			&& let Err(e) = super::server::init_server(source, server)
		{
			error!("Failed to init server for adapter {}: {}", name, e);
		}
	}
	if !config_dir().join(&name).exists() {
		let _ = create_dir_all(config_dir().join(&name)).await;
	}
	if !data_dir().join(&name).exists() {
		let _ = create_dir_all(data_dir().join(&name)).await;
	}
	if !resource_dir().join(&name).exists() {
		let _ = create_dir_all(resource_dir().join(&name)).await;
	}
	if !temp_dir().join(&name).exists() {
		let _ = create_dir_all(temp_dir().join(&name)).await;
	}
	
	if let Err(e) = adapter.init().await {
		error!("Failed to init adapter: {}", e);
	}
}
