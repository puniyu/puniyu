use serde::{Deserialize, Serialize};
use crate::{Element, ElementType};

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

impl<'r> Element for ReplyElement<'r> {
	fn r#type(&self) -> ElementType {
		ElementType::Reply
	}
}