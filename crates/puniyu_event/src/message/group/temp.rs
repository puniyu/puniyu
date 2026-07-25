use std::sync::Arc;

use super::SubEventType;
use super::impl_message;
use ecow::EcoVec;
use puniyu_bot::Bot;
use puniyu_contact::Contact;
use puniyu_contact::GroupTempContact;
use puniyu_element::receive::Elements;
use puniyu_sender::Role;
use puniyu_sender::{GroupTempSender, Sender};
use smol_str::SmolStr;

#[derive(Debug, Clone)]
pub struct GroupTempMessage {
	time: u64,
	event_id: SmolStr,
	message_id: SmolStr,
	bot: Arc<dyn Bot>,
	elements: EcoVec<Elements>,
	contact: GroupTempContact,
	sender: GroupTempSender,
}

impl PartialEq for GroupTempMessage {
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

impl Eq for GroupTempMessage {}

impl_message!(GroupTempMessage, crate::EventType::Message, SubEventType::GroupTemp, GroupTempContact, GroupTempSender);

impl GroupTempMessage {
	/// 获取群 ID
	pub fn group_id(&self) -> &str {
		self.contact.peer()
	}

	/// 判断发送者是否为管理员
	pub fn is_admin(&self) -> bool {
		matches!(self.sender.role(), Some(Role::Admin))
	}

	/// 判断发送者是否为群主
	pub fn is_owner(&self) -> bool {
		matches!(self.sender.role(), Some(Role::Owner))
	}
}
