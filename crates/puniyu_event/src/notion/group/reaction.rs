use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use super::types::GroupMessageReactionType;
use crate::notion::NotionSubEventType;

crate::notion::codegen_notion! {
	/// 群消息表情动态事件
	GroupMessageReaction, GroupContact, GroupSender, GroupMessageReactionType,
	NotionSubEventType::GroupMessageReaction, "收到群消息表情动态事件"
}
