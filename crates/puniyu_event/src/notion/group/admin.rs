use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use super::types::GroupAdminChangeType;
use crate::notion::NotionSubEventType;

crate::notion::codegen_notion! {
	/// 群聊管理员变动事件
	GroupAdminChange, GroupContact, GroupSender, GroupAdminChangeType,
	NotionSubEventType::GroupAdminChange, "收到群聊管理员变动事件"
}
