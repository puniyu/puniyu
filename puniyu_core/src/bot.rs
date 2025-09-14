use crate::config::{config_watcher, init_config};
use crate::logger::log_init;
use puniyu_registry::plugin::init_plugin;

pub struct Bot;

impl Default for Bot {
    fn default() -> Self {
        Self::new()
    }
}

impl Bot {
    /// TODO: 插件绑定， 适配器绑定
    pub fn new() -> Self {
        Self
    }
    pub async fn run(&self) {
        init_config();
        log_init();
        config_watcher();
        // 初始化插件系统
        init_plugin();
        use tokio::signal;

        signal::ctrl_c().await.unwrap();
    }
}
