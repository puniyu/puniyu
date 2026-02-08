use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordElement<'r> {
	/// 语音元素
	pub file: Bytes,
	/// 语音文件名
	pub file_name: &'r str,
}

impl<'r> RecordElement<'r> {
	pub fn new(file: impl Into<Bytes>, file_name: &'r str) -> Self {
		Self { file: file.into(), file_name }
	}
}
