use puniyu_event_message::MessageEvent;

pub enum Event {
	Message(MessageEvent),
}

impl Event {
	/// 判断是否为消息事件
	pub fn is_message(&self) -> bool {
		matches!(self, Event::Message(_))
	}
}
