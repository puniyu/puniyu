use crate::bot::Bot;
use crate::contact::FriendContact as Contact;
use crate::element::receive::Elements;
use crate::event::message::{MessageBase, MessageBuilder, MessageSubType};
use crate::event::{EventBase, EventType};
use crate::sender::{FriendSender as Sender};
#[derive(Debug, Clone)]
pub struct FriendMessage<'m> {
    bot: &'m Bot,
    event_id: &'m str,
    time: u64,
    self_id: &'m str,
    user_id: &'m str,
    message_id: &'m str,
    elements: Vec<Elements<'m>>,
    contact: &'m Contact,
    sender: &'m Sender,
}

impl<'m> FriendMessage<'m> {
    pub fn new(builder: MessageBuilder<'m, Contact, Sender>) -> Self {
        Self {
            bot: builder.bot,
            event_id: builder.event_id,
            time: builder.time,
            self_id: builder.self_id,
            user_id: builder.user_id,
            message_id: builder.message_id,
            elements: builder.elements,
            contact: builder.contact,
            sender: builder.sender,
        }
    }
}

impl<'e> EventBase<EventType, MessageSubType> for FriendMessage<'e> {
    type Contact = Contact;
    type Sender = Sender;

    fn bot(&self) -> &Bot {
        self.bot
    }

    fn time(&self) -> u64 {
        self.time
    }

    fn event(&self) -> &EventType {
        &EventType::Message
    }

    fn event_id(&self) -> &str {
        self.event_id
    }

    fn sub_event(&self) -> &MessageSubType {
        &MessageSubType::Group
    }

    fn self_id(&self) -> &str {
        self.self_id
    }

    fn user_id(&self) -> &str {
        self.user_id
    }

    fn contact(&self) -> &Self::Contact {
        &self.contact
    }

    fn sender(&self) -> &Self::Sender {
        &self.sender
    }
}

impl<'m> MessageBase for FriendMessage<'m> {
    fn message_id(&self) -> &str {
        &self.message_id
    }

    fn elements(&self) -> Vec<Elements> {
        self.elements.clone()
    }
}
