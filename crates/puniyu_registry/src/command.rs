use crate::store::STORE;
use itertools::Itertools;
use puniyu_types::command::CommandBuilder;
use std::sync::Arc;

#[derive(Clone)]
pub struct Command {
	/// 插件名称
	pub plugin_name: String,
	/// 命令前缀（来自插件或使用全局）
	pub prefix: Option<String>,
	/// 命令构建器
	pub builder: Arc<dyn CommandBuilder>,
}

impl From<Command> for puniyu_types::command::Command {
	fn from(cmd: Command) -> Self {
		Self {
			name: cmd.builder.name(),
			description: cmd.builder.description(),
			args: cmd.builder.args(),
			rank: cmd.builder.rank(),
			prefix: cmd.prefix,
			alias: cmd.builder.alias(),
			permission: cmd.builder.permission(),
		}
	}
}

pub struct CommandRegistry;

impl CommandRegistry {
	pub fn insert(plugin_name: &str, prefix: Option<&str>, builder: Arc<dyn CommandBuilder>) {
		let command = Command {
			plugin_name: plugin_name.to_string(),
			prefix: prefix.map(|s| s.to_string()),
			builder,
		};
		STORE.command().insert(command);
	}

	pub fn remove_with_id(id: u64) {
		STORE.command().remove_with_id(id);
	}

	pub fn remove_with_name(name: &str) {
		STORE.command().remove_with_name(name)
	}

	pub fn remove_with_plugin_name(plugin_name: &str) {
		STORE.command().remove_with_plugin_name(plugin_name)
	}
	pub fn get_with_id(id: u64) -> Option<Arc<Command>> {
		STORE.command().get_with_id(id)
	}

	pub fn get_with_name(name: &str) -> Option<Arc<Command>> {
		STORE.command().get_with_name(name)
	}

	pub fn get_with_plugin(plugin_name: &str, name: &str) -> Option<Arc<Command>> {
		STORE.command().get_with_plugin(plugin_name, name)
	}

	pub fn commands() -> Vec<Arc<Command>> {
		STORE.command().all()
	}
	pub fn get_plugins(command_name: &str) -> Vec<String> {
		let command_list = STORE.command().all();
		command_list
			.iter()
			.filter(|command| command.builder.name() == command_name)
			.sorted_by_key(|command| command.builder.rank())
			.map(|command| command.plugin_name.to_string())
			.collect()
	}
}
