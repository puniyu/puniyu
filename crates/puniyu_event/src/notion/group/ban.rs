use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use super::types::{GroupMemberBanType, GroupWholeBanType};
use crate::notion::NotionSubEventType;

crate::notion::codegen_notion! {
	/// 群成员禁言事件
	GroupMemberBan, GroupContact, GroupSender, GroupMemberBanType,
	NotionSubEventType::GroupMemberBan, "收到群成员禁言事件"
}

crate::notion::codegen_notion! {
	/// 群全体成员禁言事件
	GroupWholeBan, GroupContact, GroupSender, GroupWholeBanType,
	NotionSubEventType::GroupWholeBan, "收到群全体成员禁言事件"
}
