use puniyu_types::command::{Arg, Command as CMD};
use puniyu_types::event::Permission;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct Command {
	/// 插件id
	pub plugin_id: u64,
	/// 命令构建器
	pub builder: Arc<dyn CMD>,
}

impl<'c> From<Command> for CommandInfo<'c> {
	fn from(cmd: Command) -> Self {
		Self {
			plugin_id: cmd.plugin_id,
			name: cmd.builder.name(),
			description: cmd.builder.description(),
			args: cmd.builder.args(),
			rank: cmd.builder.rank(),
			alias: cmd.builder.alias(),
			permission: cmd.builder.permission(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct CommandInfo<'c> {
	/// 插件ID
	pub plugin_id: u64,
	/// 命令名
	pub name: &'c str,
	/// 命令描述
	pub description: Option<&'c str>,
	/// 命令参数列表
	pub args: Vec<Arg<'c>>,
	/// 命令优先级
	pub rank: u32,
	/// 命令别名
	pub alias: Vec<&'c str>,
	/// 权限等级
	pub permission: Permission,
}

pub(crate) enum CommandId<'c> {
	/// 命令id
	Id(u64),
	/// 命令名称
	Name(&'c str),
}

impl From<u64> for CommandId<'_> {
	fn from(id: u64) -> Self {
		Self::Id(id)
	}
}

impl<'c> From<&'c str> for CommandId<'c> {
	fn from(name: &'c str) -> Self {
		Self::Name(name)
	}
}

impl<'c> From<String> for CommandId<'c> {
	fn from(name: String) -> Self {
		Self::Name(name.as_str())
	}
}
