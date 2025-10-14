use crate::logger::error;
use crate::{
	PluginType, VERSION,
	error::Plugin as Error,
	library::PluginLibrary,
	logger::{SharedLogger, debug, owo_colors::OwoColorize, warn},
	plugin::{
		Plugin, PluginId,
		builder::PluginBuilder,
		command::Command,
		store::PluginStore,
		task::{Task, manger::TaskManager, registry::TaskRegistry},
	},
};
use libloading::Symbol;
use puniyu_utils::APP_NAME;
use std::sync::{LazyLock, Mutex, OnceLock};

static LIBRARY: OnceLock<Mutex<PluginLibrary>> = OnceLock::new();
static PLUGIN_STORE: LazyLock<PluginStore> = LazyLock::new(PluginStore::default);

#[derive(Debug, Clone, Default)]
pub struct PluginInfo {
	/// 插件名称
	pub name: &'static str,
	/// 插件版本
	pub version: &'static str,
	/// 插件作者
	pub author: &'static str,
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
						.file_stem()
						.ok_or_else(|| Error::NotFound(path.to_string_lossy().to_string()))?
						.to_string_lossy()
						.to_string();
					library.get_plugin(&name).unwrap().library.clone()
				};
				unsafe {
					let symbol: Symbol<unsafe extern "C" fn() -> *mut dyn PluginBuilder> =
						lib.get(b"plugin_info").unwrap();
					let plugin_builder = &*symbol();
					let set_logger: fn(&SharedLogger) = *lib.get(b"setup_logger").unwrap();
					set_logger(&SharedLogger::new());
					let setup_app_name: fn(name: &str) = *lib.get(b"setup_app_name").unwrap();
					setup_app_name(APP_NAME.get().unwrap());
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

					let abi_version = plugin_builder.abi_version();
					check_plugin_abi_version(abi_version, plugin_name);
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
					let plugin_info = Plugin {
						name: plugin_name,
						version: plugin_builder.version(),
						author: plugin_builder.author(),
						tasks,
						commands,
					};

					run_plugin_init(plugin_name, plugin_builder.init()).await?;
					PLUGIN_STORE.insert_plugin(plugin_info);
				}
			}
			// 静态插件
			PluginType::Builder(plugin_builder) => {
				let plugins = PLUGIN_STORE.get_all_plugins();
				let plugin_name = plugin_builder.name();
				if plugins.iter().any(|(_, plugin)| plugin.name == plugin_name) {
					return Err(crate::error::Plugin::Exists(plugin_name.to_string()));
				}
				let abi_version = plugin_builder.abi_version();

				check_plugin_abi_version(abi_version, plugin_name);

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
				let plugin_info = Plugin {
					name: plugin_name,
					version: plugin_builder.version(),
					author: plugin_builder.author(),
					tasks,
					commands,
				};

				debug!(
					"[{}:{}] 正在加载插件",
					"plugin".fg_rgb::<175, 238, 238>(),
					plugin_name.fg_rgb::<240, 128, 128>()
				);
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

	pub async fn unload_plugin(plugin: impl Into<PluginId>) -> Result<(), Error> {
		let plugin_id = plugin.into();
		PLUGIN_STORE.remove_plugin(plugin_id).await;
		Ok(())
	}

	#[inline]
	pub fn get_plugin(plugin: impl Into<PluginId>) -> Option<PluginInfo> {
		let plugin_id = plugin.into();
		match plugin_id {
			PluginId::Index(index) => PLUGIN_STORE.get_plugin_with_index(index).map(|p| p.into()),
			PluginId::Name(name) => {
				PLUGIN_STORE.get_plugin_with_name(name.as_str()).map(|p| p.into())
			}
		}
	}
}

fn check_plugin_abi_version(abi_version: &str, plugin_name: &str) {
	if abi_version != VERSION {
		warn!(
			"[{}:{}] 插件ABI版本: {}, 当前ABI版本: {}",
			"plugin".fg_rgb::<175, 238, 238>(),
			plugin_name.fg_rgb::<240, 128, 128>(),
			abi_version,
			VERSION
		)
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
