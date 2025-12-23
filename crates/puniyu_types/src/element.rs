mod message;
pub use message::Message;
pub mod receive;
mod segment;
pub mod send;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 原始消息
pub trait RawMessage: Send + Sync {
	fn raw(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, EnumString, Display, IntoStaticStr, Deserialize, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum ElementType {
	#[strum(serialize = "at")]
	/// 艾特元素
	At,
	#[strum(serialize = "reply")]
	/// 回复元素
	Reply,
	#[strum(serialize = "text")]
	/// 文本元素
	Text,
	#[strum(serialize = "image")]
	/// 图片元素
	Image,
	#[strum(serialize = "file")]
	/// 文件元素
	File,
	#[strum(serialize = "record")]
	/// 语音元素
	Record,
	#[strum(serialize = "video")]
	/// 视频元素
	Video,
	#[strum(serialize = "face")]
	/// 表情元素
	Face,
	#[strum(serialize = "json")]
	/// json元素
	Json,
	#[strum(serialize = "xml")]
	//// xml元素
	Xml,
}
