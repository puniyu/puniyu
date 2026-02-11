use puniyu_logger::{debug, info};
use puniyu_types::element::RawMessage;
use puniyu_types::event::EventBase;
use puniyu_types::event::message::{MessageBase, MessageEvent};

pub fn log(event: &MessageEvent) {
	match event {
		MessageEvent::Friend(m) => {
			debug!("收到好友消息: {:#?}", m.elements());
			info!("[Bot:{}] [好友消息:{}] {}", m.self_id(), m.user_id(), m.elements().raw());
		}
		MessageEvent::Group(m) => {
			debug!("收到群消息: {:#?}", m.elements());
			info!(
				"[Bot:{}] [群消息:{}-{}] {}",
				m.self_id(),
				m.group_id(),
				m.user_id(),
				m.elements().raw()
			);
		}
	}
}

pub fn is_at_bot(event: &MessageEvent) -> bool {
	let bot_id = event.self_id();
	event.elements().iter().any(|e| e.as_at().is_some_and(|at| at.target_id == bot_id))
}
