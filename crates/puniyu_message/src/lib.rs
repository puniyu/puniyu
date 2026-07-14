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

use ecow::{EcoVec, eco_vec};
use puniyu_element::send::{Elements, TextElement};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt, ops::Deref};

/// 消息链类型。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Message(EcoVec<Elements>);

impl Deref for Message {
	type Target = [Elements];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl fmt::Display for Message {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut iter = self.0.iter();
		if let Some(first) = iter.next() {
			if f.alternate() {
				write!(f, "{:#}", first)?;
			} else {
				write!(f, "{}", first)?;
			}
			for elem in iter {
				if f.alternate() {
					write!(f, "\n{:#}", elem)?;
				} else {
					write!(f, "{}", elem)?;
				}
			}
		}
		Ok(())
	}
}

impl From<Vec<Elements>> for Message {
	fn from(message: Vec<Elements>) -> Self {
		Self(message.into())
	}
}

impl From<EcoVec<Elements>> for Message {
	fn from(message: EcoVec<Elements>) -> Self {
		Self(message)
	}
}
impl From<Elements> for Message {
	fn from(elements: Elements) -> Self {
		Self(eco_vec![elements])
	}
}

impl From<Message> for Vec<Elements> {
	fn from(message: Message) -> Self {
		message.0.to_vec()
	}
}

impl From<&str> for Message {
	fn from(value: &str) -> Self {
		Self(eco_vec![Elements::Text(TextElement::new(value))])
	}
}

impl<'m> From<Cow<'m, str>> for Message {
	fn from(value: Cow<'m, str>) -> Self {
		Self(eco_vec![Elements::Text(TextElement::new(value.into_owned()))])
	}
}

impl From<String> for Message {
	fn from(value: String) -> Self {
		Self(eco_vec![Elements::Text(TextElement::new(value))])
	}
}
