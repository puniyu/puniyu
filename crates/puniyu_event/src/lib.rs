#[cfg(feature = "context")]
pub mod context;
#[cfg(any(feature = "message", feature = "event"))]
pub mod message;
#[cfg(feature = "notion")]
pub mod notion;
#[cfg(feature = "request")]
pub mod request;

use strum::{Display, EnumString, IntoStaticStr};

#[cfg(feature = "event")]
#[derive(Clone)]
pub enum Event {
	Message(Box<message::MessageEvent>),
	Notion(Box<notion::NotionEvent>),
	Request(Box<request::RequestEvent>),
}

#[cfg(feature = "event")]
impl std::fmt::Debug for Event {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Event::Message(message_event) => f.debug_tuple("Message").field(message_event).finish(),
			Event::Notion(notion_event) => f.debug_tuple("Notion").field(notion_event).finish(),
			Event::Request(request_event) => f.debug_tuple("Request").field(request_event).finish(),
		}
	}
}

#[cfg(feature = "message")]
impl Event {
	/// 判断是否为消息事件
	pub fn is_message(&self) -> bool {
		matches!(self, Event::Message(..))
	}
}

#[derive(Debug, Clone, EnumString, Display, IntoStaticStr)]
pub enum EventType {
	#[strum(serialize = "message")]
	Message,
	#[strum(serialize = "notice")]
	Notice,
	#[strum(serialize = "request")]
	Request,
	#[strum(serialize = "unknown")]
	Unknown,
}

#[cfg(feature = "event")]
pub trait EventBase: Send + Sync {
	type ContactType;
	type SenderType;
	/// 事件触发时间戳(秒）
	fn time(&self) -> u64;
	/// 事件类型
	fn event(&self) -> &str;
	/// 事件id
	fn event_id(&self) -> &str;
	/// 子事件类型
	fn sub_event(&self) -> &str;

	/// 机器人ID
	fn self_id(&self) -> &str;

	/// 用户ID
	fn user_id(&self) -> &str;

	/// 联系人
	fn contact(&self) -> Self::ContactType;

	/// 发送者
	fn sender(&self) -> Self::SenderType;
}
