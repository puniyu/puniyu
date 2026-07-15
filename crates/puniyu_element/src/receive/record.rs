use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::{Element, ElementType, File};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecordElement {
	/// 语音元素
	pub file: File,
	/// 文件名
	pub file_name: SmolStr,
}

impl Element for RecordElement {
	type ElementType = ElementType;

	fn r#type(&self) -> Self::ElementType {
		Self::ElementType::Record
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_type() {
		let e: RecordElement = RecordElement { file: File::Bytes(bytes::Bytes::from_static(b"silk")), file_name: "v.silk".into() };
		assert_eq!(e.r#type(), ElementType::Record);
	}

	#[test]
	fn test_serde_roundtrip() {
		let e: RecordElement = RecordElement { file: File::Bytes(bytes::Bytes::from_static(b"a")), file_name: "a.silk".into() };
		let json = serde_json::to_string(&e).expect("serialize");
		let restored: RecordElement = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.file_name, "a.silk");
	}
}
