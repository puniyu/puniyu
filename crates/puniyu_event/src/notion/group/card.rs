use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use super::types::GroupCardChangeType;
use crate::notion::NotionSubEventType;

crate::notion::codegen_notion! {
	/// 群成员名片变动事件
	GroupCardChange, GroupContact, GroupSender, GroupCardChangeType,
	NotionSubEventType::GroupCardChange, "收到群成员名片变动事件"
}
