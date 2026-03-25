use puniyu_contact::FriendContact;
use puniyu_sender::FriendSender;

use crate::notion::{FriendDecreaseType, NotionSubEventType};

crate::notion::codegen_notion! {
	/// 好友减少事件
	FriendDecrease, FriendContact, FriendSender, FriendDecreaseType,
	NotionSubEventType::FriendDecrease, "收到好友减少事件"
}
