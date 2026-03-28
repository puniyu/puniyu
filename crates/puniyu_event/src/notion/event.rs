use crate::notion::{
	ContentType, FriendAdd, FriendDecrease, GroupCardChange, GroupFileUpload,
	GroupMemberTitleChange, GroupPoke, GroupRecall, NotionBase, NotionSubEventType,
	PrivateFileUpload, PrivatePoke, PrivateRecall, ReceiveLike,
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
	/// 收到点赞
	ReceiveLike(ReceiveLike<'n>),
	/// 好友增加
	FriendAdd(FriendAdd<'n>),
	/// 好友减少
	FriendDecrease(FriendDecrease<'n>),
	/// 私聊戳一戳
	PrivatePoke(PrivatePoke<'n>),
	/// 私聊撤回
	PrivateRecall(PrivateRecall<'n>),
	/// 私聊文件上传
	PrivateFileUpload(PrivateFileUpload<'n>),
	/// 群戳一戳
	GroupPoke(GroupPoke<'n>),
	/// 群聊撤回
	GroupRecall(GroupRecall<'n>),
	/// 群文件上传
	GroupFileUpload(GroupFileUpload<'n>),
	/// 群名片修改
	GroupCardChange(GroupCardChange<'n>),
	/// 群成员头衔变动
	GroupMemberTitleChange(GroupMemberTitleChange<'n>),
}

codegen_impl_as! {
	NotionEvent {
		ReceiveLike(ReceiveLike) => as_receive_like,
		FriendAdd(FriendAdd) => as_friend_add,
		FriendDecrease(FriendDecrease) => as_friend_decrease,
		PrivatePoke(PrivatePoke) => as_private_poke,
		PrivateRecall(PrivateRecall) => as_private_recall,
		PrivateFileUpload(PrivateFileUpload) => as_private_file_upload,
		GroupPoke(GroupPoke) => as_group_poke,
		GroupRecall(GroupRecall) => as_group_recall,
		GroupFileUpload(GroupFileUpload) => as_group_file_upload,
		GroupCardChange(GroupCardChange) => as_group_card_change,
		GroupMemberTitleChange(GroupMemberTitleChange) => as_group_member_title_change,
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
			ReceiveLike,
			FriendAdd,
			FriendDecrease,
			PrivatePoke,
			PrivateRecall,
			PrivateFileUpload,
			GroupPoke,
			GroupRecall,
			GroupFileUpload,
			GroupCardChange,
			GroupMemberTitleChange
		)
	}

	fn event_type(&self) -> &Self::EventType {
		codegen_delegate_to_variants!(
			self,
			event_type,
			ReceiveLike,
			FriendAdd,
			FriendDecrease,
			PrivatePoke,
			PrivateRecall,
			PrivateFileUpload,
			GroupPoke,
			GroupRecall,
			GroupFileUpload,
			GroupCardChange,
			GroupMemberTitleChange
		)
	}

	fn event_id(&self) -> &str {
		codegen_delegate_to_variants!(
			self,
			event_id,
			ReceiveLike,
			FriendAdd,
			FriendDecrease,
			PrivatePoke,
			PrivateRecall,
			PrivateFileUpload,
			GroupPoke,
			GroupRecall,
			GroupFileUpload,
			GroupCardChange,
			GroupMemberTitleChange
		)
	}

	fn sub_event(&self) -> &Self::SubEventType {
		codegen_delegate_to_variants!(
			self,
			sub_event,
			ReceiveLike,
			FriendAdd,
			FriendDecrease,
			PrivatePoke,
			PrivateRecall,
			PrivateFileUpload,
			GroupPoke,
			GroupRecall,
			GroupFileUpload,
			GroupCardChange,
			GroupMemberTitleChange
		)
	}

	fn bot(&self) -> &Bot {
		codegen_delegate_to_variants!(
			self,
			bot,
			ReceiveLike,
			FriendAdd,
			FriendDecrease,
			PrivatePoke,
			PrivateRecall,
			PrivateFileUpload,
			GroupPoke,
			GroupRecall,
			GroupFileUpload,
			GroupCardChange,
			GroupMemberTitleChange
		)
	}

	fn self_id(&self) -> &str {
		codegen_delegate_to_variants!(
			self,
			self_id,
			ReceiveLike,
			FriendAdd,
			FriendDecrease,
			PrivatePoke,
			PrivateRecall,
			PrivateFileUpload,
			GroupPoke,
			GroupRecall,
			GroupFileUpload,
			GroupCardChange,
			GroupMemberTitleChange
		)
	}

	fn user_id(&self) -> &str {
		codegen_delegate_to_variants!(
			self,
			user_id,
			ReceiveLike,
			FriendAdd,
			FriendDecrease,
			PrivatePoke,
			PrivateRecall,
			PrivateFileUpload,
			GroupPoke,
			GroupRecall,
			GroupFileUpload,
			GroupCardChange,
			GroupMemberTitleChange
		)
	}

	fn contact(&self) -> &Self::Contact {
		match self {
			Self::ReceiveLike(inner) => inner.contact(),
			Self::FriendAdd(inner) => inner.contact(),
			Self::FriendDecrease(inner) => inner.contact(),
			Self::PrivatePoke(inner) => inner.contact(),
			Self::PrivateRecall(inner) => inner.contact(),
			Self::PrivateFileUpload(inner) => inner.contact(),
			Self::GroupPoke(inner) => inner.contact(),
			Self::GroupRecall(inner) => inner.contact(),
			Self::GroupFileUpload(inner) => inner.contact(),
			Self::GroupCardChange(inner) => inner.contact(),
			Self::GroupMemberTitleChange(inner) => inner.contact(),
		}
	}

	fn sender(&self) -> &Self::Sender {
		match self {
			Self::ReceiveLike(inner) => inner.sender(),
			Self::FriendAdd(inner) => inner.sender(),
			Self::FriendDecrease(inner) => inner.sender(),
			Self::PrivatePoke(inner) => inner.sender(),
			Self::PrivateRecall(inner) => inner.sender(),
			Self::PrivateFileUpload(inner) => inner.sender(),
			Self::GroupPoke(inner) => inner.sender(),
			Self::GroupRecall(inner) => inner.sender(),
			Self::GroupFileUpload(inner) => inner.sender(),
			Self::GroupCardChange(inner) => inner.sender(),
			Self::GroupMemberTitleChange(inner) => inner.sender(),
		}
	}
}

impl<'n> NotionBase for NotionEvent<'n> {
	type Content = ContentType;

	fn notion(&self) -> &str {
		codegen_delegate_to_variants!(
			self,
			notion,
			ReceiveLike,
			FriendAdd,
			FriendDecrease,
			PrivatePoke,
			PrivateRecall,
			PrivateFileUpload,
			GroupPoke,
			GroupRecall,
			GroupFileUpload,
			GroupCardChange,
			GroupMemberTitleChange
		)
	}

	fn content(&self) -> Self::Content {
		codegen_delegate_to_variants_convert!(
			self,
			content,
			ContentType,
			ReceiveLike,
			FriendAdd,
			FriendDecrease,
			PrivatePoke,
			PrivateRecall,
			PrivateFileUpload,
			GroupPoke,
			GroupRecall,
			GroupFileUpload,
			GroupCardChange,
			GroupMemberTitleChange
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
			ReceiveLike,
			FriendAdd,
			FriendDecrease,
			PrivatePoke,
			PrivateRecall,
			PrivateFileUpload,
			GroupPoke,
			GroupRecall,
			GroupFileUpload,
			GroupCardChange,
			GroupMemberTitleChange
		)
	}

	/// 获取通知发送者信息。
	pub fn sender(&self) -> SenderType<'_> {
		codegen_delegate_to_variants_convert!(
			self,
			sender,
			SenderType,
			ReceiveLike,
			FriendAdd,
			FriendDecrease,
			PrivatePoke,
			PrivateRecall,
			PrivateFileUpload,
			GroupPoke,
			GroupRecall,
			GroupFileUpload,
			GroupCardChange,
			GroupMemberTitleChange
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
