use std::sync::LazyLock;

mod store;
use crate::{PluginId, PluginInfo};
use puniyu_error::registry::Error;
use store::PluginStore;

static STORE: LazyLock<PluginStore> = LazyLock::new(PluginStore::new);

#[derive(Debug, Default)]
pub struct PluginRegistry;
impl<'p> PluginRegistry
where
	'p: 'static,
{
	/// 注册一个插件
	pub fn register<P>(plugin: P) -> Result<u64, Error>
	where
		P: Into<PluginInfo<'p>>,
	{
		let plugin = plugin.into();
		STORE.insert(plugin)
	}

	/// 卸载一个适配器
	pub fn unregister<P>(adapter: P) -> Result<(), Error>
	where
		P: Into<PluginId<'p>>,
	{
		let plugin_id = adapter.into();
		match plugin_id {
			PluginId::Index(index) => Self::unregister_with_index(index),
			PluginId::Name(name) => Self::unregister_with_plugin_name(&name),
		}
	}

	pub fn unregister_with_index(index: u64) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.get(&index).is_none() {
			return Err(Error::NotFound("Plugin".to_string()));
		}
		map.remove(&index);
		Ok(())
	}

	pub fn unregister_with_plugin_name(name: &str) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if !map.values().any(|v| v.name == name) {
			return Err(Error::NotFound("Plugin".to_string()));
		}
		map.retain(|_, v| v.name != name);
		Ok(())
	}

	pub fn get<P>(plugin: P) -> Vec<PluginInfo<'p>>
	where
		P: Into<PluginId<'p>>,
	{
		let plugin_id = plugin.into();
		match plugin_id {
			PluginId::Index(index) => Self::get_with_index(index).into_iter().collect(),
			PluginId::Name(name) => Self::get_with_plugin_name(&name),
		}
	}

	pub fn get_with_index(index: u64) -> Option<PluginInfo<'p>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	pub fn get_with_plugin_name(name: &str) -> Vec<PluginInfo<'p>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.name == name).cloned().collect()
	}

	pub fn all() -> Vec<PluginInfo<'p>> {
		STORE.all()
	}
}
