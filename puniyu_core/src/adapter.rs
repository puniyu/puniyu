pub mod prelude {
	pub use super::AdapterBuilder;
	pub use async_trait::async_trait;
	pub use puniyu_adapter_derive::adapter;
	pub use puniyu_registry::{bot::registry::BotRegistry, register_bot};
	pub use puniyu_utils::adapter::{
		AccountInfo, AdapterApi, AdapterCommunication, AdapterInfo, AdapterPlatform,
		AdapterProtocol, AdapterStandard, AvatarSize,
	};
	pub use puniyu_utils::contact::{Contact, FriendContact, GroupContact, Scene};
	pub use puniyu_utils::event::message::{MessageBase, MessageEvent, friend::FriendMessage};
	pub use puniyu_utils::event::{Event, send_event};
	pub use puniyu_utils::message::{Message, Segment, element::ElementType};
	pub use puniyu_utils::{
		account_info, adapter_info, contact_friend, contact_group, create_friend_message, segment,
	};
}
pub use puniyu_utils::adapter::AdapterBase as AdapterBuilder;
