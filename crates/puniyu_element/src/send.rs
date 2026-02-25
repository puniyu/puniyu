//! 发送消息元素模块
//!
//! 本模块提供用于构造和发送消息的元素类型。
//!
//! ## 特点
//!
//! - 提供便捷的构造函数（`new` 方法）
//! - 字段可以灵活设置
//! - 支持序列化和反序列化
//!
//! ## 示例
//!
//! ```rust
//! use puniyu_element::send::*;
//! use bytes::Bytes;
//!
//! // 创建文本元素
//! let text = TextElement::new("Hello!");
//!
//! // 创建图片元素
//! let image_data = Bytes::from("image data");
//! let image = ImageElement::new(image_data, "photo.jpg", None);
//!
//! // 创建表情元素
//! let face = FaceElement::new(42u64);
//! ```

use crate::codegen_reexport;
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

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
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
	pub fn as_text(&self) -> Option<&str> {
		match self {
			Elements::Text(text_element) => Some(text_element.text),
			_ => None,
		}
	}

	pub fn as_at(&self) -> Option<&AtElement<'_>> {
		match self {
			Elements::At(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_reply(&self) -> Option<&ReplyElement<'_>> {
		match self {
			Elements::Reply(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_face(&self) -> Option<&FaceElement> {
		match self {
			Elements::Face(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_image(&self) -> Option<&ImageElement<'_>> {
		match self {
			Elements::Image(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_file(&self) -> Option<&FileElement<'_>> {
		match self {
			Elements::File(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_video(&self) -> Option<&VideoElement<'_>> {
		match self {
			Elements::Video(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_record(&self) -> Option<&RecordElement<'_>> {
		match self {
			Elements::Record(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_json(&self) -> Option<&JsonElement<'_>> {
		match self {
			Elements::Json(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_xml(&self) -> Option<&XmlElement<'_>> {
		match self {
			Elements::Xml(element) => Some(element),
			_ => None,
		}
	}
}
