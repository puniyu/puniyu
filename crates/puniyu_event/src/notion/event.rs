use crate::notion::{
	ContentType, FriendAdd, FriendDecrease, GroupCardChange, GroupFileUpload,
	GroupMemberTitleChange, GroupPoke, GroupRecall, NotionBase, NotionSubEventType,
	PrivateFileUpload, PrivatePoke, PrivateRecall, ReceiveLike,
};
use crate::{
	EventBase,
	EventType,
	codegen_delegate_to_variants,
	codegen_delegate_to_variants_convert,
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

impl NotionEvent<'_> {
	pub fn time(&self) -> u64 {
		codegen_delegate_to_variants!(self, time,
			ReceiveLike, FriendAdd, FriendDecrease, PrivatePoke, PrivateRecall,
			PrivateFileUpload, GroupPoke, GroupRecall, GroupFileUpload,
			GroupCardChange, GroupMemberTitleChange)
	}

	pub fn event_type(&self) -> &EventType {
		codegen_delegate_to_variants!(self, event_type,
			ReceiveLike, FriendAdd, FriendDecrease, PrivatePoke, PrivateRecall,
			PrivateFileUpload, GroupPoke, GroupRecall, GroupFileUpload,
			GroupCardChange, GroupMemberTitleChange)
	}

	pub fn event_id(&self) -> &str {
		codegen_delegate_to_variants!(self, event_id,
			ReceiveLike, FriendAdd, FriendDecrease, PrivatePoke, PrivateRecall,
			PrivateFileUpload, GroupPoke, GroupRecall, GroupFileUpload,
			GroupCardChange, GroupMemberTitleChange)
	}

	pub fn sub_event(&self) -> &NotionSubEventType {
		codegen_delegate_to_variants!(self, sub_event,
			ReceiveLike, FriendAdd, FriendDecrease, PrivatePoke, PrivateRecall,
			PrivateFileUpload, GroupPoke, GroupRecall, GroupFileUpload,
			GroupCardChange, GroupMemberTitleChange)
	}

	pub fn bot(&self) -> &Bot {
		codegen_delegate_to_variants!(self, bot,
			ReceiveLike, FriendAdd, FriendDecrease, PrivatePoke, PrivateRecall,
			PrivateFileUpload, GroupPoke, GroupRecall, GroupFileUpload,
			GroupCardChange, GroupMemberTitleChange)
	}

	pub fn self_id(&self) -> &str {
		codegen_delegate_to_variants!(self, self_id,
			ReceiveLike, FriendAdd, FriendDecrease, PrivatePoke, PrivateRecall,
			PrivateFileUpload, GroupPoke, GroupRecall, GroupFileUpload,
			GroupCardChange, GroupMemberTitleChange)
	}

	pub fn user_id(&self) -> &str {
		codegen_delegate_to_variants!(self, user_id,
			ReceiveLike, FriendAdd, FriendDecrease, PrivatePoke, PrivateRecall,
			PrivateFileUpload, GroupPoke, GroupRecall, GroupFileUpload,
			GroupCardChange, GroupMemberTitleChange)
	}

	pub fn contact(&self) -> ContactType<'_> {
		codegen_delegate_to_variants_convert!(self, contact, ContactType,
			ReceiveLike, FriendAdd, FriendDecrease, PrivatePoke, PrivateRecall,
			PrivateFileUpload, GroupPoke, GroupRecall, GroupFileUpload,
			GroupCardChange, GroupMemberTitleChange)
	}

	pub fn sender(&self) -> SenderType<'_> {
		codegen_delegate_to_variants_convert!(self, sender, SenderType,
			ReceiveLike, FriendAdd, FriendDecrease, PrivatePoke, PrivateRecall,
			PrivateFileUpload, GroupPoke, GroupRecall, GroupFileUpload,
			GroupCardChange, GroupMemberTitleChange)
	}
}

impl NotionEvent<'_> {
	pub fn notion(&self) -> &str {
		codegen_delegate_to_variants!(self, notion,
			ReceiveLike, FriendAdd, FriendDecrease, PrivatePoke, PrivateRecall,
			PrivateFileUpload, GroupPoke, GroupRecall, GroupFileUpload,
			GroupCardChange, GroupMemberTitleChange)
	}

	pub fn content(&self) -> ContentType {
		codegen_delegate_to_variants_convert!(self, content, ContentType,
			ReceiveLike, FriendAdd, FriendDecrease, PrivatePoke, PrivateRecall,
			PrivateFileUpload, GroupPoke, GroupRecall, GroupFileUpload,
			GroupCardChange, GroupMemberTitleChange)
	}
}


