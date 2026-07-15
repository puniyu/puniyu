use crate::{Element, ElementType};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AtElement {
	/// at元素目标id
	pub target_id: SmolStr,
}

impl AtElement {
	/// 是否为艾特全体
	pub fn is_everyone(&self) -> bool {
		self.target_id == "all"
	}
}
impl From<AtElement> for String {
	fn from(elem: AtElement) -> Self {
		elem.target_id.into()
	}
}

impl From<AtElement> for SmolStr {
	fn from(elem: AtElement) -> Self {
		elem.target_id
	}
}

impl From<&str> for AtElement {
	fn from(target_id: &str) -> Self {
		Self { target_id: target_id.into() }
	}
}

impl From<String> for AtElement {
	fn from(target_id: String) -> Self {
		Self { target_id: target_id.into() }
	}
}

impl From<SmolStr> for AtElement {
	fn from(target_id: SmolStr) -> Self {
		Self { target_id }
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
		let e: AtElement = "123456".into();
		assert_eq!(e.target_id, "123456");
		assert_eq!(e.r#type(), ElementType::At);
	}

	#[test]
	fn test_is_everyone() {
		assert!(AtElement::from("all").is_everyone());
		assert!(!AtElement::from("user_1").is_everyone());
	}

	#[test]
	fn test_from_and_into() {
		let from_str: AtElement = "u1".into();
		assert_eq!(from_str.target_id, "u1");

		let s: String = AtElement::from("u3").into();
		assert_eq!(s, "u3");

		let from_string: AtElement = String::from("u4").into();
		let s: SmolStr = from_string.into();
		assert_eq!(s, "u4");
	}
}
