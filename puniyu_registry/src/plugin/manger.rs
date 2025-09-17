use super::{Plugin, PluginId, PluginType};
use crate::plugin::command::builder::CommandBuilder;
use crate::plugin::task::{builder::TaskBuilder, manger::TaskManager, registry::TaskRegistry};
use crate::{
	VERSION, error::Plugin as Error, library::PluginLibrary, logger::SharedLogger,
	plugin::builder::PluginBuilder, plugin::command::Command, plugin::task::Task,
};
use hashbrown::HashMap;
use libloading::Symbol;
use std::sync::{
	Arc, LazyLock, Mutex, OnceLock,
	atomic::{AtomicU64, Ordering},
};

static LIBRARY: OnceLock<Mutex<PluginLibrary>> = OnceLock::new();
static PLUGIN_INDEX: AtomicU64 = AtomicU64::new(0);

static PLUGIN_STORE: LazyLock<PluginStore> = LazyLock::new(PluginStore::default);

/// 收集任务
fn collect_tasks(tasks: &[Box<dyn TaskBuilder>]) -> Vec<Task> {
	tasks
		.iter()
		.map(|task_builder| Task { name: task_builder.name(), cron: task_builder.cron() })
		.collect()
}

/// 收集命令
fn collect_commands(commands: &[Box<dyn CommandBuilder>]) -> Vec<Command> {
	commands
		.iter()
		.map(|command_builder| Command {
			name: command_builder.name(),
			command: command_builder.command(),
			rank: command_builder.rank(),
		})
		.collect()
}

#[derive(Debug, Clone)]
pub struct PluginInfo {
	pub name: &'static str,
	pub version: &'static str,
	pub author: &'static str,
}

#[derive(Debug, Default, Clone)]
/// 插件存储器
pub struct PluginStore {
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

	pub(crate) fn get_plugin<T>(&self, plugin: T) -> Option<Plugin>
	where
		T: Into<PluginId>,
	{
		let plugin_id = plugin.into();
		let plugins = self.plugins.lock().unwrap();
		match plugin_id {
			PluginId::Index(index) => plugins.get(&index).cloned(),
			PluginId::Name(name) => {
				plugins.values().find(|plugin| plugin.info.name == name).cloned()
			}
		}
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

					if !plugin_name.starts_with("puniyu_plugin_") {
						// 插件名称必须以 `puniyu_plugin_` 开头
						return Ok(());
					}
					let abi_version = plugin_builder.abi_version();
					let plugin_info = PluginInfo {
						name: plugin_name,
						version: plugin_builder.version(),
						author: plugin_builder.author(),
					};
					if abi_version != VERSION {
						log::warn!(
							"插件 {} 版本 {} 不兼容当前ABI版本 {}，请联系开发者更新ABI版本",
							plugin_name,
							abi_version,
							VERSION
						);
						return Ok(());
					}
					let tasks = collect_tasks(&plugin_builder.tasks());
					for task_builder in plugin_builder.tasks() {
						let _ = TaskManager::add_task(TaskRegistry {
							plugin_name,
							builder: task_builder.into(),
						})
						.await;
					}

					let commands = collect_commands(&plugin_builder.commands());
					let plugin_obj = Plugin { info: plugin_info, tasks, commands };
					log::debug!("[plugin:{}] 正在加载插件", plugin_name);
					plugin_builder.init().await;
					log::debug!("[plugin:{}] 插件加载成功", plugin_name);

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
				if !plugin_name.starts_with("puniyu_plugin_") {
					// 插件名称必须以 `puniyu_plugin_` 开头
					return Ok(());
				}
				let abi_version = plugin_builder.abi_version();
				let plugin_info = PluginInfo {
					name: plugin_name,
					version: plugin_builder.version(),
					author: plugin_builder.author(),
				};

				if abi_version != VERSION {
					log::warn!(
						"插件 {} 版本 {} 不兼容当前版本 {}，请升级插件",
						plugin_name,
						abi_version,
						VERSION
					);
					return Ok(());
				}
				let tasks = collect_tasks(&plugin_builder.tasks());

				for task_builder in plugin_builder.tasks() {
					let _ = TaskManager::add_task(TaskRegistry {
						plugin_name,
						builder: task_builder.into(),
					})
					.await;
				}

				let commands = collect_commands(&plugin_builder.commands());
				let plugin = Plugin { info: plugin_info, tasks, commands };
				PLUGIN_STORE.insert_plugin(plugin);

				log::debug!("[plugin:{}] 正在加载插件", plugin_name);
				let plugin_future = plugin_builder.init();
				plugin_future.await;
				log::debug!("[plugin:{}] 插件加载成功", plugin_name);
			}
		}
		Ok(())
	}

	pub async fn load_plugins<T>(plugins: Vec<T>) -> Result<(), Error>
	where
		T: Into<PluginType>,
	{
		for plugin in plugins {
			Self::load_plugin(plugin).await?;
		}
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
		PLUGIN_STORE.get_plugin(plugin).map(|plugin| plugin.info)
	}
}
