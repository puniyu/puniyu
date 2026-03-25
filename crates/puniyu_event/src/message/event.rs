use super::friend::FriendMessage;
use super::group::GroupMessage;
use super::{MessageBase, MessageSubEventType};
use crate::{
	EventBase,
	EventType,
	codegen_delegate_to_variants,
	codegen_delegate_to_variants_convert,
	codegen_impl_as,
};
use puniyu_bot::Bot;
use puniyu_contact::ContactType;
use puniyu_element::receive::Elements;
use puniyu_sender::SenderType;

/// 消息事件枚举
///
/// 统一的消息事件类型，包含好友消息和群消息。
#[derive(Debug, Clone)]
pub enum MessageEvent<'m> {
	/// 好友消息事件
	Friend(FriendMessage<'m>),
	/// 群聊消息事件
	Group(GroupMessage<'m>),
}

codegen_impl_as! {
	MessageEvent {
		Friend(FriendMessage) => as_friend,
		Group(GroupMessage) => as_group,
	}
}

impl MessageEvent<'_> {
	pub fn time(&self) -> u64 {
		codegen_delegate_to_variants!(self, time, Friend, Group)
	}

	pub fn event_type(&self) -> &EventType {
		codegen_delegate_to_variants!(self, event_type, Friend, Group)
	}

	pub fn event_id(&self) -> &str {
		codegen_delegate_to_variants!(self, event_id, Friend, Group)
	}

	pub fn sub_event(&self) -> &MessageSubEventType {
		codegen_delegate_to_variants!(self, sub_event, Friend, Group)
	}

	pub fn bot(&self) -> &Bot {
		codegen_delegate_to_variants!(self, bot, Friend, Group)
	}

	pub fn self_id(&self) -> &str {
		codegen_delegate_to_variants!(self, self_id, Friend, Group)
	}

	pub fn user_id(&self) -> &str {
		codegen_delegate_to_variants!(self, user_id, Friend, Group)
	}

	pub fn contact(&self) -> ContactType<'_> {
		codegen_delegate_to_variants_convert!(self, contact, ContactType, Friend, Group)
	}

	pub fn sender(&self) -> SenderType<'_> {
		codegen_delegate_to_variants_convert!(self, sender, SenderType, Friend, Group)
	}
}

impl MessageEvent<'_> {
	pub fn message_id(&self) -> &str {
		codegen_delegate_to_variants!(self, message_id, Friend, Group)
	}

	pub fn elements(&self) -> &Vec<Elements<'_>> {
		codegen_delegate_to_variants!(self, elements, Friend, Group)
	}
}
