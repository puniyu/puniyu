pub use puniyu_plugin::PluginId;
use puniyu_plugin::PluginInfo;

pub fn get_plugin<'p>(plugin: impl Into<PluginId<'p>>) -> Option<PluginInfo<'p>> {
	use puniyu_plugin::PluginRegistry;
	let plugin_id = plugin.into();
	match plugin_id {
		PluginId::Index( index) => PluginRegistry::get_with_index( index),
		PluginId::Name( name) => PluginRegistry::get_with_plugin_name(name)
	}
}
