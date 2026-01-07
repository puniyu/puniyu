use puniyu_registry::plugin::{PluginInfo, PluginRegistry};
pub use puniyu_registry::plugin::PluginId;

pub fn get_plugin_info(plugin: impl Into<PluginId>) -> Option<PluginInfo> {
	let plugin_id = plugin.into();
	PluginRegistry::get_plugin(plugin_id)
}
