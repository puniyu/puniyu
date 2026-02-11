//! 通知事件模块
//!
//! 提供各类通知事件的类型定义，包括好友通知和群聊通知。

mod friend;
#[doc(inline)]
pub use friend::*;
mod group;
#[doc(inline)]
pub use group::*;

use super::EventBase;
use puniyu_bot::Bot;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 通知子类型枚举
///
/// 定义所有可能的通知类型。
#[derive(
	Debug,
	Clone,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
)]
pub enum NotionSubEvent {
	/// 收到点赞
	#[strum(serialize = "receiveLike")]
	ReceiveLike,
	/// 好友增加
	#[strum(serialize = "friendAdd")]
	FriendAdd,
	/// 好友删除
	#[strum(serialize = "friendDecrease")]
	FriendDecrease,
	/// 私聊戳一戳
	#[strum(serialize = "privatePoke")]
	PrivatePoke,
	/// 私聊撤回
	#[strum(serialize = "privateRecall")]
	PrivateRecall,
	/// 私聊文件上传
	#[strum(serialize = "privateFileUpload")]
	PrivateFileUpload,
	/// 群戳一戳
	#[strum(serialize = "groupPoke")]
	GroupPoke,
	/// 群聊撤回
	#[strum(serialize = "GroupRecall")]
	GroupRecall,
	/// 群文件上传
	#[strum(serialize = "groupFileUpload")]
	GroupFileUpload,
	/// 群名片修改
	#[strum(serialize = "groupCardChange")]
	GroupCardChange,
	/// 群成员头衔变动
	#[strum(serialize = "groupMemberTitleChange")]
	GroupMemberTitleChange,
	/// 群精华消息变动
	#[strum(serialize = "groupHighlightsChange")]
	GroupHighlightsChange,
	/// 群成员增加
	#[strum(serialize = "groupMemberAdd")]
	GroupMemberAdd,
	/// 群成员减少
	#[strum(serialize = "groupMemberDecrease")]
	GroupMemberDecrease,
	/// 群管理员变动
	#[strum(serialize = "groupAdminChange")]
	GroupAdminChange,
	/// 群打卡
	#[strum(serialize = "groupSignIn")]
	GroupSignIn,
	/// 群成员禁言
	#[strum(serialize = "groupMemberBan")]
	GroupMemberBan,
	/// 群全员禁言
	#[strum(serialize = "groupWholeBan")]
	GroupWholeBan,
	/// 群消息表情动态
	#[strum(serialize = "groupMessageReaction")]
	GroupMessageReaction,
	/// 群幸运王
	#[strum(serialize = "groupLuckKing")]
	GroupLuckKing,
	/// 群荣耀变动
	#[strum(serialize = "groupHonorChange")]
	GroupHonorChange,
}

/// 通知事件枚举
///
/// 包含所有类型的通知事件。
#[derive(Debug, Clone)]
pub enum NotionEvent<'n> {
	/// 收到点赞
	ReceiveLike(ReceiveLike<'n>),
	/// 好友增加
	FriendAdd(FriendDecrease<'n>),
	/// 私聊戳一戳
	PrivatePoke(PrivateRecall<'n>),
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

impl NotionEvent<'_> {
	/// 获取事件子类型
	///
	/// # 返回值
	///
	/// 返回通知事件的子类型
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let sub_event = event.sub_event();
	/// println!("通知类型: {:?}", sub_event);
	/// ```
	pub fn sub_event(&self) -> NotionSubEvent {
		match self {
			Self::ReceiveLike(_) => NotionSubEvent::ReceiveLike,
			Self::FriendAdd(_) => NotionSubEvent::FriendAdd,
			Self::PrivatePoke(_) => NotionSubEvent::PrivatePoke,
			Self::PrivateRecall(_) => NotionSubEvent::PrivateRecall,
			Self::PrivateFileUpload(_) => NotionSubEvent::PrivateFileUpload,
			Self::GroupPoke(_) => NotionSubEvent::GroupPoke,
			Self::GroupRecall(_) => NotionSubEvent::GroupRecall,
			Self::GroupFileUpload(_) => NotionSubEvent::GroupFileUpload,
			Self::GroupCardChange(_) => NotionSubEvent::GroupCardChange,
			Self::GroupMemberTitleChange(_) => NotionSubEvent::GroupMemberTitleChange,
		}
	}

	/// 获取机器人实例
	///
	/// # 返回值
	///
	/// 返回机器人实例的引用
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let bot = event.bot();
	/// println!("机器人 ID: {}", bot.account().uin);
	/// ```
	pub fn bot(&self) -> &Bot {
		match self {
			Self::ReceiveLike(n) => n.bot(),
			Self::FriendAdd(n) => n.bot(),
			Self::PrivatePoke(n) => n.bot(),
			Self::PrivateRecall(n) => n.bot(),
			Self::PrivateFileUpload(n) => n.bot(),
			Self::GroupPoke(n) => n.bot(),
			Self::GroupRecall(n) => n.bot(),
			Self::GroupFileUpload(n) => n.bot(),
			Self::GroupCardChange(n) => n.bot(),
			Self::GroupMemberTitleChange(n) => n.bot(),
		}
	}

	/// 获取事件时间戳
	///
	/// # 返回值
	///
	/// 返回事件触发的时间戳（秒）
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let timestamp = event.time();
	/// println!("事件时间: {}", timestamp);
	/// ```
	pub fn time(&self) -> u64 {
		match self {
			Self::ReceiveLike(n) => n.time(),
			Self::FriendAdd(n) => n.time(),
			Self::PrivatePoke(n) => n.time(),
			Self::PrivateRecall(n) => n.time(),
			Self::PrivateFileUpload(n) => n.time(),
			Self::GroupPoke(n) => n.time(),
			Self::GroupRecall(n) => n.time(),
			Self::GroupFileUpload(n) => n.time(),
			Self::GroupCardChange(n) => n.time(),
			Self::GroupMemberTitleChange(n) => n.time(),
		}
	}

	/// 获取事件 ID
	///
	/// # 返回值
	///
	/// 返回事件的唯一标识符
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_id = event.event_id();
	/// println!("事件 ID: {}", event_id);
	/// ```
	pub fn event_id(&self) -> &str {
		match self {
			Self::ReceiveLike(n) => n.event_id(),
			Self::FriendAdd(n) => n.event_id(),
			Self::PrivatePoke(n) => n.event_id(),
			Self::PrivateRecall(n) => n.event_id(),
			Self::PrivateFileUpload(n) => n.event_id(),
			Self::GroupPoke(n) => n.event_id(),
			Self::GroupRecall(n) => n.event_id(),
			Self::GroupFileUpload(n) => n.event_id(),
			Self::GroupCardChange(n) => n.event_id(),
			Self::GroupMemberTitleChange(n) => n.event_id(),
		}
	}

	/// 获取机器人自身 ID
	///
	/// # 返回值
	///
	/// 返回接收事件的机器人 ID
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let self_id = event.self_id();
	/// println!("机器人 ID: {}", self_id);
	/// ```
	pub fn self_id(&self) -> &str {
		match self {
			Self::ReceiveLike(n) => n.self_id(),
			Self::FriendAdd(n) => n.self_id(),
			Self::PrivatePoke(n) => n.self_id(),
			Self::PrivateRecall(n) => n.self_id(),
			Self::PrivateFileUpload(n) => n.self_id(),
			Self::GroupPoke(n) => n.self_id(),
			Self::GroupRecall(n) => n.self_id(),
			Self::GroupFileUpload(n) => n.self_id(),
			Self::GroupCardChange(n) => n.self_id(),
			Self::GroupMemberTitleChange(n) => n.self_id(),
		}
	}

	/// 获取用户 ID
	///
	/// # 返回值
	///
	/// 返回触发事件的用户 ID
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let user_id = event.user_id();
	/// println!("用户 ID: {}", user_id);
	/// ```
	pub fn user_id(&self) -> &str {
		match self {
			Self::ReceiveLike(n) => n.user_id(),
			Self::FriendAdd(n) => n.user_id(),
			Self::PrivatePoke(n) => n.user_id(),
			Self::PrivateRecall(n) => n.user_id(),
			Self::PrivateFileUpload(n) => n.user_id(),
			Self::GroupPoke(n) => n.user_id(),
			Self::GroupRecall(n) => n.user_id(),
			Self::GroupFileUpload(n) => n.user_id(),
			Self::GroupCardChange(n) => n.user_id(),
			Self::GroupMemberTitleChange(n) => n.user_id(),
		}
	}

	/// 判断发送者是否为主人
	///
	/// # 返回值
	///
	/// 如果发送者是配置的主人返回 `true`，否则返回 `false`
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if event.is_master() {
	///     println!("主人触发的事件");
	/// }
	/// ```
	pub fn is_master(&self) -> bool {
		use puniyu_config::Config;
		let config = Config::app();
		let masters = config.masters();
		masters.contains(&self.user_id().to_string())
	}

	/// 判断是否为好友通知事件
	///
	/// # 返回值
	///
	/// 如果是好友相关的通知事件返回 `true`，否则返回 `false`
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_event::notion::NotionEvent;
	///
	/// fn handle_notion(event: &NotionEvent) {
	///     if event.is_friend() {
	///         println!("这是好友通知事件");
	///     }
	/// }
	/// ```
	pub fn is_friend(&self) -> bool {
		match self {
			Self::ReceiveLike(n) => n.is_friend(),
			Self::FriendAdd(n) => n.is_friend(),
			Self::PrivatePoke(n) => n.is_friend(),
			Self::PrivateRecall(n) => n.is_friend(),
			Self::PrivateFileUpload(n) => n.is_friend(),
			_ => false,
		}
	}

	/// 判断是否为群聊通知事件
	///
	/// # 返回值
	///
	/// 如果是群聊相关的通知事件返回 `true`，否则返回 `false`
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_event::notion::NotionEvent;
	///
	/// fn handle_notion(event: &NotionEvent) {
	///     if event.is_group() {
	///         println!("这是群聊通知事件");
	///     }
	/// }
	/// ```
	pub fn is_group(&self) -> bool {
		match self {
			Self::GroupPoke(n) => n.is_group(),
			Self::GroupRecall(n) => n.is_group(),
			Self::GroupFileUpload(n) => n.is_group(),
			Self::GroupCardChange(n) => n.is_group(),
			Self::GroupMemberTitleChange(n) => n.is_group(),
			_ => false,
		}
	}
}

/// 通知基础 trait
///
/// 定义所有通知事件的通用接口。
///
/// # 关联类型
///
/// - `Content`: 通知内容的具体类型
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_event::notion::NotionBase;
///
/// fn process_notion<N: NotionBase>(notion: &N) {
///     println!("通知消息: {}", notion.notion());
///     println!("事件 ID: {}", notion.event_id());
/// }
/// ```
pub trait NotionBase: Send + Sync + EventBase {
	/// 通知内容类型
	type Content;

	/// 获取通知消息
	///
	/// # 返回值
	///
	/// 返回通知的文本消息内容
	fn notion(&self) -> &str;

	/// 获取通知内容
	///
	/// # 返回值
	///
	/// 返回通知的详细内容对象
	fn content(&self) -> &Self::Content;
}

/// 通知构建器
///
/// 用于构建通知事件的辅助结构。
pub struct NotionBuilder<'n, Contact, Sender, Content>
where
	Contact: puniyu_contact::Contact,
	Sender: puniyu_sender::Sender,
{
	/// 机器人实例
	pub bot: &'n Bot,
	/// 事件 ID
	pub event_id: &'n str,
	/// 时间戳
	pub time: u64,
	/// 机器人 ID
	pub self_id: &'n str,
	/// 用户 ID
	pub user_id: &'n str,
	/// 联系人信息
	pub contact: &'n Contact,
	/// 发送者信息
	pub sender: &'n Sender,
	/// 通知内容
	pub content: &'n Content,
}
