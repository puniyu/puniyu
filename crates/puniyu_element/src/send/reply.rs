use serde::{Deserialize, Serialize};
use crate::{ElementType, RawMessage};

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

impl<'r> RawMessage for ReplyElement<'r> {
	fn r#type(&self) -> ElementType {
		ElementType::Reply
	}

	fn raw(&self) -> String {
		self.message_id.to_string()
	}
}