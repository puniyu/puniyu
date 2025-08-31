pub mod builder;
pub mod library;
pub mod registry;
pub mod task;

use crate::plugin::{builder::PluginBuilder, task::builder::TaskBuilder as TaskInfo};
use hashbrown::HashMap;
use tokio::runtime::Handle;

/// 插件元数据
pub struct Plugin {
    pub info: Box<dyn PluginBuilder>,
    pub tasks: Vec<Box<dyn TaskInfo>>,
}

pub struct PluginManager {
    plugins: HashMap<u64, Plugin>,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: HashMap::new(),
        }
    }

    pub fn add_plugin<P>(mut self, plugin: P) -> Self
    where
        P: PluginBuilder + 'static,
    {
        let plugin_name = plugin.name();

        // let tasks: Vec<Box<dyn TaskInfo>> = inventory::iter::<TaskRegistry>
        //     .into_iter()
        //     .filter(|task| task.plugin_name == plugin_name)
        //     .map(|task| (task.builder)())
        //     .collect();

        let plugin_id = self.plugins.len() as u64;

        self.plugins.insert(
            plugin_id,
            Plugin {
                info: Box::new(plugin),
                tasks: Vec::new(),
            },
        );
        self
    }

    pub fn build(self) -> Builder {
        Builder {
            plugins: self.plugins,
        }
    }
}

pub struct Builder {
    plugins: HashMap<u64, Plugin>,
}

impl Builder {
    pub fn load_plugins(&self) {
        let handle = Handle::current();
        for plugin in self.plugins.values() {
            handle.spawn(plugin.info.init());
        }
    }
}
