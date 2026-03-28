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
/// # 示例
///
/// ```rust,ignore
/// use puniyu_event::notion::NotionBase;
///
/// fn process_notion<N>(notion: &N)
/// where
///     N: NotionBase,
/// {
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
	/// 获取通知的详细内容对象
	fn content(&self) -> Self::Content;
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

/// 生成通知事件结构体及其 EventBase、NotionBase 实现
///
/// # 参数
///
/// - `$name`: 结构体名称
/// - `$contact`: 联系人类型
/// - `$sender`: 发送者类型
/// - `$content`: 通知内容类型
/// - `$sub_event`: 通知子类型变体
/// - `$notion_str`: 通知描述字符串
macro_rules! codegen_notion {
	(
		$(#[$meta:meta])*
		$name:ident, $contact:ident, $sender:ident, $content:ty,
		$sub_event:expr, $notion_str:expr
	) => {
		$(#[$meta])*
		#[derive(Debug, Clone)]
		pub struct $name<'n> {
			bot: &'n puniyu_bot::Bot,
			event_id: &'n str,
			time: u64,
			user_id: &'n str,
			contact: &'n $contact<'n>,
			sender: &'n $sender<'n>,
			content: &'n $content,
		}

		impl<'n> $name<'n> {
			#[doc = concat!("使用 [`crate::notion::NotionBuilder`] 构建 [`", stringify!($name), "`]。")]
			pub fn new(builder: $crate::notion::NotionBuilder<'n, $contact<'n>, $sender<'n>, $content>) -> Self {
				Self {
					bot: builder.bot,
					event_id: builder.event_id,
					time: builder.time,
					user_id: builder.user_id,
					contact: builder.contact,
					sender: builder.sender,
					content: builder.content,
				}
			}
		}

		impl<'n> $crate::EventBase for $name<'n> {
			type EventType = $crate::EventType;
			type SubEventType = $crate::notion::NotionSubEventType;
			type Contact = $contact<'n>;
			type Sender = $sender<'n>;

			fn time(&self) -> u64 { self.time }
			fn event_type(&self) -> &$crate::EventType { &$crate::EventType::Notion }
			fn event_id(&self) -> &str { self.event_id }
			fn sub_event(&self) -> &$crate::notion::NotionSubEventType { &$sub_event }
			fn bot(&self) -> &puniyu_bot::Bot { self.bot }
			fn self_id(&self) -> &str { self.bot.account().uin.as_str() }
			fn user_id(&self) -> &str { self.user_id }
			fn contact(&self) -> &Self::Contact { self.contact }
			fn sender(&self) -> &Self::Sender { self.sender }
		}

		impl<'n> $crate::notion::NotionBase for $name<'n> {
			type Content = &'n $content;
			fn notion(&self) -> &str { $notion_str }
			fn content(&self) -> Self::Content { self.content }
		}
	};
}

pub(crate) use codegen_notion;
