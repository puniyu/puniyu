use crate::bot::Bot;
use async_trait::async_trait;
use prost::Message;
use puniyu_adapter::prelude::{ContactType, Scene, SendMsgType};
use puniyu_adapter::{AccountApi, Error, FriendApi, GroupApi, MessageApi, Result};
use puniyu_types::account::AccountInfo;
use puniyu_types::adapter::AdapterInfo;
use puniyu_types::contact::Contact;
use puniyu_types::element::send::Elements;
use puniyu_types::sender::SenderType;
use std::sync::Arc;

#[derive(Default)]
pub struct ServerMessageApi {
	pub(crate) bot: Option<Arc<Bot>>,
}

impl ServerMessageApi {
	fn get_bot(&self) -> Result<&Arc<Bot>> {
		self.bot.as_ref().ok_or_else(|| Error::Other(crate::error::Error::Session.to_string()))
	}

	fn create_bot_info(
		adapter: &Arc<AdapterInfo>,
		account: &Arc<AccountInfo>,
	) -> puniyu_protocol::bot::BotInfo {
		use puniyu_protocol::bot::BotInfo;
		BotInfo {
			adapter: Some((**adapter).clone().into()),
			account: Some((**account).clone().into()),
		}
	}

	fn create_friend_sender(
		sender: &puniyu_types::sender::FriendSender,
	) -> puniyu_protocol::sender::FriendSender {
		use puniyu_protocol::sender::{FriendSender, Sex};
		FriendSender {
			user_id: sender.user_id.clone(),
			nick: sender.nick.clone(),
			sex: Sex::from(sender.sex.clone()).into(),
			age: sender.age.map(|i| i.into()),
		}
	}

	fn create_group_sender(
		sender: &puniyu_types::sender::GroupSender,
	) -> puniyu_protocol::sender::GroupSender {
		use puniyu_protocol::sender::{GroupSender, Role, Sex};
		GroupSender {
			user_id: sender.user_id.clone(),
			nick: sender.nick.clone(),
			sex: Sex::from(sender.sex.clone()).into(),
			age: sender.age.map(|i| i.into()),
			role: Role::from(sender.role.clone()).into(),
			card: sender.card.clone(),
			level: sender.level.map(|i| i.into()),
			title: sender.title.clone(),
		}
	}

	fn build_contact(contact: &ContactType) -> puniyu_protocol::contact::Contact {
		use puniyu_protocol::contact::{Contact, SceneType};
		Contact {
			scene: SceneType::from(contact.scene()).into(),
			peer: String::from(contact.peer()),
			name: contact.name().map(|s| s.to_string()),
		}
	}
}

#[async_trait]
impl MessageApi for ServerMessageApi {
	async fn send_msg(
		&self,
		contact: ContactType,
		message: puniyu_adapter::prelude::Message,
	) -> Result<SendMsgType> {
		use puniyu_protocol::event::message::{MessageEventSend, message_event_send::MessageEvent};
		use puniyu_protocol::event::{EventSend, event_send};

		let bot = self.get_bot()?;
		let event = &bot.event;
		let message_id = Arc::clone(&event.message_id);
		let elements: Vec<Elements> = message.into();
		let contact_type = Self::build_contact(&contact);

		let message = match contact.scene() {
			Scene::Friend => {
				use puniyu_protocol::event::message::send::FriendMessage;
				let sender_type = if let SenderType::Friend(sender) = &event.sender {
					Self::create_friend_sender(sender)
				} else {
					return Err(Error::Other(crate::error::Error::Event.to_string()));
				};
				let bot_info = Self::create_bot_info(&bot.adapter, &bot.account);
				MessageEvent::FriendMessage(FriendMessage {
					friend_message_bot: Some(bot_info),
					event_id: event.event_id.to_string(),
					time: event.time,
					self_id: event.self_id.to_string(),
					user_id: event.user_id.to_string(),
					message_id: message_id.to_string(),
					elements: elements.into_iter().map(|e| e.into()).collect(),
					contact: Some(contact_type),
					sender: Some(sender_type),
				})
			}
			Scene::Group => {
				use puniyu_protocol::event::message::send::GroupMessage;
				let sender_type = if let SenderType::Group(sender) = &event.sender {
					Self::create_group_sender(sender)
				} else {
					return Err(Error::Other(crate::error::Error::Event.to_string()));
				};
				let bot_info = Self::create_bot_info(&bot.adapter, &bot.account);
				MessageEvent::GroupMessage(GroupMessage {
					group_message_bot: Some(bot_info),
					event_id: event.event_id.to_string(),
					time: event.time,
					self_id: event.self_id.to_string(),
					user_id: event.user_id.to_string(),
					message_id: message_id.to_string(),
					elements: elements.into_iter().map(|e| e.into()).collect(),
					contact: Some(contact_type),
					sender: Some(sender_type),
				})
			}
		};
		let message_event = MessageEventSend { message_event: Some(message) };
		let pb = EventSend { event: Some(event_send::Event::MessageEvent(message_event)) }
			.encode_to_vec();
		bot.session.lock().await.binary(pb).await.map_err(|e| Error::Other(e.to_string()))?;
		Ok(SendMsgType { message_id: message_id.to_string(), time: event.time })
	}
}

pub struct ServerAccountApi;

impl AccountApi for ServerAccountApi {}

pub struct ServerFriendApi;

impl FriendApi for ServerFriendApi {}

pub struct ServerGroupApi;
impl GroupApi for ServerGroupApi {}
