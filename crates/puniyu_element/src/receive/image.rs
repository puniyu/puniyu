use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::{Element, ElementType, File};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImageElement {
	/// 图片元素
	pub file: File,
	/// 图片文件名
	pub file_name: SmolStr,
	/// 图片外显, 默认拿不到则默认为图片文件名
	pub summary: Option<SmolStr>,
	/// 图片宽度
	pub width: u32,
	/// 图片高度
	pub height: u32,
}

impl Element for ImageElement {
	type ElementType = ElementType;

	fn r#type(&self) -> Self::ElementType {
		Self::ElementType::Image
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_type() {
		let e: ImageElement = ImageElement {
			file: File::Bytes(bytes::Bytes::from_static(b"\x89PNG")),
			file_name: "img.png".into(),
			summary: Some("示例".into()),
			width: 100,
			height: 200,
		};
		assert_eq!(e.r#type(), ElementType::Image);
	}

	#[test]
	fn test_serde_roundtrip() {
		let e: ImageElement = ImageElement {
			file: File::Bytes(bytes::Bytes::from_static(b"x")),
			file_name: "i.png".into(),
			summary: Some("s".into()),
			width: 10,
			height: 20,
		};
		let json = serde_json::to_string(&e).expect("serialize");
		let restored: ImageElement = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.file_name, "i.png");
		assert_eq!(restored.summary.as_deref(), Some("s"));
		assert_eq!(restored.width, 10);
		assert_eq!(restored.height, 20);
	}
}
