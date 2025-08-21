use crate::plugin::PluginRegistry;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

static PLUGIN_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// 插件管理器
pub struct PluginManager {
    plugins: HashMap<u64, PluginRegistry>,
}

impl PluginManager {
    /// 创建一个新的插件管理器
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }
}
