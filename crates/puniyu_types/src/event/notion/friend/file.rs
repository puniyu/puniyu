use crate::bot::Bot;
use crate::contact::FriendContact as Contact;
use crate::event::notion::{NotionBase, NotionBuilder, NotionSubEvent, PrivateFileUploadOption};
use crate::event::{EventBase, EventType};
use crate::sender::FriendSender as Sender;

#[derive(Debug, Clone)]
pub struct PrivateFileUpload<'n> {
    bot: &'n Bot,
    event_id: &'n str,
    time: u64,
    self_id: &'n str,
    user_id: &'n str,
    contact: &'n Contact,
    sender: &'n Sender,
    content: PrivateFileUploadOption,
}

impl<'n> PrivateFileUpload<'n> {
    pub fn new(
        notion_builder: NotionBuilder<'n, Contact, Sender>,
        content: PrivateFileUploadOption,
    ) -> Self {
        Self {
            bot: notion_builder.bot,
            event_id: notion_builder.event_id,
            time: notion_builder.time,
            self_id: notion_builder.self_id,
            user_id: notion_builder.user_id,
            contact: notion_builder.contact,
            sender: notion_builder.sender,
            content,
        }
    }
}

impl EventBase<EventType, NotionSubEvent> for PrivateFileUpload<'_> {
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

    fn sub_event(&self) -> &NotionSubEvent {
        &NotionSubEvent::PrivateFileUpload
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

impl NotionBase for PrivateFileUpload<'_> {
    type Content = PrivateFileUploadOption;
    fn notion(&self) -> &str {
        "收到好友撤回事件"
    }
    fn content(&self) -> &Self::Content {
        &self.content
    }
}
