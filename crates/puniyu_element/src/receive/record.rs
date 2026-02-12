use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::{ElementType, RawMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordElement<'r> {
	/// 语音元素
	pub file: Bytes,
	/// 文件名
	pub file_name: &'r str
}

impl<'r> RawMessage for RecordElement<'r> {
	fn r#type(&self) -> ElementType {
		ElementType::Record
	}

	fn raw(&self) -> String {
		self.file_name.to_string()
	}
}
