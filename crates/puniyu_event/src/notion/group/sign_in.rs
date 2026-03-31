use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use crate::notion::{GroupSignInType, NotionSubEventType};

crate::notion::codegen_notion! {
	/// 群成员打卡事件
	GroupSignIn, GroupContact, GroupSender, GroupSignInType,
	NotionSubEventType::GroupSignIn, "收到群成员打卡事件"
}
