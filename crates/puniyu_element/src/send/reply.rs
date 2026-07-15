use crate::{Element, ElementType};
use bon::Builder;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder)]
#[builder(on(SmolStr, into))]
pub struct ReplyElement {
	/// 回复元素id
	pub message_id: SmolStr,
}

impl ReplyElement {
	pub fn new(message_id: impl Into<SmolStr>) -> Self {
		Self { message_id: message_id.into() }
	}
}

impl From<&str> for ReplyElement {
	#[inline]
	fn from(message_id: &str) -> Self {
		Self::new(message_id)
	}
}

impl From<String> for ReplyElement {
	#[inline]
	fn from(message_id: String) -> Self {
		Self::new(message_id)
	}
}

impl From<SmolStr> for ReplyElement {
	#[inline]
	fn from(message_id: SmolStr) -> Self {
		Self::new(message_id)
	}
}

impl From<ReplyElement> for String {
	#[inline]
	fn from(element: ReplyElement) -> Self {
		element.message_id.to_string()
	}
}

impl From<ReplyElement> for SmolStr {
	#[inline]
	fn from(element: ReplyElement) -> Self {
		element.message_id
	}
}

impl AsRef<str> for ReplyElement {
	fn as_ref(&self) -> &str {
		self.message_id.as_ref()
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
	fn test_new_and_type() {
		let e = ReplyElement::new("msg-001");
		assert_eq!(e.message_id, "msg-001");
		assert_eq!(e.r#type(), ElementType::Reply);
	}

	#[test]
	fn test_from_and_into_and_asref() {
		let from_str: ReplyElement = "m".into();
		assert_eq!(from_str.message_id, "m");
		let s: String = ReplyElement::new("x").into();
		assert_eq!(s, "x");
		assert_eq!(ReplyElement::new("a").as_ref(), "a");
	}
}
