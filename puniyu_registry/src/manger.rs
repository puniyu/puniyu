use crate::error::Plugin as Error;
use crate::plugin::{builder::PluginBuilder, library::PluginLibrary, task::builder::TaskBuilder};
use hashbrown::HashMap;
use libloading::Symbol;
use std::sync::{Arc, Mutex, OnceLock};
use tokio::runtime::Handle;

static LIBRARY: OnceLock<Mutex<PluginLibrary>> = OnceLock::new();

pub trait PluginSource {
    fn load_plugin(self, manager: &mut PluginManager) -> Result<(), Error>;
}

impl PluginSource for &'static dyn PluginBuilder {
    fn load_plugin(self, manager: &mut PluginManager) -> Result<(), Error> {
        let plugins = manager.store.get_all_plugins();
        if plugins.contains_key(&self.name().to_string()) {
            return Err(Error::Exists(self.name().to_string()));
        }
        let plugin_info = PluginInfo {
            name: self.name(),
            version: self.version(),
            rustc_version: self.rustc_version(),
            author: self.author(),
        };
        let tasks: Vec<TaskInfo> = self
            .tasks()
            .into_iter()
            .map(|task_builder| task_builder.into_task_info())
            .collect();
        let plugin = Plugin {
            info: plugin_info,
            tasks,
        };
        manager.store.insert_plugin(self.name().to_string(), plugin);
        Handle::current().spawn(async move {
            self.init().await;
        });
        Ok(())
    }
}

impl PluginSource for &'static str {
    fn load_plugin(self, manager: &mut PluginManager) -> Result<(), Error> {
        let client = LIBRARY.get_or_init(|| Mutex::new(PluginLibrary::new()));
        let lib = {
            let mut library = client.lock().unwrap();
            library.load_library(self).unwrap();
            let lib = library.get(self).unwrap().clone();
            lib
        };
        unsafe {
            let symbol: Symbol<unsafe extern "C" fn() -> *mut dyn PluginBuilder> =
                lib.get(b"plugin_info").unwrap();
            let plugin = &*symbol();
            let plugins = manager.store.get_all_plugins();
            if plugins.contains_key(&plugin.name().to_string()) {
                return Err(Error::Exists(plugin.name().to_string()));
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
            Handle::current().spawn(async move {
                plugin.init().await;
            });
        }

        Ok(())
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
        Self {
            plugins: Arc::new(Mutex::new(HashMap::new())),
        }
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

    pub fn load_plugin(&mut self, plugin: impl PluginSource) -> Result<&mut Self, Error> {
        plugin.load_plugin(self)?;
        Ok(self)
    }

    pub fn load_plugins(&mut self, plugins: Vec<impl PluginSource>) -> Result<&mut Self, Error> {
        for plugin in plugins {
            self.load_plugin(plugin)?;
        }
        Ok(self)
    }
}
