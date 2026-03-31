use puniyu_error::registry::Error;
use puniyu_loader::Loader;
use std::sync::Arc;

pub async fn init_loader(loader: Arc<dyn Loader>) -> Result<(), Error> {
	let plugins = loader.plugins();
	for plugin in plugins {
		super::plugin::init_plugin(plugin).await
	}
	Ok(())
}
