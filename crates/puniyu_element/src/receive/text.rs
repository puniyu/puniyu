use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::{Element, ElementType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TextElement {
	/// 文本元素内容
	pub text: SmolStr,
}

impl From<TextElement> for String {
	fn from(elem: TextElement) -> Self {
		elem.text.into()
	}
}

impl From<TextElement> for SmolStr {
	fn from(elem: TextElement) -> Self {
		elem.text
	}
}

impl From<&str> for TextElement {
	fn from(text: &str) -> Self {
		Self { text: text.into() }
	}
}

impl From<String> for TextElement {
	fn from(text: String) -> Self {
		Self { text: text.into() }
	}
}

impl From<SmolStr> for TextElement {
	fn from(text: SmolStr) -> Self {
		Self { text }
	}
}

impl Element for TextElement {
	type ElementType = ElementType;

	fn r#type(&self) -> Self::ElementType {
		Self::ElementType::Text
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

		let from_string: TextElement = String::from("中文内容").into();
		let s: SmolStr = from_string.into();
		assert_eq!(s, "中文内容");
	}

	#[test]
	fn test_long_text() {
		let text = "这是一段长度超过 SmolStr 内联容量的消息正文";
		let element = TextElement::from(text);
		assert_eq!(element.text, text);
	}

	#[test]
	fn test_serde_roundtrip() {
		let e: TextElement = "hi".into();
		let json = serde_json::to_string(&e).expect("serialize");
		let restored: TextElement = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.text, "hi");
	}
}
