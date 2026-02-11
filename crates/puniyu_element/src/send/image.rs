use bytes::Bytes;
use crate::{ElementType, RawMessage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageElement<'m> {
	/// 图片元素
	pub file: Bytes,
	/// 图片文件名
	pub file_name: &'m str,
	/// 图片外显
	pub summary: &'m str,
}

impl<'m> ImageElement<'m> {
	pub fn new(file: impl Into<Bytes>, file_name: &'m str, summary: Option<&'m str>) -> Self {
		Self { file: file.into(), file_name, summary: summary.unwrap_or(file_name) }
	}
}

impl<'m> RawMessage for ImageElement<'m> {
	fn r#type(&self) -> ElementType {
		ElementType::Image
	}

	fn raw(&self) -> String {
		self.file_name.to_string()
	}
}
