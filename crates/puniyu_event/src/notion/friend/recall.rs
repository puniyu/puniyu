use puniyu_contact::FriendContact;
use puniyu_sender::FriendSender;

use crate::notion::{NotionSubEventType, PrivateRecallType};

crate::notion::codegen_notion! {
	/// 私聊撤回事件
	PrivateRecall, FriendContact, FriendSender, PrivateRecallType,
	NotionSubEventType::PrivateRecall, "收到好友撤回事件"
}
