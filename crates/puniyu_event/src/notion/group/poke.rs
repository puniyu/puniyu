use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use super::types::GroupPokeType;
use crate::notion::NotionSubEventType;

crate::notion::codegen_notion! {
	/// 群聊戳一戳事件
	GroupPoke, GroupContact, GroupSender, GroupPokeType,
	NotionSubEventType::GroupPoke, "收到群聊戳一戳事件"
}
