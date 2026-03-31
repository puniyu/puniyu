pub use puniyu_plugin_core::PluginId;
use puniyu_version::Version;

#[derive(Debug, Clone, PartialEq)]
pub struct PluginInfo<'p> {
	pub name: &'p str,
	pub author: Vec<&'p str>,
	pub description: Option<&'p str>,
	pub version: Version,
}

pub fn get_plugin<'p>(plugin: impl Into<PluginId<'p>>) -> Option<PluginInfo<'p>> {
	use puniyu_plugin_core::PluginRegistry;
	let plugin_id = plugin.into();
	let plugin = match plugin_id {
		PluginId::Index(index) => PluginRegistry::get_with_index(index),
		PluginId::Name(name) => PluginRegistry::get_with_plugin_name(name.as_ref()),
	};
	plugin.map(|plugin| PluginInfo {
		name: plugin.name(),
		author: plugin.author(),
		description: plugin.description(),
		version: plugin.version(),
	})
}
