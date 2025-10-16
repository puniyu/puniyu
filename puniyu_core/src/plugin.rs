use puniyu_plugin::{Plugin, PluginId, PluginRegistry};

pub fn get_plugin_info(plugin: impl Into<PluginId>) -> Option<Plugin> {
	let plugin_id = plugin.into();
	PluginRegistry::get_plugin(plugin_id)
}

pub mod prelude {
	pub use puniyu_adapter::{
		AccountInfo, AdapterApi, AdapterBuilder, AdapterCommunication, AdapterInfo,
		AdapterPlatform, AdapterProtocol, AdapterStandard, AvatarSize,
	};
	pub use puniyu_command::CommandBuilder;
	pub use puniyu_element::{ElementType, Message, Segment, segment};
	pub use puniyu_event_utils::contact::{Contact, FriendContact, GroupContact, Scene};
	pub use puniyu_event_utils::{contact_friend, contact_group};
	pub use puniyu_plugin_derive::{plugin, task};
	pub use serde_json;
}
pub use puniyu_command::CommandBuilder;
pub use puniyu_plugin::PluginBuilder;
pub use puniyu_task::TaskBuilder;
