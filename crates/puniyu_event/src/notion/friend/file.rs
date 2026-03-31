use puniyu_contact::FriendContact;
use puniyu_sender::FriendSender;

use crate::notion::{NotionSubEventType, PrivateFileUploadType};

crate::notion::codegen_notion! {
	/// 好友文件上传事件
	PrivateFileUpload, FriendContact, FriendSender, PrivateFileUploadType,
	NotionSubEventType::PrivateFileUpload, "收到好友文件上传事件"
}
