pub mod message;
use puniyu_types::event as puniyu_event;
use puniyu_types::event::message as puniyu_message;
include!(concat!(env!("OUT_DIR"), "/puniyu.event.rs"));

impl From<event_receive::Event> for puniyu_event::Event {
	fn from(event: event_receive::Event) -> Self {
		match event {
			event_receive::Event::MessageEvent(message) => {
				let message: puniyu_message::MessageEvent =
					puniyu_message::MessageEvent::from(message);
				Self::Message(Box::new(message))
			}
		}
	}
}

impl From<puniyu_event::Event> for event_receive::Event {
	fn from(event: puniyu_event::Event) -> Self {
		match event {
			puniyu_event::Event::Message(message) => {
				let message: puniyu_message::MessageEvent = *message;
				Self::MessageEvent(message.into())
			}
			_ => panic!("Unsupported event type"),
		}
	}
}

impl From<puniyu_event::Event> for EventReceive {
	fn from(event: puniyu_event::Event) -> Self {
		let event: event_receive::Event = event.into();
		Self { event: Some(event) }
	}
}

impl From<EventReceive> for puniyu_event::Event {
	fn from(event: EventReceive) -> Self {
		let event: event_receive::Event = event.event.unwrap();
		event.into()
	}
}

