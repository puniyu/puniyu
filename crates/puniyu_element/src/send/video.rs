use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::{Element, ElementType, RawMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoElement<'v> {
	/// 视频元素
	pub file: Bytes,
	/// 视频文件名
	pub file_name: &'v str,
}

impl<'v> VideoElement<'v> {
	pub fn new(file: impl Into<Bytes>, file_name: &'v str) -> Self {
		Self { file: file.into(), file_name }
	}
}

impl<'v> Element for VideoElement<'v> {
	fn r#type(&self) -> ElementType {
		ElementType::Video
	}
}

impl<'v> RawMessage for VideoElement<'v> {
	fn raw(&self) -> String {
		format!("[video:{}]", self.file_name)
	}
}