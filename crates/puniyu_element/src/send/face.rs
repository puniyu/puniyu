use serde::{Deserialize, Serialize};

use puniyu_core::element::Element;

use crate::ElementType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FaceElement {
	/// 表情元素id
	pub id: u64,
}

impl FaceElement {
	pub fn new(id: impl Into<u64>) -> Self {
		Self { id: id.into() }
	}
}

impl Element for FaceElement {
	type ElementType = ElementType;
	fn r#type(&self) -> ElementType {
		ElementType::Face
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new_and_type() {
		let e = FaceElement::new(123u64);
		assert_eq!(e.id, 123);
		assert_eq!(e.r#type(), ElementType::Face);
	}

	#[test]
	fn test_serde_roundtrip() {
		let e = FaceElement::new(456u64);
		let json = serde_json::to_string(&e).expect("serialize");
		let restored: FaceElement = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.id, 456);
	}
}
