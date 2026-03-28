use crate::request::{
	ContentType, GroupApply, GroupInvite, PrivateApply, RequestBase, RequestSubEventType,
};
use crate::{
	EventBase, EventType, codegen_delegate_to_variants, codegen_delegate_to_variants_convert,
	codegen_impl_as,
};
use puniyu_bot::Bot;
use puniyu_contact::ContactType;
use puniyu_sender::SenderType;

/// 请求事件枚举
///
/// 包含所有类型的请求事件。
#[derive(Debug, Clone)]
pub enum RequestEvent<'r> {
	/// 好友申请
	PrivateApply(PrivateApply<'r>),
	/// 群申请
	GroupApply(GroupApply<'r>),
	/// 邀请入群
	GroupInvite(GroupInvite<'r>),
}

codegen_impl_as! {
	RequestEvent {
		PrivateApply(PrivateApply) => as_private_apply,
		GroupApply(GroupApply) => as_group_apply,
		GroupInvite(GroupInvite) => as_group_invite,
	}
}

impl<'r> EventBase for RequestEvent<'r> {
	type EventType = EventType;
	type SubEventType = RequestSubEventType;
	type Contact = dyn puniyu_contact::Contact + 'r;
	type Sender = dyn puniyu_sender::Sender + 'r;

	fn time(&self) -> u64 {
		codegen_delegate_to_variants!(self, time, PrivateApply, GroupApply, GroupInvite)
	}

	fn event_type(&self) -> &Self::EventType {
		codegen_delegate_to_variants!(self, event_type, PrivateApply, GroupApply, GroupInvite)
	}

	fn event_id(&self) -> &str {
		codegen_delegate_to_variants!(self, event_id, PrivateApply, GroupApply, GroupInvite)
	}

	fn sub_event(&self) -> &Self::SubEventType {
		codegen_delegate_to_variants!(self, sub_event, PrivateApply, GroupApply, GroupInvite)
	}

	fn bot(&self) -> &Bot {
		codegen_delegate_to_variants!(self, bot, PrivateApply, GroupApply, GroupInvite)
	}

	fn self_id(&self) -> &str {
		codegen_delegate_to_variants!(self, self_id, PrivateApply, GroupApply, GroupInvite)
	}

	fn user_id(&self) -> &str {
		codegen_delegate_to_variants!(self, user_id, PrivateApply, GroupApply, GroupInvite)
	}

	fn contact(&self) -> &Self::Contact {
		match self {
			Self::PrivateApply(inner) => inner.contact(),
			Self::GroupApply(inner) => inner.contact(),
			Self::GroupInvite(inner) => inner.contact(),
		}
	}

	fn sender(&self) -> &Self::Sender {
		match self {
			Self::PrivateApply(inner) => inner.sender(),
			Self::GroupApply(inner) => inner.sender(),
			Self::GroupInvite(inner) => inner.sender(),
		}
	}
}

impl<'r> RequestBase for RequestEvent<'r> {
	type Content = ContentType;

	fn request(&self) -> &str {
		codegen_delegate_to_variants!(self, request, PrivateApply, GroupApply, GroupInvite)
	}

	fn content(&self) -> Self::Content {
		codegen_delegate_to_variants_convert!(
			self,
			content,
			ContentType,
			PrivateApply,
			GroupApply,
			GroupInvite
		)
	}
}

impl RequestEvent<'_> {
	/// 获取请求事件时间戳。
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

	/// 获取请求子类型。
	pub fn sub_event(&self) -> &RequestSubEventType {
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

	/// 获取请求对应的联系人信息。
	pub fn contact(&self) -> ContactType<'_> {
		codegen_delegate_to_variants_convert!(
			self,
			contact,
			ContactType,
			PrivateApply,
			GroupApply,
			GroupInvite
		)
	}

	/// 获取请求发送者信息。
	pub fn sender(&self) -> SenderType<'_> {
		codegen_delegate_to_variants_convert!(
			self,
			sender,
			SenderType,
			PrivateApply,
			GroupApply,
			GroupInvite
		)
	}
}

impl RequestEvent<'_> {
	/// 获取请求描述文本。
	pub fn request(&self) -> &str {
		RequestBase::request(self)
	}

	/// 获取统一的请求内容枚举。
	pub fn content(&self) -> ContentType {
		RequestBase::content(self)
	}
}
