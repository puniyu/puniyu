use crate::{Element, ElementType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AtElement<'a> {
	/// at元素目标id
	#[serde(borrow)]
	pub target_id: &'a str,
}

impl<'a> AtElement<'a> {
	/// 是否为艾特全体
	pub fn is_everyone(&self) -> bool {
		self.target_id == "all"
	}
}
impl<'a> From<AtElement<'a>> for String {
	fn from(elem: AtElement<'a>) -> Self {
		elem.target_id.to_string()
	}
}

impl<'a> From<&'a str> for AtElement<'a> {
	fn from(target_id: &'a str) -> Self {
		Self { target_id }
	}
}

impl<'a> Element for AtElement<'a> {
	type ElementType = ElementType;

	fn r#type(&self) -> ElementType {
		ElementType::At
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
	}
}
