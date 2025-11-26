mod api;
mod common;

use crate::common::make_message_id;
use async_trait::async_trait;
use puniyu_adapter::Result;
use puniyu_adapter::logger::info;
use puniyu_adapter::prelude::*;
use puniyu_core::APP_NAME;
use std::env;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

static EVENT_ID: AtomicU64 = AtomicU64::new(0);

#[adapter]
struct Console;

#[async_trait]
impl AdapterBuilder for Console {
	fn info(&self) -> AdapterInfo {
		use std::time::SystemTime;
		use std::time::UNIX_EPOCH;
		let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
		adapter_info!(
			name: AdapterProtocol::Console.to_string(),
			platform: AdapterPlatform::Other,
			standard: AdapterStandard::Other,
			protocol: AdapterProtocol::Console,
			communication: AdapterCommunication::Other,
			connect_time: start_time
		)
	}

	fn api(&self) -> &'static dyn AdapterApi {
		&api::ConsoleAdapterApi
	}
	async fn init(&self) -> Result<()> {
		use std::time::{SystemTime, UNIX_EPOCH};

		let bot_id = "console";
		let name = APP_NAME.get().unwrap();
		let account_info = account_info!(
			uin: bot_id,
			name: format!("{}/{}", name, bot_id),
			avatar: api::AVATAR_URL
		);
		register_bot!(self.info(), account_info.clone(), self.api());

		info!("适配器: {} 初始化完成", self.info().name);

		let account_info = account_info.clone();
		let adapter_info = self.info().clone();
		thread::spawn(move || {
			loop {
				static FILE_ID: AtomicU64 = AtomicU64::new(0);
				let message = {
					let mut input = String::new();
					std::io::stdin().read_line(&mut input).unwrap();
					input.trim_end().to_string()
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
						vec![Elements::At(AtElement {
							target_id: target_id.to_string(),
							name: None,
						})]
					}
					Some(("text", text_content)) => {
						vec![Elements::Text(TextElement { text: text_content.to_string() })]
					}
					Some(("image", image_url)) => {
						vec![Elements::Image(ImageElement {
							file: Vec::from(image_url),
							is_flash: false,
							height: 0,
							width: 0,
							summary: None,
						})]
					}
					Some(("json", json_content)) => {
						vec![Elements::Json(JsonElement { data: json_content.to_string() })]
					}
					Some(("video", video_url)) => {
						vec![Elements::Video(VideoElement {
							file: Vec::from(video_url),
							file_name: AdapterProtocol::Console.to_string(),
						})]
					}
					Some(("record", record_url)) => {
						vec![Elements::Record(RecordElement { file: Vec::from(record_url) })]
					}
					Some(("file", file_url)) => {
						vec![Elements::File(FileElement {
							file: Vec::from(file_url),
							file_id: FILE_ID.fetch_add(1, Ordering::Relaxed).to_string(),
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
				let message_id = make_message_id();
				let adapter = &api::ConsoleAdapterApi as &'static dyn AdapterApi;
				let event_id = EVENT_ID.fetch_add(1, Ordering::Relaxed).to_string();

				let bot = Bot {
					adapter: adapter_info.clone(),
					account: account_info.clone(),
					api: adapter,
				};

				match msg_type {
					"group" => {
						let contact = contact_group!(name, name);
						let sender = group_sender!(user_id: name, nick: name, sex: Sex::Unknown, age: 0, role: Role::Member);

						create_group_message!(
							bot, event_id, contact, bot_id, name, message_id, elements, sender,
							timestamp
						);
					}
					"friend" => {
						let contact = contact_friend!(name, name);
						let sender =
							friend_sender!(user_id: name, nick: name, sex: Sex::Unknown, age: 0);

						create_friend_message!(
							bot, event_id, contact, bot_id, name, message_id, elements, sender,
							timestamp
						);
					}
					_ => {
						let contact = contact_friend!(name, name);
						let sender =
							friend_sender!(user_id: name, nick: name, sex: Sex::Unknown, age: 0);

						create_friend_message!(
							bot, event_id, contact, bot_id, name, message_id, elements, sender,
							timestamp
						);
					}
				};
			}
		});

		Ok(())
	}
}
