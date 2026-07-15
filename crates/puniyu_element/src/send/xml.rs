use crate::{Element, ElementType};
use bon::Builder;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder)]
#[builder(on(SmolStr, into))]
pub struct XmlElement {
	/// Xml数据，未序列化
	pub data: SmolStr,
}

impl XmlElement {
	pub fn new(data: impl Into<SmolStr>) -> Self {
		Self { data: data.into() }
	}
}

impl From<&str> for XmlElement {
	#[inline]
	fn from(data: &str) -> Self {
		Self::new(data)
	}
}

impl From<String> for XmlElement {
	#[inline]
	fn from(data: String) -> Self {
		Self::new(data)
	}
}

impl From<SmolStr> for XmlElement {
	#[inline]
	fn from(data: SmolStr) -> Self {
		Self::new(data)
	}
}

impl From<XmlElement> for String {
	#[inline]
	fn from(element: XmlElement) -> Self {
		element.data.to_string()
	}
}

impl From<XmlElement> for SmolStr {
	#[inline]
	fn from(element: XmlElement) -> Self {
		element.data
	}
}

impl AsRef<str> for XmlElement {
	fn as_ref(&self) -> &str {
		self.data.as_ref()
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
	fn test_new_and_type() {
		let e = XmlElement::new("<root/>");
		assert_eq!(e.data, "<root/>");
		assert_eq!(e.r#type(), ElementType::Xml);
	}

	#[test]
	fn test_from_and_into_and_asref() {
		let from_str: XmlElement = "<a/>".into();
		assert_eq!(from_str.data, "<a/>");
		let s: String = XmlElement::new("x").into();
		assert_eq!(s, "x");
		assert_eq!(XmlElement::new("y").as_ref(), "y");
	}
}
