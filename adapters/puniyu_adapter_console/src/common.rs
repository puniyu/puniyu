use crate::input::{ConsolePayload, ParsedConsoleInput};
use puniyu_adapter::bot::Bot;
use puniyu_adapter::contact::SceneType;
use puniyu_adapter::element::receive::*;
use puniyu_adapter::event::send_event;
use puniyu_adapter::macros::*;
use puniyu_adapter::sender::{Role, Sex};
use puniyu_adapter::types::*;
use rand::distr::{Alphanumeric, SampleString};
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn make_random_id() -> String {
	Alphanumeric.sample_string(&mut rand::rng(), 32)
}

const DEFAULT_GUILD_ID: &str = "test_guild";
const DEFAULT_GUILD_NAME: &str = "test_guild";
const DEFAULT_GUILD_SUB_NAME: &str = "test_channel";

pub async fn dispatch_event(bot: &Bot, input: &ParsedConsoleInput, default_name: &str) {
	let elements = build_elements(&input.payload);
	let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
	let message_id = make_random_id();
	let event_id = make_random_id();

	match input.scene {
		SceneType::Friend => {
			create_friend_event(bot, default_name, &event_id, timestamp, &message_id, &elements)
				.await
		}
		SceneType::Group => {
			create_group_event(bot, default_name, &event_id, timestamp, &message_id, &elements)
				.await
		}
		SceneType::GroupTemp => {
			create_group_temp_event(bot, default_name, &event_id, timestamp, &message_id, &elements)
				.await
		}
		SceneType::Guild => {
			create_guild_event(bot, default_name, &event_id, timestamp, &message_id, &elements)
				.await
		}
	}
}

fn build_elements(payload: &ConsolePayload) -> Vec<Elements<'_>> {
	match payload {
		ConsolePayload::At(target_id) => vec![Elements::At(AtElement { target_id })],
		ConsolePayload::Text(text) => vec![Elements::Text(TextElement { text })],
		ConsolePayload::Image(image_url) => vec![Elements::Image(ImageElement {
			file: image_url.as_str().to_string().into(),
			file_name: "image.png",
			summary: "image",
			width: 100,
			height: 100,
		})],
		ConsolePayload::Json(json_content) => {
			vec![Elements::Json(JsonElement { data: json_content })]
		}
		ConsolePayload::Video(video_url) => vec![Elements::Video(VideoElement {
			file: video_url.as_str().to_string().into(),
			file_name: AdapterProtocol::Console.into(),
		})],
		ConsolePayload::Record(record_url) => vec![Elements::Record(RecordElement {
			file: record_url.as_str().to_string().into(),
			file_name: "语音",
		})],
		ConsolePayload::File(file_url) => vec![Elements::File(FileElement {
			file: file_url.as_str().to_string().into(),
			file_size: 1024,
			file_name: AdapterProtocol::Console.into(),
		})],
		ConsolePayload::Xml(xml_content) => vec![Elements::Xml(XmlElement { data: xml_content })],
	}
}

async fn create_friend_event<'a>(
	bot: &'a Bot,
	default_name: &'a str,
	event_id: &'a str,
	timestamp: u64,
	message_id: &'a str,
	elements: &'a Vec<Elements<'a>>,
) {
	let contact = contact_friend!(default_name, default_name);
	let sender = sender_friend!(user_id: default_name, nick: default_name);
	let event = create_event!(
		Message,
		create_message!(
			Friend,
			crate_friend_message!(
				bot: bot,
				event_id: event_id,
				user_id: default_name,
				contact: &contact,
				sender: &sender,
				time: timestamp,
				message_id: message_id,
				elements: elements,
			)
		)
	);
	send_event(event).await;
}

async fn create_group_event<'a>(
	bot: &'a Bot,
	default_name: &'a str,
	event_id: &'a str,
	timestamp: u64,
	message_id: &'a str,
	elements: &'a Vec<Elements<'a>>,
) {
	let contact = contact_group!(default_name, default_name);
	let sender = sender_group!(user_id: default_name, nick: default_name, sex: Sex::Unknown, age: 0, role: Role::Member);
	let event = create_event!(
		Message,
		create_message!(
			Group,
			crate_group_message!(
				bot: bot,
				event_id: event_id,
				user_id: default_name,
				contact: &contact,
				sender: &sender,
				time: timestamp,
				message_id: message_id,
				elements: elements,
			)
		)
	);
	send_event(event).await;
}

async fn create_group_temp_event<'a>(
	bot: &'a Bot,
	default_name: &'a str,
	event_id: &'a str,
	timestamp: u64,
	message_id: &'a str,
	elements: &'a Vec<Elements<'a>>,
) {
	let contact = contact_group_temp!(default_name, default_name);
	let sender = sender_group_temp!(user_id: default_name, nick: default_name, sex: Sex::Unknown, age: 0, role: Role::Member);
	let event = create_event!(
		Message,
		create_message!(
			GroupTemp,
			crate_group_temp_message!(
				bot: bot,
				event_id: event_id,
				user_id: default_name,
				contact: &contact,
				sender: &sender,
				time: timestamp,
				message_id: message_id,
				elements: elements,
			)
		)
	);
	send_event(event).await;
}

async fn create_guild_event<'a>(
	bot: &'a Bot,
	default_name: &'a str,
	event_id: &'a str,
	timestamp: u64,
	message_id: &'a str,
	elements: &'a Vec<Elements<'a>>,
) {
	let contact = contact_guild!(peer: DEFAULT_GUILD_ID, name: DEFAULT_GUILD_NAME, sub_name: DEFAULT_GUILD_SUB_NAME);
	let sender = sender_guild!(user_id: default_name, nick: default_name, sex: Sex::Unknown, age: 0, role: Role::Member);
	let event = create_event!(
		Message,
		create_message!(
			Guild,
			crate_guild_message!(
				bot: bot,
				event_id: event_id,
				user_id: default_name,
				contact: &contact,
				sender: &sender,
				time: timestamp,
				message_id: message_id,
				elements: elements,
			)
		)
	);
	send_event(event).await;
}
