use puniyu_protocol::event::{EventSend, event_send};
use puniyu_protocol::event::message::{MessageEventSend, message_event_send};

#[inline]
pub(crate) fn handle_event(event: EventSend) {
    if let Some(event) = event.event {
        match event {
            event_send::Event::MessageEvent(message) => handel_message_event(message)
        };
    };
}

fn handel_message_event(message: MessageEventSend) {
    if let Some(message ) = message.message_event {
        match message {
            message_event_send::MessageEvent::FriendMessage(message ) => {
                println!("{:?}", message);
            },
            message_event_send::MessageEvent::GroupMessage(message ) => {
                println!("{:?}", message);
            }
        }
    }
}