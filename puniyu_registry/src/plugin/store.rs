use crate::plugin::{Plugin, PluginId, task::manger::TaskManager};
use std::{
	collections::HashMap,
	sync::{
		Arc, Mutex,
		atomic::{AtomicU64, Ordering},
	},
};

static PLUGIN_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Default, Clone)]
/// 插件存储器
pub(crate) struct PluginStore(Arc<Mutex<HashMap<u64, Plugin>>>);
impl PluginStore {
	pub fn insert_plugin(&self, plugin: Plugin) {
		let index = PLUGIN_INDEX.fetch_add(1, Ordering::Relaxed);
		let mut plugins = self.0.lock().unwrap();
		plugins.insert(index, plugin);
	}

	pub fn get_plugins(&self) -> Arc<Mutex<HashMap<u64, Plugin>>> {
		self.0.clone()
	}

	pub fn get_all_plugins(&self) -> HashMap<u64, Plugin> {
		let plugins = self.0.lock().unwrap();
		plugins.clone()
	}

	pub fn get_plugin_with_index(&self, index: u64) -> Option<Plugin> {
		let plugins = self.0.lock().unwrap();
		plugins.get(&index).cloned()
	}

	pub fn get_plugin_with_name(&self, name: &str) -> Option<Plugin> {
		let plugins = self.0.lock().unwrap();
		plugins.values().find(|plugin| plugin.name == name).cloned()
	}

	pub(crate) async fn remove_plugin(&self, plugin: impl Into<PluginId>) -> bool {
		let plugin_id = plugin.into();

		match plugin_id {
			PluginId::Index(index) => {
				let plugin_name = {
					let mut plugins = self.0.lock().unwrap();
					plugins.remove(&index).map(|p| p.name)
				};

				if let Some(name) = plugin_name {
					TaskManager::remove_task(name).await;
					true
				} else {
					false
				}
			}

			PluginId::Name(name) => {
				let (index, exists) = {
					let plugins = self.0.lock().unwrap();
					if let Some((idx, _)) = plugins
						.iter()
						.find(|(_, plugin)| plugin.name == name)
						.map(|(idx, p)| (*idx, p.clone()))
					{
						(Some(idx), true)
					} else {
						(None, false)
					}
				};

				if !exists {
					return false;
				}

				TaskManager::remove_task(name.clone()).await;

				if let Some(idx) = index {
					let mut plugins = self.0.lock().unwrap();
					plugins.remove(&idx).is_some()
				} else {
					false
				}
			}
		}
	}
}
