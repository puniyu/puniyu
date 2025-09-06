use crate::logger::SharedLogger;
use crate::{
    error::Plugin as Error, library::PluginLibrary, plugin::builder::PluginBuilder,
    plugin::task::builder::TaskBuilder, utils::execute_tasks,
};
use hashbrown::HashMap;
use libloading::Symbol;
use std::{
    pin::Pin,
    sync::{Arc, LazyLock, Mutex, OnceLock},
};

static LIBRARY: OnceLock<Mutex<PluginLibrary>> = OnceLock::new();

type PluginFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
static PLUGIN_INIT: LazyLock<Mutex<HashMap<String, PluginFuture>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// 定义插件类型枚举
pub enum PluginType {
    /// 基于文件路径加载的动态库插件
    Dynamic(&'static str),
    /// 静态链接的插件
    Static(&'static dyn PluginBuilder),
}

pub trait AddPlugin {
    fn add_plugin(&self, manager: &mut PluginManager) -> Result<(), Error>;
}

impl AddPlugin for PluginType {
    fn add_plugin(&self, manager: &mut PluginManager) -> Result<(), Error> {
        match self {
            PluginType::Dynamic(name) => {
                let client = LIBRARY.get_or_init(|| Mutex::new(PluginLibrary::new()));
                let lib = {
                    let mut library = client.lock().unwrap();
                    library.load_plugin(name).unwrap();
                    library.get_plugin(name).unwrap().clone()
                };
                unsafe {
                    let symbol: Symbol<unsafe extern "C" fn() -> *mut dyn PluginBuilder> =
                        lib.get(b"plugin_info").unwrap();
                    let plugin = &*symbol();
                    let set_logger: fn(&SharedLogger) = *lib.get(b"setup_logger").unwrap();
                    set_logger(&SharedLogger::new());
                    let plugins = manager.store.get_all_plugins();
                    if plugins.contains_key(&plugin.name().to_string()) {
                        return Err(Error::Exists(plugin.name().to_string()));
                    }
                    if !plugin.name().starts_with("puniyu_plugin_") {
                        // 插件名称必须以 `puniyu_plugin_` 开头
                        return Ok(());
                    }
                    let plugin_info = PluginInfo {
                        name: plugin.name(),
                        version: plugin.version(),
                        rustc_version: plugin.rustc_version(),
                        author: plugin.author(),
                    };
                    let tasks: Vec<TaskInfo> = plugin
                        .tasks()
                        .into_iter()
                        .map(|task_builder| task_builder.into_task_info())
                        .collect();
                    let plugin_obj = Plugin {
                        info: plugin_info,
                        tasks,
                    };
                    manager
                        .store
                        .insert_plugin(plugin_obj.info.name.to_string(), plugin_obj);
                    PLUGIN_INIT
                        .lock()
                        .unwrap()
                        .insert(plugin.name().to_string(), plugin.init());
                }

                Ok(())
            }
            PluginType::Static(plugin_builder) => {
                let plugins = manager.store.get_all_plugins();
                if plugins.contains_key(&plugin_builder.name().to_string()) {
                    return Err(Error::Exists(plugin_builder.name().to_string()));
                }
                if !plugin_builder.name().starts_with("puniyu_plugin_") {
                    // 插件名称必须以 `puniyu_plugin_` 开头
                    return Ok(());
                }
                let plugin_info = PluginInfo {
                    name: plugin_builder.name(),
                    version: plugin_builder.version(),
                    rustc_version: plugin_builder.rustc_version(),
                    author: plugin_builder.author(),
                };
                let tasks: Vec<TaskInfo> = plugin_builder
                    .tasks()
                    .into_iter()
                    .map(|task_builder| task_builder.into_task_info())
                    .collect();
                let plugin = Plugin {
                    info: plugin_info,
                    tasks,
                };
                manager
                    .store
                    .insert_plugin(plugin_builder.name().to_string(), plugin);
                PLUGIN_INIT
                    .lock()
                    .unwrap()
                    .insert(plugin_builder.name().to_string(), plugin_builder.init());
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub rustc_version: &'static str,
    pub author: &'static str,
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub name: &'static str,
    pub cron: &'static str,
}

pub trait IntoTaskInfo {
    fn into_task_info(self) -> TaskInfo;
}

impl IntoTaskInfo for Box<dyn TaskBuilder> {
    fn into_task_info(self) -> TaskInfo {
        TaskInfo {
            name: self.name(),
            cron: self.cron(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Plugin {
    pub info: PluginInfo,
    pub tasks: Vec<TaskInfo>,
}

#[derive(Debug, Default, Clone)]
/// 插件存储器
pub struct PluginStore {
    plugins: Arc<Mutex<HashMap<String, Plugin>>>,
}

impl PluginStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_plugin(&self, name: String, plugin: Plugin) {
        let mut plugins = self.plugins.lock().unwrap();
        plugins.insert(name, plugin);
    }

    pub fn get_plugins(&self) -> Arc<Mutex<HashMap<String, Plugin>>> {
        self.plugins.clone()
    }

    pub fn get_all_plugins(&self) -> HashMap<String, Plugin> {
        let plugins = self.plugins.lock().unwrap();
        plugins.clone()
    }
}

#[derive(Default)]
pub struct PluginManager {
    store: PluginStore,
}

impl PluginManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_plugin<T>(&mut self, plugin: T) -> Result<&mut Self, Error>
    where
        T: AddPlugin,
    {
        plugin.add_plugin(self)?;
        Ok(self)
    }

    pub fn add_plugins<T>(&mut self, plugins: Vec<T>) -> Result<&mut Self, Error>
    where
        T: AddPlugin,
    {
        for plugin in plugins {
            self.add_plugin(plugin)?;
        }
        Ok(self)
    }

    pub fn build(self) -> Builder {
        Builder
    }
}

#[derive(Debug)]
pub struct Builder;

impl Builder {
    pub async fn load_plugin(&mut self, plugin_name: &str) {
        let plugin_future = {
            let mut guard = PLUGIN_INIT.lock().unwrap();
            guard.remove(plugin_name)
        };
        tracing::debug!("[plugin:{}] 开始加载插件", plugin_name);
        match plugin_future {
            Some(future) => {
                future.await;
                tracing::debug!("[plugin:{}] 插件加载成功", plugin_name);
            }
            None => {
                tracing::error!("[plugin:{}] 插件加载失败", plugin_name);
            }
        }
    }

    pub async fn load_plugins(&mut self) {
        let init_futures: Vec<(String, PluginFuture)> = {
            let mut guard = PLUGIN_INIT.lock().unwrap();
            std::mem::take(&mut *guard).into_iter().collect()
        };

        if init_futures.is_empty() {
            return;
        }

        let plugin_names: Vec<String> = init_futures.iter().map(|(name, _)| name.clone()).collect();
        for plugin_name in &plugin_names {
            tracing::debug!("[plugin:{}] 开始加载插件", plugin_name);
        }

        let results = execute_tasks(init_futures).await;

        for (i, result) in results.into_iter().enumerate() {
            let plugin_name = &plugin_names[i];
            match result {
                Ok(_) => {
                    tracing::debug!("[plugin:{}] 插件加载成功", plugin_name);
                }
                Err(_) => {
                    tracing::error!("[plugin:{}] 插件加载失败", plugin_name);
                }
            }
        }
    }
}
