use puniyu_config::ConfigRegistry;
use puniyu_error::AnyError;
use puniyu_plugin_core::PluginRegistry;
use std::sync::Arc;

pub async fn init(plugin: Arc<dyn puniyu_plugin_core::Plugin>) -> AnyError {
	let id = PluginRegistry::register(plugin.clone())?;

	for config in plugin.config() {
		ConfigRegistry::register(config.name(), config.path(), config.value())?;
	}

	#[cfg(feature = "server")]
	if let Some(router) = plugin.server() {
		let source = puniyu_server::SourceType::Plugin(id);
		crate::app::server::init_server(source, router)?;
	}

	for task in plugin.tasks() {
		puniyu_task::TaskRegistry::register(id, task).await?;
	}

	plugin.on_load().await?;

	Ok(())
}
