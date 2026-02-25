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
