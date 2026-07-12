//! 发送消息元素模块
//!
//! 用于构造发送消息元素。

use crate::{Element, ElementType, codegen_reexport};
use serde::{Deserialize, Serialize};
use strum::Display;

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

/// 发送元素枚举。
#[derive(Debug, Clone, Serialize, Deserialize, Display, PartialEq, Eq, Hash)]
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
			Elements::Text(element) => Some(element.text.as_str()),
			_ => None,
		}
	}

	/// 若为 At 元素，返回元素引用。
	pub fn as_at(&self) -> Option<&AtElement> {
		match self {
			Elements::At(element) => Some(element),
			_ => None,
		}
	}

	/// 若为回复元素，返回元素引用。
	pub fn as_reply(&self) -> Option<&ReplyElement> {
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
	pub fn as_image(&self) -> Option<&ImageElement> {
		match self {
			Elements::Image(element) => Some(element),
			_ => None,
		}
	}

	/// 若为文件元素，返回元素引用。
	pub fn as_file(&self) -> Option<&FileElement> {
		match self {
			Elements::File(element) => Some(element),
			_ => None,
		}
	}

	/// 若为视频元素，返回元素引用。
	pub fn as_video(&self) -> Option<&VideoElement> {
		match self {
			Elements::Video(element) => Some(element),
			_ => None,
		}
	}

	/// 若为语音元素，返回元素引用。
	pub fn as_record(&self) -> Option<&RecordElement> {
		match self {
			Elements::Record(element) => Some(element),
			_ => None,
		}
	}

	/// 若为 JSON 元素，返回元素引用。
	pub fn as_json(&self) -> Option<&JsonElement> {
		match self {
			Elements::Json(element) => Some(element),
			_ => None,
		}
	}

	/// 若为 XML 元素，返回元素引用。
	pub fn as_xml(&self) -> Option<&XmlElement> {
		match self {
			Elements::Xml(element) => Some(element),
			_ => None,
		}
	}
}

impl Element for Elements {
	type ElementType = ElementType;
	fn r#type(&self) -> ElementType {
		match self {
			Elements::Text(element) => element.r#type(),
			Elements::At(element) => element.r#type(),
			Elements::Reply(element) => element.r#type(),
			Elements::Face(element) => element.r#type(),
			Elements::Image(element) => element.r#type(),
			Elements::File(element) => element.r#type(),
			Elements::Video(element) => element.r#type(),
			Elements::Record(element) => element.r#type(),
			Elements::Json(element) => element.r#type(),
			Elements::Xml(element) => element.r#type(),
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

	#[test]
	fn test_send_variants_report_their_types() {
		let cases = [
			(Elements::from(TextElement::new("hi")), ElementType::Text),
			(Elements::from(AtElement::new("u1")), ElementType::At),
			(Elements::from(ReplyElement::new("m1")), ElementType::Reply),
			(Elements::from(FaceElement::new(1u64)), ElementType::Face),
			(
				Elements::from(FileElement::new(bytes::Bytes::from_static(b"x"), "x.bin")),
				ElementType::File,
			),
			(
				Elements::from(VideoElement::new(bytes::Bytes::from_static(b"v"), "v.mp4")),
				ElementType::Video,
			),
			(
				Elements::from(RecordElement::new(bytes::Bytes::from_static(b"a"), "a.silk")),
				ElementType::Record,
			),
			(Elements::from(JsonElement::new("{}")), ElementType::Json),
			(Elements::from(XmlElement::new("<r/>")), ElementType::Xml),
		];
		for (element, expected) in cases {
			assert_eq!(element.r#type(), expected);
		}
	}

	#[test]
	fn test_as_accessors() {
		let friend = Elements::from(TextElement::new("hi"));
		assert!(friend.as_text().is_some());
		assert_eq!(friend.as_text().expect("text"), "hi");
		assert!(friend.as_at().is_none());
		assert!(friend.as_image().is_none());
		assert!(friend.as_xml().is_none());

		let image = Elements::from(ImageElement::new(
			bytes::Bytes::from_static(b"\x89PNG"),
			"i.png",
			None::<&str>,
		));
		assert!(image.as_image().is_some());
		assert!(image.as_text().is_none());
	}

	#[test]
	fn test_elements_serde_roundtrip() {
		let text = Elements::from(TextElement::new("hi"));
		let json = serde_json::to_string(&text).expect("serialize");
		assert!(json.contains("\"type\":\"text\""));
		let restored: Elements = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.r#type(), ElementType::Text);
	}
}
