mod store;
mod types;

pub use types::CommandInfo;

use crate::{Error, Result};
use std::sync::LazyLock;
use store::CommandStore;

static STORE: LazyLock<CommandStore> = LazyLock::new(CommandStore::new);

pub struct CommandRegistry;

impl<'c> CommandRegistry {
	/// 注册命令
	pub fn register<C>(command: C) -> Result<u64>
	where
		C: Into<types::Command>,
	{
		let command = command.into();
		STORE.insert(command.into())
	}
	/// 卸载已注册的指令
	pub fn unregister<C>(command: C) -> Result<()>
	where
		C: Into<types::CommandId<'c>>,
	{
		let command = command.into();
		match command {
			types::CommandId::Id(v) => Self::unregister_with_index(v),
			types::CommandId::Name(v) => Self::unregister_with_command_name(v),
		}
	}

	pub fn unregister_with_index(command_id: u64) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.get(&command_id).is_none() {
			return Err(Error::NotFound("Command".to_string()));
		}
		map.remove(&command_id);
		Ok(())
	}

	pub fn unregister_with_command_name(name: &str) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if !map.values().any(|v| v.name == name) {
			return Err(Error::NotFound("Command".to_string()));
		}
		map.retain(|_, v| v.name != name);
		Ok(())
	}

	pub fn unregister_with_plugin_id(plugin_id: u64) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if !map.values().any(|v| v.plugin_id == plugin_id) {
			return Err(Error::NotFound("Command".to_string()));
		}
		map.retain(|_, v| v.plugin_id != plugin_id);
		Ok(())
	}
	pub fn get<C, D>(command: C) -> Vec<CommandInfo<'c>>
	where
		C: Into<types::CommandId<'c>>,
	{
		let command = command.into();
		match command {
			types::CommandId::Id(v) => Self::get_with_command_id(v).into_iter().collect(),
			types::CommandId::Name(v) => Self::get_with_command_name(v),
		}
	}
	pub fn get_with_command_id(id: u64) -> Option<CommandInfo<'c>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&id).cloned()
	}

	pub fn get_with_command_name(name: &str) -> Vec<CommandInfo<'c>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.name == name).cloned().collect::<Vec<CommandInfo<'c>>>()
	}

	pub fn get_with_plugin_id(plugin_id: u64) -> Vec<CommandInfo<'c>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.plugin_id == plugin_id).cloned().collect::<Vec<CommandInfo<'c>>>()
	}

	pub fn all() -> Vec<CommandInfo<'c>> {
		STORE.all()
	}
}
