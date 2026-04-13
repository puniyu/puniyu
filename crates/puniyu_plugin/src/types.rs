pub use puniyu_plugin_core::PluginId;
use puniyu_version::Version;

#[derive(Debug, Clone, PartialEq)]
pub struct PluginInfo {
	pub name: String,
	pub author: Vec<String>,
	pub description: Option<String>,
	pub version: Version,
}

pub fn get_plugin<'p>(plugin: impl Into<PluginId<'p>>) -> Option<PluginInfo> {
	use puniyu_plugin_core::PluginRegistry;
	let plugin_id = plugin.into();
	let plugin = match plugin_id {
		PluginId::Index(index) => PluginRegistry::get_with_index(index),
		PluginId::Name(name) => PluginRegistry::get_with_plugin_name(name.as_ref()),
	};
	plugin.map(|plugin| PluginInfo {
		name: plugin.name().to_string(),
		author: plugin.author().into_iter().map(|s|s.to_string()).collect(),
		description: plugin.description().map(|s|s.to_string()),
		version: plugin.version(),
	})
}
