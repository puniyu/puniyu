mod store;
mod types;
pub use types::*;

use store::PluginStore;

use crate::{Error, Result};
use puniyu_types::plugin::Plugin;
use std::sync::{Arc, LazyLock};

static STORE: LazyLock<PluginStore> = LazyLock::new(PluginStore::new);

#[derive(Debug, Default)]
pub struct PluginRegistry;
impl PluginRegistry {
	/// 注册一个插件
	pub fn register<P>(plugin: P) -> Result<u64>
	where
		P: Into<Arc<dyn Plugin>>,
	{
		let adapter = plugin.into();
		STORE.insert(adapter)
	}

	/// 卸载一个适配器
	pub fn unregister<P>(adapter: P) -> Result<()>
	where
		P: Into<PluginId>,
	{
		let plugin_id = adapter.into();
		match plugin_id {
			PluginId::Index(index) => Self::unregister_with_index(index),
			PluginId::Name(name) => Self::unregister_with_plugin_name(&name)
		}
	}

	pub fn unregister_with_index(index: u64) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.get(&index).is_none() {
			return Err(Error::NotFound("Plugin".to_string()));
		}
		map.remove(&index);
		Ok(())
	}

	pub fn unregister_with_plugin_name(name: &str) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if !map.values().any(|v| v.info().name == name) {
			return Err(Error::NotFound("Plugin".to_string()));
		}
		map.retain(|_, v| v.info().name != name);
		Ok(())
	}

	pub fn get<P>(plugin: P) -> Vec<Arc<dyn Plugin>>
	where
		P: Into<PluginId>,
	{
		let plugin_id = plugin.into();
		match plugin_id {
			PluginId::Index(index) => Self::get_with_index(index).into_iter().collect(),
			PluginId::Name(name) => Self::get_with_plugin_name(&name),
		}
	}

	pub fn get_with_index(index: u64) -> Option<Arc<dyn Plugin>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	pub fn get_with_plugin_name(name: &str) -> Vec<Arc<dyn Plugin>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.name() == name).collect()
	}

	pub fn all() -> Vec<Arc<dyn Plugin>> {
		STORE.all()
	}
}
