mod friend;
pub use friend::FriendMessage;
mod group;
pub use group::GroupMessage;

use super::EventBase;
use crate::element::receive::Elements;
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, EnumString, Display, IntoStaticStr)]
pub enum MessageSubType {
	#[strum(serialize = "friend")]
	Friend,
	#[strum(serialize = "group")]
	Group,
	#[strum(serialize = "guild")]
	Guild,
}

#[derive(Debug, Clone)]
pub enum MessageEvent {
	Friend(FriendMessage),
	Group(GroupMessage),
}

impl MessageEvent {
	pub fn is_friend(&self) -> bool {
		matches!(self, MessageEvent::Friend(_))
	}

	pub fn is_group(&self) -> bool {
		matches!(self, MessageEvent::Group(_))
	}

	pub fn as_friend(&self) -> Option<&FriendMessage> {
		match self {
			MessageEvent::Friend(msg) => Some(msg),
			_ => None,
		}
	}
	pub fn as_group(&self) -> Option<&GroupMessage> {
		match self {
			MessageEvent::Group(msg) => Some(msg),
			_ => None,
		}
	}

	pub fn time(&self) -> u64 {
		match self {
			MessageEvent::Friend(msg) => msg.time(),
			MessageEvent::Group(msg) => msg.time(),
		}
	}

	pub fn event(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.event(),
			MessageEvent::Group(msg) => msg.event(),
		}
	}

	pub fn event_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.event_id(),
			MessageEvent::Group(msg) => msg.event_id(),
		}
	}

	pub fn sub_event(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.sub_event(),
			MessageEvent::Group(msg) => msg.sub_event(),
		}
	}

	pub fn self_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.self_id(),
			MessageEvent::Group(msg) => msg.self_id(),
		}
	}

	pub fn user_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.user_id(),
			MessageEvent::Group(msg) => msg.user_id(),
		}
	}

	pub fn message_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.message_id(),
			MessageEvent::Group(msg) => msg.message_id(),
		}
	}

	pub fn elements(&self) -> Vec<Elements> {
		match self {
			MessageEvent::Friend(msg) => msg.elements(),
			MessageEvent::Group(msg) => msg.elements(),
		}
	}

	pub fn get_at(&self) -> Option<Vec<String>> {
		match self {
			MessageEvent::Friend(msg) => msg.get_at(),
			MessageEvent::Group(msg) => msg.get_at(),
		}
	}

	pub fn get_image(&self) -> Option<String> {
		match self {
			MessageEvent::Friend(msg) => msg.get_image(),
			MessageEvent::Group(msg) => msg.get_image(),
		}
	}

	pub fn get_record(&self) -> Option<Vec<u8>> {
		match self {
			MessageEvent::Friend(msg) => msg.get_record(),
			MessageEvent::Group(msg) => msg.get_record(),
		}
	}

	pub fn get_reply_id(&self) -> Option<String> {
		match self {
			MessageEvent::Friend(msg) => msg.get_reply_id(),
			MessageEvent::Group(msg) => msg.get_reply_id(),
		}
	}

	pub fn group_id(&self) -> Option<&str> {
		match self {
			MessageEvent::Group(msg) => Some(msg.group_id()),
			_ => None,
		}
	}

	pub fn is_master(&self) -> bool {
		match self {
			MessageEvent::Friend(msg) => msg.is_master(),
			MessageEvent::Group(msg) => msg.is_master(),
		}
	}
}

impl std::fmt::Display for MessageEvent {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MessageEvent::Friend(msg) => write!(f, "{}", msg),
			MessageEvent::Group(msg) => write!(f, "{}", msg),
		}
	}
}

pub trait MessageBase: Send + Sync + EventBase {
	/// message_id
	fn message_id(&self) -> &str;
	/// 消息元素
	fn elements(&self) -> Vec<Elements>;

	/// 获取艾特元素
	fn get_at(&self) -> Option<Vec<String>> {
		let at_list = self
			.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::At(at) => Some(at.target_id.to_string()),
				_ => None,
			})
			.collect::<Vec<String>>();
		if at_list.is_empty() { None } else { Some(at_list) }
	}

	/// 获取图片元素
	fn get_image(&self) -> Option<String> {
		self.elements()
			.into_iter()
			.filter_map(|e| match e {
				Elements::Image(image) => Some(image.file),
				_ => None,
			})
			.next()
	}

	/// 获取语音元素
	fn get_record(&self) -> Option<Vec<u8>> {
		self.elements()
			.into_iter()
			.filter_map(|e| match e {
				Elements::Record(record) => Some(record.file),
				_ => None,
			})
			.next()
	}

	/// 获取回复Id
	fn get_reply_id(&self) -> Option<String> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::Reply(reply) => Some(reply.message_id.clone()),
				_ => None,
			})
			.next()
	}

	fn is_master(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct MessageBuilder<Contact, Sender> {
	pub event_id: String,
	pub self_id: String,
	pub user_id: String,
	pub contact: Contact,
	pub sender: Sender,
	pub time: u64,
	pub message_id: String,
	pub elements: Vec<Elements>,
}
