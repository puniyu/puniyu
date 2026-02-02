use crate::api;
use actix_ws::Session;
use puniyu_adapter::prelude::*;
use puniyu_probuff::event::message::{MessageEventReceive, message_event_receive};
use puniyu_probuff::event::{EventReceive, event_receive};
use std::sync::Arc;
use tokio::sync::Mutex;

#[inline]
pub(crate) fn handle_event(event: EventReceive, session: &Arc<Mutex<Session>>) {
	if let Some(event) = event.event {
		match event {
			event_receive::Event::MessageEvent(message) => handel_message_event(message, session),
		};
	};
}

fn create_message_api(
	adapter: Arc<AdapterInfo>,
	account: Arc<AccountInfo>,
	session: &Arc<Mutex<Session>>,
	event: crate::bot::Event,
) -> Arc<api::ServerMessageApi> {
	Arc::new(api::ServerMessageApi {
		bot: Some(Arc::new(crate::bot::Bot {
			adapter,
			account,
			session: Arc::clone(session),
			event,
		})),
	})
}

fn create_adapter_bot(
	adapter: Arc<AdapterInfo>,
	account: Arc<AccountInfo>,
	message_api: Arc<api::ServerMessageApi>,
) -> Arc<Bot> {
	let group_api = Arc::new(api::ServerGroupApi);
	let friend_api = Arc::new(api::ServerFriendApi);
	let account_api = Arc::new(api::ServerAccountApi);

	Arc::new(Bot {
		adapter: (*adapter).clone(),
		api: AdapterApi::new(group_api, friend_api, account_api, message_api),
		account: (*account).clone(),
	})
}

fn handel_message_event(message: MessageEventReceive, session: &Arc<Mutex<Session>>) {
	if let Some(message) = message.message_event {
		match message {
			message_event_receive::MessageEvent::FriendMessage(message) => {
				let bot = message.friend_message_bot.unwrap_or_default();
				let adapter = Arc::new(bot.adapter.unwrap().into());
				let account = Arc::new(bot.account.unwrap().into());
				let event_id: Arc<str> = Arc::from(message.event_id.as_str());
				let self_id: Arc<str> = Arc::from(message.self_id.as_str());
				let user_id: Arc<str> = Arc::from(message.user_id.as_str());
				let time = message.time;
				let message_id: Arc<str> = Arc::from(message.message_id.as_str());
				let contact = message.contact.unwrap_or_default();
				let sender = message.sender.unwrap_or_default();
				let elements: Vec<_> =
					message.elements.into_iter().map(|element| element.into()).collect();

				let event = crate::bot::Event {
					event_id: Arc::clone(&event_id),
					self_id: Arc::clone(&self_id),
					user_id: Arc::clone(&user_id),
					contact: contact.clone().into(),
					sender: sender.clone().into(),
					time,
					message_id: Arc::clone(&message_id),
					elements: elements.clone(),
				};

				let message_api =
					create_message_api(Arc::clone(&adapter), Arc::clone(&account), session, event);
				let bot = create_adapter_bot(adapter, account, message_api);

				create_message_event!(Friend,
					{
						bot: bot,
						event_id: event_id.to_string(),
						self_id: self_id.to_string(),
						user_id: user_id.to_string(),
						contact: contact.into(),
						sender: sender.into(),
						time: time,
						message_id: message_id.to_string(),
						elements: elements,
					}
				)
			}
			message_event_receive::MessageEvent::GroupMessage(message) => {
				let bot = message.group_message_bot.unwrap_or_default();
				let adapter = Arc::new(bot.adapter.unwrap().into());
				let account = Arc::new(bot.account.unwrap().into());
				let event_id: Arc<str> = Arc::from(message.event_id.as_str());
				let self_id: Arc<str> = Arc::from(message.self_id.as_str());
				let user_id: Arc<str> = Arc::from(message.user_id.as_str());
				let time = message.time;
				let message_id: Arc<str> = Arc::from(message.message_id.as_str());
				let contact = message.contact.unwrap_or_default();
				let sender = message.sender.unwrap_or_default();
				let elements: Vec<_> =
					message.elements.into_iter().map(|element| element.into()).collect();

				let event = crate::bot::Event {
					event_id: Arc::clone(&event_id),
					self_id: Arc::clone(&self_id),
					user_id: Arc::clone(&user_id),
					contact: contact.clone().into(),
					sender: sender.clone().into(),
					time,
					message_id: Arc::clone(&message_id),
					elements: elements.clone(),
				};

				let message_api =
					create_message_api(Arc::clone(&adapter), Arc::clone(&account), session, event);
				let bot = create_adapter_bot(adapter, account, message_api);

				create_message_event!(Group,
					{
						bot: bot,
						event_id: event_id.to_string(),
						self_id: self_id.to_string(),
						user_id: user_id.to_string(),
						contact: contact.into(),
						sender: sender.into(),
						time: time,
						message_id: message_id.to_string(),
						elements: elements,
					}
				)
			}
		}
	};
}
