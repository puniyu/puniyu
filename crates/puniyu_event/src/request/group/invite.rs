use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use super::types::GroupInviteType;
use crate::request::RequestSubEventType;

crate::request::codegen_request! {
	/// 群组邀请事件
	GroupInvite, GroupContact, GroupSender, GroupInviteType,
	RequestSubEventType::GroupInvite, "收到群组邀请"
}
