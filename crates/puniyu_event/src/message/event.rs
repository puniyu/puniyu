use super::friend::FriendMessage;
use super::group::GroupMessage;
use super::MessageBase;
use crate::{
	ContactType, EventBase, EventType, SenderType, SubEventType, codegen_delegate_to_variants,
	codegen_delegate_to_variants_convert, codegen_impl_as,
};
use puniyu_bot::Bot;
use puniyu_element::receive::Elements;

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
	fn time(&self) -> u64 {
		codegen_delegate_to_variants!(self, time, Friend, Group)
	}

	fn event_type(&self) -> EventType {
		codegen_delegate_to_variants!(self, event_type, Friend, Group)
	}

	fn event_id(&self) -> &str {
		codegen_delegate_to_variants!(self, event_id, Friend, Group)
	}

	fn sub_event(&self) -> SubEventType {
		codegen_delegate_to_variants_convert!(self, sub_event, SubEventType, Friend, Group)
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

	fn contact(&self) -> ContactType<'_> {
		match self {
			Self::Friend(inner) => inner.contact(),
			Self::Group(inner) => inner.contact(),
		}
	}

	fn sender(&self) -> SenderType<'_> {
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
