use crate::notion::{
	ContentType, FriendAdd, FriendDecrease, GroupFileUpload, GroupRecall, NotionBase,
	NotionSubEventType, PrivateFileUpload, PrivateRecall,
};
use crate::{
	EventBase, EventType, codegen_delegate_to_variants, codegen_delegate_to_variants_convert,
	codegen_impl_as,
};
use puniyu_bot::Bot;
use puniyu_contact::ContactType;
use puniyu_sender::SenderType;

/// 通知事件枚举
///
/// 包含所有类型的通知事件。
#[derive(Debug, Clone)]
pub enum NotionEvent<'n> {
	/// 好友增加
	FriendAdd(FriendAdd<'n>),
	/// 好友减少
	FriendDecrease(FriendDecrease<'n>),
	/// 私聊撤回
	PrivateRecall(PrivateRecall<'n>),
	/// 私聊文件上传
	PrivateFileUpload(PrivateFileUpload<'n>),
	/// 群聊撤回
	GroupRecall(GroupRecall<'n>),
	/// 群文件上传
	GroupFileUpload(GroupFileUpload<'n>),
}
codegen_impl_as! {
	NotionEvent {
		FriendAdd(FriendAdd) => as_friend_add,
		FriendDecrease(FriendDecrease) => as_friend_decrease,
		PrivateRecall(PrivateRecall) => as_private_recall,
		PrivateFileUpload(PrivateFileUpload) => as_private_file_upload,
		GroupRecall(GroupRecall) => as_group_recall,
		GroupFileUpload(GroupFileUpload) => as_group_file_upload,
	}
}
impl<'n> EventBase for NotionEvent<'n> {
	type EventType = EventType;
	type SubEventType = NotionSubEventType;
	type Contact = dyn puniyu_contact::Contact + 'n;
	type Sender = dyn puniyu_sender::Sender + 'n;

	fn time(&self) -> u64 {
		codegen_delegate_to_variants!(
			self,
			time,
			FriendAdd,
			FriendDecrease,
			PrivateRecall,
			PrivateFileUpload,
			GroupRecall,
			GroupFileUpload
		)
	}

	fn event_type(&self) -> &Self::EventType {
		codegen_delegate_to_variants!(
			self,
			event_type,
			FriendAdd,
			FriendDecrease,
			PrivateRecall,
			PrivateFileUpload,
			GroupRecall,
			GroupFileUpload
		)
	}

	fn event_id(&self) -> &str {
		codegen_delegate_to_variants!(
			self,
			event_id,
			FriendAdd,
			FriendDecrease,
			PrivateRecall,
			PrivateFileUpload,
			GroupRecall,
			GroupFileUpload
		)
	}

	fn sub_event(&self) -> &Self::SubEventType {
		codegen_delegate_to_variants!(
			self,
			sub_event,
			FriendAdd,
			FriendDecrease,
			PrivateRecall,
			PrivateFileUpload,
			GroupRecall,
			GroupFileUpload
		)
	}

	fn bot(&self) -> &Bot {
		codegen_delegate_to_variants!(
			self,
			bot,
			FriendAdd,
			FriendDecrease,
			PrivateRecall,
			PrivateFileUpload,
			GroupRecall,
			GroupFileUpload
		)
	}

	fn self_id(&self) -> &str {
		codegen_delegate_to_variants!(
			self,
			self_id,
			FriendAdd,
			FriendDecrease,
			PrivateRecall,
			PrivateFileUpload,
			GroupRecall,
			GroupFileUpload
		)
	}

	fn user_id(&self) -> &str {
		codegen_delegate_to_variants!(
			self,
			user_id,
			FriendAdd,
			FriendDecrease,
			PrivateRecall,
			PrivateFileUpload,
			GroupRecall,
			GroupFileUpload
		)
	}

	fn contact(&self) -> &Self::Contact {
		match self {
			Self::FriendAdd(inner) => inner.contact(),
			Self::FriendDecrease(inner) => inner.contact(),
			Self::PrivateRecall(inner) => inner.contact(),
			Self::PrivateFileUpload(inner) => inner.contact(),
			Self::GroupRecall(inner) => inner.contact(),
			Self::GroupFileUpload(inner) => inner.contact(),
		}
	}

	fn sender(&self) -> &Self::Sender {
		match self {
			Self::FriendAdd(inner) => inner.sender(),
			Self::FriendDecrease(inner) => inner.sender(),
			Self::PrivateRecall(inner) => inner.sender(),
			Self::PrivateFileUpload(inner) => inner.sender(),
			Self::GroupRecall(inner) => inner.sender(),
			Self::GroupFileUpload(inner) => inner.sender(),
		}
	}
}

impl<'n> NotionBase for NotionEvent<'n> {
	type Content = ContentType;

	fn notion(&self) -> &str {
		codegen_delegate_to_variants!(
			self,
			notion,
			FriendAdd,
			FriendDecrease,
			PrivateRecall,
			PrivateFileUpload,
			GroupRecall,
			GroupFileUpload
		)
	}

	fn content(&self) -> Self::Content {
		codegen_delegate_to_variants_convert!(
			self,
			content,
			ContentType,
			FriendAdd,
			FriendDecrease,
			PrivateRecall,
			PrivateFileUpload,
			GroupRecall,
			GroupFileUpload
		)
	}
}

impl NotionEvent<'_> {
	/// 获取通知事件时间戳。
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

	/// 获取通知子类型。
	pub fn sub_event(&self) -> &NotionSubEventType {
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

	/// 获取通知对应的联系人信息。
	pub fn contact(&self) -> ContactType<'_> {
		codegen_delegate_to_variants_convert!(
			self,
			contact,
			ContactType,
			FriendAdd,
			FriendDecrease,
			PrivateRecall,
			PrivateFileUpload,
			GroupRecall,
			GroupFileUpload
		)
	}

	/// 获取通知发送者信息。
	pub fn sender(&self) -> SenderType<'_> {
		codegen_delegate_to_variants_convert!(
			self,
			sender,
			SenderType,
			FriendAdd,
			FriendDecrease,
			PrivateRecall,
			PrivateFileUpload,
			GroupRecall,
			GroupFileUpload
		)
	}
}

impl NotionEvent<'_> {
	/// 获取通知描述文本。
	pub fn notion(&self) -> &str {
		NotionBase::notion(self)
	}

	/// 获取统一的通知内容枚举。
	pub fn content(&self) -> ContentType {
		NotionBase::content(self)
	}
}
