use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::{Element, ElementType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JsonElement {
	/// Json数据，未序列化
	pub data: SmolStr,
}

impl From<JsonElement> for String {
	fn from(elem: JsonElement) -> Self {
		elem.data.into()
	}
}

impl From<JsonElement> for SmolStr {
	fn from(elem: JsonElement) -> Self {
		elem.data
	}
}

impl From<&str> for JsonElement {
	fn from(data: &str) -> Self {
		Self { data: data.into() }
	}
}

impl From<String> for JsonElement {
	fn from(data: String) -> Self {
		Self { data: data.into() }
	}
}

impl From<SmolStr> for JsonElement {
	fn from(data: SmolStr) -> Self {
		Self { data }
	}
}

impl Element for JsonElement {
	type ElementType = ElementType;

	fn r#type(&self) -> Self::ElementType {
		Self::ElementType::Json
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_type() {
		let e: JsonElement = r#"{"a":1}"#.into();
		assert_eq!(e.r#type(), ElementType::Json);
	}

	#[test]
	fn test_from_and_into() {
		let e: JsonElement = r#"{"k":1}"#.into();
		assert_eq!(e.data, r#"{"k":1}"#);
		let s: String = JsonElement::from("abc").into();
		assert_eq!(s, "abc");
	}

	#[test]
	fn test_serde_roundtrip() {
		let e: JsonElement = "abc".into();
		let json = serde_json::to_string(&e).expect("serialize");
		assert!(json.contains("\"data\":\"abc\""));
	}
}
