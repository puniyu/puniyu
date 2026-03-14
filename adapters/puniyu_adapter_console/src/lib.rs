mod api;
mod common;

use crate::common::make_random_id;
use puniyu_adapter_api::prelude::*;
use std::env;
use std::sync::Arc;

fn info() -> AdapterInfo {
	adapter_info!(
		platform: AdapterPlatform::Other,
		standard: AdapterStandard::Other,
		protocol: AdapterProtocol::Console,
		communication: AdapterCommunication::Other
	)
}
#[adapter(info = info, api = api::api)]
async fn main() -> Result {
	use std::time::{SystemTime, UNIX_EPOCH};

	let bot_id = "console";
	let name = app::app_name();
	let account_info = account_info!(
		uin: bot_id,
		name: format!("{}/{}", name, bot_id),
		avatar: api::AVATAR.into()
	);
	register_bot!(Adapter, account_info.clone());

	info!("{} v{} 初始化完成", info().name, info().version);

	let bot = Arc::new(Bot::new(Adapter, account_info));

	std::thread::spawn(move || {
		loop {
			let message = {
				let mut input = String::new();
				match std::io::stdin().read_line(&mut input) {
					Ok(0) | Err(_) => break,
					Ok(_) => input.trim_end().to_string(),
				}
			};

			if matches!(message.as_str(), "quit" | "exit" | "q") {
				std::process::exit(0);
			}

			let (msg_type, content) = match message.split_once(':') {
				Some(("group", rest)) => ("group", rest.to_string()),
				Some(("friend", rest)) => ("friend", rest.to_string()),
				_ => ("friend", message.clone()),
			};

			let elements: Vec<Elements> = match content.split_once(':') {
				Some(("at", target_id)) if !target_id.is_empty() => {
					vec![Elements::At(AtElement { target_id })]
				}
				Some(("text", text_content)) => {
					vec![Elements::Text(TextElement { text: text_content.to_string() })]
				}
				Some(("image", image_url)) => {
					vec![Elements::Image(ImageElement {
						file: image_url.to_string().into(),
						summary: "图片".to_string(),
					})]
				}
				Some(("json", json_content)) => {
					vec![Elements::Json(JsonElement { data: json_content.to_string() })]
				}
				Some(("video", video_url)) => {
					vec![Elements::Video(VideoElement {
						file: video_url.to_string().into(),
						file_name: AdapterProtocol::Console.to_string(),
					})]
				}
				Some(("record", record_url)) => {
					vec![Elements::Record(RecordElement {
						file: record_url.to_string().into(),
						file_name: "语音",
					})]
				}
				Some(("file", file_url)) => {
					vec![Elements::File(FileElement {
						file: file_url.to_string().into(),
						file_name: AdapterProtocol::Console.to_string(),
						file_size: 100,
					})]
				}
				Some(("xml", xml_content)) => {
					vec![Elements::Xml(XmlElement { data: xml_content.to_string() })]
				}
				_ => {
					vec![Elements::Text(TextElement { text: content.to_string() })]
				}
			};

			let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
			let message_id = make_random_id();
			let event_id = make_random_id();

			match msg_type {
				"group" => {
					let contact = contact_group!(name, name);
					let sender = group_sender!(user_id: name, nick: name, sex: Sex::Unknown, age: 0, role: Role::Member);
					let builder = MessageBuilder {
						bot: &bot,
						event_id: &event_id,
						self_id: bot_id,
						user_id: &name,
						contact: &contact,
						sender: &sender,
						time: timestamp,
						message_id: &message_id,
						elements,
					};
					let event = GroupMessage::new(builder);
					send_event!(event)
				}
				"friend" => {
					let contact = contact_friend!(name, name);
					let sender = friend_sender!(user_id: name, nick: name);
					let builder = MessageBuilder {
						bot: &bot,
						event_id: &event_id,
						self_id: bot_id,
						user_id: &name,
						contact: &contact,
						sender: &sender,
						time: timestamp,
						message_id: &message_id,
						elements,
					};
					let event = FriendMessage::new(builder);
					send_event!(event)
				}
				_ => {
					let contact = contact_friend!(name, name);
					let sender = friend_sender!(user_id: name, nick: name);
					let builder = MessageBuilder {
						bot: &bot,
						event_id: &event_id,
						self_id: bot_id,
						user_id: &name,
						contact: &contact,
						sender: &sender,
						time: timestamp,
						message_id: &message_id,
						elements,
					};
					let event = GroupMessage::new(builder);
					send_event!(event)
				}
			};
		}
	});

	Ok(())
}
