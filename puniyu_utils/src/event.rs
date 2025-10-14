use crate::adapter::AdapterApi;
use crate::message::Message;
use strum::{Display, EnumString, IntoStaticStr};

mod bus;
mod handler;
mod matcher;
pub mod message;
pub use bus::init_event_bus;
pub use bus::{EVENT_BUS, Event, EventBus};

/// 事件类型枚举
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

impl EventType {
	/// 判断是否为消息事件
	pub fn is_message(&self) -> bool {
		matches!(self, EventType::Message)
	}

	/// 判断是否为通知事件
	pub fn is_notice(&self) -> bool {
		matches!(self, EventType::Notice)
	}

	/// 判断是否为请求事件
	pub fn is_request(&self) -> bool {
		matches!(self, EventType::Request)
	}

	/// 判断是否为未知事件
	pub fn is_unknown(&self) -> bool {
		matches!(self, EventType::Unknown)
	}
}

/// TODO:
///     - 发送消息
pub struct Context<'c> {
	pub bot: &'c dyn AdapterApi,
	pub user_id: &'c str,
	pub self_id: &'c str,
}

impl<'c> Context<'c> {
	pub fn reply(&self, message: Message) {}
}
