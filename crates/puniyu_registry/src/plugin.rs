mod error;
pub use error::Error;
mod version;
pub use version::VERSION;

use crate::command::CommandRegistry;
use crate::server::ServerRegistry;
use crate::store::STORE;
use crate::task::TaskRegistry;
use convert_case::{Case, Casing};
use futures::future::join_all;
use puniyu_common::APP_NAME;
use puniyu_common::path::{PLUGIN_CONFIG_DIR, PLUGIN_DATA_DIR, PLUGIN_RESOURCE_DIR};
use puniyu_common::{merge_config, read_config, write_config};
use puniyu_config::ConfigRegistry;
use puniyu_library::{LibraryRegistry, libloading};
use puniyu_logger::{SharedLogger, debug, error, owo_colors::OwoColorize, warn};
use puniyu_types::plugin::{Plugin, PluginBuilder, PluginId, PluginType};
use puniyu_types::version::Version;
use std::sync::Arc;
use tokio::fs;

fn create_plugin_info(name: impl Into<String>, version: Version, author: Option<String>) -> Plugin {
	Plugin { name: name.into(), version, author }
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
				let lib = {
					LibraryRegistry::load_library(&path).unwrap();
					let name = path
						.file_name()
						.map(|n| n.to_string_lossy().to_string())
						.ok_or_else(|| Error::NotFound(path.to_string_lossy().to_string()))?;
					LibraryRegistry::get_library(&name).unwrap().library.clone()
				};
				unsafe {
					let symbol: libloading::Symbol<
						unsafe extern "C" fn() -> *mut dyn PluginBuilder,
					> = lib.get(b"plugin_info").unwrap();
					let plugin_builder = &*symbol();
					let set_logger: fn(&SharedLogger) = *lib.get(b"setup_logger").unwrap();
					set_logger(&SharedLogger::new());
					let setup_app_name: fn(name: String) = *lib.get(b"setup_app_name").unwrap();
					setup_app_name(APP_NAME.get().unwrap().to_string());
					let plugins = STORE.plugin().get_all_plugins();
					let plugin_name = plugin_builder.name();
					let plugin_version = plugin_builder.version().to_string();
					debug!(
						"[{}:{}({})] 正在加载插件",
						"plugin".fg_rgb::<175, 238, 238>(),
						plugin_name.fg_rgb::<240, 128, 128>(),
						format!("v {}", plugin_version).fg_rgb::<144, 238, 144>()
					);
					if plugins.iter().any(|(_, plugin)| plugin.name == plugin_name) {
						return Err(Error::Exists(plugin_name.to_string()));
					}

					let plugin_abi_version = plugin_builder.abi_version();

					if plugin_abi_version != VERSION {
						warn!(
							"[{}:{}] ABI版本不匹配, 当前ABI版本: {}, 插件ABI版本: {}",
							"plugin".fg_rgb::<175, 238, 238>(),
							plugin_name.fg_rgb::<240, 128, 128>(),
							plugin_abi_version,
							VERSION.to_string()
						);

						debug!(
							"[{}:{}] 检测到配置，开始强制加载",
							"plugin".fg_rgb::<175, 238, 238>(),
							plugin_name.fg_rgb::<240, 128, 128>()
						);
					}

					let tasks: Vec<_> = plugin_builder
						.tasks()
						.into_iter()
						.map(|task_builder| {
							TaskRegistry::add_task(plugin_name, task_builder.into())
						})
						.collect();

					let prefix = plugin_builder.prefix();
					let commands = plugin_builder.commands();
					commands.into_iter().for_each(|command| {
						CommandRegistry::insert(plugin_name, prefix, Arc::from(command));
					});

					join_all(tasks).await;

					let plugin_info = create_plugin_info(
						plugin_name,
						plugin_builder.version(),
						plugin_builder.author().map(|s| s.to_string()),
					);

					if let Some(server) = plugin_builder.server() {
						ServerRegistry::insert(plugin_name, server);
					}

					let config_dir =
						PLUGIN_CONFIG_DIR.as_path().join(plugin_name.to_case(Case::Snake));

					if !config_dir.exists() {
						let _ = fs::create_dir_all(&config_dir).await;
					}

					if let Some(configs) = plugin_builder.config() {
						configs.iter().for_each(|config| {
							let cfg = read_config::<toml::Value>(&config_dir, config.name());

							match cfg {
								Ok(cfg) => {
									merge_config(
										&config_dir,
										config.name(),
										&config.config(),
										&cfg,
									)
									.expect("合并插件配置文件失败");
									let cfg =
										read_config::<toml::Value>(&config_dir, config.name())
											.unwrap();
									let path = config_dir.join(format!("{}.toml", config.name()));
									ConfigRegistry::register(path.as_path(), cfg);
								}
								Err(_) => {
									debug!(
										"[{}:{}] 配置文件 {} 不存在，正在创建默认配置",
										"plugin".fg_rgb::<175, 238, 238>(),
										plugin_name.fg_rgb::<240, 128, 128>(),
										config.name()
									);
									write_config(&config_dir, config.name(), &config.config())
										.expect("创建默认配置文件失败");
								}
							}
						});
					}

					create_data_dir(plugin_name).await;
					create_resource_dir(plugin_name).await;
					run_plugin_init(plugin_name, plugin_builder.init()).await?;
					STORE.plugin().insert(plugin_info);
				}
			}
			// 静态插件
			PluginType::Builder(plugin_builder) => {
				let plugins = STORE.plugin().get_all_plugins();
				let plugin_name = plugin_builder.name();
				let plugin_version = plugin_builder.version().to_string();
				if plugins.iter().any(|(_, plugin)| plugin.name == plugin_name) {
					return Err(Error::Exists(plugin_name.to_string()));
				}

				debug!(
					"[{}:{}({})] 正在加载插件",
					"plugin".fg_rgb::<175, 238, 238>(),
					plugin_name.fg_rgb::<240, 128, 128>(),
					format!("v {}", plugin_version).fg_rgb::<144, 238, 144>()
				);

				let plugin_abi_version = plugin_builder.abi_version();

				if plugin_abi_version != VERSION {
					let plugin_tag = "plugin".fg_rgb::<175, 238, 238>();
					let plugin_name = plugin_name.fg_rgb::<240, 128, 128>();

					warn!(
						"[{}:{}] ABI版本不匹配, 当前ABI版本: {}, 插件ABI版本: {}",
						plugin_tag,
						plugin_name,
						plugin_abi_version,
						VERSION.to_string()
					);

					debug!("[{}:{}] 检测到配置，开始强制加载", plugin_tag, plugin_name);
				}

				let tasks: Vec<_> = plugin_builder
					.tasks()
					.into_iter()
					.map(|task_builder| TaskRegistry::add_task(plugin_name, task_builder.into()))
					.collect();

				join_all(tasks).await;

				let prefix = plugin_builder.prefix();
				let commands = plugin_builder.commands();
				commands.into_iter().for_each(|command| {
					CommandRegistry::insert(plugin_name, prefix, Arc::from(command));
				});

				let plugin_info = create_plugin_info(
					plugin_name,
					plugin_builder.version(),
					plugin_builder.author().map(|s| s.to_string()),
				);

				if let Some(server) = plugin_builder.server() {
					ServerRegistry::insert(plugin_name, server);
				}

				let config_dir = PLUGIN_CONFIG_DIR.as_path().join(plugin_name.to_case(Case::Snake));

				if !config_dir.exists() {
					let _ = fs::create_dir_all(&config_dir).await;
				}

				if let Some(configs) = plugin_builder.config() {
					configs.iter().for_each(|config| {
						let cfg = read_config::<toml::Value>(&config_dir, config.name());

						match cfg {
							Ok(cfg) => {
								merge_config(&config_dir, config.name(), &config.config(), &cfg)
									.expect("合并插件配置文件失败");
								let cfg =
									read_config::<toml::Value>(&config_dir, config.name()).unwrap();
								let path = config_dir.join(format!("{}.toml", config.name()));
								ConfigRegistry::register(path.as_path(), cfg);
							}
							Err(_) => {
								debug!(
									"[{}:{}] 配置文件 {} 不存在，正在创建默认配置",
									"plugin".fg_rgb::<175, 238, 238>(),
									plugin_name.fg_rgb::<240, 128, 128>(),
									config.name()
								);
								write_config(&config_dir, config.name(), &config.config())
									.expect("创建默认配置文件失败");
							}
						}
					});
				}

				create_data_dir(plugin_name).await;
				create_resource_dir(plugin_name).await;
				run_plugin_init(plugin_name, plugin_builder.init()).await?;
				STORE.plugin().insert(plugin_info);
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
	pub async fn unload_plugin(plugin: impl Into<PluginId>) -> Result<bool, Error> {
		let plugin_id = plugin.into();
		let result = match plugin_id {
			PluginId::Index(index) => {
				let plugin_name = STORE.plugin().remove(index).map(|p| p.name);
				if let Some(name) = plugin_name {
					TaskRegistry::remove_task(name.as_str()).await;
					CommandRegistry::remove_with_plugin_name(name.as_str());
					ServerRegistry::remove(name.as_str());
					let _ = puniyu_types::server::restart_server();
					true
				} else {
					false
				}
			}
			PluginId::Name(name) => {
				let index = STORE.plugin().find_index_by_name(name.as_str());
				if let Some(idx) = index {
					TaskRegistry::remove_task(name.as_str()).await;
					CommandRegistry::remove_with_plugin_name(name.as_str());
					ServerRegistry::remove(name.as_str());
					let _ = puniyu_types::server::restart_server();
					STORE.plugin().remove(idx).is_some()
				} else {
					false
				}
			}
		};
		Ok(result)
	}

	#[inline]
	pub fn get_plugin(plugin: impl Into<PluginId>) -> Option<Plugin> {
		let plugin_id = plugin.into();
		match plugin_id {
			PluginId::Index(index) => STORE.plugin().get_plugin_with_index(index),
			PluginId::Name(name) => STORE.plugin().get_plugin_with_name(name.as_str()),
		}
	}

	#[inline]
	pub fn get_all_plugins() -> Vec<Plugin> {
		STORE.plugin().get_all_plugins().into_values().collect()
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
	let data_dir = PLUGIN_DATA_DIR.as_path().join(name.to_case(Case::Snake));
	if !data_dir.exists() {
		let _ = fs::create_dir_all(&data_dir).await;
	}
}

async fn create_resource_dir(name: &str) {
	let resource_dir = PLUGIN_RESOURCE_DIR.as_path().join(name.to_case(Case::Snake));
	if !resource_dir.exists() {
		let _ = fs::create_dir_all(&resource_dir).await;
	}
}
