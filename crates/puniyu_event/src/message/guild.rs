use super::SubEventType;
use super::impl_message;
use ecow::EcoVec;
use puniyu_bot::Bot;
use puniyu_contact::Contact;
use puniyu_contact::GuildContact;
use puniyu_element::receive::Elements;
use puniyu_sender::Role;
use puniyu_sender::{GuildSender, Sender};
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuildMessage {
	time: u64,
	event_id: SmolStr,
	message_id: SmolStr,
	bot: Bot,
	elements: EcoVec<Elements>,
	contact: GuildContact,
	sender: GuildSender,
}

impl_message!(GuildMessage, GuildContact, GuildSender, SubEventType::Guild);

impl GuildMessage {
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
