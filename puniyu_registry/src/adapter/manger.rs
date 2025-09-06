use crate::{error::Adapter as Error, library::AdapterLibrary, utils::execute_tasks};
use hashbrown::HashMap;
use libloading::Symbol;
use puniyu_utils::adapter::{Adapter, AdapterBase as AdapterBuilder};
use std::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    sync::{Arc, LazyLock, Mutex, OnceLock},
};

static LIBRARY: OnceLock<Mutex<AdapterLibrary>> = OnceLock::new();

type AdapterFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
static ADAPTER_INIT: LazyLock<Mutex<HashMap<String, AdapterFuture>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static ADAPTER_COUNTER: AtomicU64 = AtomicU64::new(0);
pub enum AdapterType {
    Dynamic(&'static str),
    Static(&'static dyn AdapterBuilder),
}

pub trait AddAdapter {
    fn register(&self, manager: &mut AdapterManger) -> Result<(), Error>;
}

impl AddAdapter for AdapterType {
    fn register(&self, manager: &mut AdapterManger) -> Result<(), Error> {
        match self {
            AdapterType::Dynamic(name) => {
                let client = LIBRARY.get_or_init(|| Mutex::new(AdapterLibrary::new()));
                let lib = {
                    let mut library = client.lock().unwrap();
                    library.load_adapter(name).unwrap();
                    library.get(name).unwrap().clone()
                };
                unsafe {
                    let symbol: Symbol<unsafe extern "C" fn() -> *mut dyn AdapterBuilder> =
                        lib.get(b"adapter_info").unwrap();
                    let adapter_builder = &*symbol();
                    let adapters = manager.store.get_all_adapters();
                    let adapter_name = adapter_builder.adapter().name;
                    if adapters.contains_key(adapter_name) {
                        return Err(Error::Exists(adapter_name.to_string()));
                    }
                    if !adapter_name.starts_with("puniyu_adapter_") {
                        // 适配器名称必须以 `puniyu_adapter_` 开头
                        return Ok(());
                    }

                    let adapter = Adapter {
                        index: ADAPTER_COUNTER.fetch_add(1, Ordering::Relaxed),
                        adapter: adapter_builder.adapter(),
                        account: adapter_builder.account(),
                        self_id: adapter_builder.self_id(),
                        self_name: adapter_builder.self_name(),
                        api: adapter_builder.api(),
                    };
                    manager.store.insert_adapter(adapter_name, adapter);
                    ADAPTER_INIT
                        .lock()
                        .unwrap()
                        .insert(adapter_name.to_string(), adapter_builder.init());
                }
                Ok(())
            }
            AdapterType::Static(adapter_builder) => {
                let adapter_info = adapter_builder.adapter();
                let adapter_name = adapter_info.name;
                let adapters = manager.store.get_all_adapters();
                if adapters.contains_key(adapter_name) {
                    return Err(Error::Exists(adapter_name.to_string()));
                }

                let adapter = Adapter {
                    index: ADAPTER_COUNTER.fetch_add(1, Ordering::Relaxed),
                    adapter: adapter_info,
                    account: adapter_builder.account(),
                    self_id: adapter_builder.self_id(),
                    self_name: adapter_builder.self_name(),
                    api: adapter_builder.api(),
                };

                manager.store.insert_adapter(adapter_name, adapter);

                ADAPTER_INIT
                    .lock()
                    .unwrap()
                    .insert(adapter_name.to_string(), adapter_builder.init());

                Ok(())
            }
        }
    }
}

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
pub struct AdapterManger {
    store: AdapterStore,
}

impl AdapterManger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_adapter<T>(&mut self, adapter: T) -> Result<&mut Self, Error>
    where
        T: AddAdapter,
    {
        adapter.register(self)?;
        Ok(self)
    }

    pub fn register_adapters<T>(&mut self, adapters: Vec<T>) -> Result<&mut Self, Error>
    where
        T: AddAdapter,
    {
        for adapter in adapters {
            self.register_adapter(adapter)?;
        }
        Ok(self)
    }

    pub fn build(&self) -> Builder {
        Builder
    }
}

pub struct Builder;

impl Builder {
    pub async fn load_adapter(&self, name: &str) {
        let plugin_future = {
            let mut guard = ADAPTER_INIT.lock().unwrap();
            guard.remove(name)
        };
        tracing::debug!("[adapter:{}] 开始加载适配器", name);
        match plugin_future {
            Some(future) => {
                future.await;
                tracing::debug!("[adapter:{}] 适配器加载成功", name);
            }
            None => {
                tracing::error!("[adapter:{}] 适配器加载失败", name);
            }
        }
    }

    pub async fn load_adapters(&self) {
        let init_futures: Vec<(String, AdapterFuture)> = {
            let mut guard = ADAPTER_INIT.lock().unwrap();
            std::mem::take(&mut *guard).into_iter().collect()
        };

        if init_futures.is_empty() {
            return;
        }

        let adapter_names: Vec<String> =
            init_futures.iter().map(|(name, _)| name.clone()).collect();
        for name in &adapter_names {
            tracing::debug!("[adapter:{}] 开始加载适配器", name);
        }

        let results = execute_tasks(init_futures).await;

        for (i, result) in results.into_iter().enumerate() {
            let adapter_name = &adapter_names[i];
            match result {
                Ok(_) => {
                    tracing::debug!("[adapter:{}] 适配器加载成功", adapter_name);
                }
                Err(_) => {
                    tracing::error!("[adapter:{}] 适配器加载失败", adapter_name);
                }
            }
        }
    }
}
