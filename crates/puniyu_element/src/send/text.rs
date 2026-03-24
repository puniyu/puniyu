use serde::{Deserialize, Serialize};
use crate::{Element, ElementType, RawMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement<'t> {
	/// 文本元素内容
	pub text: &'t str,
}

impl<'t> TextElement<'t> {
	pub fn new(text: &'t str) -> Self {
		Self { text }
	}
}

impl<'t> Element for TextElement<'t> {
	fn r#type(&self) -> ElementType {
		ElementType::Text
	}
}

impl<'t> RawMessage for TextElement<'t> {
	fn raw(&self) -> String {
		self.text.to_string()
	}
}