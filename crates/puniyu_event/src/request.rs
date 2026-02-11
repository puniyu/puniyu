//! 请求事件模块
//!
//! 提供好友申请、群申请等请求事件的类型定义。

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

/// 请求子类型枚举
///
/// 定义所有可能的请求类型。
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
pub enum RequestSubEvent {
	/// 好友申请
	#[strum(serialize = "privateApply")]
	PrivateApply,
	/// 群申请
	#[strum(serialize = "groupApply")]
	GroupApply,
	/// 邀请入群
	#[strum(serialize = "groupInvite")]
	GroupInvite,
}

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

impl RequestEvent<'_> {
	/// 获取事件子类型
	///
	/// # 返回值
	///
	/// 返回请求事件的子类型
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let sub_event = event.sub_event();
	/// println!("请求类型: {:?}", sub_event);
	/// ```
	pub fn sub_event(&self) -> RequestSubEvent {
		match self {
			Self::PrivateApply(_) => RequestSubEvent::PrivateApply,
			Self::GroupApply(_) => RequestSubEvent::GroupApply,
			Self::GroupInvite(_) => RequestSubEvent::GroupInvite,
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
			Self::PrivateApply(r) => r.bot(),
			Self::GroupApply(r) => r.bot(),
			Self::GroupInvite(r) => r.bot(),
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
			Self::PrivateApply(r) => r.time(),
			Self::GroupApply(r) => r.time(),
			Self::GroupInvite(r) => r.time(),
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
			Self::PrivateApply(r) => r.event_id(),
			Self::GroupApply(r) => r.event_id(),
			Self::GroupInvite(r) => r.event_id(),
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
			Self::PrivateApply(r) => r.self_id(),
			Self::GroupApply(r) => r.self_id(),
			Self::GroupInvite(r) => r.self_id(),
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
			Self::PrivateApply(r) => r.user_id(),
			Self::GroupApply(r) => r.user_id(),
			Self::GroupInvite(r) => r.user_id(),
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

	/// 判断是否为好友请求事件
	///
	/// # 返回值
	///
	/// 如果是好友相关的请求事件返回 `true`，否则返回 `false`
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_event::request::RequestEvent;
	///
	/// fn handle_request(event: &RequestEvent) {
	///     if event.is_friend() {
	///         println!("这是好友请求事件");
	///     }
	/// }
	/// ```
	pub fn is_friend(&self) -> bool {
		match self {
			Self::PrivateApply(r) => r.is_friend(),
			Self::GroupApply(r) => r.is_friend(),
			Self::GroupInvite(r) => r.is_friend(),
		}
	}

	/// 判断是否为群聊请求事件
	///
	/// # 返回值
	///
	/// 如果是群聊相关的请求事件返回 `true`，否则返回 `false`
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_event::request::RequestEvent;
	///
	/// fn handle_request(event: &RequestEvent) {
	///     if event.is_group() {
	///         println!("这是群聊请求事件");
	///     }
	/// }
	/// ```
	pub fn is_group(&self) -> bool {
		match self {
			Self::PrivateApply(r) => r.is_group(),
			Self::GroupApply(r) => r.is_group(),
			Self::GroupInvite(r) => r.is_group(),
		}
	}
}
/// 请求基础 trait
///
/// 定义所有请求事件的通用接口。
///
/// # 关联类型
///
/// - `Content`: 请求内容的具体类型
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_event::request::RequestBase;
///
/// fn process_request<R: RequestBase>(request: &R) {
///     println!("请求消息: {}", request.notion());
///     println!("事件 ID: {}", request.event_id());
/// }
/// ```
pub trait RequestBase: Send + Sync + EventBase {
	/// 请求内容类型
	type Content;

	/// 获取请求消息
	///
	/// # 返回值
	///
	/// 返回请求的文本消息内容
	fn notion(&self) -> &str;

	/// 获取请求内容
	///
	/// # 返回值
	///
	/// 返回请求的详细内容对象
	fn content(&self) -> &Self::Content;
}

/// 请求构建器
///
/// 用于构建请求事件的辅助结构。
#[derive(Debug, Clone)]
pub struct RequestBuilder<'r, Contact, Sender, Content>
where
	Contact: puniyu_contact::Contact,
	Sender: puniyu_sender::Sender,
{
	/// 机器人实例
	pub bot: &'r Bot,
	/// 事件 ID
	pub event_id: &'r str,
	/// 时间戳
	pub time: u64,
	/// 机器人 ID
	pub self_id: &'r str,
	/// 用户 ID
	pub user_id: &'r str,
	/// 联系人信息
	pub contact: &'r Contact,
	/// 发送者信息
	pub sender: &'r Sender,
	/// 请求内容
	pub content: &'r Content,
}
