use log::info;
use puniyu_context::MessageContext;
use puniyu_element::receive::Elements;
use puniyu_event::{EventBase, message::MessageBase};
use puniyu_logger::owo_colors::OwoColorize;
pub fn log(event: &MessageContext) {
	let raw_message = event
		.elements()
		.iter()
		.map(|e| match e {
			Elements::Text(t) => format!("text:{}", t.text),
			Elements::At(a) => format!("at:{}", a.target_id),
			Elements::Reply(r) => format!("reply:{}", r.message_id),
			Elements::Face(f) => format!("face:{}", f.id),
			Elements::Image(i) => format!("image:{}", i.summary),
			Elements::File(f) => format!("file:{}", f.file_name),
			Elements::Video(v) => format!("video:{}", v.file_name),
			Elements::Record(r) => format!("record:{}", r.file_name),
			Elements::Json(j) => format!("json:{}", j.data),
			Elements::Xml(x) => format!("xml:{}", x.data),
		})
		.collect::<Vec<_>>()
		.join("");
	if let Some(event) = event.as_group() {
		info!(
			"[{}:{}][{}:{}-{}]: {}",
			"Bot".yellow(),
			event.self_id().green(),
			"GroupMessage".yellow(),
			event.group_id().green(),
			event.user_id().green(),
			raw_message
		);
		return;
	}
	if let Some(event) = event.as_group_temp() {
		info!(
			"[{}:{}][{}:{}-{}]: {}",
			"Bot".yellow(),
			event.self_id().green(),
			"GroupTempMessage".yellow(),
			event.group_id().green(),
			event.user_id().green(),
			raw_message
		);
		return;
	}
	if let Some(event) = event.as_guild() {
		info!(
			"[{}:{}][{}:{}-{}]: {}",
			"Bot".yellow(),
			event.self_id().green(),
			"GuildMessage".yellow(),
			event.guild_id().green(),
			event.user_id().green(),
			raw_message
		);
		return;
	}
	info!(
		"[{}:{}][{}:{}]: {}",
		"Bot".yellow(),
		event.self_id().green(),
		"FriendMessage".yellow(),
		event.user_id().green(),
		raw_message
	);
}
