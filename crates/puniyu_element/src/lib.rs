mod element;
pub use element::*;
mod message;
pub use message::Message;
mod segment;
pub use segment::Segment;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

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
	#[strum(serialize = "reply")]
	Reply,
	#[strum(serialize = "text")]
	Text,
	#[strum(serialize = "image")]
	Image,
	#[strum(serialize = "file")]
	File,
	#[strum(serialize = "record")]
	Record,
	#[strum(serialize = "video")]
	Video,
	#[strum(serialize = "face")]
	Face,
	#[strum(serialize = "json")]
	Json,
	#[strum(serialize = "xml")]
	Xml,
}
