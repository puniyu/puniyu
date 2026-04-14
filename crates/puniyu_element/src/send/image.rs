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
	pub fn new<B, N, S>(
		file: B,
		file_name: N,
		summary: Option<S>,
	) -> Self
	where
		B: Into<Bytes>,
		N: Into<String>,
		S: Into<String>,
	{
		let file_name = file_name.into();
		let summary = summary.map(|s| s.into()).unwrap_or(file_name.clone());
		Self { file: file.into(), file_name, summary }
	}
}

impl Element for ImageElement {
	fn r#type(&self) -> ElementType {
		ElementType::Image
	}
}
