use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageElement<'i> {
	/// 图片元素
	pub file: Bytes,
	/// 图片外显
	pub summary: &'i str,
}

impl<'i> ImageElement<'i> {
	pub fn new(file: impl Into<Bytes>, summary: &'i str) -> Self {
		Self { file: file.into(), summary }
	}
}
