use crate::logger::debug;
use crate::{adapter::AdapterType, error::Adapter as Error, library::AdapterLibrary};
use libloading::Symbol;
use puniyu_utils::adapter::{Adapter, AdapterBase as AdapterBuilder};
use std::{
	collections::HashMap,
	sync::{
		Arc, LazyLock, Mutex, OnceLock,
		atomic::{AtomicU64, Ordering},
	},
};

static LIBRARY: OnceLock<Mutex<AdapterLibrary>> = OnceLock::new();

static ADAPTER_COUNTER: AtomicU64 = AtomicU64::new(0);
static ADAPTER_STORE: LazyLock<AdapterStore> = LazyLock::new(AdapterStore::new);

/// 适配器存储器
#[derive(Default, Clone)]
pub struct AdapterStore {
	adapters: Arc<Mutex<HashMap<String, Adapter>>>,
}

impl AdapterStore {
	pub fn new() -> Self {
		Self::default()
	}

	/// TODO: 事件完善
	pub fn insert_adapter(&self, name: &str, adapter: Adapter) {
		let mut adapters = self.adapters.lock().unwrap();
		adapters.insert(name.to_string(), adapter);
	}

	pub fn get_adapters(&self, name: &str) -> Option<Adapter> {
		let adapters = self.adapters.lock().unwrap();
		adapters.get(name).cloned()
	}

	pub fn get_all_adapters(&self) -> HashMap<String, Adapter> {
		let adapters = self.adapters.lock().unwrap();
		adapters.clone()
	}

	pub fn remove_adapter(&self, name: &str) {
		let mut adapters = self.adapters.lock().unwrap();
		adapters.remove(name);
	}
}

#[derive(Default)]
pub struct AdapterManger;

impl AdapterManger {
	pub async fn load_adapter<T: Into<AdapterType>>(adapter: T) -> Result<(), Error> {
		let adapter = adapter.into();
		match adapter {
			AdapterType::Path(name) => {
				let client = LIBRARY.get_or_init(|| Mutex::new(AdapterLibrary::new()));
				let lib = {
					let mut library = client.lock().unwrap();
					library.load_adapter(&name).unwrap();
					library.get(&name).unwrap().clone()
				};
				unsafe {
					let symbol: Symbol<unsafe extern "C" fn() -> *mut dyn AdapterBuilder> =
						lib.get(b"adapter_info").unwrap();
					let adapter_builder = &*symbol();
					let adapters = ADAPTER_STORE.get_all_adapters();
					let adapter_name = adapter_builder.adapter().name;
					if adapters.contains_key(adapter_name) {
						return Err(Error::Exists(adapter_name.to_string()));
					}

					let adapter = Adapter {
						index: ADAPTER_COUNTER.fetch_add(1, Ordering::Relaxed),
						adapter: adapter_builder.adapter(),
						account: adapter_builder.account(),
						self_id: adapter_builder.self_id(),
						self_name: adapter_builder.self_name(),
						api: adapter_builder.api(),
					};
					debug!("[adapter:{}] 正在加载适配器", adapter_name);
					adapter_builder.init().await;
					debug!("[adapter:{}] 适配器加载成功", adapter_name);
					ADAPTER_STORE.insert_adapter(adapter_name, adapter);
					Ok(())
				}
			}
			AdapterType::Builder(builder) => {
				let adapters = ADAPTER_STORE.get_all_adapters();
				let adapter_info = builder.adapter();
				let adapter_name = adapter_info.name;
				if adapters.contains_key(adapter_name) {
					return Err(Error::Exists(adapter_name.to_string()));
				}
				let adapter = Adapter {
					index: ADAPTER_COUNTER.fetch_add(1, Ordering::Relaxed),
					adapter: adapter_info,
					account: builder.account(),
					self_id: builder.self_id(),
					self_name: builder.self_name(),
					api: builder.api(),
				};
				debug!("[adapter:{}] 正在加载适配器", adapter_name);
				builder.init().await;
				debug!("[adapter:{}] 适配器加载成功", adapter_name);
				ADAPTER_STORE.insert_adapter(adapter_name, adapter);
				Ok(())
			}
		}
	}

	pub async fn load_adapters<T: Into<AdapterType>>(adapters: Vec<T>) -> Result<(), Error> {
		futures::future::try_join_all(
			adapters.into_iter().map(|adapter| Self::load_adapter(adapter)),
		)
		.await?;

		Ok(())
	}
	/// TODO: 此部分待完成
	pub async fn register_adapters<T: Into<AdapterType>>(adapters: Vec<T>) -> Result<(), Error> {
		Ok(())
	}
}
