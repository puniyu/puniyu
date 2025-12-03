use super::{Plugin, PluginId};
use crate::command::CommandRegistry;
use crate::server::ServerRegistry;
use crate::task::TaskRegistry;
use puniyu_types::server::restart_server;
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
pub struct PluginStore(Arc<Mutex<HashMap<u64, Plugin>>>);
impl PluginStore {
	pub fn insert(&self, plugin: Plugin) {
		let mut plugins = self.0.lock().unwrap();
		let exists = plugins.values().any(|p| p.name == plugin.name);
		if !exists {
			let index = PLUGIN_INDEX.fetch_add(1, Ordering::Relaxed);
			plugins.insert(index, plugin);
		}
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

	pub async fn remove_plugin(&self, plugin: impl Into<PluginId>) -> bool {
		let plugin_id = plugin.into();

		match plugin_id {
			PluginId::Index(index) => {
				let plugin_name = {
					let mut plugins = self.0.lock().unwrap();
					plugins.remove(&index).map(|p| p.name)
				};

				if let Some(name) = plugin_name {
					TaskRegistry::remove_task(name.as_str()).await;
					CommandRegistry::remove_with_plugin_name(name.as_str());
					ServerRegistry::remove(name.as_str());
					let _ = restart_server();
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

				TaskRegistry::remove_task(name.clone()).await;
				CommandRegistry::remove_with_plugin_name(name.as_str());
				ServerRegistry::remove(name.as_str());
				let _ = restart_server();

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
