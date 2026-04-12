use puniyu_command::Command;
use puniyu_common::source::SourceType;
use puniyu_error::registry::Error;
use puniyu_path::plugin::*;
use puniyu_plugin_core::Plugin;
use puniyu_plugin_core::PluginRegistry;
use puniyu_task::Task;
use std::sync::Arc;
use tokio::fs::create_dir_all;

pub async fn init_plugin(plugin: Arc<dyn Plugin>) {
	let name = plugin.name();
	let hooks = plugin.hooks();
	let commands = plugin.commands();
	let tasks = plugin.tasks();
	#[cfg(feature = "server")]
	let servers = plugin.server();
	let index = PluginRegistry::register(Arc::clone(&plugin))
		.unwrap_or_else(|e| panic!("Failed to register plugin {}: {:?}", name, e));
	let source = SourceType::Plugin(index);
	if !hooks.is_empty() {
		super::hook::init_hook(source, hooks)
			.unwrap_or_else(|e| panic!("Failed to register hook for plugin {}: {:?}", name, e));
	}

	if !commands.is_empty() {
		init_command(index, commands)
			.unwrap_or_else(|e| panic!("Failed to register command for plugin {}: {:?}", name, e));
	}

	if !tasks.is_empty() {
		init_task(index, tasks)
			.await
			.unwrap_or_else(|e| panic!("Failed to register task for plugin {}: {:?}", name, e));
	}
	#[cfg(feature = "server")]
	{
		if let Some(server) = servers {
			super::server::init_server(source, server).unwrap_or_else(|e| {
				panic!("Failed to register server for plugin {}: {:?}", name, e)
			});
		}
	}
	if !config_dir().join(name).exists() {
		create_dir_all(config_dir().join(name))
			.await
			.unwrap_or_else(|e| panic!("Failed to create config dir for plugin {}: {}", name, e));
	}
	if !data_dir().join(name).exists() {
		create_dir_all(data_dir().join(name))
			.await
			.unwrap_or_else(|e| panic!("Failed to create data dir for plugin {}: {}", name, e));
	}
	if !resource_dir().join(name).exists() {
		create_dir_all(resource_dir().join(name))
			.await
			.unwrap_or_else(|e| panic!("Failed to create resource dir for plugin {}: {}", name, e));
	}
	if !temp_dir().join(name).exists() {
		create_dir_all(temp_dir().join(name))
			.await
			.unwrap_or_else(|e| panic!("Failed to create temp dir for plugin {}: {}", name, e));
	}
	plugin
		.init()
		.await
		.unwrap_or_else(|e| panic!("Failed to init plugin {}: {:?}", name, e));
	super::config::init_config(name, plugin.config()).await;
}

fn init_command(plugin_id: u64, commands: Vec<Arc<dyn Command>>) -> Result<(), Error> {
	use puniyu_command::CommandRegistry;
	for command in commands {
		CommandRegistry::register(plugin_id, command)?;
	}
	Ok(())
}

async fn init_task(plugin_id: u64, tasks: Vec<Arc<dyn Task>>) -> Result<(), Error> {
	use puniyu_task::TaskRegistry;
	for task in tasks {
		TaskRegistry::register(plugin_id, task).await?;
	}
	Ok(())
}
