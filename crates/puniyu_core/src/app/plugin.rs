use puniyu_command::Command;
use puniyu_common::source::SourceType;
use puniyu_error::registry::Error;
use puniyu_logger::error;
use puniyu_plugin_core::Plugin;
use puniyu_task::Task;
use std::sync::Arc;

pub async fn init_plugin(plugin: Arc<dyn Plugin>) -> Result<(), Error> {
	use puniyu_plugin_core::PluginRegistry;
	let hooks = plugin.hooks();
	let commands = plugin.commands();
	let tasks = plugin.tasks();
	#[cfg(feature = "server")]
	let servers = plugin.server();
	let index = PluginRegistry::register(plugin)?;
	let source = SourceType::Plugin(index);
	super::hook::init_hook(source, hooks)?;
	init_command(index, commands)?;
	init_task(index, tasks).await?;
	#[cfg(feature = "server")]
	{
		if let Some(server) = servers {
			super::server::init_server(source, server)?;
		}
	}

	Ok(())
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
