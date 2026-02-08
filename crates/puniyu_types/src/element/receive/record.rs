use super::RawMessage;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordElement<'r> {
	/// 语音元素
	pub file: Bytes,
	/// 文件名
	pub file_name: &'r str
}

impl<'r> RawMessage for RecordElement<'r> {
	fn raw(&self) -> String {
		format!("[record:{}]", self.file_name)
	}
}
