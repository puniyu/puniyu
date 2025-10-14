pub mod friend;

use crate::event::message::friend::FriendMessage;
use crate::message::element::receive::ReceiveElements;
use std::fmt::{Display, Formatter};
use strum::{Display, IntoStaticStr};

#[derive(Debug, Clone, Display, IntoStaticStr)]
pub enum MessageSubType {
	#[strum(serialize = "friend")]
	Friend,
	#[strum(serialize = "group")]
	Group,
	#[strum(serialize = "guild")]
	Guild,
}

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

	/// 机器人ID
	fn self_id(&self) -> &str;

	/// 用户ID
	fn get_user_id(&self) -> &str;

	/// 消息元素
	fn elements(&self) -> Vec<ReceiveElements>;

	/// message_id
	fn message_id(&self) -> &str;

	/// 获取艾特元素
	fn get_at(&self) -> Vec<String> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				ReceiveElements::At(at) => Some(at.target_id.to_string()),
				_ => None,
			})
			.collect()
	}

	fn get_at_all(&self) -> bool {
		self.elements().iter().any(|e| matches!(e, ReceiveElements::At(at) if at.is_all()))
	}
	fn get_at_bot(&self) -> bool {
		self.elements()
			.iter()
			.any(|e| matches!(e, ReceiveElements::At(at) if at.target_id.contains(self.self_id())))
	}

	fn get_at_online(&self) -> bool {
		self.elements().iter().any(|e| matches!(e, ReceiveElements::At(at) if at.is_online()))
	}

	/// 获取图片元素
	fn get_image(&self) -> Vec<String> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				ReceiveElements::Image(image) => Some(image.file.to_string()),
				_ => None,
			})
			.collect()
	}

	fn get_record(&self) -> Vec<String> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				ReceiveElements::Record(record) => Some(record.file.to_string()),
				_ => None,
			})
			.collect()
	}

	fn get_reply_id(&self) -> Option<String> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				ReceiveElements::Reply(reply) => Some(reply.message_id.to_string()),
				_ => None,
			})
			.next()
	}
}

impl Display for dyn MessageBase {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}
