mod temp;
use puniyu_contact::Contact;
use puniyu_sender::Role;
#[doc(inline)]
pub use temp::GroupTempMessage;

use super::SubEventType;
use super::impl_message;
use ecow::EcoVec;
use puniyu_bot::Bot;
use puniyu_contact::GroupContact;
use puniyu_element::receive::Elements;
use puniyu_sender::{GroupSender, Sender};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupMessage<'m> {
	time: u64,
	event_id: &'m str,
	message_id: &'m str,
	bot: &'m Bot,
	elements: EcoVec<Elements<'m>>,
	contact: &'m GroupContact,
	sender: &'m GroupSender,
}

impl_message!(GroupMessage, GroupContact, GroupSender, SubEventType::Group);

impl GroupMessage<'_> {
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
