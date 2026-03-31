use super::friend::FriendMessage;
use super::group::GroupMessage;
use super::{MessageBase, MessageSubEventType};
use crate::{
	EventBase, EventType, codegen_delegate_to_variants, codegen_delegate_to_variants_convert,
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

impl<'m> EventBase for MessageEvent<'m> {
	type EventType = EventType;
	type SubEventType = MessageSubEventType;
	type Contact = dyn puniyu_contact::Contact + 'm;
	type Sender = dyn puniyu_sender::Sender + 'm;

	fn time(&self) -> u64 {
		codegen_delegate_to_variants!(self, time, Friend, Group)
	}

	fn event_type(&self) -> &Self::EventType {
		codegen_delegate_to_variants!(self, event_type, Friend, Group)
	}

	fn event_id(&self) -> &str {
		codegen_delegate_to_variants!(self, event_id, Friend, Group)
	}

	fn sub_event(&self) -> &Self::SubEventType {
		codegen_delegate_to_variants!(self, sub_event, Friend, Group)
	}

	fn bot(&self) -> &Bot {
		codegen_delegate_to_variants!(self, bot, Friend, Group)
	}

	fn self_id(&self) -> &str {
		codegen_delegate_to_variants!(self, self_id, Friend, Group)
	}

	fn user_id(&self) -> &str {
		codegen_delegate_to_variants!(self, user_id, Friend, Group)
	}

	fn contact(&self) -> &Self::Contact {
		match self {
			Self::Friend(inner) => inner.contact(),
			Self::Group(inner) => inner.contact(),
		}
	}

	fn sender(&self) -> &Self::Sender {
		match self {
			Self::Friend(inner) => inner.sender(),
			Self::Group(inner) => inner.sender(),
		}
	}
}

impl<'m> MessageBase for MessageEvent<'m> {
	fn message_id(&self) -> &str {
		codegen_delegate_to_variants!(self, message_id, Friend, Group)
	}

	fn elements(&self) -> &Vec<Elements<'_>> {
		codegen_delegate_to_variants!(self, elements, Friend, Group)
	}
}

impl MessageEvent<'_> {
	/// 获取消息事件时间戳。
	pub fn time(&self) -> u64 {
		EventBase::time(self)
	}

	/// 获取事件类型。
	pub fn event_type(&self) -> &EventType {
		EventBase::event_type(self)
	}

	/// 获取事件 ID。
	pub fn event_id(&self) -> &str {
		EventBase::event_id(self)
	}

	/// 获取消息子类型。
	pub fn sub_event(&self) -> &MessageSubEventType {
		EventBase::sub_event(self)
	}

	/// 获取关联的机器人实例。
	pub fn bot(&self) -> &Bot {
		EventBase::bot(self)
	}

	/// 获取机器人自身 ID。
	pub fn self_id(&self) -> &str {
		EventBase::self_id(self)
	}

	/// 获取触发事件的用户 ID。
	pub fn user_id(&self) -> &str {
		EventBase::user_id(self)
	}

	/// 获取消息对应的联系人信息。
	pub fn contact(&self) -> ContactType<'_> {
		codegen_delegate_to_variants_convert!(self, contact, ContactType, Friend, Group)
	}

	/// 获取消息发送者信息。
	pub fn sender(&self) -> SenderType<'_> {
		codegen_delegate_to_variants_convert!(self, sender, SenderType, Friend, Group)
	}
}

impl MessageEvent<'_> {
	/// 获取消息 ID。
	pub fn message_id(&self) -> &str {
		MessageBase::message_id(self)
	}

	/// 获取消息元素列表。
	pub fn elements(&self) -> &Vec<Elements<'_>> {
		MessageBase::elements(self)
	}
}
