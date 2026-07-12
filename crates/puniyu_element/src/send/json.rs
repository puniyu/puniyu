use serde::{Deserialize, Serialize};

use puniyu_core::element::Element;
use smol_str::SmolStr;

use crate::ElementType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JsonElement {
	/// Json数据，未序列化
	pub data: SmolStr,
}

impl JsonElement {
	pub fn new(data: impl Into<SmolStr>) -> Self {
		Self { data: data.into() }
	}
}

impl From<&str> for JsonElement {
	#[inline]
	fn from(data: &str) -> Self {
		Self::new(data)
	}
}

impl From<String> for JsonElement {
	#[inline]
	fn from(data: String) -> Self {
		Self::new(data)
	}
}

impl From<SmolStr> for JsonElement {
	#[inline]
	fn from(data: SmolStr) -> Self {
		Self::new(data)
	}
}

impl From<JsonElement> for String {
	#[inline]
	fn from(element: JsonElement) -> Self {
		element.data.to_string()
	}
}

impl From<JsonElement> for SmolStr {
	#[inline]
	fn from(element: JsonElement) -> Self {
		element.data
	}
}

impl AsRef<str> for JsonElement {
	fn as_ref(&self) -> &str {
		self.data.as_ref()
	}
}

impl Element for JsonElement {
	type ElementType = ElementType;
	fn r#type(&self) -> ElementType {
		ElementType::Json
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new_and_type() {
		let e = JsonElement::new(r#"{"a":1}"#);
		assert_eq!(e.data, r#"{"a":1}"#);
		assert_eq!(e.r#type(), ElementType::Json);
	}

	#[test]
	fn test_from_and_into() {
		let from_str: JsonElement = r#"{"x":2}"#.into();
		assert_eq!(from_str.data, r#"{"x":2}"#);
		let s: String = JsonElement::new("abc").into();
		assert_eq!(s, "abc");
	}

	#[test]
	fn test_serde_roundtrip() {
		let e = JsonElement::new(r#"{"k":"v"}"#);
		let json = serde_json::to_string(&e).expect("serialize");
		let restored: JsonElement = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.data, r#"{"k":"v"}"#);
	}
}