use serde::{Deserialize, Serialize};

use crate::{Element, ElementType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TextElement<'t> {
	/// 文本元素内容
	#[serde(borrow)]
	pub text: &'t str,
}

impl<'t> From<TextElement<'t>> for String {
	fn from(elem: TextElement<'t>) -> Self {
		elem.text.to_string()
	}
}

impl<'t> From<&'t str> for TextElement<'t> {
	fn from(text: &'t str) -> Self {
		Self { text }
	}
}

impl<'t> Element for TextElement<'t> {
	type ElementType = ElementType;

	fn r#type(&self) -> ElementType {
		ElementType::Text
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new_and_type() {
		let e: TextElement = "hello".into();
		assert_eq!(e.text, "hello");
		assert_eq!(e.r#type(), ElementType::Text);
	}

	#[test]
	fn test_from_and_into() {
		let from_str: TextElement = "abc".into();
		assert_eq!(from_str.text, "abc");

		let s: String = TextElement::from("abc").into();
		assert_eq!(s, "abc");
	}

	#[test]
	fn test_serde_roundtrip() {
		let e: TextElement = "hi".into();
		let json = serde_json::to_string(&e).expect("serialize");
		let restored: TextElement = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.text, "hi");
	}
}