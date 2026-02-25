//! 命令注册表模块

mod store;

use crate::Command;
use crate::types::{CommandId, CommandInfo};
use puniyu_error::registry::Error;
use std::sync::{Arc, LazyLock};
use store::CommandStore;

static STORE: LazyLock<CommandStore> = LazyLock::new(CommandStore::new);

/// 命令注册表
///
/// 提供命令的注册、卸载和查询功能。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_command::CommandRegistry;
///
/// // 注册命令
/// let index = CommandRegistry::register(command_info)?;
///
/// // 查询命令
/// let commands = CommandRegistry::all();
///
/// // 卸载命令
/// CommandRegistry::unregister(index)?;
/// ```
pub struct CommandRegistry;

impl<'c> CommandRegistry {
	/// 注册命令
	///
	/// # 参数
	///
	/// - `plugin_id` - 插件 ID
	/// - `command` - 命令信息
	///
	/// # 返回值
	///
	/// 返回命令的索引 ID
	///
	/// # 错误
	///
	/// 如果命令已存在，返回错误
	pub fn register(plugin_id: u64, command: Arc<dyn Command>) -> Result<u64, Error> {
		let command = CommandInfo { plugin_id, builder: command };
		STORE.insert(command)
	}

	/// 卸载已注册的命令
	///
	/// # 参数
	///
	/// - `command` - 命令 ID（索引或名称）
	///
	/// # 错误
	///
	/// 如果命令不存在，返回错误
	pub fn unregister<C>(command: C) -> Result<(), Error>
	where
		C: Into<CommandId<'c>>,
	{
		let command = command.into();
		match command {
			CommandId::Id(v) => Self::unregister_with_index(v),
			CommandId::Name(v) => Self::unregister_with_command_name(v),
		}
	}

	/// 通过索引卸载命令
	pub fn unregister_with_index(command_id: u64) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.get(&command_id).is_none() {
			return Err(Error::NotFound("Command".to_string()));
		}
		map.remove(&command_id);
		Ok(())
	}

	/// 通过名称卸载命令
	pub fn unregister_with_command_name(name: &str) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if !map.values().any(|v| v.builder.name() == name) {
			return Err(Error::NotFound("Command".to_string()));
		}
		map.retain(|_, v| v.builder.name() != name);
		Ok(())
	}

	/// 通过插件 ID 卸载所有命令
	pub fn unregister_with_plugin_id(plugin_id: u64) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if !map.values().any(|v| v.plugin_id == plugin_id) {
			return Err(Error::NotFound("Command".to_string()));
		}
		map.retain(|_, v| v.plugin_id != plugin_id);
		Ok(())
	}

	/// 查询命令
	///
	/// # 参数
	///
	/// - `command` - 命令 ID（索引或名称）
	///
	/// # 返回值
	///
	/// 返回匹配的命令信息列表
	pub fn get<C, D>(command: C) -> Vec<CommandInfo>
	where
		C: Into<CommandId<'c>>,
	{
		let command = command.into();
		match command {
			CommandId::Id(v) => Self::get_with_command_id(v).into_iter().collect(),
			CommandId::Name(v) => Self::get_with_command_name(v),
		}
	}

	/// 通过索引查询命令
	pub fn get_with_command_id(id: u64) -> Option<CommandInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&id).cloned()
	}

	/// 通过命令名称查询命令
	pub fn get_with_command_name(name: &str) -> Vec<CommandInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.builder.name() == name).cloned().collect::<Vec<CommandInfo>>()
	}

	/// 通过命令别名查询命令
	pub fn get_with_command_alias(alias: &str) -> Vec<CommandInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values()
			.filter(|v| v.builder.alias().contains(&alias))
			.cloned()
			.collect::<Vec<CommandInfo>>()
	}

	/// 通过插件 ID 查询所有命令
	pub fn get_with_plugin_id(plugin_id: u64) -> Vec<CommandInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.plugin_id == plugin_id).cloned().collect::<Vec<CommandInfo>>()
	}

	/// 获取所有命令
	pub fn all() -> Vec<CommandInfo> {
		STORE.all()
	}
}
