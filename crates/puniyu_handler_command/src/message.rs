use puniyu_context::MessageContext;
use puniyu_element::receive::Elements;
use puniyu_event::{EventBase, message::MessageBase};
use puniyu_logger::info;
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
			"[Bot:{}] [群消息:{}-{}] {}",
			event.self_id(),
			event.group_id(),
			event.user_id(),
			raw_message
		);
	}
	info!("[Bot:{}] [好友消息:{}] {}", event.self_id(), event.user_id(), raw_message);
}
