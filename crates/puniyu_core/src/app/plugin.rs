use puniyu_command::Command;
use puniyu_common::source::SourceType;
use puniyu_config::ConfigRegistry;
use puniyu_error::registry::Error;
use puniyu_logger::error;
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
	let index = match PluginRegistry::register(Arc::clone(&plugin)) {
		Ok(index) => index,
		Err(e) => {
			return error!("Failed to register plugin: {:?}", e);
		}
	};
	let source = SourceType::Plugin(index);
	if !hooks.is_empty()
		&& let Err(e) = super::hook::init_hook(source, hooks)
	{
		error!("Failed to register hook for plugin: {:?}", e);
	}

	if !commands.is_empty()
		&& let Err(e) = init_command(index, commands)
	{
		error!("Failed to register command for plugin: {:?}", e);
	};

	if !tasks.is_empty()
		&& let Err(e) = init_task(index, tasks).await
	{
		error!("Failed to register task for plugin: {:?}", e);
	};
	#[cfg(feature = "server")]
	{
		if let Some(server) = servers
			&& let Err(e) = super::server::init_server(source, server)
		{
			error!("Failed to register server for plugin: {:?}", e);
		}
	}
	if !config_dir().join(name).exists() {
		let _ = create_dir_all(config_dir().join(name)).await;
	}
	if !data_dir().join(name).exists() {
		let _ = create_dir_all(data_dir().join(name)).await;
	}
	if !resource_dir().join(name).exists() {
		let _ = create_dir_all(resource_dir().join(name)).await;
	}
	if !temp_dir().join(name).exists() {
		let _ = create_dir_all(temp_dir().join(name)).await;
	}
	if let Err(e) = plugin.init().await {
		error!("Failed to init plugin: {:?}", e);
	}
	let configs = plugin.config();
	if !configs.is_empty() {
		for config in configs {
			let config = config.config();
			let path = config_dir().join(name).join(&config.name);
			if !path.exists() {
				let _ = create_dir_all(path).await;
			}
			let _ = ConfigRegistry::register(config);
		}
	}
}

fn init_command(plugin_id: u64, commands: Vec<Arc<dyn Command>>) -> Result<(), Error> {
	use puniyu_command::CommandRegistry;
	for command in commands {
		if let Err(e) = CommandRegistry::register(plugin_id, command) {
			error!("Failed to register command: {:?}", e);
		}
	}
	Ok(())
}

async fn init_task(plugin_id: u64, tasks: Vec<Arc<dyn Task>>) -> Result<(), Error> {
	use puniyu_task::TaskRegistry;
	for task in tasks {
		if let Err(e) = TaskRegistry::register(plugin_id, task).await {
			error!("Failed to register task: {:?}", e);
		}
	}
	Ok(())
}
