//! # puniyu_message
//!
//! 消息构建和处理库，提供便捷的方式来创建和操作消息链。
//!
//! ## 概述
//!
//! `puniyu_message` 提供了 `Message` 类型，用于封装消息元素链。
//! 它支持多种消息元素的组合，并提供了便捷的宏来简化消息构建。
//!
//! ## 使用方式
//!
//! ### 1. 使用 `Message::from` 方法
//!
//! ```rust
//! use puniyu_message::Message;
//! use puniyu_element::send::{Elements, TextElement, AtElement};
//!
//! // 从单个元素创建消息
//! let text = TextElement::new("Hello!");
//! let msg = Message::from(Elements::Text(text));
//!
//! // 从元素向量创建消息
//! let elements = vec![
//!     Elements::At(AtElement::new("123456")),
//!     Elements::Text(TextElement::new(" Hello!")),
//! ];
//! let msg = Message::from(elements);
//!
//! // 从字符串创建文本消息
//! let msg = Message::from("Simple text message");
//! ```
//!
//! ### 2. 使用 `message!` 宏（推荐）
//!
//! ```rust
//! use puniyu_message::message;
//! use puniyu_element::send::{Elements, TextElement, AtElement, FaceElement};
//!
//! // 单个元素
//! let msg = message!(Elements::Text(TextElement::new("Hello")));
//!
//! // 多个元素
//! let msg = message!(
//!     Elements::At(AtElement::new("123456")),
//!     Elements::Text(TextElement::new(" Hello!")),
//!     Elements::Face(FaceElement::new(178u64)),
//! );
//! ```
//!
//! ## 消息格式化
//!
//! `Message` 实现了 `Display` trait，支持两种格式化方式：
//!
//! ```rust
//! use puniyu_message::message;
//! use puniyu_element::send::{Elements, TextElement};
//!
//! let msg = message!(
//!     Elements::Text(TextElement::new("Hello")),
//!     Elements::Text(TextElement::new(" World")),
//! );
//!
//! // 普通格式：连接所有元素
//! println!("{}", msg);
//!
//! // 详细格式：每个元素一行
//! println!("{:#}", msg);
//! ```

mod macros;

use puniyu_element::send::{Elements, TextElement};
use serde::{Deserialize, Serialize};
use std::fmt;

/// 消息类型
///
/// 封装了一个消息元素链，提供了便捷的创建和转换方法。
///
/// # 示例
///
/// ```rust
/// use puniyu_message::Message;
/// use puniyu_element::send::{Elements, TextElement};
///
/// // 从字符串创建
/// let msg = Message::from("Hello, World!");
///
/// // 从元素向量创建
/// let elements = vec![Elements::Text(TextElement::new("Hello"))];
/// let msg = Message::from(elements);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'm"))]
pub struct Message<'m>(Vec<Elements<'m>>);

impl<'m> fmt::Display for Message<'m> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if f.alternate() {
			let segments: Vec<String> = self.0.iter().map(|s| format!("{:#}", s)).collect();
			write!(f, "{}", segments.join("\n"))
		} else {
			let segments: Vec<String> = self.0.iter().map(|s| s.to_string()).collect();
			write!(f, "{}", segments.join(""))
		}
	}
}

impl<'m> From<Vec<Elements<'m>>> for Message<'m> {
	fn from(message: Vec<Elements<'m>>) -> Self {
		Message(message)
	}
}

impl<'e> From<Elements<'e>> for Message<'e> {
	fn from(elements: Elements<'e>) -> Self {
		Message(vec![elements])
	}
}

impl<'m> From<Message<'m>> for Vec<Elements<'m>> {
	fn from(message: Message<'m>) -> Self {
		message.0
	}
}

impl<'m> From<&'m str> for Message<'m> {
	fn from(v: &'m str) -> Self {
		Self(vec![Elements::Text(TextElement { text: v })])
	}
}