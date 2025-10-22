mod friend;
mod group;

pub use friend::FriendMessage;
pub use group::GroupMessage;

use crate::EventBase;
use puniyu_element::Elements;
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

pub trait MessageBase: Send + Sync + EventBase {
	/// message_id
	fn message_id(&self) -> &str;
	/// 消息元素
	fn elements(&self) -> Vec<Elements>;

	/// 获取艾特元素
	fn get_at(&self) -> Vec<String> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::At(at) => Some(at.target_id.to_string()),
				_ => None,
			})
			.collect()
	}

	fn get_at_all(&self) -> bool {
		self.elements().iter().any(|e| matches!(e, Elements::At(at) if at.is_all()))
	}
	fn get_at_bot(&self) -> bool {
		self.elements()
			.iter()
			.any(|e| matches!(e, Elements::At(at) if at.target_id.contains(self.self_id())))
	}

	/// 获取图片元素
	fn get_image(&self) -> Vec<String> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::Image(image) => Some(image.file.to_string()),
				_ => None,
			})
			.collect()
	}

	fn get_record(&self) -> Vec<String> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::Record(record) => Some(record.file.to_string()),
				_ => None,
			})
			.collect()
	}

	fn get_reply_id(&self) -> Option<String> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::Reply(reply) => Some(reply.message_id.to_string()),
				_ => None,
			})
			.next()
	}
}
