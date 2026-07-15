use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::{Element, ElementType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct XmlElement {
	/// Xml数据，未序列化
	pub data: SmolStr,
}

impl From<XmlElement> for String {
	fn from(elem: XmlElement) -> Self {
		elem.data.into()
	}
}

impl From<XmlElement> for SmolStr {
	fn from(elem: XmlElement) -> Self {
		elem.data
	}
}

impl From<&str> for XmlElement {
	fn from(data: &str) -> Self {
		Self { data: data.into() }
	}
}

impl From<String> for XmlElement {
	fn from(data: String) -> Self {
		Self { data: data.into() }
	}
}

impl From<SmolStr> for XmlElement {
	fn from(data: SmolStr) -> Self {
		Self { data }
	}
}

impl Element for XmlElement {
	type ElementType = ElementType;

	fn r#type(&self) -> Self::ElementType {
		Self::ElementType::Xml
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_type() {
		let e: XmlElement = "<root/>".into();
		assert_eq!(e.r#type(), ElementType::Xml);
	}

	#[test]
	fn test_from_and_into() {
		let e: XmlElement = "<a/>".into();
		assert_eq!(e.data, "<a/>");
		let s: String = XmlElement::from("x").into();
		assert_eq!(s, "x");
	}
}
