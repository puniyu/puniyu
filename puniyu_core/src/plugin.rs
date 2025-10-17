use puniyu_plugin_builder::{Plugin, PluginId};
use puniyu_plugin_registry::PluginRegistry;

pub fn get_plugin_info(plugin: impl Into<PluginId>) -> Option<Plugin> {
	let plugin_id = plugin.into();
	PluginRegistry::get_plugin(plugin_id)
}
