use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::{Element, ElementType, File};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VideoElement {
	/// 视频元素
	pub file: File,
	/// 视频文件名
	pub file_name: SmolStr,
}

impl Element for VideoElement {
	type ElementType = ElementType;

	fn r#type(&self) -> Self::ElementType {
		Self::ElementType::Video
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_type() {
		let e: VideoElement = VideoElement { file: File::Bytes(bytes::Bytes::from_static(b"mp4")), file_name: "v.mp4".into() };
		assert_eq!(e.r#type(), ElementType::Video);
	}

	#[test]
	fn test_serde_roundtrip() {
		let e: VideoElement = VideoElement { file: File::Bytes(bytes::Bytes::from_static(b"a")), file_name: "v.mp4".into() };
		let json = serde_json::to_string(&e).expect("serialize");
		let restored: VideoElement = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.file_name, "v.mp4");
	}
}
