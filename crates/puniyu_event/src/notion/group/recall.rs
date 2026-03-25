use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use super::types::GroupRecallType;
use crate::notion::NotionSubEventType;

crate::notion::codegen_notion! {
	/// 群聊撤回事件
	GroupRecall, GroupContact, GroupSender, GroupRecallType,
	NotionSubEventType::GroupRecall, "收到群聊撤回事件"
}
