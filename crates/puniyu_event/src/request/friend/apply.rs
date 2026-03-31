use puniyu_contact::FriendContact;
use puniyu_sender::FriendSender;

use super::types::PrivateApplyType;
use crate::request::RequestSubEventType;

crate::request::codegen_request! {
	/// 好友申请事件
	PrivateApply, FriendContact, FriendSender, PrivateApplyType,
	RequestSubEventType::PrivateApply, "收到好友申请请求"
}
