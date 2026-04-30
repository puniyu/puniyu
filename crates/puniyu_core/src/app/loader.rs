use puniyu_error::Result;
use puniyu_loader::Loader;
use std::sync::Arc;

pub async fn init_loader(loader: Arc<dyn Loader>) -> Result {
	let plugins = loader.plugins().await;
	for plugin in plugins {
		super::plugin::init_plugin(plugin).await?;
	}
	Ok(())
}
