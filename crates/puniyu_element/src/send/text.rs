use crate::{Element, ElementType};
use bon::Builder;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder)]
#[builder(on(SmolStr, into))]
pub struct TextElement {
	/// 文本元素内容
	pub text: SmolStr,
}

impl TextElement {
	pub fn new(text: impl Into<SmolStr>) -> Self {
		Self { text: text.into() }
	}
}

impl From<&str> for TextElement {
	fn from(text: &str) -> Self {
		Self::new(text)
	}
}

impl From<String> for TextElement {
	fn from(text: String) -> Self {
		Self::new(text)
	}
}

impl From<SmolStr> for TextElement {
	#[inline]
	fn from(text: SmolStr) -> Self {
		Self::new(text)
	}
}

impl From<TextElement> for String {
	fn from(element: TextElement) -> Self {
		element.text.to_string()
	}
}

impl From<TextElement> for SmolStr {
	#[inline]
	fn from(element: TextElement) -> Self {
		element.text
	}
}

impl AsRef<str> for TextElement {
	fn as_ref(&self) -> &str {
		self.text.as_ref()
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
		let e = TextElement::new("hello");
		assert_eq!(e.text, "hello");
		assert_eq!(e.r#type(), ElementType::Text);
	}

	#[test]
	fn test_type() {
		let e = TextElement::new("x");
		assert_eq!(e.r#type(), ElementType::Text);
	}

	#[test]
	fn test_from_str_and_string() {
		let from_str: TextElement = "hi".into();
		let from_string: TextElement = String::from("hi").into();
		assert_eq!(from_str.text, "hi");
		assert_eq!(from_string.text, "hi");
	}

	#[test]
	fn test_into_string_and_asref() {
		let e = TextElement::new("hello");
		let s: String = e.clone().into();
		assert_eq!(s, "hello");
		assert_eq!(e.as_ref(), "hello");
	}

	#[test]
	fn test_serde_roundtrip() {
		let e = TextElement::new("hello");
		let json = serde_json::to_string(&e).expect("serialize");
		let restored: TextElement = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.text, "hello");
	}
}
