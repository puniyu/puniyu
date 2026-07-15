use super::SubEventType;
use super::impl_message;
use ecow::EcoVec;
use puniyu_bot::Bot;
use puniyu_contact::FriendContact;
use puniyu_element::receive::Elements;
use puniyu_sender::{FriendSender, Sender};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FriendMessage<'m> {
	time: u64,
	event_id: &'m str,
	message_id: &'m str,
	bot: &'m Bot,
	elements: EcoVec<Elements>,
	contact: &'m FriendContact,
	sender: &'m FriendSender,
}

impl_message!(FriendMessage, FriendContact, FriendSender, SubEventType::Friend);
