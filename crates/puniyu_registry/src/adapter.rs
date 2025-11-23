mod error;
mod store;

use crate::server::ServerRegistry;
use convert_case::{Case, Casing};
pub use error::Error;
use puniyu_common::path::{ADAPTER_CONFIG_DIR, ADAPTER_DATA_DIR, ADAPTER_RESOURCE_DIR};
use puniyu_common::{merge_config, read_config, write_config};
use puniyu_logger::{debug, error, owo_colors::OwoColorize};
use puniyu_types::adapter::{Adapter, AdapterBuilder};
use std::sync::LazyLock;
use store::AdapterStore;
use tokio::fs;

static ADAPTER_STORE: LazyLock<AdapterStore> = LazyLock::new(AdapterStore::new);

pub struct AdapterRegistry;

impl AdapterRegistry {
	pub async fn load_adapter(adapter: &'static dyn AdapterBuilder) -> Result<(), Error> {
		let adapters = ADAPTER_STORE.get_all_adapters();
		let adapter_info = adapter.info();
		let adapter_name = adapter_info.name.clone();
		if adapters.values().any(|adapter| adapter.info.name == adapter_name) {
			return Err(Error::Exists(adapter_name));
		}

		if let Some(config) = adapter.config()
			&& let Some(first_config) = config.first()
		{
			debug!(
				"[{}:{}] 配置: {} - {}",
				"adapter".fg_rgb::<175, 238, 238>(),
				adapter_name.fg_rgb::<240, 128, 128>(),
				first_config.name(),
				first_config.config()
			);
		}
		debug!(
			"[{}:{}] 正在加载适配器",
			"adapter".fg_rgb::<175, 238, 238>(),
			adapter_name.fg_rgb::<240, 128, 128>()
		);
		let config_dir =
			ADAPTER_CONFIG_DIR.as_path().join(adapter_name.as_str().to_case(Case::Snake));

		if !config_dir.exists() {
			let _ = fs::create_dir_all(&config_dir).await;
		}

		if let Some(configs) = adapter.config() {
			configs.iter().for_each(|config| {
				let cfg = read_config::<toml::Value>(&config_dir, config.name());

				match cfg {
					Ok(cfg) => {
						merge_config(&config_dir, config.name(), &config.config(), &cfg)
							.expect("合并插件配置文件失败");
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
			});
		}
		let data_dir = ADAPTER_DATA_DIR.as_path().join(adapter_name.as_str());
		if !data_dir.exists() {
			let _ = fs::create_dir_all(&data_dir).await;
		}
		let resource_dir = ADAPTER_RESOURCE_DIR.as_path().join(adapter_name.as_str());
		if !resource_dir.exists() {
			let _ = fs::create_dir_all(&resource_dir).await;
		}
		if let Some(server) = adapter.server() {
			ServerRegistry::insert(adapter_name.clone(), server);
		}
		run_adapter_init(adapter_name.as_str(), adapter.init()).await?;
		let adapter = Adapter { info: adapter_info, api: adapter.api() };
		ADAPTER_STORE.insert_adapter(adapter);
		Ok(())
	}

	/// 加载多个适配器
	pub async fn load_adapters(adapters: Vec<&'static dyn AdapterBuilder>) -> Result<(), Error> {
		futures::future::try_join_all(adapters.into_iter().map(Self::load_adapter)).await?;
		Ok(())
	}

	/// 卸载一个适配器，包括适配器中的Bot实例
	pub fn unload_adapter(name: &str) -> Result<(), Error> {
		ADAPTER_STORE.remove_adapter(name);
		Ok(())
	}

	pub fn get_adapter(name: &str) -> Option<Adapter> {
		ADAPTER_STORE.get_adapter(name)
	}
	pub fn get_all_adapters() -> Vec<Adapter> {
		ADAPTER_STORE.get_all_adapters().values().cloned().collect()
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
