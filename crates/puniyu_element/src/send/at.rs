use serde::{Deserialize, Serialize};
use crate::{ElementType, RawMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtElement<'a> {
	/// 目标id
	pub target_id: &'a str,
}

impl<'a> AtElement<'a> {
	pub fn new(target_id: &'a str) -> Self {
		Self { target_id }
	}
}

impl<'a> RawMessage for AtElement<'a> {
	fn r#type(&self) -> ElementType {
		ElementType::At
	}

	fn raw(&self) -> String {
		self.target_id.to_string()
	}
}