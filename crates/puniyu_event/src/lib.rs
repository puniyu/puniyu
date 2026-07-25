pub mod message;
mod types;
#[doc(inline)]
pub use types::*;
use std::sync::Arc;

use ecow::EcoVec;
use puniyu_bot::Bot;
use puniyu_contact::{Contact, ContactType};
use puniyu_element::receive::Elements;
use puniyu_sender::{SenderType, Sender};

#[derive(Debug, Clone)]
pub enum Event {
	Message(message::MessageEvent),
}

impl Event {
	#[allow(unreachable_patterns)]
	pub fn as_message(&self) -> Option<&message::MessageEvent> {
		match self {
			Self::Message(event) => Some(event),
			_ => None,
		}
	}
}

macro_rules! forward_event {
	($name:ident -> $ret:ty) => {
		impl Event {
			pub fn $name(&self) -> $ret {
				match self {
					Self::Message(e) => e.$name(),
				}
			}
		}
	};
}

forward_event!(time -> u64);
forward_event!(event_id -> &str);
impl Event {
	pub fn event_type(&self) -> crate::EventType {
		match self {
			Self::Message(e) => e.event_type(),
		}
	}
	pub fn sub_event(&self) -> SubEventType {
		match self {
			Self::Message(e) => SubEventType::Message(e.sub_event()),
		}
	}
}
forward_event!(bot -> Arc<dyn Bot>);
forward_event!(user_id -> &str);
forward_event!(contact -> ContactType);
forward_event!(sender -> SenderType);
forward_event!(message_id -> &str);
forward_event!(elements -> &EcoVec<Elements>);
forward_event!(get_text -> Vec<&str>);
forward_event!(get_at -> Vec<&str>);
forward_event!(get_reply_id -> Option<&str>);


pub trait EventBase: Send + Sync + PartialEq + Eq {
	type Contact: Contact;
	type Sender: Sender;
	type EventType: Copy;
	type SubEventType: Copy;

	/// 获取事件触发时间戳（秒）
	fn time(&self) -> u64;

	/// 获取事件类型。
	fn event_type(&self) -> Self::EventType;

	/// 获取事件 ID。
	fn event_id(&self) -> &str;

	/// 获取事件子类型。
	fn sub_event(&self) -> Self::SubEventType;

	/// 获取机器人实例。
	fn bot(&self) -> Arc<dyn Bot>;

	/// 获取机器人ID
	fn self_id(&self) -> &str;

	/// 获取用户ID
	fn user_id(&self) -> &str;

	/// 获取联系人信息
	fn contact(&self) -> Self::Contact;

	/// 获取发送者信息
	fn sender(&self) -> Self::Sender;
}