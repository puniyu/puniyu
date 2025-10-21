pub mod types;

use crate::types::AvatarSize;
use puniyu_contact::Contact;
use puniyu_element::Message;

pub trait AdapterApi: Send + Sync + 'static {
	fn send_msg(&self, contact: Contact, element: Message);
	/// 获取头像URL
	fn get_avatar_url(&self, user_id: &str, size: AvatarSize) -> String;

	/// 获取群头像URL
	fn get_group_avatar_url(&self, group_id: &str, size: AvatarSize) -> String;
}
