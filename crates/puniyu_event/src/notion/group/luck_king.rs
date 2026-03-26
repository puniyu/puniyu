use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use super::types::GroupLuckKingType;
use crate::notion::NotionSubEventType;

crate::notion::codegen_notion! {
	/// 群运气王事件
	GroupLuckKing, GroupContact, GroupSender, GroupLuckKingType,
	NotionSubEventType::GroupLuckKing, "收到群运气王事件"
}
