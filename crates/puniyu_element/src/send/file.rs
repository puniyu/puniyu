use bon::Builder;
use serde::{Deserialize, Serialize};

use puniyu_core::element::Element;
use smol_str::SmolStr;

use crate::{ElementType, File};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder)]
pub struct FileElement {
	/// 文件元素
	#[builder(into)]
	pub file: File,
	/// 文件名
	#[builder(into)]
	pub file_name: SmolStr,
}

impl FileElement {
	pub fn new(file: impl Into<File>, file_name: impl Into<SmolStr>) -> Self {
		Self { file: file.into(), file_name: file_name.into() }
	}
}

impl Element for FileElement {
	type ElementType = ElementType;
	fn r#type(&self) -> Self::ElementType {
		Self::ElementType::File
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new_and_type() {
		let e = FileElement::new(File::Bytes(bytes::Bytes::from_static(b"hello")), "hello.txt");
		assert_eq!(e.file_name, "hello.txt");
		assert_eq!(e.r#type(), ElementType::File);
	}

	#[test]
	fn test_serde_roundtrip() {
		let e = FileElement::new(File::Bytes(bytes::Bytes::from_static(b"data")), "f.bin");
		let json = serde_json::to_string(&e).expect("serialize");
		let restored: FileElement = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.file_name, "f.bin");
	}
}
