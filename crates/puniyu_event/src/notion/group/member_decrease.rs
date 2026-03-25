use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use super::types::GroupMemberDecreaseType;
use crate::notion::NotionSubEventType;

crate::notion::codegen_notion!{
	/// 群成员减少事件
	GroupMemberDecrease, GroupContact, GroupSender, GroupMemberDecreaseType,
	NotionSubEventType::GroupMemberDecrease, "收到群成员减少事件"
}
