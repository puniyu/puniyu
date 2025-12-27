pub mod send;
pub mod receive;

include!(concat!(env!("OUT_DIR"), "/puniyu.event.message.rs"));
use puniyu_types::event::message as puniyu_message;


impl From<message_event_receive::MessageEvent> for puniyu_message::MessageEvent {
    fn from(message: message_event_receive::MessageEvent) -> Self {
        match message {
            message_event_receive::MessageEvent::FriendMessage(friend) => {
                puniyu_message::MessageEvent::Friend(friend.into())
            }
            message_event_receive::MessageEvent::GroupMessage(group) => {
                puniyu_message::MessageEvent::Group(group.into())
            }
        }
    }
}

impl From<puniyu_message::MessageEvent> for message_event_receive::MessageEvent {
    fn from(message: puniyu_message::MessageEvent) -> Self {
        match message {
            puniyu_message::MessageEvent::Friend(friend) => {
                message_event_receive::MessageEvent::FriendMessage(friend.into())
            }
            puniyu_message::MessageEvent::Group(group) => {
                message_event_receive::MessageEvent::GroupMessage(group.into())
            }
        }
    }
}


impl From<MessageEventReceive> for puniyu_message::MessageEvent {
    fn from(message: MessageEventReceive) -> Self {
        match message.message_event.unwrap() {
            message_event_receive::MessageEvent::FriendMessage(message) => {
                puniyu_message::MessageEvent::Friend(message.into())
            }
            message_event_receive::MessageEvent::GroupMessage(message) => {
                puniyu_message::MessageEvent::Group(message.into())
            }
        }
    }
}



impl From<puniyu_message::MessageEvent> for MessageEventReceive {
    fn from(message: puniyu_message::MessageEvent) -> Self {
        Self { message_event: Some(message.into()) }
    }
}

