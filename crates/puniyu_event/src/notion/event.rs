use crate::notion::{
	ContentType, FriendAdd, FriendDecrease, GroupCardChange, GroupFileUpload,
	GroupMemberTitleChange, GroupPoke, GroupRecall, NotionBase, NotionSubEventType,
	PrivateFileUpload, PrivatePoke, PrivateRecall, ReceiveLike,
};
use crate::{EventBase, EventType};
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

macro_rules! delegate_to_variants {
	($self:ident, $method:ident() -> $ret:ty) => {
		match $self {
			Self::ReceiveLike(inner) => inner.$method(),
			Self::FriendAdd(inner) => inner.$method(),
			Self::FriendDecrease(inner) => inner.$method(),
			Self::PrivatePoke(inner) => inner.$method(),
			Self::PrivateRecall(inner) => inner.$method(),
			Self::PrivateFileUpload(inner) => inner.$method(),
			Self::GroupPoke(inner) => inner.$method(),
			Self::GroupRecall(inner) => inner.$method(),
			Self::GroupFileUpload(inner) => inner.$method(),
			Self::GroupCardChange(inner) => inner.$method(),
			Self::GroupMemberTitleChange(inner) => inner.$method(),
		}
	};
}

macro_rules! impl_as {
    ($enum_name:ident { $($variant:ident => $method_name:ident),* $(,)? }) => {
        impl $enum_name<'_> {
            $(
                pub fn $method_name(&self) -> Option<&$variant<'_>> {
                    match self {
                        Self::$variant(inner) => Some(inner),
                        _ => None,
                    }
                }
            )*
        }
    };
}
impl_as! {
	NotionEvent {
		ReceiveLike => as_receive_like,
		FriendAdd => as_friend_add,
		FriendDecrease => as_friend_decrease,
		PrivatePoke => as_private_poke,
		PrivateRecall => as_private_recall,
		PrivateFileUpload => as_private_file_upload,
		GroupPoke => as_group_poke,
		GroupRecall => as_group_recall,
		GroupFileUpload => as_group_file_upload,
		GroupCardChange => as_group_card_change,
		GroupMemberTitleChange => as_group_member_title_change,
	}
}
impl NotionEvent<'_> {
	/// 获取通知触发时间戳
	///
	/// # 返回值
	///
	/// 返回 Unix 时间戳（秒）
	pub fn time(&self) -> u64 {
		delegate_to_variants!(self, time() -> u64)
	}

	/// 获取事件类型
	///
	/// # 返回值
	///
	/// 返回 `EventType::Notion`
	pub fn event(&self) -> &EventType {
		delegate_to_variants!(self, event() -> EventType)
	}

	/// 获取事件 ID
	///
	/// # 返回值
	///
	/// 返回事件的唯一标识符
	pub fn event_id(&self) -> &str {
		delegate_to_variants!(self, event_id() -> &str)
	}

	/// 获取通知子类型
	///
	/// # 返回值
	///
	/// 返回通知的具体子类型
	pub fn sub_event(&self) -> &NotionSubEventType {
		delegate_to_variants!(self, sub_event() -> &NotionSubEvent)
	}

	/// 获取机器人实例
	///
	/// # 返回值
	///
	/// 返回处理该通知的机器人实例引用
	pub fn bot(&self) -> &Bot {
		delegate_to_variants!(self, bot() -> &Bot)
	}

	/// 获取机器人 ID
	///
	/// # 返回值
	///
	/// 返回机器人的唯一标识符
	pub fn self_id(&self) -> &str {
		delegate_to_variants!(self, self_id() -> &str)
	}

	/// 获取用户 ID
	///
	/// # 返回值
	///
	/// 返回触发通知的用户 ID
	pub fn user_id(&self) -> &str {
		delegate_to_variants!(self, user_id() -> &str)
	}

	/// 获取联系人信息
	///
	/// # 返回值
	///
	/// 返回通知发生的联系人信息
	pub fn contact(&self) -> ContactType<'_> {
		match self {
			Self::ReceiveLike(inner) => ContactType::from(inner.contact().clone()),
			Self::FriendDecrease(inner) => ContactType::from(inner.contact().clone()),
			Self::FriendAdd(inner) => ContactType::from(inner.contact().clone()),
			Self::PrivatePoke(inner) => ContactType::from(inner.contact().clone()),
			Self::PrivateRecall(inner) => ContactType::from(inner.contact().clone()),
			Self::PrivateFileUpload(inner) => ContactType::from(inner.contact().clone()),
			Self::GroupPoke(inner) => ContactType::from(inner.contact().clone()),
			Self::GroupRecall(inner) => ContactType::from(inner.contact().clone()),
			Self::GroupFileUpload(inner) => ContactType::from(inner.contact().clone()),
			Self::GroupCardChange(inner) => ContactType::from(inner.contact().clone()),
			Self::GroupMemberTitleChange(inner) => ContactType::from(inner.contact().clone()),
		}
	}

	/// 获取发送者信息
	///
	/// # 返回值
	///
	/// 返回触发通知的用户详细信息
	pub fn sender(&self) -> SenderType<'_> {
		match self {
			Self::ReceiveLike(inner) => SenderType::from(inner.sender().clone()),
			Self::FriendAdd(inner) => SenderType::from(inner.sender().clone()),
			Self::FriendDecrease(inner) => SenderType::from(inner.sender().clone()),
			Self::PrivatePoke(inner) => SenderType::from(inner.sender().clone()),
			Self::PrivateRecall(inner) => SenderType::from(inner.sender().clone()),
			Self::PrivateFileUpload(inner) => SenderType::from(inner.sender().clone()),
			Self::GroupPoke(inner) => SenderType::from(inner.sender().clone()),
			Self::GroupRecall(inner) => SenderType::from(inner.sender().clone()),
			Self::GroupFileUpload(inner) => SenderType::from(inner.sender().clone()),
			Self::GroupCardChange(inner) => SenderType::from(inner.sender().clone()),
			Self::GroupMemberTitleChange(inner) => SenderType::from(inner.sender().clone()),
		}
	}
}

impl NotionEvent<'_> {
	/// 获取通知消息
	///
	/// # 返回值
	///
	/// 返回通知的文本消息内容
	pub fn notion(&self) -> &str {
		delegate_to_variants!(self, notion() -> &str)
	}

	/// 获取通知内容
	///
	/// # 返回值
	///
	/// 返回通知的详细内容对象
	pub fn content(&self) -> ContentType {
		match self {
			Self::ReceiveLike(inner) => ContentType::from(inner.content().clone()),
			Self::FriendAdd(inner) => ContentType::from(inner.content().clone()),
			Self::FriendDecrease(inner) => ContentType::from(inner.content().clone()),
			Self::PrivatePoke(inner) => ContentType::from(inner.content().clone()),
			Self::PrivateRecall(inner) => ContentType::from(inner.content().clone()),
			Self::PrivateFileUpload(inner) => ContentType::from(inner.content().clone()),
			Self::GroupPoke(inner) => ContentType::from(inner.content().clone()),
			Self::GroupRecall(inner) => ContentType::from(inner.content().clone()),
			Self::GroupFileUpload(inner) => ContentType::from(inner.content().clone()),
			Self::GroupCardChange(inner) => ContentType::from(inner.content().clone()),
			Self::GroupMemberTitleChange(inner) => ContentType::from(inner.content().clone()),
		}
	}
}
