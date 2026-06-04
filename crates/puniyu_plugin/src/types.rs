use puniyu_core::Version;
pub use puniyu_core::adapter::types::*;
pub use puniyu_core::plugin::{PluginId, PluginRegistry};
pub use puniyu_core::plugin::Plugin;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PluginInfo {
	pub name: SmolStr,
	pub author: Vec<SmolStr>,
	pub description: Option<SmolStr>,
	pub version: Version,
}
pub fn get_plugin<'a>(plugin: impl Into<PluginId<'a>>) -> Option<PluginInfo> {
	let plugin_id = plugin.into();
	let plugin = match plugin_id {
		PluginId::Index(index) => PluginRegistry::get_with_index(index),
		PluginId::Name(name) => PluginRegistry::get_with_plugin_name(name.as_ref()),
	};
	plugin.map(|plugin| PluginInfo {
		name: plugin.name().into(),
		author: plugin.author().into_iter().map(Into::into).collect(),
		description: plugin.description().map(Into::into),
		version: plugin.version(),
	})
}
