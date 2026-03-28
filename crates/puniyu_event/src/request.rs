//! 请求事件模块
//!
//! 提供好友申请、群申请等请求事件的类型定义。

mod friend;
#[doc(inline)]
pub use friend::*;
mod group;
#[doc(inline)]
pub use group::*;
mod event;
#[doc(inline)]
pub use event::RequestEvent;
mod types;
#[doc(inline)]
pub use types::*;

use super::EventBase;
use puniyu_bot::Bot;

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
///     println!("请求消息: {}", request.request());
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
	fn request(&self) -> &str;
	/// 获取请求内容
	///
	/// # 返回值
	///
	/// 返回请求的详细内容对象
	fn content(&self) -> Self::Content;
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
	/// 用户 ID
	pub user_id: &'r str,
	/// 联系人信息
	pub contact: &'r Contact,
	/// 发送者信息
	pub sender: &'r Sender,
	/// 请求内容
	pub content: &'r Content,
}

/// 生成请求事件结构体及其 EventBase、RequestBase 实现
///
/// # 参数
///
/// - `$name`: 结构体名称
/// - `$contact`: 联系人类型
/// - `$sender`: 发送者类型
/// - `$content`: 请求内容类型
/// - `$sub_event`: 请求子类型变体
/// - `$request_str`: 请求描述字符串
macro_rules! codegen_request {
	(
		$(#[$meta:meta])*
		$name:ident, $contact:ident, $sender:ident, $content:ty,
		$sub_event:expr, $request_str:expr
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
			#[doc = concat!("使用 [`crate::request::RequestBuilder`] 构建 [`", stringify!($name), "`]。")]
			pub fn new(builder: $crate::request::RequestBuilder<'n, $contact<'n>, $sender<'n>, $content>) -> Self {
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
			type SubEventType = $crate::request::RequestSubEventType;
			type Contact = $contact<'n>;
			type Sender = $sender<'n>;

			fn time(&self) -> u64 { self.time }
			fn event_type(&self) -> &$crate::EventType { &$crate::EventType::Request }
			fn event_id(&self) -> &str { self.event_id }
			fn sub_event(&self) -> &$crate::request::RequestSubEventType { &$sub_event }
			fn bot(&self) -> &puniyu_bot::Bot { self.bot }
			fn self_id(&self) -> &str { self.bot.account().uin.as_str() }
			fn user_id(&self) -> &str { self.user_id }
			fn contact(&self) -> &Self::Contact { self.contact }
			fn sender(&self) -> &Self::Sender { self.sender }
		}

		impl<'n> $crate::request::RequestBase for $name<'n> {
			type Content = &'n $content;
			fn request(&self) -> &str { $request_str }
			fn content(&self) -> Self::Content { self.content }
		}
	};
}
pub(crate) use codegen_request;
