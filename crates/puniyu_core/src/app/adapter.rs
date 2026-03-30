use puniyu_adapter_core::Adapter;
use puniyu_common::source::SourceType;
use std::sync::Arc;

pub async fn init_adapter(adapter: Arc<dyn Adapter>) -> puniyu_error::Result {
	use puniyu_adapter_core::AdapterRegistry;
	let hooks = adapter.hook();
	#[cfg(feature = "server")]
	let servers = adapter.server();
	let index = AdapterRegistry::register(Arc::clone(&adapter))?;
	let source = SourceType::Adapter(index);
	super::hook::init_hook(source, hooks)?;
	#[cfg(feature = "server")]
	{
		if let Some(server) = servers {
			super::server::init_server(source, server)?;
		}
	}
	adapter.init().await?;
	Ok(())
}
