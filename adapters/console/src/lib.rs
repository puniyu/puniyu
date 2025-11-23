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
		let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
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

				let elements = match content.split_once(':') {
					Some(("at", target_id)) if !target_id.is_empty() => {
						vec![element!(text, target_id)]
					}
					Some(("text", text_content)) => {
						vec![element!(text, text_content.to_string())]
					}
					Some(("image", image_url)) => {
						vec![element!(image, image_url)]
					}
					Some(("json", json_content)) => {
						vec![element!(json, json_content)]
					}
					Some(("video", video_url)) => {
						vec![element!(video, video_url, name)]
					}
					Some(("record", record_url)) => {
						vec![element!(record, record_url)]
					}
					Some(("file", file_url)) => {
						vec![element!(file, file_url, name, 100u64, name)]
					}
					Some(("xml", xml_content)) => {
						vec![element!(xml, xml_content)]
					}
					_ => {
						vec![element!(text, content)]
					}
				};

				let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
				let message_id = make_message_id();
				let adapter = &api::ConsoleAdapterApi as &'static dyn AdapterApi;
				let event_id = EVENT_ID.fetch_add(1, Ordering::Relaxed).to_string();

				let bot = Bot { adapter: adapter_info.clone(), account: account_info.clone(), api: adapter };

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
