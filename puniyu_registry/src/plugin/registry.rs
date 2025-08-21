use crate::plugin::{command::PluginCommand, info::PluginInfo};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, LazyLock};

pub type Func = fn() -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'static>>;

#[derive(Clone)]
pub struct PluginRegistry {
    pub id: u64,
    pub info: Arc<dyn PluginInfo + Send + Sync + 'static>,
}

inventory::collect!(PluginRegistry);

/// 注册插件的宏
#[macro_export]
macro_rules! register_plugin {
    ($plugin:ty) => {
        const _: () = {
            inventory::submit! {
                $crate::plugin::PluginRegistry {
                    id: 0,
                    info: std::sync::Arc::new($plugin),
                    command: Vec::new(),
                }
            }
        };
    };
}

pub struct CommandRegistry {
    pub plugin_name: &'static str,
    pub command_name: &'static str,
    pub priority: u64,
    pub func: Func,
}
