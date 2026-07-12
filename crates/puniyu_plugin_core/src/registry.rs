mod store;

use crate::Plugin;
use crate::error::Error;
use crate::types::PluginId;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};
use store::PluginStore;

static PLUGIN_INDEX: AtomicU64 = AtomicU64::new(0);
static STORE: LazyLock<PluginStore> = LazyLock::new(PluginStore::new);

#[derive(Debug, Default)]
pub struct PluginRegistry;

impl<'p> PluginRegistry {
	/// 注册一个插件。
	pub fn register(plugin: impl Into<Arc<dyn Plugin>>) -> Result<u64, Error> {
		let plugin = plugin.into();
		let mut map = STORE.raw().write().expect("poisoned lock");

		if map.values().any(|v| v.name() == plugin.name()) {
			return Err(Error::Exists);
		}

		let index = PLUGIN_INDEX.fetch_add(1, Ordering::Relaxed);
		map.insert(index, plugin);
		Ok(index)
	}

	/// 按索引或名称卸载插件。
	pub fn unregister<P>(plugin: P) -> Result<(), Error>
	where
		P: Into<PluginId<'p>>,
	{
		let plugin_id = plugin.into();
		match plugin_id {
			PluginId::Index(index) => Self::unregister_with_index(index),
			PluginId::Name(name) => Self::unregister_with_plugin_name(name.as_ref()),
		}
	}

	/// 通过索引卸载插件。
	pub fn unregister_with_index(index: u64) -> Result<(), Error> {
		let mut map = STORE.raw().write().expect("poisoned lock");
		if map.remove(&index).is_none() {
			return Err(Error::NotFound);
		}
		Ok(())
	}

	/// 通过名称卸载插件。
	pub fn unregister_with_plugin_name(name: &str) -> Result<(), Error> {
		let mut map = STORE.raw().write().expect("poisoned lock");
		if !map.values().any(|v| v.name() == name) {
			return Err(Error::NotFound);
		}
		map.retain(|_, v| v.name() != name);
		Ok(())
	}

	/// 按索引或名称查询插件。
	pub fn get<P>(plugin: P) -> Option<Arc<dyn Plugin>>
	where
		P: Into<PluginId<'p>>,
	{
		let plugin_id = plugin.into();
		match plugin_id {
			PluginId::Index(index) => Self::get_with_index(index),
			PluginId::Name(name) => Self::get_with_plugin_name(name.as_ref()),
		}
	}

	/// 通过索引查询插件。
	pub fn get_with_index(index: u64) -> Option<Arc<dyn Plugin>> {
		let map = STORE.raw().read().expect("poisoned lock");
		map.get(&index).cloned()
	}

	/// 通过名称查询插件。
	pub fn get_with_plugin_name(name: &str) -> Option<Arc<dyn Plugin>> {
		let map = STORE.raw().read().expect("poisoned lock");
		map.values().find(|v| v.name() == name).cloned()
	}

	/// 获取所有已注册插件。
	pub fn all() -> Vec<Arc<dyn Plugin>> {
		let map = STORE.raw().read().expect("poisoned lock");
		map.values().cloned().collect()
	}
}
