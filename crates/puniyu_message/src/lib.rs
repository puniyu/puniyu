//! # puniyu_message
//!
//! 消息链封装库，统一组合发送元素。
//!
//! ## 特性
//!
//! - `Message` 封装构建完成的消息元素链
//! - `MessageBuilder` 提供领域化链式构建
//! - `message!` 宏快速组合具体发送元素
//! - 支持常见集合与字符串转换
//!
//! ```rust
//! use puniyu_element::send::{AtElement, TextElement};
//! use puniyu_message::message;
//!
//! let msg = message!(AtElement::new("123456"), TextElement::new(" hello"));
//! assert_eq!(msg.len(), 2);
//! ```

mod macros;

use std::{borrow::Cow, fmt, ops::Deref};

use bon::Builder;
use ecow::{EcoVec, eco_vec};
use puniyu_element::{
	File,
	send::{
		AtElement, Elements, FaceElement, FileElement, ImageElement, JsonElement, RecordElement,
		ReplyElement, TextElement, VideoElement, XmlElement,
	},
};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// 构建完成的发送消息元素链。
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Message {
	#[builder(field)]
	elements: EcoVec<Elements>,
}

impl<S: message_builder::State> MessageBuilder<S> {
	/// 添加一个可转换为发送元素的值。
	#[inline]
	pub fn element(mut self, element: impl Into<Elements>) -> Self {
		self.elements.push(element.into());
		self
	}


	/// 添加 At 元素。
	#[inline]
	pub fn at(self, target_id: impl Into<SmolStr>) -> Self {
		self.element(AtElement::new(target_id))
	}

	/// 添加 At 全体元素。
	#[inline]
	pub fn at_everyone(self) -> Self {
		self.element(AtElement::everyone())
	}

	/// 添加文本元素。
	#[inline]
	pub fn text(self, text: impl Into<SmolStr>) -> Self {
		self.element(TextElement::new(text))
	}

	/// 添加表情元素。
	#[inline]
	pub fn face(self, id: u64) -> Self {
		self.element(FaceElement::new(id))
	}

	/// 添加回复元素。
	#[inline]
	pub fn reply(self, message_id: impl Into<SmolStr>) -> Self {
		self.element(ReplyElement::new(message_id))
	}

	/// 添加图片元素。
	#[inline]
	pub fn image<Summary>(
		self,
		file: impl Into<File>,
		file_name: impl Into<SmolStr>,
		summary: Option<Summary>,
	) -> Self
	where
		Summary: Into<SmolStr>,
	{
		self.element(ImageElement::new(file, file_name, summary))
	}

	/// 添加文件元素。
	#[inline]
	pub fn file(self, file: impl Into<File>, file_name: impl Into<SmolStr>) -> Self {
		self.element(FileElement::new(file, file_name))
	}

	/// 添加语音元素。
	#[inline]
	pub fn record(self, file: impl Into<File>, file_name: impl Into<SmolStr>) -> Self {
		self.element(RecordElement::new(file, file_name))
	}

	/// 添加视频元素。
	#[inline]
	pub fn video(self, file: impl Into<File>, file_name: impl Into<SmolStr>) -> Self {
		self.element(VideoElement::new(file, file_name))
	}

	/// 添加 JSON 元素。
	#[inline]
	pub fn json(self, data: impl Into<SmolStr>) -> Self {
		self.element(JsonElement::new(data))
	}

	/// 添加 XML 元素。
	#[inline]
	pub fn xml(self, data: impl Into<SmolStr>) -> Self {
		self.element(XmlElement::new(data))
	}
}

impl Message {
	/// 使用完整元素集合创建消息。
	#[inline]
	pub fn new(elements: impl Into<EcoVec<Elements>>) -> Self {
		Self { elements: elements.into() }
	}

	/// 获取消息元素切片。
	#[inline]
	pub fn as_slice(&self) -> &[Elements] {
		self.elements.as_slice()
	}

	/// 消费消息并返回底层元素集合
	#[inline]
	pub fn into_elements(self) -> EcoVec<Elements> {
		self.elements
	}
}

impl Deref for Message {
	type Target = [Elements];

	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

impl AsRef<[Elements]> for Message {
	fn as_ref(&self) -> &[Elements] {
		self.as_slice()
	}
}

impl fmt::Display for Message {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for (i, elem) in self.elements.iter().enumerate() {
			if i > 0 {
				write!(f, "{}", if f.alternate() { "\n" } else { "" })?;
			}
			write!(f, "{}", elem)?;
		}
		Ok(())
	}
}

impl<E> FromIterator<E> for Message
where
	E: Into<Elements>,
{
	fn from_iter<I: IntoIterator<Item = E>>(iter: I) -> Self {
		Self::new(iter.into_iter().map(Into::into).collect::<EcoVec<_>>())
	}
}

impl<'a> IntoIterator for &'a Message {
	type IntoIter = std::slice::Iter<'a, Elements>;
	type Item = &'a Elements;

	fn into_iter(self) -> Self::IntoIter {
		self.elements.iter()
	}
}

impl IntoIterator for Message {
	type IntoIter = ecow::vec::IntoIter<Elements>;
	type Item = Elements;

	fn into_iter(self) -> Self::IntoIter {
		self.elements.into_iter()
	}
}

impl From<Vec<Elements>> for Message {
	fn from(elements: Vec<Elements>) -> Self {
		Self::new(EcoVec::from(elements))
	}
}

impl From<EcoVec<Elements>> for Message {
	fn from(elements: EcoVec<Elements>) -> Self {
		Self::new(elements)
	}
}
impl From<Elements> for Message {
	fn from(element: Elements) -> Self {
		Self::new(eco_vec![element])
	}
}

impl From<Message> for EcoVec<Elements> {
	fn from(message: Message) -> Self {
		message.into_elements()
	}
}

impl From<Message> for Vec<Elements> {
	fn from(message: Message) -> Self {
		message.into_iter().collect()
	}
}

impl From<&str> for Message {
	fn from(value: &str) -> Self {
		Self::from(Elements::from(TextElement::new(value)))
	}
}

impl From<Cow<'_, str>> for Message {
	fn from(value: Cow<'_, str>) -> Self {
		Self::from(Elements::from(TextElement::new(value.into_owned())))
	}
}

impl From<String> for Message {
	fn from(value: String) -> Self {
		Self::from(Elements::from(TextElement::new(value)))
	}
}
