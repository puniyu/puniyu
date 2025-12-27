mod error;

use crate::server::ServerRegistry;
use crate::store::STORE;
use convert_case::{Case, Casing};
pub use error::Error;
use puniyu_common::path::{ADAPTER_CONFIG_DIR, ADAPTER_DATA_DIR, ADAPTER_RESOURCE_DIR};
use puniyu_common::{merge_config, read_config, write_config};
use puniyu_config::{Config, ConfigRegistry};
use puniyu_logger::warn;
use puniyu_logger::{debug, error, owo_colors::OwoColorize};
use puniyu_types::adapter::{Adapter, AdapterBuilder};
use tokio::fs;

pub struct AdapterRegistry;

impl AdapterRegistry {
	pub async fn load_adapter(adapter: &'static dyn AdapterBuilder) -> Result<(), Error> {
		let adapters = STORE.adapter().get_all();
		let adapter_name = adapter.name().to_string();
		let adapter_version = adapter.version().to_string();
		if adapters.values().any(|a| a.name == adapter_name) {
			return Err(Error::Exists(adapter_name));
		}

		debug!(
			"[{}:{}({})] 正在加载适配器",
			"adapter".fg_rgb::<175, 238, 238>(),
			adapter_name.fg_rgb::<240, 128, 128>(),
			format!("v {}", adapter_version).fg_rgb::<144, 238, 144>()
		);

		let adapter_enabled = match adapter_name.as_str() {
			"puniyu_adapter_console" => Config::app().adapter().console(),
			_ => false,
		};
		if !adapter_enabled {
			warn!(
				"[{}:{}] 跳过加载",
				"adapter".fg_rgb::<175, 238, 238>(),
				adapter_name.fg_rgb::<240, 128, 128>()
			);
			return Ok(());
		}
		let config_dir =
			ADAPTER_CONFIG_DIR.as_path().join(adapter_name.as_str().to_case(Case::Snake));
		let data_dir = ADAPTER_DATA_DIR.as_path().join(adapter_name.to_case(Case::Snake));
		let resource_dir = ADAPTER_RESOURCE_DIR.as_path().join(adapter_name.to_case(Case::Snake));

		let _ = tokio::join!(
			fs::create_dir_all(&config_dir),
			fs::create_dir_all(&data_dir),
			fs::create_dir_all(&resource_dir)
		);

		if let Some(configs) = adapter.config() {
			for config in configs {
				match read_config::<toml::Value>(&config_dir, config.name()) {
					Ok(cfg) => {
						merge_config(&config_dir, config.name(), &config.config(), &cfg)
							.expect("合并插件配置文件失败");
						let cfg = read_config::<toml::Value>(&config_dir, config.name()).unwrap();
						let path = config_dir.join(format!("{}.toml", config.name()));
						ConfigRegistry::register(path.as_path(), cfg);
					}
					Err(_) => {
						debug!(
							"[{}:{}] 配置文件 {} 不存在，正在创建默认配置",
							"adapter".fg_rgb::<175, 238, 238>(),
							adapter_name.fg_rgb::<240, 128, 128>(),
							config.name()
						);
						write_config(&config_dir, config.name(), &config.config())
							.expect("创建默认配置文件失败");
					}
				}
			}
		}

		if let Some(server) = adapter.server() {
			ServerRegistry::insert(adapter_name.clone(), server);
		}

		run_adapter_init(adapter_name.as_str(), adapter.init()).await?;
		STORE.adapter().insert(Adapter {
			name: adapter_name,
			version: adapter_version,
			api: adapter.api(),
		});
		Ok(())
	}

	/// 加载多个适配器
	pub async fn load_adapters(adapters: Vec<&'static dyn AdapterBuilder>) -> Result<(), Error> {
		futures::future::try_join_all(adapters.into_iter().map(Self::load_adapter)).await?;
		Ok(())
	}

	/// 卸载一个适配器，包括适配器中的Bot实例
	pub fn unload_adapter(name: &str) -> Result<(), Error> {
		STORE.adapter().remove(name);
		Ok(())
	}

	pub fn get_adapter(name: &str) -> Option<Adapter> {
		STORE.adapter().get(name)
	}
	pub fn get_all_adapters() -> Vec<Adapter> {
		STORE.adapter().get_all().values().cloned().collect()
	}
}

async fn run_adapter_init<F>(name: &str, init_fn: F) -> Result<(), Error>
where
	F: Future<Output = puniyu_types::adapter::Result<()>>,
{
	match init_fn.await {
		Ok(()) => {
			debug!(
				"[{}:{}] 适配器加载成功",
				"adapter".fg_rgb::<175, 238, 238>(),
				name.fg_rgb::<240, 128, 128>()
			);
			Ok(())
		}
		Err(e) => {
			error!(
				"[{}:{}] 适配器加载失败：{}",
				"adapter".fg_rgb::<175, 238, 238>(),
				name.fg_rgb::<240, 128, 128>(),
				e.to_string()
			);
			Err(Error::Init(e.to_string()))
		}
	}
}
