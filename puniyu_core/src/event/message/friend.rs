use crate::event::EventType;
use crate::event::message::{MessageBase, MessageSubType};
use crate::message::element::receive::ReceiveElements;

pub struct FriendMessage {
    event_id: String,
    self_id: String,
    message_id: String,
    elements: Vec<ReceiveElements>,
    msg: Option<String>,
    raw_message: Option<String>,
}

impl FriendMessage {
    pub fn new(
        event_id: String,
        self_id: String,
        message_id: String,
        elements: Vec<ReceiveElements>,
    ) -> Self {
        Self {
            event_id,
            self_id,
            message_id,
            elements,
            msg: None,
            raw_message: None,
        }
    }
}

impl MessageBase for FriendMessage {
    fn event_id(&self) -> &str {
        &self.event_id
    }

    fn event(&self) -> &str {
        EventType::Message.into()
    }

    fn sub_event(&self) -> &str {
        MessageSubType::Friend.into()
    }

    fn self_id(&self) -> &str {
        &self.self_id
    }

    fn elements(&self) -> &Vec<ReceiveElements> {
        &self.elements
    }

    fn msg(&self) -> Option<&str> {
        self.msg.as_deref()
    }

    fn raw_message(&self) -> Option<&str> {
        self.raw_message.as_deref()
    }

    fn message_id(&self) -> &str {
        &self.message_id
    }
}
