use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use super::types::GroupMemberTitleChangeType;
use crate::notion::NotionSubEventType;

crate::notion::codegen_notion! {
	/// 群成员头衔变动事件
	GroupMemberTitleChange, GroupContact, GroupSender, GroupMemberTitleChangeType,
	NotionSubEventType::GroupMemberTitleChange, "收到群成员头衔变动事件"
}
