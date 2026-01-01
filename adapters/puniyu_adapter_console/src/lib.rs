mod api;
mod common;

use crate::common::make_random_id;
use async_trait::async_trait;
use puniyu_adapter::Result;
use puniyu_adapter::logger::info;
use puniyu_adapter::prelude::*;
use puniyu_core::APP_NAME;
use std::env;
use std::sync::Arc;

#[adapter]
struct Console;

#[async_trait]
impl AdapterBuilder for Console {
	fn name(&self) -> &'static str {
		env!("CARGO_PKG_NAME")
	}

	fn api(&self) -> &'static dyn AdapterApi {
		&api::ConsoleAdapterApi
	}

	async fn init(&self) -> Result<()> {
		use std::time::{SystemTime, UNIX_EPOCH};

		let adapter_info = adapter_info!(
			name: self.name(),
			version: self.version(),
			platform: AdapterPlatform::Other,
			standard: AdapterStandard::Other,
			protocol: AdapterProtocol::Console,
			communication: AdapterCommunication::Other
		);

		let bot_id = "console";
		let name = APP_NAME.get().unwrap();
		let account_info = account_info!(
			uin: bot_id,
			name: format!("{}/{}", name, bot_id),
			avatar: api::AVATAR_URL
		);
		register_bot!(adapter_info.clone(), account_info.clone(), self.api());

		info!("{} v{} 初始化完成", self.name(), self.version());

		let account_info = account_info.clone();
		let adapter_info = adapter_info.clone();
		let adapter = &api::ConsoleAdapterApi as &'static dyn AdapterApi;

		let bot = Arc::new(Bot {
			adapter: adapter_info.clone(),
			account: account_info.clone(),
			api: adapter,
		});

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
						vec![Elements::At(AtElement { target_id: target_id.to_string() })]
					}
					Some(("text", text_content)) => {
						vec![Elements::Text(TextElement { text: text_content.to_string() })]
					}
					Some(("image", image_url)) => {
						vec![Elements::Image(ImageElement {
							file: image_url.to_string().into(),
							height: 0,
							width: 0,
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
						})]
					}
					Some(("file", file_url)) => {
						vec![Elements::File(FileElement {
							file: file_url.to_string().into(),
							file_id: make_random_id(),
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

						create_message_event!(
							Group,
							bot: bot,
							event_id: event_id,
							contact: contact,
							self_id: bot_id,
							user_id: name,
							message_id: message_id,
							elements: elements,
							sender: sender,
							time: timestamp
						);
					}
					"friend" => {
						let contact = contact_friend!(name, name);
						let sender = friend_sender!(user_id: name, nick: name);
						create_message_event!(Friend,
							bot: bot,
							event_id: event_id,
							self_id: bot_id,
							user_id: name,
							message_id: message_id,
							elements: elements,
							sender: sender,
							contact: contact,
							time: timestamp
						);
					}
					_ => {
						let contact = contact_friend!(name, name);
						let sender = friend_sender!(user_id: name, nick: name);

						create_message_event!(Friend,
							bot: bot,
							event_id: event_id,
							self_id: bot_id,
							user_id: name,
							message_id: message_id,
							elements: elements,
							contact: contact,
							sender: sender,
							time: timestamp
						);
					}
				};
			}
		});

		Ok(())
	}
}
