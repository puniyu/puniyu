use serde::{Deserialize, Serialize};

use crate::{Element, ElementType, File};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileElement<'f> {
	/// 文件元素
	pub file: File,
	/// 文件大小, 单位字节
	pub file_size: u64,
	/// 文件名称
	#[serde(borrow)]
	pub file_name: &'f str,
}

impl<'f> Element for FileElement<'f> {
	type ElementType = ElementType;

	fn r#type(&self) -> ElementType {
		ElementType::File
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_type() {
		let e: FileElement = FileElement { file: File::Bytes(bytes::Bytes::from_static(b"x")), file_size: 1, file_name: "x.bin" };
		assert_eq!(e.r#type(), ElementType::File);
	}

	#[test]
	fn test_serde_roundtrip() {
		let e: FileElement = FileElement { file: File::Bytes(bytes::Bytes::from_static(b"data")), file_size: 4, file_name: "f.bin" };
		let json = serde_json::to_string(&e).expect("serialize");
		let restored: FileElement = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.file_size, 4);
		assert_eq!(restored.file_name, "f.bin");
	}
}