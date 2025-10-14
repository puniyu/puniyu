use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

pub mod receive;
pub mod send;

/// 消息段
pub trait Element: RawMessage + Send + Sync {
	fn element_type(&self) -> &str;
}

/// 消息日志
pub trait RawMessage: Send + Sync {
	fn raw(&self) -> String;
}

/// 消息文本
pub trait TextMessage: Send + Sync {
	fn text(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, EnumString, Display, IntoStaticStr, Deserialize, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum ElementType {
	#[strum(serialize = "at")]
	At,
	#[strum(serialize = "text")]
	Text,
	#[strum(serialize = "image")]
	Image,
	#[strum(serialize = "file")]
	File,
}
