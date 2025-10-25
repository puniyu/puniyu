mod store;

use crate::TaskRegistry;
use crate::error::Plugin as Error;
use futures::future::join_all;
use puniyu_builder::plugin::{Plugin, PluginBuilder, PluginId, PluginType, VERSION as ABI_VERSION};
use puniyu_command::register_command;
use puniyu_command::{Command, CommandRegistry};
use puniyu_common::APP_NAME;
use puniyu_common::path::PLUGIN_DATA_DIR;
use puniyu_config::Config;
use puniyu_library::PluginLibrary;
use puniyu_library::libloading::Symbol;
use puniyu_logger::{SharedLogger, debug, error, owo_colors::OwoColorize, warn};
use puniyu_task::Task;
use std::sync::{LazyLock, Mutex, OnceLock};
use store::PluginStore;
use tokio::fs;

static LIBRARY: OnceLock<Mutex<PluginLibrary>> = OnceLock::new();
static PLUGIN_STORE: LazyLock<PluginStore> = LazyLock::new(PluginStore::default);

macro_rules! create_task_registry {
	($plugin_name:expr, $task_builder:expr) => {
		Task { plugin_name: $plugin_name, builder: $task_builder.into() }
	};
}

macro_rules! create_plugin_info {
	($name:expr, $version:expr, $author:expr) => {
		Plugin { name: $name, version: $version, author: $author }
	};
}

#[derive(Debug, Default)]
pub struct PluginRegistry;
impl PluginRegistry {
	/// 加载一个插件
	#[inline]
	pub async fn load_plugin(plugin: impl Into<PluginType>) -> Result<(), Error> {
		let plugin_id = plugin.into();
		match plugin_id {
			// 动态链接插件
			PluginType::Path(path) => {
				let client = LIBRARY.get_or_init(|| Mutex::new(PluginLibrary::new()));
				let lib = {
					let mut library = client.lock().unwrap();
					library.load_plugin(&path).unwrap();
					let name = path
						.file_name()
						.map(|n| n.to_string_lossy().to_string())
						.ok_or_else(|| Error::NotFound(path.to_string_lossy().to_string()))?;
					library.get_plugin(&name).unwrap().library.clone()
				};
				unsafe {
					let symbol: Symbol<unsafe extern "C" fn() -> *mut dyn PluginBuilder> =
						lib.get(b"plugin_info").unwrap();
					let plugin_builder = &*symbol();
					let set_logger: fn(&SharedLogger) = *lib.get(b"setup_logger").unwrap();
					set_logger(&SharedLogger::new());
					let setup_app_name: fn(name: String) = *lib.get(b"setup_app_name").unwrap();
					setup_app_name(APP_NAME.get().unwrap().to_string());
					let plugins = PLUGIN_STORE.get_all_plugins();
					let plugin_name = plugin_builder.name();
					debug!(
						"[{}:{}] 正在加载插件",
						"plugin".fg_rgb::<175, 238, 238>(),
						plugin_name.fg_rgb::<240, 128, 128>()
					);
					if plugins.iter().any(|(_, plugin)| plugin.name == plugin_name) {
						return Err(Error::Exists(plugin_name.to_string()));
					}

					let plugin_abi_version = plugin_builder.abi_version();
					let force_plugin = Config::app().load().force_plugin();

					if plugin_abi_version != ABI_VERSION {
						let plugin_tag = "plugin".fg_rgb::<175, 238, 238>();
						let plugin_name = plugin_name.fg_rgb::<240, 128, 128>();

						warn!(
							"[{}:{}] ABI版本不匹配, 当前ABI版本: {}, 插件ABI版本: {}",
							plugin_tag, plugin_name, plugin_abi_version, ABI_VERSION
						);

						if !force_plugin {
							return Ok(());
						}

						debug!("[{}:{}] 检测到配置，开始强制加载", plugin_tag, plugin_name);
					}

					let tasks: Vec<_> = plugin_builder
						.tasks()
						.into_iter()
						.map(|task_builder| {
							TaskRegistry::add_task(create_task_registry!(plugin_name, task_builder))
						})
						.collect();

					let commands = plugin_builder.commands();
					commands.into_iter().for_each(|command| {
						register_command!(plugin_name, command);
					});

					join_all(tasks).await;

					let plugin_info = create_plugin_info!(
						plugin_name,
						plugin_builder.version(),
						plugin_builder.author()
					);

					create_data_dir(plugin_name).await;
					run_plugin_init(plugin_name, plugin_builder.init()).await?;
					PLUGIN_STORE.insert_plugin(plugin_info);
				}
			}
			// 静态插件
			PluginType::Builder(plugin_builder) => {
				let plugins = PLUGIN_STORE.get_all_plugins();
				let plugin_name = plugin_builder.name();
				if plugins.iter().any(|(_, plugin)| plugin.name == plugin_name) {
					return Err(Error::Exists(plugin_name.to_string()));
				}
				let plugin_abi_version = plugin_builder.abi_version();
				let force_plugin = Config::app().load().force_plugin();

				if plugin_abi_version != ABI_VERSION {
					let plugin_tag = "plugin".fg_rgb::<175, 238, 238>();
					let plugin_name = plugin_name.fg_rgb::<240, 128, 128>();

					warn!(
						"[{}:{}] ABI版本不匹配, 当前ABI版本: {}, 插件ABI版本: {}",
						plugin_tag, plugin_name, plugin_abi_version, ABI_VERSION
					);

					if !force_plugin {
						return Ok(());
					}

					debug!("[{}:{}] 检测到配置，开始强制加载", plugin_tag, plugin_name);
				}

				let tasks: Vec<_> = plugin_builder
					.tasks()
					.into_iter()
					.map(|task_builder| {
						TaskRegistry::add_task(create_task_registry!(plugin_name, task_builder))
					})
					.collect();

				join_all(tasks).await;

				let commands = plugin_builder.commands();
				commands.into_iter().for_each(|command| {
					register_command!(plugin_name, command);
				});

				let plugin_info = create_plugin_info!(
					plugin_name,
					plugin_builder.version(),
					plugin_builder.author()
				);

				debug!(
					"[{}:{}] 正在加载插件",
					"plugin".fg_rgb::<175, 238, 238>(),
					plugin_name.fg_rgb::<240, 128, 128>()
				);

				create_data_dir(plugin_name).await;
				run_plugin_init(plugin_name, plugin_builder.init()).await?;
				PLUGIN_STORE.insert_plugin(plugin_info);
			}
		}
		Ok(())
	}

	pub async fn load_plugins(plugins: Vec<impl Into<PluginType>>) -> Result<(), Error> {
		futures::future::try_join_all(plugins.into_iter().map(|plugin| Self::load_plugin(plugin)))
			.await?;

		Ok(())
	}

	#[inline]
	pub async fn unload_plugin(plugin: impl Into<PluginId>) -> Result<(), Error> {
		let plugin_id = plugin.into();
		PLUGIN_STORE.remove_plugin(plugin_id).await;
		Ok(())
	}

	#[inline]
	pub fn get_plugin(plugin: impl Into<PluginId>) -> Option<Plugin> {
		let plugin_id = plugin.into();
		match plugin_id {
			PluginId::Index(index) => PLUGIN_STORE.get_plugin_with_index(index),
			PluginId::Name(name) => PLUGIN_STORE.get_plugin_with_name(name.as_str()),
		}
	}

	#[inline]
	pub fn get_all_plugins() -> Vec<Plugin> {
		PLUGIN_STORE.get_all_plugins().into_values().collect()
	}
}

async fn run_plugin_init<F>(name: &str, init_fn: F) -> Result<(), Error>
where
	F: Future<Output = Result<(), Box<dyn std::error::Error>>>,
{
	match init_fn.await {
		Ok(()) => {
			debug!(
				"[{}:{}] 插件加载成功",
				"plugin".fg_rgb::<175, 238, 238>(),
				name.fg_rgb::<240, 128, 128>()
			);
			Ok(())
		}
		Err(e) => {
			error!(
				"[{}:{}] 插件加载失败：{}",
				"plugin".fg_rgb::<175, 238, 238>(),
				name.fg_rgb::<240, 128, 128>(),
				e.to_string()
			);
			Err(Error::Init(e.to_string()))
		}
	}
}

async fn create_data_dir(name: &str) {
	let data_dir = PLUGIN_DATA_DIR.as_path();
	let adapter_data_dir = data_dir.join(name);
	if !adapter_data_dir.exists() {
		let _ = fs::create_dir_all(&adapter_data_dir).await;
	}
}
