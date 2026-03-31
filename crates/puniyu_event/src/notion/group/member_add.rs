use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use super::types::GroupMemberAddType;
use crate::notion::NotionSubEventType;

crate::notion::codegen_notion! {
	/// 群成员添加事件
	GroupMemberAdd, GroupContact, GroupSender, GroupMemberAddType,
	NotionSubEventType::GroupMemberAdd, "收到群成员添加事件"
}
