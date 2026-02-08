use bytes::Bytes;
use serde::{Deserialize, Serialize};

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
