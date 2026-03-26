use puniyu_contact::FriendContact;
use puniyu_sender::FriendSender;

use crate::notion::{FriendAddType, NotionSubEventType};

crate::notion::codegen_notion! {
	/// 好友增加事件
	FriendAdd, FriendContact, FriendSender, FriendAddType,
	NotionSubEventType::FriendAdd, "收到好友增加事件"
}
