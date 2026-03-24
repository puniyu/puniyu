use serde::{Deserialize, Serialize};
use crate::{Element, ElementType, RawMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
	fn r#type(&self) -> ElementType {
		ElementType::Face
	}
}

impl RawMessage for FaceElement {
	fn raw(&self) -> String {
		self.id.to_string()
	}
}
