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

pub struct CommandRegistry {
    pub name: &'static str,
    pub priority: u64,
    pub func: Func,
}
