use puniyu_registry::PluginManager;
use puniyu_registry::plugin::{PluginId, manger::PluginInfo};

pub fn get_plugin_info<T: Into<PluginId>>(plugin: T) -> Option<PluginInfo> {
	let plugin_id = plugin.into();
	PluginManager::get_plugin(plugin_id)
}
