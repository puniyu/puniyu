use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use super::types::GroupHonorChangeType;
use crate::notion::NotionSubEventType;

crate::notion::codegen_notion! {
	/// 群聊荣誉变更事件
	GroupHonorChange, GroupContact, GroupSender, GroupHonorChangeType,
	NotionSubEventType::GroupHonorChange, "收到群聊荣誉变更事件"
}
