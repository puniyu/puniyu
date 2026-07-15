use super::SubEventType;
use super::impl_message;
use ecow::EcoVec;
use puniyu_bot::Bot;
use puniyu_contact::Contact;
use puniyu_contact::GroupTempContact;
use puniyu_element::receive::Elements;
use puniyu_sender::Role;
use puniyu_sender::{GroupTempSender, Sender};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupTempMessage<'m> {
	time: u64,
	event_id: &'m str,
	message_id: &'m str,
	bot: &'m Bot,
	elements: EcoVec<Elements>,
	contact: &'m GroupTempContact,
	sender: &'m GroupTempSender,
}

impl_message!(GroupTempMessage, GroupTempContact, GroupTempSender, SubEventType::GroupTemp);

impl GroupTempMessage<'_> {
	/// 获取群 ID
	pub fn group_id(&self) -> &str {
		self.contact.peer()
	}

	/// 判断发送者是否为管理员
	pub fn is_admin(&self) -> bool {
		matches!(self.sender.role(), Role::Admin)
	}

	/// 判断发送者是否为群主
	pub fn is_owner(&self) -> bool {
		matches!(self.sender.role(), Role::Owner)
	}
}
