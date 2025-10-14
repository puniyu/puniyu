use puniyu_registry::PluginRegistry;
use puniyu_registry::plugin::{PluginId, registry::PluginInfo};

pub fn get_plugin_info(plugin: impl Into<PluginId>) -> Option<PluginInfo> {
	let plugin_id = plugin.into();
	PluginRegistry::get_plugin(plugin_id)
}

pub mod prelude {
	pub use puniyu_plugin_derive::{plugin, task};
	pub use puniyu_utils::adapter::{
		AccountInfo, AdapterApi, AdapterBase as AdapterBuilder, AdapterCommunication, AdapterInfo,
		AdapterPlatform, AdapterProtocol, AdapterStandard, AvatarSize,
	};
	pub use puniyu_utils::contact::{Contact, FriendContact, GroupContact, Scene};
	pub use puniyu_utils::{
		contact_friend, contact_group,
		message::{Message, Segment, element::ElementType},
		segment,
	};
	pub use serde_json;
}
