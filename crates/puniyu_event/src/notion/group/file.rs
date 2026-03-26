use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

use super::types::GroupFileUploadType;
use crate::notion::NotionSubEventType;

crate::notion::codegen_notion! {
	/// 群文件上传事件
	GroupFileUpload, GroupContact, GroupSender, GroupFileUploadType,
	NotionSubEventType::GroupFileUpload, "收到群聊文件上传事件"
}
