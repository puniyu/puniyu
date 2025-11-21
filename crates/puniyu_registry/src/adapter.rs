mod error;
mod store;

use crate::server::ServerRegistry;
pub use error::Error;
use puniyu_common::path::ADAPTER_DATA_DIR;
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
		debug!(
			"[{}:{}] 正在加载适配器",
			"adapter".fg_rgb::<175, 238, 238>(),
			adapter_name.fg_rgb::<240, 128, 128>()
		);
		create_data_dir(adapter_name.as_str()).await;
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

async fn create_data_dir(name: &str) {
	let data_dir = ADAPTER_DATA_DIR.as_path();
	let adapter_data_dir = data_dir.join(name);
	if !adapter_data_dir.exists() {
		let _ = fs::create_dir_all(&adapter_data_dir).await;
	}
	for subdir in ["data", "config", "resource"] {
		let path = adapter_data_dir.join(subdir);
		if !path.exists() {
			let _ = fs::create_dir_all(&path).await;
		}
	}
}
