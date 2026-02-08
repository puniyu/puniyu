use super::send::Elements;
use serde::{Deserialize, Serialize};
use std::fmt;

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
		Self(vec![Elements::Text(v.into())])
	}
}

/// 消息构建宏
///
/// 这个宏提供了一种便捷的方式来创建 `Message` 对象。
/// 它支持两种使用方式：
///
/// # 用法
///
/// ## 单个元素
/// 从单个消息元素创建消息：
/// ```rust
/// use puniyu_types::message;
/// use puniyu_types::element::send::TextElement;
///
/// let msg = message!(TextElement::new("你好"));
/// ```
///
/// ## 多个元素
/// 从多个消息元素创建消息:
/// ```rust
/// use puniyu_types::message;
/// use puniyu_types::element::send::{TextElement, AtElement};
/// let msg = message!(
///     AtElement::new("123456"),
///     TextElement::new("hello"),
/// );
/// ```
///
#[cfg(feature = "element")]
#[macro_export]
macro_rules! message {
    ($single:expr) => {
        $crate::element::Message::from($single)
    };
    ($first:expr, $($rest:expr),+ $(,)?) => {
        $crate::element::Message::from(vec![$first, $($rest),+])
    };
}
