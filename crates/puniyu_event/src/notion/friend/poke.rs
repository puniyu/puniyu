use puniyu_contact::FriendContact;
use puniyu_sender::FriendSender;

use crate::notion::{NotionSubEventType, PrivatePokeType};

crate::notion::codegen_notion! {
	/// 私聊戳一戳事件
	PrivatePoke, FriendContact, FriendSender, PrivatePokeType,
	NotionSubEventType::PrivatePoke, "收到好友戳一戳事件"
}
