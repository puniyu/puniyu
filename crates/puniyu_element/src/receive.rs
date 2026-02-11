//! 接收消息元素模块
//!
//! 本模块提供用于解析从聊天平台接收到的消息元素类型。
//!
//! ## 特点
//!
//! - 包含更多元数据（如图片尺寸、文件大小等）
//! - 字段通常为只读
//! - 实现了 `RawMessage` trait，可以获取原始消息内容
//!
//! ## 示例
//!
//! ```rust
//! use puniyu_element::receive::*;
//! use puniyu_element::RawMessage;
//!
//! // 创建文本元素
//! let text = TextElement { text: "Hello, World!" };
//! println!("Message: {}", text.raw());
//!
//! // 创建 @提及元素
//! let at = AtElement { target_id: "123456" };
//! if at.is_everyone() {
//!     println!("@Everyone");
//! } else {
//!     println!("@User: {}", at.target_id);
//! }
//!
//! // 使用元素枚举
//! let element = Elements::Text(TextElement { text: "Test message" });
//! if let Some(text) = element.as_text() {
//!     println!("Text content: {}", text);
//! }
//! ```

macro_rules! codegen_reexport {
    ($($module:ident => $type:ident),*) => {
        $(
            mod $module;
			#[doc(inline)]
            pub use $module::$type;
        )*
    };
}

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

use crate::{ElementType, RawMessage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
			Elements::Text(element) => Some(element.text),
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

impl<'e> RawMessage for Elements<'e> {
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

	fn raw(&self) -> String {
		match self {
			Elements::Text(element) => element.raw(),
			Elements::File(element) => element.raw(),
			Elements::Face(element) => element.raw(),
			Elements::Image(element) => element.raw(),
			Elements::Json(element) => element.raw(),
			Elements::Record(element) => element.raw(),
			Elements::Reply(element) => element.raw(),
			Elements::Video(element) => element.raw(),
			Elements::Xml(element) => element.raw(),
			Elements::At(element) => element.raw(),
		}
	}
}
