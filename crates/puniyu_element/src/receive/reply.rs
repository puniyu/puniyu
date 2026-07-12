use serde::{Deserialize, Serialize};

use crate::{Element, ElementType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReplyElement<'r> {
	/// 回复元素id
	#[serde(borrow)]
	pub message_id: &'r str,
}

impl<'r> From<ReplyElement<'r>> for String {
	fn from(elem: ReplyElement<'r>) -> Self {
		elem.message_id.to_string()
	}
}

impl<'r> From<&'r str> for ReplyElement<'r> {
	fn from(text: &'r str) -> Self {
		Self { message_id: text }
	}
}

impl<'r> Element for ReplyElement<'r> {
	type ElementType = ElementType;

	fn r#type(&self) -> ElementType {
		ElementType::Reply
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_type() {
		let e: ReplyElement = "msg-1".into();
		assert_eq!(e.r#type(), ElementType::Reply);
	}

	#[test]
	fn test_from_and_into() {
		let e: ReplyElement = "m".into();
		assert_eq!(e.message_id, "m");
		let s: String = ReplyElement::from("x").into();
		assert_eq!(s, "x");
	}
}