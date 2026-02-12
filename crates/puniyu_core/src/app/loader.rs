use puniyu_error::registry::Error;
use puniyu_loader::Loader;
use puniyu_logger::error;
use std::sync::Arc;

pub async fn init_loader(loader: Arc<dyn Loader>) -> Result<(), Error> {
	let plugins = loader.plugins();
	for plugin in plugins {
		if let Err(e) = super::plugin::init_plugin(plugin).await {
			error!("Failed to register plugin: {:?}", e);
		}
	}
	Ok(())
}
