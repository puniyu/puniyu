use bon::Builder;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::{Element, ElementType, File};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder)]
pub struct VideoElement {
	/// 视频元素
	#[builder(into)]
	pub file: File,
	/// 视频文件名
	#[builder(into)]
	pub file_name: SmolStr,
}

impl VideoElement {
	pub fn new(file: impl Into<File>, file_name: impl Into<SmolStr>) -> Self {
		Self { file: file.into(), file_name: file_name.into() }
	}
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
	fn test_new_and_type() {
		let e = VideoElement::new(File::Bytes(bytes::Bytes::from_static(b"mp4")), "v.mp4");
		assert_eq!(e.file_name, "v.mp4");
		assert_eq!(e.r#type(), ElementType::Video);
	}
}
