use super::{Plugin, PluginFuture};
use crate::plugin::command::builder::CommandBuilder;
use crate::plugin::task::builder::TaskBuilder;
use crate::{
    VERSION, error::Plugin as Error, library::PluginLibrary, logger::SharedLogger,
    plugin::builder::PluginBuilder, plugin::command::Command, plugin::task::Task,
    plugin::task::manger::TaskManager, utils::execute_tasks,
};
use hashbrown::HashMap;
use libloading::Symbol;
use std::{
    pin::Pin,
    sync::{Arc, LazyLock, Mutex, OnceLock},
};

static LIBRARY: OnceLock<Mutex<PluginLibrary>> = OnceLock::new();

static PLUGIN_INIT: LazyLock<Mutex<HashMap<String, PluginFuture>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// 定义插件类型枚举
pub enum PluginType {
    /// 基于文件路径加载的动态库插件
    Path(String),
    /// 静态链接的插件
    Builder(&'static dyn PluginBuilder),
}
impl From<&str> for PluginType {
    fn from(path: &str) -> Self {
        PluginType::Path(path.to_string())
    }
}

impl From<String> for PluginType {
    fn from(path: String) -> Self {
        PluginType::Path(path)
    }
}

impl From<&'static dyn PluginBuilder> for PluginType {
    fn from(builder: &'static dyn PluginBuilder) -> Self {
        PluginType::Builder(builder)
    }
}

/// 收集任务
fn collect_tasks(tasks: Vec<Box<dyn TaskBuilder>>) -> Vec<Task> {
    tasks
        .into_iter()
        .map(|task_builder| Task {
            name: task_builder.name(),
            cron: task_builder.cron(),
        })
        .collect()
}

/// 收集命令
fn collect_commands(commands: Vec<Box<dyn CommandBuilder>>) -> Vec<Command> {
    commands
        .into_iter()
        .map(|command_builder| Command {
            name: command_builder.name(),
            command: command_builder.command(),
            rank: command_builder.rank(),
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub abi_version: &'static str,
    pub author: &'static str,
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

    pub fn add_plugin<T: Into<PluginType>>(&mut self, plugin: T) -> Result<&mut Self, Error> {
        let plugin_id: PluginType = plugin.into();
        match plugin_id {
            // 动态链接插件
            PluginType::Path(path) => {
                let client = LIBRARY.get_or_init(|| Mutex::new(PluginLibrary::new()));
                let lib = {
                    let mut library = client.lock().unwrap();
                    library.load_plugin(&path).unwrap();
                    library.get_plugin(&path).unwrap().clone()
                };
                unsafe {
                    let symbol: Symbol<unsafe extern "C" fn() -> *mut dyn PluginBuilder> =
                        lib.get(b"plugin_info").unwrap();
                    let plugin_builder = &*symbol();
                    let set_logger: fn(&SharedLogger) = *lib.get(b"setup_logger").unwrap();
                    set_logger(&SharedLogger::new());
                    let plugins = self.store.get_all_plugins();
                    if plugins.contains_key(&plugin_builder.name().to_string()) {
                        return Err(Error::Exists(plugin_builder.name().to_string()));
                    }
                    let plugin_name = plugin_builder.name();
                    if !plugin_name.starts_with("puniyu_plugin_") {
                        // 插件名称必须以 `puniyu_plugin_` 开头
                        return Ok(self);
                    }
                    let abi_version = plugin_builder.abi_version();
                    let plugin_info = PluginInfo {
                        name: plugin_name,
                        version: plugin_builder.version(),
                        abi_version,
                        author: plugin_builder.author(),
                    };
                    if abi_version != VERSION {
                        log::warn!(
                            "插件 {} 版本 {} 不兼容当前ABI版本 {}，请联系开发者更新ABI版本",
                            plugin_name,
                            abi_version,
                            VERSION
                        );
                        return Ok(self);
                    }
                    let tasks = collect_tasks(plugin_builder.tasks());
                    for task_builder in plugin_builder.tasks() {
                        TaskManager::register_task(task_builder);
                    }

                    let commands = collect_commands(plugin_builder.commands());
                    let plugin_obj = Plugin {
                        info: plugin_info,
                        tasks,
                        commands,
                    };

                    self.store
                        .insert_plugin(plugin_obj.info.name.to_string(), plugin_obj);
                    PLUGIN_INIT
                        .lock()
                        .unwrap()
                        .insert(plugin_name.to_string(), plugin_builder.init());
                }
            }
            // 静态插件
            PluginType::Builder(plugin_builder) => {
                let plugins = self.store.get_all_plugins();
                if plugins.contains_key(&plugin_builder.name().to_string()) {
                    return Err(Error::Exists(plugin_builder.name().to_string()));
                }
                let plugin_name = plugin_builder.name();
                if !plugin_name.starts_with("puniyu_plugin_") {
                    // 插件名称必须以 `puniyu_plugin_` 开头
                    return Ok(self);
                }
                let abi_version = plugin_builder.abi_version();
                let plugin_info = PluginInfo {
                    name: plugin_name,
                    version: plugin_builder.version(),
                    abi_version,
                    author: plugin_builder.author(),
                };

                if abi_version != VERSION {
                    log::warn!(
                        "插件 {} 版本 {} 不兼容当前版本 {}，请升级插件",
                        plugin_name,
                        abi_version,
                        VERSION
                    );
                    return Ok(self);
                }
                let tasks = collect_tasks(plugin_builder.tasks());

                for task_builder in plugin_builder.tasks() {
                    TaskManager::register_task(task_builder);
                }

                let commands = collect_commands(plugin_builder.commands());
                let plugin = Plugin {
                    info: plugin_info,
                    tasks,
                    commands,
                };
                self.store
                    .insert_plugin(plugin_builder.name().to_string(), plugin);
                PLUGIN_INIT
                    .lock()
                    .unwrap()
                    .insert(plugin_name.to_string(), plugin_builder.init());
            }
        }
        Ok(self)
    }

    pub fn add_plugins<T: Into<PluginType>>(
        &mut self,
        plugins: Vec<T>,
    ) -> Result<&mut Self, Error> {
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
        log::debug!("[plugin:{}] 开始加载插件", plugin_name);
        match plugin_future {
            Some(future) => {
                future.await;
                log::debug!("[plugin:{}] 插件加载成功", plugin_name);
            }
            None => {
                log::error!("[plugin:{}] 插件加载失败", plugin_name);
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
            log::debug!("[plugin:{}] 开始加载插件", plugin_name);
        }

        let results = execute_tasks(init_futures).await;

        for (i, result) in results.into_iter().enumerate() {
            let plugin_name = &plugin_names[i];
            match result {
                Ok(_) => {
                    log::debug!("[plugin:{}] 插件加载成功", plugin_name);
                }
                Err(_) => {
                    log::error!("[plugin:{}] 插件加载失败", plugin_name);
                }
            }
        }
    }
}
