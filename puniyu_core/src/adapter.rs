pub mod prelude {
	pub use async_trait::async_trait;
	pub use puniyu_adapter::AdapterBuilder;
	pub use puniyu_adapter::{
		AccountInfo, AdapterApi, AdapterCommunication, AdapterInfo, AdapterPlatform,
		AdapterProtocol, AdapterStandard, AvatarSize,
	};
	pub use puniyu_adapter::{account_info, adapter_info};
	pub use puniyu_adapter_derive::adapter;
	pub use puniyu_bot::{BotRegistry, register_bot};
	pub use puniyu_element::{ElementType, Message, Segment, segment};
	pub use puniyu_event_bus::{EVENT_BUS, EventBus, send_event, setup_event_bus};
	pub use puniyu_event_core::Event;
	pub use puniyu_event_message::create_friend_message;
	pub use puniyu_event_message::{FriendMessage, MessageBase, MessageEvent};
	pub use puniyu_event_utils::contact::{Contact, FriendContact, GroupContact, Scene};
	pub use puniyu_event_utils::sender::{FriendSender, Role, Sender, Sex};
	pub use puniyu_event_utils::{contact_friend, contact_group, friend_sender};
}
pub use puniyu_adapter::AdapterBuilder;
pub use puniyu_event_bus::{EVENT_BUS, EventBus, send_event, setup_event_bus};
