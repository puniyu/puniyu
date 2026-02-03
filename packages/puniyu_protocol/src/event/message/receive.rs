use puniyu_types::event::message as puniyu_message;
use puniyu_types::event::message::MessageBuilder;
use puniyu_types::event::{EventBase, message::MessageBase};
use std::sync::Arc;

include!(concat!(env!("OUT_DIR"), "/puniyu.event.message.receive.rs"));

impl From<FriendMessage> for puniyu_message::FriendMessage {
	fn from(message: FriendMessage) -> Self {
		let builder = MessageBuilder {
			bot: Arc::new(message.friend_message_bot.unwrap().into()),
			event_id: message.event_id,
			self_id: message.self_id,
			user_id: message.user_id,
			contact: message.contact.unwrap().into(),
			sender: message.sender.unwrap().into(),
			time: message.time,
			message_id: message.message_id,
			elements: message
				.elements
				.into_iter()
				.map(|element| element.into())
				.collect::<Vec<_>>(),
		};
		Self::new(builder)
	}
}

impl From<puniyu_message::FriendMessage> for FriendMessage {
	fn from(message: puniyu_message::FriendMessage) -> Self {
		let bot = message.bot().clone();
		Self {
			friend_message_bot: Some(bot.into()),
			event_id: message.event_id().to_string(),
			time: message.time(),
			self_id: message.self_id().to_string(),
			user_id: message.user_id().to_string(),
			message_id: message.message_id().to_string(),
			elements: message
				.elements()
				.into_iter()
				.map(|element| element.into())
				.collect::<Vec<_>>(),
			contact: Some(message.contact().into()),
			sender: Some(message.sender().into()),
		}
	}
}

impl From<GroupMessage> for puniyu_message::GroupMessage {
	fn from(message: GroupMessage) -> Self {
		let builder = MessageBuilder {
			bot: Arc::new(message.group_message_bot.unwrap_or_default().into()),
			event_id: message.event_id,
			self_id: message.self_id,
			user_id: message.user_id,
			contact: message.contact.unwrap().into(),
			sender: message.sender.unwrap().into(),
			time: message.time,
			message_id: message.message_id,
			elements: message
				.elements
				.into_iter()
				.map(|element| element.into())
				.collect::<Vec<_>>(),
		};
		Self::new(builder)
	}
}

impl From<puniyu_message::GroupMessage> for GroupMessage {
	fn from(message: puniyu_message::GroupMessage) -> Self {
		let bot = message.bot().clone();
		Self {
			group_message_bot: Some(bot.into()),
			event_id: message.event_id().to_string(),
			time: message.time(),
			self_id: message.self_id().to_string(),
			user_id: message.user_id().to_string(),
			message_id: message.message_id().to_string(),
			elements: message
				.elements()
				.into_iter()
				.map(|element| element.into())
				.collect::<Vec<_>>(),
			contact: Some(message.contact().into()),
			sender: Some(message.sender().into()),
		}
	}
}
