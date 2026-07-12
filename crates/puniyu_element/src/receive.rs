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
#[serde(rename_all = "lowercase", tag = "type", content = "field0", bound(deserialize = "'de: 'e"))]
pub enum Elements<'e> {
	Text(TextElement<'e>),
	At(AtElement<'e>),
	Reply(ReplyElement<'e>),
	Face(FaceElement),
	Image(ImageElement<'e>),
	File(FileElement<'e>),
	Video(VideoElement<'e>),
	Record(RecordElement<'e>),
	Json(JsonElement<'e>),
	Xml(XmlElement<'e>),
}

impl<'e> Elements<'e> {
	/// 若为文本元素，返回文本内容。
	pub fn as_text(&self) -> Option<&str> {
		match self {
			Elements::Text(element) => Some(element.text),
			_ => None,
		}
	}

	/// 若为 At 元素，返回元素引用。
	pub fn as_at(&self) -> Option<&AtElement<'_>> {
		match self {
			Elements::At(element) => Some(element),
			_ => None,
		}
	}

	/// 若为回复元素，返回元素引用。
	pub fn as_reply(&self) -> Option<&ReplyElement<'_>> {
		match self {
			Elements::Reply(element) => Some(element),
			_ => None,
		}
	}

	/// 若为表情元素，返回元素引用。
	pub fn as_face(&self) -> Option<&FaceElement> {
		match self {
			Elements::Face(element) => Some(element),
			_ => None,
		}
	}

	/// 若为图片元素，返回元素引用。
	pub fn as_image(&self) -> Option<&ImageElement<'_>> {
		match self {
			Elements::Image(element) => Some(element),
			_ => None,
		}
	}

	/// 若为文件元素，返回元素引用。
	pub fn as_file(&self) -> Option<&FileElement<'_>> {
		match self {
			Elements::File(element) => Some(element),
			_ => None,
		}
	}

	/// 若为视频元素，返回元素引用。
	pub fn as_video(&self) -> Option<&VideoElement<'_>> {
		match self {
			Elements::Video(element) => Some(element),
			_ => None,
		}
	}

	/// 若为语音元素，返回元素引用。
	pub fn as_record(&self) -> Option<&RecordElement<'_>> {
		match self {
			Elements::Record(element) => Some(element),
			_ => None,
		}
	}

	/// 若为 JSON 元素，返回元素引用。
	pub fn as_json(&self) -> Option<&JsonElement<'_>> {
		match self {
			Elements::Json(element) => Some(element),
			_ => None,
		}
	}

	/// 若为 XML 元素，返回元素引用。
	pub fn as_xml(&self) -> Option<&XmlElement<'_>> {
		match self {
			Elements::Xml(element) => Some(element),
			_ => None,
		}
	}
}

impl<'e> Element for Elements<'e> {
	type ElementType = ElementType;
	fn r#type(&self) -> ElementType {
		match self {
			Elements::Text(element) => element.r#type(),
			Elements::File(element) => element.r#type(),
			Elements::Face(element) => element.r#type(),
			Elements::Image(element) => element.r#type(),
			Elements::Json(element) => element.r#type(),
			Elements::Record(element) => element.r#type(),
			Elements::Reply(element) => element.r#type(),
			Elements::Video(element) => element.r#type(),
			Elements::Xml(element) => element.r#type(),
			Elements::At(element) => element.r#type(),
		}
	}
}

impl<'e> From<TextElement<'e>> for Elements<'e> {
	#[inline]
	fn from(element: TextElement<'e>) -> Self {
		Self::Text(element)
	}
}

impl<'e> From<AtElement<'e>> for Elements<'e> {
	#[inline]
	fn from(element: AtElement<'e>) -> Self {
		Self::At(element)
	}
}

impl<'e> From<ReplyElement<'e>> for Elements<'e> {
	#[inline]
	fn from(element: ReplyElement<'e>) -> Self {
		Self::Reply(element)
	}
}

impl From<FaceElement> for Elements<'_> {
	#[inline]
	fn from(element: FaceElement) -> Self {
		Self::Face(element)
	}
}

impl<'e> From<ImageElement<'e>> for Elements<'e> {
	#[inline]
	fn from(element: ImageElement<'e>) -> Self {
		Self::Image(element)
	}
}

impl<'e> From<FileElement<'e>> for Elements<'e> {
	#[inline]
	fn from(element: FileElement<'e>) -> Self {
		Self::File(element)
	}
}

impl<'e> From<VideoElement<'e>> for Elements<'e> {
	#[inline]
	fn from(element: VideoElement<'e>) -> Self {
		Self::Video(element)
	}
}

impl<'e> From<RecordElement<'e>> for Elements<'e> {
	#[inline]
	fn from(element: RecordElement<'e>) -> Self {
		Self::Record(element)
	}
}

impl<'e> From<JsonElement<'e>> for Elements<'e> {
	#[inline]
	fn from(element: JsonElement<'e>) -> Self {
		Self::Json(element)
	}
}

impl<'e> From<XmlElement<'e>> for Elements<'e> {
	#[inline]
	fn from(element: XmlElement<'e>) -> Self {
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
			file_name: "x.bin",
		};
		let video: VideoElement =
			VideoElement { file: File::Bytes(bytes::Bytes::from_static(b"v")), file_name: "v.mp4" };
		let record: RecordElement = RecordElement {
			file: File::Bytes(bytes::Bytes::from_static(b"a")),
			file_name: "a.silk",
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
