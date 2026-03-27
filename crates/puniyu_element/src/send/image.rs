use crate::{Element, ElementType};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageElement {
	/// 图片元素
	pub file: Bytes,
	/// 图片文件名
	pub file_name: String,
	/// 图片外显
	pub summary: String,
}

impl ImageElement {
	pub fn new(file: impl Into<Bytes>, file_name: impl Into<String>) -> Self {
		let file_name = file_name.into();
		Self { file: file.into(), file_name: file_name.clone(), summary: file_name }
	}

	pub fn with_summary(
		file: impl Into<Bytes>,
		file_name: impl Into<String>,
		summary: impl Into<String>,
	) -> Self {
		let file_name = file_name.into();
		let summary = summary.into();
		Self { file: file.into(), file_name, summary }
	}
}

impl Element for ImageElement {
	fn r#type(&self) -> ElementType {
		ElementType::Image
	}
}
