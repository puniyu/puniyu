use serde::{Deserialize, Serialize};

use crate::{Element, ElementType, File};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecordElement<'r> {
	/// 语音元素
	pub file: File,
	/// 文件名
	#[serde(borrow)]
	pub file_name: &'r str,
}

impl<'r> Element for RecordElement<'r> {
	type ElementType = ElementType;

	fn r#type(&self) -> ElementType {
		ElementType::Record
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_type() {
		let e: RecordElement = RecordElement { file: File::Bytes(bytes::Bytes::from_static(b"silk")), file_name: "v.silk" };
		assert_eq!(e.r#type(), ElementType::Record);
	}

	#[test]
	fn test_serde_roundtrip() {
		let e: RecordElement = RecordElement { file: File::Bytes(bytes::Bytes::from_static(b"a")), file_name: "a.silk" };
		let json = serde_json::to_string(&e).expect("serialize");
		let restored: RecordElement = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.file_name, "a.silk");
	}
}