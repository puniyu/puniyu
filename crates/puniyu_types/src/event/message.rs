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
	fn get_image(&self) -> Option<Vec<u8>> {
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
