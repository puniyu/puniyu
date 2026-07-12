use serde::{Deserialize, Serialize};

use crate::{Element, ElementType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JsonElement<'j> {
	/// Json数据，未序列化
	#[serde(borrow)]
	pub data: &'j str,
}

impl<'t> From<JsonElement<'t>> for String {
	fn from(elem: JsonElement<'t>) -> Self {
		elem.data.to_string()
	}
}

impl<'t> From<&'t str> for JsonElement<'t> {
	fn from(text: &'t str) -> Self {
		Self { data: text }
	}
}

impl<'j> Element for JsonElement<'j> {
	type ElementType = ElementType;

	fn r#type(&self) -> ElementType {
		ElementType::Json
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