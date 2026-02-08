use crate::bot::Bot;
use crate::contact::FriendContact as Contact;
use crate::event::{EventBase, EventType};
use crate::event::request::{RequestBuilder, RequestBase, RequestSubEvent};
use crate::event::request::friend::types::PrivateApplyType;
use crate::sender::FriendSender as Sender;

#[derive(Debug, Clone)]
pub struct PrivateApply<'n> {
    bot: &'n Bot,
    event_id: &'n str,
    time: u64,
    self_id: &'n str,
    user_id: &'n str,
    contact: &'n Contact,
    sender: &'n Sender,
    content: PrivateApplyType,
}

impl<'n> PrivateApply<'n> {
    pub fn new(
        request_builder: RequestBuilder<'n, Contact, Sender>,
        content: PrivateApplyType,
    ) -> Self {
        Self {
            bot: request_builder.bot,
            event_id: request_builder.event_id,
            time: request_builder.time,
            self_id: request_builder.self_id,
            user_id: request_builder.user_id,
            contact: request_builder.contact,
            sender: request_builder.sender,
            content,
        }
    }
}

impl EventBase<EventType, RequestSubEvent> for PrivateApply<'_> {
    type Contact = Contact;
    type Sender = Sender;

    fn bot(&self) -> &Bot {
        &self.bot
    }

    fn time(&self) -> u64 {
        self.time
    }

    fn event(&self) -> &EventType {
        &EventType::Notion
    }

    fn event_id(&self) -> &str {
        &self.event_id
    }

    fn sub_event(&self) -> &RequestSubEvent {
        &RequestSubEvent::PrivateApply
    }

    fn self_id(&self) -> &str {
        &self.self_id
    }

    fn user_id(&self) -> &str {
        &self.user_id
    }

    fn contact(&self) -> &Self::Contact {
        &self.contact
    }

    fn sender(&self) -> &Self::Sender {
        &self.sender
    }
}

impl RequestBase for PrivateApply<'_> {
    type Content = PrivateApplyType;

    fn notion(&self) -> &str {
        "收到好友申请请求"
    }

    fn content(&self) -> &Self::Content {
        &self.content
    }
}
