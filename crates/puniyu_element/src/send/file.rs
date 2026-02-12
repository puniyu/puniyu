use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::{ElementType, RawMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileElement<'f> {
	/// 文件元素
	pub file: Bytes,
	/// 文件名
	pub file_name: &'f str,
}

impl<'f> FileElement<'f> {
	pub fn new(file: impl Into<Bytes>, file_name: &'f str) -> Self {
		Self { file: file.into(), file_name }
	}
}

impl<'f> RawMessage for FileElement<'f> {
	fn r#type(&self) -> ElementType {
		ElementType::File
	}

	fn raw(&self) -> String {
		self.file_name.to_string()
	}
}
