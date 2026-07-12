//! # puniyu_segment
//!
//! 消息段构建库，统一生成 `Elements`。
//!
//! ## 特性
//!
//! - `Segment` 链式构建
//! - `segment!` 宏构建
//!
//! ## 链式构建示例
//!
//! ```rust
//! use puniyu_segment::Segment;
//!
//! let msg = Segment::new()
//!     .at("123456")
//!     .text("hello")
//!     .face(178u64)
//!     .to_message();
//! ```
//!
//! ## 宏构建示例
//!
//! ```rust
//! use puniyu_segment::segment;
//! use puniyu_element::send::Elements;
//!
//! let message: Vec<Elements> = vec![
//!     segment!(at, "123456"),
//!     segment!(text, " hello"),
//!     segment!(face, 178u64),
//! ];
//! ```

use ecow::EcoVec;
use puniyu_element::File;
use puniyu_element::send::{
	AtElement, Elements, FaceElement, FileElement, ImageElement, JsonElement, RecordElement,
	ReplyElement, TextElement, VideoElement, XmlElement,
};
use puniyu_message::Message;
use smol_str::SmolStr;

mod macros;

/// 创建 At 元素
#[inline]
pub fn at(target_id: impl Into<SmolStr>) -> Elements {
	Elements::At(AtElement::new(target_id))
}

/// 创建文本元素
#[inline]
pub fn text(text: impl Into<SmolStr>) -> Elements {
	Elements::Text(TextElement::new(text))
}

/// 创建表情元素
#[inline]
pub fn face(id: u64) -> Elements {
	Elements::Face(FaceElement::new(id))
}

/// 创建回复元素
#[inline]
pub fn reply(message_id: impl Into<SmolStr>) -> Elements {
	Elements::Reply(ReplyElement::new(message_id))
}

/// 创建图片元素
#[inline]
pub fn image(
	file: impl Into<File>,
	name: impl Into<SmolStr>,
	summary: Option<impl Into<SmolStr>>,
) -> Elements {
	Elements::Image(ImageElement::new(file.into(), name, summary))
}

/// 创建语音元素
#[inline]
pub fn record(file: impl Into<File>, name: impl Into<SmolStr>) -> Elements {
	Elements::Record(RecordElement::new(file.into(), name))
}

/// 创建文件元素
#[inline]
pub fn file(file: impl Into<File>, name: impl Into<SmolStr>) -> Elements {
	Elements::File(FileElement::new(file.into(), name))
}

/// 创建视频元素
#[inline]
pub fn video(file: impl Into<File>, name: impl Into<SmolStr>) -> Elements {
	Elements::Video(VideoElement::new(file.into(), name))
}

/// 创建 JSON 元素
#[inline]
pub fn json(data: impl Into<SmolStr>) -> Elements {
	Elements::Json(JsonElement::new(data))
}

/// 创建 XML 元素
#[inline]
pub fn xml(data: impl Into<SmolStr>) -> Elements {
	Elements::Xml(XmlElement::new(data))
}

/// 消息段构建器
#[derive(Debug, Default)]
pub struct Segment {
	elements: EcoVec<Elements>,
}

impl Segment {
	/// 创建新的构建器。
	#[inline]
	pub fn new() -> Self {
		Self::default()
	}

	/// 添加 At 元素。
	#[inline]
	pub fn at(mut self, target_id: impl Into<SmolStr>) -> Self {
		self.elements.push(self::at(target_id));
		self
	}

	/// 添加文本元素。
	#[inline]
	pub fn text(mut self, text: impl Into<SmolStr>) -> Self {
		self.elements.push(self::text(text));
		self
	}

	/// 添加表情元素。
	#[inline]
	pub fn face(mut self, id: u64) -> Self {
		self.elements.push(self::face(id));
		self
	}

	/// 添加回复元素。
	#[inline]
	pub fn reply(mut self, message_id: impl Into<SmolStr>) -> Self {
		self.elements.push(self::reply(message_id));
		self
	}

	/// 添加图片元素。
	#[inline]
	pub fn image(
		mut self,
		file: impl Into<File>,
		name: impl Into<SmolStr>,
		summary: Option<impl Into<SmolStr>>,
	) -> Self {
		self.elements.push(self::image(file, name, summary));
		self
	}

	/// 添加语音元素。
	#[inline]
	pub fn record(mut self, file: impl Into<File>, name: impl Into<SmolStr>) -> Self {
		self.elements.push(self::record(file, name));
		self
	}

	/// 添加文件元素。
	#[inline]
	pub fn file(mut self, file: impl Into<File>, name: impl Into<SmolStr>) -> Self {
		self.elements.push(self::file(file, name));
		self
	}

	/// 添加视频元素。
	#[inline]
	pub fn video(mut self, file: impl Into<File>, name: impl Into<SmolStr>) -> Self {
		self.elements.push(self::video(file, name));
		self
	}

	/// 添加 JSON 元素。
	#[inline]
	pub fn json(mut self, data: impl Into<SmolStr>) -> Self {
		self.elements.push(self::json(data));
		self
	}

	/// 添加 XML 元素。
	#[inline]
	pub fn xml(mut self, data: impl Into<SmolStr>) -> Self {
		self.elements.push(self::xml(data));
		self
	}

	/// 获取所有元素
	#[inline]
	pub fn to_elements(self) -> EcoVec<Elements> {
		self.elements
	}

	/// 转换为 Message。
	#[inline]
	pub fn to_message(self) -> Message {
		Message::from(self.elements)
	}
}
