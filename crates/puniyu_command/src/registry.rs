mod store;

use crate::Command;
use crate::error::Error;
use crate::types::CommandId;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::RwLock;
use std::sync::atomic::{AtomicU64, Ordering};
use store::CommandStore;

static COMMAND_ID: AtomicU64 = AtomicU64::new(0);
static STORE: LazyLock<CommandStore> = LazyLock::new(CommandStore::new);
static PLUGIN_MAP: LazyLock<RwLock<HashMap<u64, Vec<u64>>>> =
	LazyLock::new(|| RwLock::new(HashMap::new()));

pub struct CommandRegistry;

impl<'c> CommandRegistry {
	/// 注册命令。
	pub fn register(plugin_id: u64, command: impl Into<Arc<dyn Command>>) -> Result<u64, Error> {
		let command = command.into();
		let mut map = STORE.raw().write().expect("poisoned lock");

		if map.values().any(|c| c.name() == command.name()) {
			return Err(Error::Exists);
		}

		let index = COMMAND_ID.fetch_add(1, Ordering::Relaxed);
		map.insert(index, command);
		drop(map);

		PLUGIN_MAP
			.write()
			.expect("poisoned lock")
			.entry(plugin_id)
			.or_default()
			.push(index);

		Ok(index)
	}

	/// 按索引或名称卸载命令。
	pub fn unregister<C>(command: C) -> Result<(), Error>
	where
		C: Into<CommandId<'c>>,
	{
		let command = command.into();
		match command {
			CommandId::Id(v) => Self::unregister_with_index(v),
			CommandId::Name(v) => Self::unregister_with_command_name(v.as_ref()),
		}
	}

	/// 通过索引卸载命令。
	pub fn unregister_with_index(command_id: u64) -> Result<(), Error> {
		let mut map = STORE.raw().write().expect("poisoned lock");
		if map.remove(&command_id).is_none() {
			return Err(Error::NotFound);
		}
		drop(map);

		let mut plugin_map = PLUGIN_MAP.write().expect("poisoned lock");
		plugin_map.values_mut().for_each(|ids| ids.retain(|id| *id != command_id));

		Ok(())
	}

	/// 通过名称卸载命令。
	pub fn unregister_with_command_name(name: &str) -> Result<(), Error> {
		let mut map = STORE.raw().write().expect("poisoned lock");
		let ids: Vec<u64> = map
			.iter()
			.filter(|(_, c)| c.name() == name)
			.map(|(&id, _)| id)
			.collect();

		if ids.is_empty() {
			return Err(Error::NotFound);
		}

		for id in &ids {
			map.remove(id);
		}
		drop(map);

		let mut plugin_map = PLUGIN_MAP.write().expect("poisoned lock");
		plugin_map.values_mut().for_each(|v| v.retain(|id| !ids.contains(id)));

		Ok(())
	}

	/// 通过插件 ID 卸载所有命令。
	pub fn unregister_with_plugin_id(plugin_id: u64) -> Result<(), Error> {
		let mut plugin_map = PLUGIN_MAP.write().expect("poisoned lock");
		let ids = plugin_map.remove(&plugin_id).unwrap_or_default();
		drop(plugin_map);

		if ids.is_empty() {
			return Err(Error::NotFound);
		}

		let mut map = STORE.raw().write().expect("poisoned lock");
		for id in ids {
			map.remove(&id);
		}

		Ok(())
	}

	/// 按索引或名称查询命令。
	pub fn get<C>(command: C) -> Vec<Arc<dyn Command>>
	where
		C: Into<CommandId<'c>>,
	{
		let command = command.into();
		match command {
			CommandId::Id(v) => Self::get_with_command_id(v).into_iter().collect(),
			CommandId::Name(v) => Self::get_with_command_name(v.as_ref()),
		}
	}

	/// 通过索引查询命令。
	pub fn get_with_command_id(id: u64) -> Option<Arc<dyn Command>> {
		let map = STORE.raw().read().expect("poisoned lock");
		map.get(&id).cloned()
	}

	/// 通过命令名称查询命令。
	pub fn get_with_command_name(name: &str) -> Vec<Arc<dyn Command>> {
		let map = STORE.raw().read().expect("poisoned lock");
		map.values().filter(|c| c.name() == name).cloned().collect()
	}

	/// 通过命令别名查询命令。
	pub fn get_with_command_alias(alias: &str) -> Vec<Arc<dyn Command>> {
		let map = STORE.raw().read().expect("poisoned lock");
		map.values().filter(|c| c.alias().contains(&alias)).cloned().collect()
	}

	/// 通过插件 ID 查询所有命令。
	pub fn get_with_plugin_id(plugin_id: u64) -> Vec<Arc<dyn Command>> {
		let plugin_map = PLUGIN_MAP.read().expect("poisoned lock");
		let ids = match plugin_map.get(&plugin_id) {
			Some(ids) => ids.clone(),
			None => return Vec::new(),
		};
		drop(plugin_map);

		let map = STORE.raw().read().expect("poisoned lock");
		ids.iter().filter_map(|id| map.get(id).cloned()).collect()
	}

	/// 获取所有已注册命令。
	pub fn all() -> Vec<Arc<dyn Command>> {
		let map = STORE.raw().read().expect("poisoned lock");
		map.values().cloned().collect()
	}
}
