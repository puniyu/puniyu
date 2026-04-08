mod common;
mod runtime;
pub use runtime::ConsoleRuntime as Runtime;

use crate::common::make_random_id;
use log::info;
use puniyu_adapter::app_name;
use puniyu_adapter::bot::get_bot;
use puniyu_adapter::element::receive::*;
use puniyu_adapter::event::send_event;
use puniyu_adapter::macros::*;
use puniyu_adapter::sender::{Role, Sex};
use puniyu_adapter::types::*;

const VERSION: puniyu_adapter::Version = pkg_version!();

#[inline]
fn info() -> AdapterInfo {
	adapter_info!(
		name: env!("CARGO_PKG_NAME"),
		version: VERSION,
		platform: AdapterPlatform::Other,
		standard: AdapterStandard::Other,
		protocol: AdapterProtocol::Console,
		communication: AdapterCommunication::Other
	)
}
#[adapter(info = info, runtime = runtime::runtime)]
async fn main() -> puniyu_adapter::Result {
	use std::time::{SystemTime, UNIX_EPOCH};

	let bot_id = "console";
	let name = app_name();
	let account_info = account_info!(
		uin: bot_id,
		name: format!("{}/{}", name, bot_id),
		avatar: runtime::AVATAR.clone()
	);
	let bot_index =
		register_bot!(adapter: info(), runtime: runtime::runtime(), account: account_info)?;

	info!("{} v{} 初始化完成", &info().name, info().version);

	let bot = get_bot(bot_index).expect("bot just registered");

	let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
	std::thread::spawn(move || {
		use std::io::BufRead;
		let stdin = std::io::stdin();
		for line in stdin.lock().lines() {
			match line {
				Ok(s) => {
					let _ = tx.send(s);
				}
				Err(_) => break,
			}
		}
	});
	tokio::spawn(async move {
		loop {
			let message = tokio::select! {
				_ = tokio::signal::ctrl_c() => break,
				msg = rx.recv() => match msg {
					Some(s) if !matches!(s.as_str(), "quit" | "exit" | "q") => s,
					_ => break,
				},
			};

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
					vec![Elements::Text(TextElement { text: text_content })]
				}
				Some(("image", image_url)) => {
					vec![Elements::Image(ImageElement {
						file: image_url.to_string().into(),
						file_name: "image.png",
						summary: "image",
						width: 100,
						height: 100,
					})]
				}
				Some(("json", json_content)) => {
					vec![Elements::Json(JsonElement { data: json_content })]
				}
				Some(("video", video_url)) => {
					vec![Elements::Video(VideoElement {
						file: video_url.to_string().into(),
						file_name: AdapterProtocol::Console.into(),
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
						file_size: 1024,
						file_name: AdapterProtocol::Console.into(),
					})]
				}
				Some(("xml", xml_content)) => {
					vec![Elements::Xml(XmlElement { data: xml_content })]
				}
				_ => {
					vec![Elements::Text(TextElement { text: content.as_str() })]
				}
			};

			let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
			let message_id = make_random_id();
			let event_id = make_random_id();

			match msg_type {
				"group" => {
					let contact = contact_group!(name, name);
					let sender = sender_group!(user_id: name, nick: name, sex: Sex::Unknown, age: 0, role: Role::Member);

					let event = create_event!(
						Message,
						create_message!(
							Group,
							crate_group_message!(
								bot: &bot,
								event_id: &event_id,
								user_id: &name,
								contact: &contact,
								sender: &sender,
								time: timestamp,
								message_id: &message_id,
								elements: &elements,
							)
						)
					);
					send_event(event).await
				}
				"friend" => {
					let contact = contact_friend!(name, name);
					let sender = sender_friend!(user_id: name, nick: name);
					let event = create_event!(
						Message,
						create_message!(
							Friend,
							crate_friend_message!(
								bot: &bot,
								event_id: &event_id,
								user_id: &name,
								contact: &contact,
								sender: &sender,
								time: timestamp,
								message_id: &message_id,
								elements: &elements,
							)
						)
					);
					send_event(event).await
				}
				_ => {
					let contact = contact_friend!(name, name);
					let sender = sender_friend!(user_id: name, nick: name);
					let event = create_event!(
						Message,
						create_message!(
							Friend,
							crate_friend_message!(
								bot: &bot,
								event_id: &event_id,
								user_id: &name,
								contact: &contact,
								sender: &sender,
								time: timestamp,
								message_id: &message_id,
								elements: &elements,
							)
						)
					);
					send_event(event).await
				}
			};
		}
	});

	Ok(())
}
