use std::sync::Arc;

use super::SubEventType;
use super::impl_message;
use ecow::EcoVec;
use puniyu_bot::Bot;
use puniyu_contact::FriendContact;
use puniyu_element::receive::Elements;
use puniyu_sender::{FriendSender, Sender};
use smol_str::SmolStr;

#[derive(Debug, Clone)]
pub struct FriendMessage {
	time: u64,
	event_id: SmolStr,
	message_id: SmolStr,
	bot: Arc<dyn Bot>,
	elements: EcoVec<Elements>,
	contact: FriendContact,
	sender: FriendSender,
}

impl PartialEq for FriendMessage {
	fn eq(&self, other: &Self) -> bool {
		self.time == other.time
			&& self.event_id == other.event_id
			&& self.message_id == other.message_id
			&& *self.bot == *other.bot
			&& self.elements == other.elements
			&& self.contact == other.contact
			&& self.sender == other.sender
	}
}

impl Eq for FriendMessage {}

impl_message!(FriendMessage, crate::EventType::Message, SubEventType::Friend, FriendContact, FriendSender);
