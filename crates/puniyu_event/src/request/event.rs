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

impl RequestEvent<'_> {
	pub fn time(&self) -> u64 {
		codegen_delegate_to_variants!(self, time, PrivateApply, GroupApply, GroupInvite)
	}

	pub fn event_type(&self) -> &EventType {
		codegen_delegate_to_variants!(self, event_type, PrivateApply, GroupApply, GroupInvite)
	}

	pub fn event_id(&self) -> &str {
		codegen_delegate_to_variants!(self, event_id, PrivateApply, GroupApply, GroupInvite)
	}

	pub fn sub_event(&self) -> &RequestSubEventType {
		codegen_delegate_to_variants!(self, sub_event, PrivateApply, GroupApply, GroupInvite)
	}

	pub fn bot(&self) -> &Bot {
		codegen_delegate_to_variants!(self, bot, PrivateApply, GroupApply, GroupInvite)
	}

	pub fn self_id(&self) -> &str {
		codegen_delegate_to_variants!(self, self_id, PrivateApply, GroupApply, GroupInvite)
	}

	pub fn user_id(&self) -> &str {
		codegen_delegate_to_variants!(self, user_id, PrivateApply, GroupApply, GroupInvite)
	}

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
	pub fn request(&self) -> &str {
		codegen_delegate_to_variants!(self, request, PrivateApply, GroupApply, GroupInvite)
	}

	pub fn content(&self) -> ContentType {
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
