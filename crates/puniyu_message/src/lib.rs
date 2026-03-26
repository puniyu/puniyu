//! # puniyu_message
//!
//! 消息链封装库，统一组合 `Elements`。
//!
//! ## 特性
//!
//! - `Message` 封装消息元素链
//! - `message!` 宏快速构建
//! - 支持常见类型转换
//!
//! ```rust
//! use puniyu_message::message;
//! use puniyu_element::send::{AtElement, Elements, TextElement};
//!
//! let msg = message!(
//!     Elements::At(AtElement::new("123456")),
//!     Elements::Text(TextElement::new(" hello")),
//! );
//! ```

mod macros;

use puniyu_element::send::{Elements, TextElement};
use serde::{Deserialize, Serialize};
use std::fmt;

/// 消息链类型。
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
		Self(vec![Elements::Text(TextElement::new(v))])
	}
}
