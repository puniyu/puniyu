use serde::{Deserialize, Serialize};
use crate::{Element, ElementType};

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

impl<'a> Element for AtElement<'a> {
	fn r#type(&self) -> ElementType {
		ElementType::At
	}
}