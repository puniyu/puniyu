use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::{Element, ElementType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReplyElement {
	/// 回复元素id
	pub message_id: SmolStr,
}

impl From<ReplyElement> for String {
	fn from(elem: ReplyElement) -> Self {
		elem.message_id.into()
	}
}

impl From<ReplyElement> for SmolStr {
	fn from(elem: ReplyElement) -> Self {
		elem.message_id
	}
}

impl From<&str> for ReplyElement {
	fn from(message_id: &str) -> Self {
		Self { message_id: message_id.into() }
	}
}

impl From<String> for ReplyElement {
	fn from(message_id: String) -> Self {
		Self { message_id: message_id.into() }
	}
}

impl From<SmolStr> for ReplyElement {
	fn from(message_id: SmolStr) -> Self {
		Self { message_id }
	}
}

impl Element for ReplyElement {
	type ElementType = ElementType;

	fn r#type(&self) -> Self::ElementType {
		Self::ElementType::Reply
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

		let from_smol = ReplyElement::from(SmolStr::new("y"));
		let s: SmolStr = from_smol.into();
		assert_eq!(s, "y");
	}
}
