mod friend;
#[doc(inline)]
pub use friend::FriendMessage;
mod group;
#[doc(inline)]
pub use group::GroupMessage;
mod event;
#[doc(inline)]
pub use event::MessageEvent;

use super::EventBase;
use crate::bot::Bot;
use crate::element::receive::Elements;
use crate::event::EventType;
use bytes::Bytes;
use puniyu_config::Config;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(
	Debug,
	Clone,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
)]
pub enum MessageSubType {
	/// 好友消息
	#[strum(serialize = "friend")]
	Friend,
	/// 群消息
	#[strum(serialize = "group")]
	Group,
	/// 频道消息
	#[strum(serialize = "guild")]
	Guild,
}

pub trait MessageBase: Send + Sync + EventBase<EventType, MessageSubType> {
	/// message_id
	fn message_id(&self) -> &str;

	/// 是否为好友事件
	fn is_friend(&self) -> bool {
		matches!(self.sub_event(), MessageSubType::Friend)
	}

	/// 是否为群事件
	fn is_group(&self) -> bool {
		matches!(self.sub_event(), MessageSubType::Group)
	}
	/// 消息元素
	fn elements(&self) -> Vec<Elements>;

	/// 获取文本元素
	fn get_text(&self) -> String {
		self.elements()
			.into_iter()
			.filter_map(|e| match e {
				Elements::Text(text) => Some(text.text),
				_ => None,
			})
			.collect::<Vec<&str>>()
			.join("")
	}

	/// 获取艾特元素
	fn get_at(&self) -> Vec<String> {
		self
			.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::At(at) => Some(at.target_id.to_string()),
				_ => None,
			})
			.collect::<Vec<String>>()
	}

	/// 获取图片元素
	fn get_image(&self) -> Option<Bytes> {
		self.elements()
			.into_iter()
			.filter_map(|e| match e {
				Elements::Image(image) => Some(image.file.clone()),
				_ => None,
			})
			.next()
	}

	/// 获取语音元素
	fn get_record(&self) -> Option<Bytes> {
		self.elements()
			.into_iter()
			.filter_map(|e| match e {
				Elements::Record(record) => Some(record.file.clone()),
				_ => None,
			})
			.next()
	}

	/// 获取回复Id
	fn get_reply_id(&self) -> Option<String> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::Reply(reply) => Some(reply.message_id.to_string()),
				_ => None,
			})
			.next()
	}

	fn is_master(&self) -> bool {
		let config = Config::app();
		let masters = config.masters();
		masters.contains(&self.user_id().to_string())
	}
}

#[derive(Debug)]
pub struct MessageBuilder<'m, Contact, Sender>
where
	Contact: crate::contact::Contact,
	Sender: crate::sender::Sender,
{
	pub bot: &'m Bot,
	pub event_id: &'m str,
	pub self_id: &'m str,
	pub user_id: &'m str,
	pub contact: &'m Contact,
	pub sender: &'m Sender,
	pub time: u64,
	pub message_id: &'m str,
	pub elements: Vec<Elements<'m>>,
}