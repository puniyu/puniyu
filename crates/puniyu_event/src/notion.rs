//! 通知事件模块
//!
//! 提供各类通知事件的类型定义，包括好友通知和群聊通知。

mod friend;
#[doc(inline)]
pub use friend::*;
mod group;
#[doc(inline)]
pub use group::*;
mod event;
#[doc(inline)]
pub use event::NotionEvent;
mod types;
#[doc(inline)]
pub use types::*;

use super::EventBase;
use puniyu_bot::Bot;
use serde::Deserialize;


/// 通知基础 trait
///
/// 定义所有通知事件的通用接口。
///
/// # 泛型参数
///
/// - `Content`: 通知内容的具体类型
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_event::notion::NotionBase;
///
/// fn process_notion<N, C>(notion: &N)
/// where
///     N: NotionBase<C>,
/// {
///     println!("通知消息: {}", notion.notion());
///     println!("事件 ID: {}", notion.event_id());
/// }
/// ```
pub trait NotionBase<Content>: Send + Sync + EventBase {
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
	/// 获取通知的详细内容对象引用
	fn content(&self) -> &Content;
}

/// 通知构建器
///
/// 用于构建通知事件的辅助结构。
pub struct NotionBuilder<'n, Contact, Sender, Content>
where
	Contact: puniyu_contact::Contact,
	Sender: puniyu_sender::Sender,
	Content: Deserialize<'n>,
{
	/// 机器人实例
	pub bot: &'n Bot,
	/// 事件 ID
	pub event_id: &'n str,
	/// 时间戳
	pub time: u64,
	/// 用户 ID
	pub user_id: &'n str,
	/// 联系人信息
	pub contact: &'n Contact,
	/// 发送者信息
	pub sender: &'n Sender,
	/// 通知内容
	pub content: &'n Content,
}
