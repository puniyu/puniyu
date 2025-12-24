mod message;
use puniyu_types::event as puniyu_event;
use puniyu_types::event::message as puniyu_message;
include!(concat!(env!("OUT_DIR"), "/puniyu.event.rs"));

impl From<event::Event> for puniyu_types::event::Event {
	fn from(event: event::Event) -> Self {
		match event {
			event::Event::MessageEvent(message) => {
				let message: puniyu_message::MessageEvent =
					puniyu_message::MessageEvent::from(*message);
				Self::Message(Box::new(message))
			}
		}
	}
}

impl From<puniyu_event::Event> for event::Event {
	fn from(event: puniyu_event::Event) -> Self {
		match event {
			puniyu_event::Event::Message(message) => {
				let message: puniyu_message::MessageEvent = *message;
				Self::MessageEvent(Box::new(message.into()))
			}
			_ => panic!("Unsupported event type"),
		}
	}
}
