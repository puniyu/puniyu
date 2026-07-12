use ecow::EcoVec;
use puniyu_bot::Bot;
use puniyu_contact::Contact;
use puniyu_element::receive::Elements;
use puniyu_contact::GuildContact;
use puniyu_sender::Role;
use puniyu_sender::{GuildSender, Sender};
use super::SubEventType;
use super::impl_message;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuildMessage<'m> {
    time: u64,
    event_id: &'m str,
    message_id: &'m str,
    bot: &'m Bot,
    elements: EcoVec<Elements<'m>>,
    contact: &'m GuildContact,
    sender: &'m GuildSender,
}

impl_message!(GuildMessage, GuildContact, GuildSender, SubEventType::Guild);

impl GuildMessage<'_> {
	/// 获取频道 ID
	pub fn guild_id(&self) -> &str {
		self.contact.peer()
	}

	/// 判断发送者是否为频道管理员
	pub fn is_admin(&self) -> bool {
		matches!(self.sender.role(), Role::Admin)
	}

	/// 判断发送者是否为频道主
	pub fn is_owner(&self) -> bool {
		matches!(self.sender.role(), Role::Owner)
	}
}