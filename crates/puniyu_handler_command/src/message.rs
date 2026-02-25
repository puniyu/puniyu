use puniyu_context::MessageContext;
use puniyu_event::EventBase;
use puniyu_event::message::RawMessage;
use puniyu_logger::info;

pub fn log(event: &MessageContext) {
	let raw_message = event.elements().iter().map(|e| e.raw()).collect::<Vec<_>>().join("");
	if let Some(event) = event.as_group() {
		info!(
			"[Bot:{}] [群消息:{}-{}] {}",
			event.self_id(),
			event.group_id(),
			event.user_id(),
			raw_message
		);
	} else {
		info!("[Bot:{}] [好友消息:{}] {}", event.self_id(), event.user_id(), raw_message);
	}
}
