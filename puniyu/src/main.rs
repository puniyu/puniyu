use emitter_rs::EventEmitter as Emitter;
use log::info;
use puniyu_core::config::init_config;
use puniyu_core::logger::log_init;
use puniyu_registry::plugin::manger::{PluginManager, PluginType};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    log_init();
    init_config();

    let mut manager = PluginManager::new();
    manager
        .add_plugin(PluginType::Dynamic("puniyu_plugin_test"))
        .unwrap();
    let mut manager = manager.build();

    manager.load_plugins().await;
}
