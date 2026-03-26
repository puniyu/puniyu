use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use super::types::GroupHighlightsChangeType;
use crate::notion::NotionSubEventType;

crate::notion::codegen_notion! {
	/// 群精华消息变动事件
	GroupHighlightsChange, GroupContact, GroupSender, GroupHighlightsChangeType,
	NotionSubEventType::GroupHighlightsChange, "收到群精华消息变动事件"
}
