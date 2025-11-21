use puniyu_registry::plugin::PluginRegistry;
use puniyu_types::plugin::{Plugin, PluginId};

pub fn get_plugin_info(plugin: impl Into<PluginId>) -> Option<Plugin> {
	let plugin_id = plugin.into();
	PluginRegistry::get_plugin(plugin_id)
}
