//! 接收消息元素模块
//!
//! 用于表示接收消息元素。

use crate::{Element, ElementType, codegen_reexport};
use serde::{Deserialize, Serialize};

codegen_reexport! {
	text => TextElement,
	at => AtElement,
	reply => ReplyElement,
	face => FaceElement,
	image => ImageElement,
	file => FileElement,
	video => VideoElement,
	record => RecordElement,
	json => JsonElement,
	xml => XmlElement
}

/// 接收元素枚举。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0")]
pub enum Elements {
	Text(TextElement),
	At(AtElement),
	Reply(ReplyElement),
	Face(FaceElement),
	Image(ImageElement),
	File(FileElement),
	Video(VideoElement),
	Record(RecordElement),
	Json(JsonElement),
	Xml(XmlElement),
}

impl Elements {
	/// 若为文本元素，返回文本内容。
	pub fn as_text(&self) -> Option<&str> {
		match self {
			Self::Text(element) => Some(element.text.as_str()),
			_ => None,
		}
	}

	/// 若为 At 元素，返回元素引用。
	pub fn as_at(&self) -> Option<&AtElement> {
		match self {
			Self::At(element) => Some(element),
			_ => None,
		}
	}

	/// 若为回复元素，返回元素引用。
	pub fn as_reply(&self) -> Option<&ReplyElement> {
		match self {
			Self::Reply(element) => Some(element),
			_ => None,
		}
	}

	/// 若为表情元素，返回元素引用。
	pub fn as_face(&self) -> Option<&FaceElement> {
		match self {
			Self::Face(element) => Some(element),
			_ => None,
		}
	}

	/// 若为图片元素，返回元素引用。
	pub fn as_image(&self) -> Option<&ImageElement> {
		match self {
			Self::Image(element) => Some(element),
			_ => None,
		}
	}

	/// 若为文件元素，返回元素引用。
	pub fn as_file(&self) -> Option<&FileElement> {
		match self {
			Self::File(element) => Some(element),
			_ => None,
		}
	}

	/// 若为视频元素，返回元素引用。
	pub fn as_video(&self) -> Option<&VideoElement> {
		match self {
			Self::Video(element) => Some(element),
			_ => None,
		}
	}

	/// 若为语音元素，返回元素引用。
	pub fn as_record(&self) -> Option<&RecordElement> {
		match self {
			Self::Record(element) => Some(element),
			_ => None,
		}
	}

	/// 若为 JSON 元素，返回元素引用。
	pub fn as_json(&self) -> Option<&JsonElement> {
		match self {
			Self::Json(element) => Some(element),
			_ => None,
		}
	}

	/// 若为 XML 元素，返回元素引用。
	pub fn as_xml(&self) -> Option<&XmlElement> {
		match self {
			Self::Xml(element) => Some(element),
			_ => None,
		}
	}
}

impl Element for Elements {
	type ElementType = ElementType;
	fn r#type(&self) -> Self::ElementType {
		match self {
			Self::Text(element) => element.r#type(),
			Self::File(element) => element.r#type(),
			Self::Face(element) => element.r#type(),
			Self::Image(element) => element.r#type(),
			Self::Json(element) => element.r#type(),
			Self::Record(element) => element.r#type(),
			Self::Reply(element) => element.r#type(),
			Self::Video(element) => element.r#type(),
			Self::Xml(element) => element.r#type(),
			Self::At(element) => element.r#type(),
		}
	}
}

impl From<TextElement> for Elements {
	#[inline]
	fn from(element: TextElement) -> Self {
		Self::Text(element)
	}
}

impl From<AtElement> for Elements {
	#[inline]
	fn from(element: AtElement) -> Self {
		Self::At(element)
	}
}

impl From<ReplyElement> for Elements {
	#[inline]
	fn from(element: ReplyElement) -> Self {
		Self::Reply(element)
	}
}

impl From<FaceElement> for Elements {
	#[inline]
	fn from(element: FaceElement) -> Self {
		Self::Face(element)
	}
}

impl From<ImageElement> for Elements {
	#[inline]
	fn from(element: ImageElement) -> Self {
		Self::Image(element)
	}
}

impl From<FileElement> for Elements {
	#[inline]
	fn from(element: FileElement) -> Self {
		Self::File(element)
	}
}

impl From<VideoElement> for Elements {
	#[inline]
	fn from(element: VideoElement) -> Self {
		Self::Video(element)
	}
}

impl From<RecordElement> for Elements {
	#[inline]
	fn from(element: RecordElement) -> Self {
		Self::Record(element)
	}
}

impl From<JsonElement> for Elements {
	#[inline]
	fn from(element: JsonElement) -> Self {
		Self::Json(element)
	}
}

impl From<XmlElement> for Elements {
	#[inline]
	fn from(element: XmlElement) -> Self {
		Self::Xml(element)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::File;

	#[test]
	fn test_receive_variants_report_their_types() {
		let text: TextElement = "hi".into();
		let at: AtElement = "u1".into();
		let reply: ReplyElement = "m1".into();
		let face = FaceElement { id: 1 };
		let file: FileElement = FileElement {
			file: File::Bytes(bytes::Bytes::from_static(b"x")),
			file_size: 1,
			file_name: "x.bin".into(),
		};
		let video: VideoElement = VideoElement {
			file: File::Bytes(bytes::Bytes::from_static(b"v")),
			file_name: "v.mp4".into(),
		};
		let record: RecordElement = RecordElement {
			file: File::Bytes(bytes::Bytes::from_static(b"a")),
			file_name: "a.silk".into(),
		};
		let json_e: JsonElement = "{}".into();
		let xml: XmlElement = "<r/>".into();

		let cases = [
			(Elements::Text(text), ElementType::Text),
			(Elements::At(at), ElementType::At),
			(Elements::Reply(reply), ElementType::Reply),
			(Elements::Face(face), ElementType::Face),
			(Elements::File(file), ElementType::File),
			(Elements::Video(video), ElementType::Video),
			(Elements::Record(record), ElementType::Record),
			(Elements::Json(json_e), ElementType::Json),
			(Elements::Xml(xml), ElementType::Xml),
		];
		for (element, expected) in cases {
			assert_eq!(element.r#type(), expected);
		}
	}

	#[test]
	fn test_as_accessors() {
		let text: TextElement = "hi".into();
		let elements = Elements::Text(text);
		assert_eq!(elements.as_text(), Some("hi"));
		assert!(elements.as_at().is_none());
		assert!(elements.as_xml().is_none());
		assert!(elements.as_record().is_none());

		let at: AtElement = "all".into();
		let elements = Elements::At(at);
		assert!(elements.as_at().is_some());
		assert!(elements.as_text().is_none());
	}

	#[test]
	fn test_elements_serde_roundtrip() {
		let at_json = r#"{"type":"at","field0":{"target_id":"u1"}}"#;
		let element: Elements = serde_json::from_str(at_json).expect("deserialize");
		assert_eq!(element.r#type(), ElementType::At);

		let restored_json = serde_json::to_string(&element).expect("serialize");
		assert!(restored_json.contains("\"type\":\"at\""));
	}
}
