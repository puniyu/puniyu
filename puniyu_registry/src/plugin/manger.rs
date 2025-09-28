use super::{Plugin, PluginId, PluginType};
use crate::{
	VERSION,
	error::Load as Error,
	library::PluginLibrary,
	logger::SharedLogger,
	logger::{debug, warn},
	plugin::builder::PluginBuilder,
	plugin::{
		command::Command,
		task::{Task, manger::TaskManager, registry::TaskRegistry},
	},
};
use libloading::Symbol;
use std::{
	collections::HashMap,
	sync::{
		Arc, LazyLock, Mutex, OnceLock,
		atomic::{AtomicU64, Ordering},
	},
};

static LIBRARY: OnceLock<Mutex<PluginLibrary>> = OnceLock::new();
static PLUGIN_INDEX: AtomicU64 = AtomicU64::new(0);

static PLUGIN_STORE: LazyLock<PluginStore> = LazyLock::new(PluginStore::default);

#[derive(Debug, Clone)]
pub struct PluginInfo {
	pub name: &'static str,
	pub version: &'static str,
	pub author: &'static str,
}

#[derive(Debug, Default, Clone)]
/// 插件存储器
pub(crate) struct PluginStore {
	plugins: Arc<Mutex<HashMap<u64, Plugin>>>,
}

impl PluginStore {
	pub fn insert_plugin(&self, plugin: Plugin) {
		let index = PLUGIN_INDEX.fetch_add(1, Ordering::Relaxed);
		let mut plugins = self.plugins.lock().unwrap();
		plugins.insert(index, plugin);
	}

	pub fn get_plugins(&self) -> Arc<Mutex<HashMap<u64, Plugin>>> {
		self.plugins.clone()
	}

	pub fn get_all_plugins(&self) -> HashMap<u64, Plugin> {
		let plugins = self.plugins.lock().unwrap();
		plugins.clone()
	}

	pub fn get_plugin_with_index(&self, index: u64) -> Option<Plugin> {
		let plugins = self.plugins.lock().unwrap();
		plugins.get(&index).cloned()
	}

	pub fn get_plugin_with_name(&self, name: &str) -> Option<Plugin> {
		let plugins = self.plugins.lock().unwrap();
		plugins.values().find(|plugin| plugin.info.name == name).cloned()
	}

	pub(crate) async fn remove_plugin<T>(&self, plugin: T) -> bool
	where
		T: Into<PluginId>,
	{
		let plugin_id = plugin.into();

		match plugin_id {
			PluginId::Index(index) => {
				let plugin_name = {
					let mut plugins = self.plugins.lock().unwrap();
					plugins.remove(&index).map(|p| p.info.name)
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
					let plugins = self.plugins.lock().unwrap();
					if let Some((idx, _)) = plugins
						.iter()
						.find(|(_, plugin)| plugin.info.name == name)
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
					let mut plugins = self.plugins.lock().unwrap();
					plugins.remove(&idx).is_some()
				} else {
					false
				}
			}
		}
	}
}

pub struct PluginManager;

impl PluginManager {
	pub async fn load_plugin<T>(plugin: T) -> Result<(), Error>
	where
		T: Into<PluginType>,
	{
		let plugin_id: PluginType = plugin.into();
		match plugin_id {
			// 动态链接插件
			PluginType::Path(name) => {
				let client = LIBRARY.get_or_init(|| Mutex::new(PluginLibrary::new()));
				let lib = {
					let mut library = client.lock().unwrap();
					library.load_plugin(&name).unwrap();
					library.get_plugin(&name).unwrap().clone()
				};
				unsafe {
					let symbol: Symbol<unsafe extern "C" fn() -> *mut dyn PluginBuilder> =
						lib.get(b"plugin_info").unwrap();
					let plugin_builder = &*symbol();
					let set_logger: fn(&SharedLogger) = *lib.get(b"setup_logger").unwrap();
					set_logger(&SharedLogger::new());
					let plugins = PLUGIN_STORE.get_all_plugins();
					let plugin_name = plugin_builder.name();
					if plugins.iter().any(|(_, plugin)| plugin.info.name == plugin_name) {
						return Err(Error::Exists(plugin_name.to_string()));
					}

					let abi_version = plugin_builder.abi_version();
					let plugin_info = PluginInfo {
						name: plugin_name,
						version: plugin_builder.version(),
						author: plugin_builder.author(),
					};
					if abi_version != VERSION {
						warn!(
							"插件 {} 版本 {} 不兼容当前ABI版本 {}，请联系开发者更新ABI版本",
							plugin_name, abi_version, VERSION
						);
						return Ok(());
					}
					let tasks: Vec<Task> =
						plugin_builder.tasks().into_iter().map(Into::into).collect();
					for task_builder in plugin_builder.tasks() {
						let _ = TaskManager::add_task(TaskRegistry {
							plugin_name,
							builder: task_builder.into(),
						})
						.await;
					}

					let commands: Vec<Command> =
						plugin_builder.commands().into_iter().map(Into::into).collect();
					let plugin_obj = Plugin { info: plugin_info, tasks, commands };
					debug!("[plugin:{}] 正在加载插件", plugin_name);
					plugin_builder.init().await;
					debug!("[plugin:{}] 插件加载成功", plugin_name);

					PLUGIN_STORE.insert_plugin(plugin_obj);
				}
			}
			// 静态插件
			PluginType::Builder(plugin_builder) => {
				let plugins = PLUGIN_STORE.get_all_plugins();
				let plugin_name = plugin_builder.name();
				if plugins.iter().any(|(_, plugin)| plugin.info.name == plugin_name) {
					return Err(Error::Exists(plugin_name.to_string()));
				}
				let abi_version = plugin_builder.abi_version();
				let plugin_info = PluginInfo {
					name: plugin_name,
					version: plugin_builder.version(),
					author: plugin_builder.author(),
				};

				if abi_version != VERSION {
					warn!(
						"插件 {} 版本 {} 不兼容当前版本 {}，请升级插件",
						plugin_name, abi_version, VERSION
					);
					return Ok(());
				}

				let tasks: Vec<Task> = plugin_builder.tasks().into_iter().map(Into::into).collect();

				for task_builder in plugin_builder.tasks() {
					let _ = TaskManager::add_task(TaskRegistry {
						plugin_name,
						builder: task_builder.into(),
					})
					.await;
				}

				let commands: Vec<Command> =
					plugin_builder.commands().into_iter().map(Into::into).collect();
				let plugin = Plugin { info: plugin_info, tasks, commands };
				PLUGIN_STORE.insert_plugin(plugin);

				debug!("[plugin:{}] 正在加载插件", plugin_name);
				plugin_builder.init().await;
				debug!("[plugin:{}] 插件加载成功", plugin_name);
			}
		}
		Ok(())
	}

	pub async fn load_plugins<T>(plugins: Vec<T>) -> Result<(), Error>
	where
		T: Into<PluginType>,
	{
		futures::future::try_join_all(plugins.into_iter().map(|plugin| Self::load_plugin(plugin)))
			.await?;

		Ok(())
	}

	pub async fn unload_plugin<T>(plugin: T) -> Result<(), Error>
	where
		T: Into<PluginId>,
	{
		let plugin_id = plugin.into();
		PLUGIN_STORE.remove_plugin(plugin_id).await;
		Ok(())
	}

	pub fn get_plugin<T>(plugin: T) -> Option<PluginInfo>
	where
		T: Into<PluginId>,
	{
		let plugin_id = plugin.into();
		match plugin_id {
			PluginId::Index(index) => PLUGIN_STORE.get_plugin_with_index(index).map(|p| p.info),
			PluginId::Name(name) => {
				PLUGIN_STORE.get_plugin_with_name(name.as_str()).map(|p| p.info)
			}
		}
	}
}
