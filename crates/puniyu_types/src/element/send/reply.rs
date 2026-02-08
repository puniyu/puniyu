use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyElement<'r> {
	/// 回复元素id
	pub message_id: &'r str,
}

impl<'r> ReplyElement<'r> {
	pub fn new(message_id: &'r str) -> Self {
		Self { message_id }
	}
}

impl<'r> From<&'r str> for ReplyElement<'r> {
	fn from(message_id: &'r str) -> Self {
		Self { message_id }
	}
}

impl<'r> From<ReplyElement<'r>> for String {
	fn from(elem: ReplyElement<'r>) -> Self {
		elem.message_id.to_string()
	}
}

impl<'r> From<ReplyElement<'r>> for &'r str {
	fn from(elem: ReplyElement<'r>) -> Self {
		elem.message_id
	}
}
