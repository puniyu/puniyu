use bon::Builder;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::{Element, ElementType, File};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder)]
pub struct RecordElement {
	/// 语音元素
	#[builder(into)]
	pub file: File,
	/// 语音文件名
	#[builder(into)]
	pub file_name: SmolStr,
}

impl RecordElement {
	pub fn new(file: impl Into<File>, file_name: impl Into<SmolStr>) -> Self {
		Self { file: file.into(), file_name: file_name.into() }
	}
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
	fn test_new_and_type() {
		let e = RecordElement::new(File::Bytes(bytes::Bytes::from_static(b"silk")), "voice.silk");
		assert_eq!(e.file_name, "voice.silk");
		assert_eq!(e.r#type(), ElementType::Record);
	}

	#[test]
	fn test_serde_roundtrip() {
		let e = RecordElement::new(File::Bytes(bytes::Bytes::from_static(b"a")), "a.silk");
		let json = serde_json::to_string(&e).expect("serialize");
		let restored: RecordElement = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.file_name, "a.silk");
	}
}
