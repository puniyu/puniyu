use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use super::types::GroupApplyType;
use crate::request::RequestSubEventType;

crate::request::codegen_request! {
	/// 群组加入申请事件
	GroupApply, GroupContact, GroupSender, GroupApplyType,
	RequestSubEventType::GroupApply, "收到群组加入申请"
}
