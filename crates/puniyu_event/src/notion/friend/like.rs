use puniyu_contact::FriendContact;
use puniyu_sender::FriendSender;

use super::types::ReceiveLikeType;
use crate::notion::NotionSubEventType;

crate::notion::codegen_notion! {
	/// 收到点赞事件
	ReceiveLike, FriendContact, FriendSender, ReceiveLikeType,
	NotionSubEventType::ReceiveLike, "收到点赞事件"
}
