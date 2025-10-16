mod friend;

pub use friend::FriendMessage;
use serde::{Deserialize, Serialize};

use puniyu_element::Elements;
use puniyu_event_utils::contact::Contact;
use puniyu_event_utils::sender::Sender;
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
}

pub trait MessageBase: Send + Sync {
	/// 事件id
	fn event_id(&self) -> &str;

	/// 事件类型
	fn event(&self) -> &str;
	/// 子事件类型
	fn sub_event(&self) -> &str;

	/// 联系人
	fn contact(&self) -> Contact;

	/// 机器人ID
	fn self_id(&self) -> &str;

	/// 用户ID
	fn user_id(&self) -> &str;

	/// 发送者
	fn sender(&self) -> Sender;

	/// 消息元素
	fn elements(&self) -> Vec<Elements>;

	/// message_id
	fn message_id(&self) -> &str;

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

	fn get_at_online(&self) -> bool {
		self.elements().iter().any(|e| matches!(e, Elements::At(at) if at.is_online()))
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
