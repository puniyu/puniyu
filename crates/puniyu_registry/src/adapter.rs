mod store;

use crate::adapter::store::AdapterStore;
use crate::error::Adapter as Error;
use puniyu_builder::adapter::{Adapter, AdapterBuilder, AdapterType, VERSION as ABI_VERSION};
use puniyu_common::APP_NAME;
use puniyu_event_bus::{EVENT_BUS, EventBus};
use puniyu_library::AdapterLibrary;
use puniyu_library::libloading::Symbol;
use puniyu_logger::{debug, error, owo_colors::OwoColorize, warn};
use std::sync::{
	Arc, LazyLock, Mutex, OnceLock,
	atomic::{AtomicU64, Ordering},
};

static LIBRARY: OnceLock<Mutex<AdapterLibrary>> = OnceLock::new();

static ADAPTER_COUNTER: AtomicU64 = AtomicU64::new(0);
static ADAPTER_STORE: LazyLock<AdapterStore> = LazyLock::new(AdapterStore::new);
pub struct AdapterRegistry;

impl AdapterRegistry {
	pub async fn load_adapter(adapter: impl Into<AdapterType>) -> Result<(), Error> {
		let adapter = adapter.into();
		match adapter {
			AdapterType::Path(path) => {
				let client = LIBRARY.get_or_init(|| Mutex::new(AdapterLibrary::new()));
				let lib = {
					let mut library = client.lock().unwrap();
					library.load_adapter(&path).unwrap();
					let name = path
						.file_name()
						.map(|n| n.to_string_lossy().to_string())
						.ok_or_else(|| Error::NotFound(path.to_string_lossy().to_string()))?;
					library.get_adapter(&name).unwrap().library.clone()
				};
				unsafe {
					let symbol: Symbol<unsafe extern "C" fn() -> *mut dyn AdapterBuilder> =
						lib.get(b"get_adapter_info").unwrap();
					let adapter_builder = &*symbol();
					let setup_event_bus: fn(bus: Arc<Mutex<EventBus>>) =
						*lib.get(b"setup_event_bus").unwrap();
					let event_bus = EVENT_BUS.get().unwrap().clone();
					setup_event_bus(event_bus);
					let setup_app_name: fn(name: String) = *lib.get(b"setup_app_name").unwrap();
					setup_app_name(APP_NAME.get().unwrap().to_string());
					let adapters = ADAPTER_STORE.get_all_adapters();
					let adapter_name = adapter_builder.info().name;
					if adapters.contains_key(adapter_name.as_str()) {
						return Err(Error::Exists(adapter_name));
					}

					let adapter = Adapter {
						index: ADAPTER_COUNTER.fetch_add(1, Ordering::Relaxed),
						info: adapter_builder.info(),
						api: adapter_builder.api(),
					};
					debug!(
						"[{}:{}] 正在加载适配器",
						"adapter".fg_rgb::<175, 238, 238>(),
						adapter_name.fg_rgb::<240, 128, 128>()
					);

					let abi_version = adapter_builder.abi_version();

					check_plugin_abi_version(abi_version, adapter_name.as_str());

					run_adapter_init(adapter_name.as_str(), adapter_builder.init()).await?;
					ADAPTER_STORE.insert_adapter(adapter_name.as_str(), adapter);
					Ok(())
				}
			}
			AdapterType::Builder(builder) => {
				let adapters = ADAPTER_STORE.get_all_adapters();
				let adapter_info = builder.info();
				let adapter_name = adapter_info.name.clone();
				if adapters.contains_key(adapter_name.as_str()) {
					return Err(Error::Exists(adapter_name));
				}
				let adapter = Adapter {
					index: ADAPTER_COUNTER.fetch_add(1, Ordering::Relaxed),
					info: adapter_info,
					api: builder.api(),
				};
				debug!(
					"[{}:{}] 正在加载适配器",
					"adapter".fg_rgb::<175, 238, 238>(),
					adapter_name.fg_rgb::<240, 128, 128>()
				);

				let abi_version = builder.abi_version();

				check_plugin_abi_version(abi_version, adapter_name.as_str());

				run_adapter_init(adapter_name.as_str(), builder.init()).await?;
				ADAPTER_STORE.insert_adapter(adapter_name.as_str(), adapter);
				Ok(())
			}
		}
	}

	/// 加载多个适配器
	pub async fn load_adapters(adapters: Vec<impl Into<AdapterType>>) -> Result<(), Error> {
		futures::future::try_join_all(
			adapters.into_iter().map(|adapter| Self::load_adapter(adapter)),
		)
		.await?;
		Ok(())
	}

	/// 卸载一个适配器，包括适配器中的Bot实例
	pub fn unload_adapter(name: &str) -> Result<(), Error> {
		ADAPTER_STORE.remove_adapter(name);
		Ok(())
	}

	#[inline]
	pub fn get_adapter(name: &str) -> Option<Adapter> {
		ADAPTER_STORE.get_adapter(name)
	}

	#[inline]
	pub fn get_all_adapters() -> Vec<Adapter> {
		ADAPTER_STORE.get_all_adapters().values().cloned().collect()
	}
}

fn check_plugin_abi_version(version: &str, plugin_name: &str) {
	if version != ABI_VERSION {
		warn!(
			"[{}:{}] ABI版本不匹配, 当前ABI版本: {}, 适配器ABI版本: {}",
			"adapter".fg_rgb::<175, 238, 238>(),
			plugin_name.fg_rgb::<240, 128, 128>(),
			version,
			version
		)
	}
}

async fn run_adapter_init<F>(name: &str, init_fn: F) -> Result<(), Error>
where
	F: Future<Output = Result<(), Box<dyn std::error::Error>>>,
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
