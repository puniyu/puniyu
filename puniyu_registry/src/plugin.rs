use crate::PluginManager;
use crate::plugin::{
    command::Command, manger::PluginInfo, manger::PluginType, task::Task, task::manger::TaskManager,
};
use puniyu_utils::path::PLUGIN_DIR;

pub mod builder;
pub mod command;
pub mod manger;
pub mod registry;
pub mod task;

#[derive(Debug, Clone)]
pub struct Plugin {
    pub info: PluginInfo,
    pub tasks: Vec<Task>,
    pub commands: Vec<Command>,
}

pub fn init_plugin() {
    if !PLUGIN_DIR.as_path().exists() {
        std::fs::create_dir_all(PLUGIN_DIR.as_path()).unwrap();
    }

    tokio::spawn(async {
        // 获取插件名称列表
        let plugins: Vec<String> = std::fs::read_dir(PLUGIN_DIR.as_path())
            .into_iter()
            .flatten()
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.file_name().to_str().map(|s| s.to_string()))
            .filter(|name| name.starts_with("puniyu_plugin_"))
            .filter_map(|name| name.split('.').next().map(|s| s.to_string()))
            .collect();

        let mut manager = PluginManager::new();
        let plugin_types: Vec<PluginType> = plugins
            .iter()
            .map(|plugin_name| PluginType::Dynamic(Box::leak(plugin_name.clone().into_boxed_str())))
            .collect();
        // 添加到插件注册表
        manager.add_plugins(plugin_types).unwrap();
        let mut builder = manager.build();
        // 加载全部插件
        builder.load_plugins().await;
        // 初始化定时任务系统
        TaskManager::init_scheduler().await
    });
}
