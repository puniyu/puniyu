pub mod message;
pub mod notion;
mod permission;
pub mod request;

pub use permission::Permission;

use crate::bot::Bot;
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Clone)]
pub enum Event<'e> {
	Message(Box<message::MessageEvent<'e>>),
	Notion(Box<notion::NotionEvent<'e>>),
	Request(Box<request::RequestEvent<'e>>),
}

impl<'e> From<Event<'e>> for EventType {
	fn from(event: Event) -> Self {
		match event {
			Event::Message(_) => EventType::Message,
			Event::Notion(_) => EventType::Notion,
			Event::Request(_) => EventType::Request,
		}
	}
}
impl<'e> From<&Event<'e>> for EventType {
	fn from(event: &Event) -> Self {
		match event {
			Event::Message(_) => EventType::Message,
			Event::Notion(_) => EventType::Notion,
			Event::Request(_) => EventType::Request,
		}
	}
}

#[cfg(feature = "event")]
impl<'e> std::fmt::Debug for Event<'e> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Event::Message(message_event) => f.debug_tuple("Message").field(message_event).finish(),
			Event::Notion(notion_event) => f.debug_tuple("Notion").field(notion_event).finish(),
			Event::Request(request_event) => f.debug_tuple("Request").field(request_event).finish(),
		}
	}
}

impl<'e> Event<'e> {
	/// 获取事件类型
	pub fn event_type(&self) -> EventType {
		match self {
			Event::Message(_) => EventType::Message,
			Event::Notion(_) => EventType::Notion,
			Event::Request(_) => EventType::Request,
		}
	}
	/// 判断是否为消息事件
	pub fn is_message(&self) -> bool {
		matches!(self, Event::Message(..))
	}

	/// 判断是否为通知事件
	pub fn is_notice(&self) -> bool {
		matches!(self, Event::Notion(..))
	}

	/// 判断是否为请求事件
	pub fn is_request(&self) -> bool {
		matches!(self, Event::Request(..))
	}
}

#[derive(
	Debug, Default, Clone, EnumString, Display, IntoStaticStr, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum EventType {
	#[strum(serialize = "message")]
	Message,
	#[strum(serialize = "notion")]
	Notion,
	#[strum(serialize = "request")]
	Request,
	#[strum(serialize = "unknown")]
	#[default]
	Unknown,
}

pub trait EventBase<EventType, SubEventType>: Send + Sync
where
	EventType: PartialEq,
	SubEventType: PartialEq,
{
	type Contact: crate::contact::Contact;
	type Sender: crate::sender::Sender;

	/// 获取bot实例
	fn bot(&self) -> &Bot;
	/// 事件触发时间戳(秒）
	fn time(&self) -> u64;
	/// 事件类型
	fn event(&self) -> &EventType;
	/// 事件id
	fn event_id(&self) -> &str;
	/// 事件子类型
	fn sub_event(&self) -> &SubEventType;

	/// 机器人ID
	fn self_id(&self) -> &str;

	/// 用户ID
	fn user_id(&self) -> &str;

	/// 联系人
	fn contact(&self) -> &Self::Contact;

	/// 发送者
	fn sender(&self) -> &Self::Sender;
}
