use super::SubEventType;
use super::impl_message;
use ecow::EcoVec;
use puniyu_bot::Bot;
use puniyu_contact::FriendContact;
use puniyu_element::receive::Elements;
use puniyu_sender::{FriendSender, Sender};
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FriendMessage {
	time: u64,
	event_id: SmolStr,
	message_id: SmolStr,
	bot: Bot,
	elements: EcoVec<Elements>,
	contact: FriendContact,
	sender: FriendSender,
}

impl_message!(FriendMessage, FriendContact, FriendSender, SubEventType::Friend);
