use serde::{Deserialize, Serialize};

use crate::{Element, ElementType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct XmlElement<'x> {
	/// Xml数据，未序列化
	#[serde(borrow)]
	pub data: &'x str,
}

impl<'t> From<XmlElement<'t>> for String {
	fn from(elem: XmlElement<'t>) -> Self {
		elem.data.to_string()
	}
}

impl<'t> From<&'t str> for XmlElement<'t> {
	fn from(text: &'t str) -> Self {
		Self { data: text }
	}
}

impl<'x> Element for XmlElement<'x> {
	type ElementType = ElementType;

	fn r#type(&self) -> ElementType {
		ElementType::Xml
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