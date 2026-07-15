use bon::Builder;
use serde::{Deserialize, Serialize};

use puniyu_core::element::Element;
use smol_str::SmolStr;

use crate::ElementType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder)]
#[builder(on(SmolStr, into))]
pub struct AtElement {
	/// 目标id
	pub target_id: SmolStr,
}

impl AtElement {
	pub fn new(target_id: impl Into<SmolStr>) -> Self {
		Self { target_id: target_id.into() }
	}
}

impl From<&str> for AtElement {
	#[inline]
	fn from(target_id: &str) -> Self {
		Self::new(target_id)
	}
}

impl From<String> for AtElement {
	#[inline]
	fn from(target_id: String) -> Self {
		Self::new(target_id)
	}
}

impl From<SmolStr> for AtElement {
	#[inline]
	fn from(target_id: SmolStr) -> Self {
		Self::new(target_id)
	}
}

impl From<AtElement> for String {
	fn from(element: AtElement) -> Self {
		element.target_id.to_string()
	}
}

impl From<AtElement> for SmolStr {
	fn from(element: AtElement) -> Self {
		element.target_id
	}
}

impl AsRef<str> for AtElement {
	fn as_ref(&self) -> &str {
		self.target_id.as_ref()
	}
}

impl AtElement {
	/// 构造艾特全体元素
	#[inline]
	pub const fn everyone() -> Self {
		Self { target_id: SmolStr::new_static("all") }
	}
}

impl Element for AtElement {
	type ElementType = ElementType;
	fn r#type(&self) -> Self::ElementType {
		Self::ElementType::At
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new_and_type() {
		let e = AtElement::new("123456");
		assert_eq!(e.target_id, "123456");
		assert_eq!(e.r#type(), ElementType::At);
	}

	#[test]
	fn test_everyone_constructor() {
		let e = AtElement::everyone();
		assert_eq!(e.target_id, "all");
	}

	#[test]
	fn test_from_conversions() {
		let from_str: AtElement = "u1".into();
		assert_eq!(from_str.target_id, "u1");
		let from_str_ref = AtElement::from("u2");
		assert_eq!(from_str_ref.target_id, "u2");

		let from_string = AtElement::from(String::from("u3"));
		assert_eq!(from_string.target_id, "u3");

		let s: String = AtElement::new("u4").into();
		assert_eq!(s, "u4");
	}

	#[test]
	fn test_serde_roundtrip() {
		let e = AtElement::new("123456");
		let json = serde_json::to_string(&e).expect("serialize");
		let restored: AtElement = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.target_id, "123456");
	}
}
