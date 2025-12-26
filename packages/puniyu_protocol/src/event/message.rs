use super::{FriendMessage, GroupMessage, MessageEvent, message_event};
use puniyu_types::event::message::{MessageBase, MessageBuilder};
use puniyu_types::event::{EventBase, message};

impl From<FriendMessage> for message::FriendMessage {
	fn from(message: FriendMessage) -> Self {
		let builder = MessageBuilder {
			bot: message.friend_message_bot.unwrap().into(),
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

impl From<message::FriendMessage> for FriendMessage {
	fn from(message: message::FriendMessage) -> Self {
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

impl From<GroupMessage> for message::GroupMessage {
	fn from(message: GroupMessage) -> Self {
		let builder = MessageBuilder {
			bot: message.group_message_bot.unwrap().into(),
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

impl From<message::GroupMessage> for GroupMessage {
	fn from(message: message::GroupMessage) -> Self {
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

impl From<message_event::MessageEvent> for message::MessageEvent {
	fn from(message: message_event::MessageEvent) -> Self {
		match message {
			message_event::MessageEvent::FriendMessage(friend) => {
				message::MessageEvent::Friend(friend.into())
			}
			message_event::MessageEvent::GroupMessage(group) => {
				message::MessageEvent::Group(group.into())
			}
		}
	}
}

impl From<message::MessageEvent> for message_event::MessageEvent {
	fn from(message: message::MessageEvent) -> Self {
		match message {
			message::MessageEvent::Friend(friend) => {
				message_event::MessageEvent::FriendMessage(friend.into())
			}
			message::MessageEvent::Group(group) => {
				message_event::MessageEvent::GroupMessage(group.into())
			}
		}
	}
}

impl From<MessageEvent> for message::MessageEvent {
	fn from(message: MessageEvent) -> Self {
		let message = message.message_event.unwrap();
		message.into()
	}
}


impl From<message::MessageEvent> for MessageEvent {
	fn from(message: message::MessageEvent) -> Self {
		Self { message_event: Some(message.into()) }
	}
}
